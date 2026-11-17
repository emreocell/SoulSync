// Takes the README screenshots from the dashboard's demo. Sprites come from
// the server's sprite source, as in the app.
//
//   cargo run -p soulsync-server &
//   node docs/screenshots/take.js        # needs Playwright and ImageMagick
//
// Set SOULSYNC_URL if the server is not on http://localhost:3000, and
// SOULSYNC_PLACEHOLDERS=1 to draw original placeholder figures instead.
const { chromium } = require("playwright");
const { execFileSync } = require("child_process");
const path = require("path");
const URL = (process.env.SOULSYNC_URL || "http://localhost:3000") + "/demo";
const OUT = __dirname;

const ident = () => {
  // Original placeholder creatures: a mirrored pixel pattern per Dex number.
  window.SOULSYNC_SPRITES = id => {
    let s = (Number(id) * 2654435761) >>> 0;
    const rnd = () => (s = (s * 1103515245 + 12345) >>> 0) / 4294967296;
    const hue = (Number(id) * 47) % 360;
    const body = `hsl(${hue} 62% 58%)`, dark = `hsl(${hue} 55% 32%)`, light = `hsl(${hue} 80% 80%)`;
    const N = 10, cells = [];
    for (let y = 1; y < 9; y++) for (let x = 0; x < 5; x++) {
      const d = Math.hypot(x - 4.5, (y - 4.8) * 1.05);
      if (d < 3.2 + rnd() * 1.6 && rnd() > 0.12) cells.push([x, y, rnd() < .18 ? light : body]);
    }
    let r = "";
    const add = (x, y, c) => r += `<rect x="${x}" y="${y}" width="1.02" height="1.02" fill="${c}"/>`;
    for (const [x, y, c] of cells) { add(x, y, c); add(N - 1 - x, y, c); }
    // outline-free eyes
    add(3, 4, dark); add(6, 4, dark); add(3, 3, "#fff"); add(6, 3, "#fff");
    add(3, 4, dark); add(6, 4, dark);
    const svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="-1 -1 12 12" shape-rendering="crispEdges">${r}</svg>`;
    return "data:image/svg+xml," + encodeURIComponent(svg);
  };
};


async function open(browser, colorScheme, height = 900) {
  const page = await browser.newPage({ viewport: { width: 1440, height }, deviceScaleFactor: 2, colorScheme });
  if (process.env.SOULSYNC_PLACEHOLDERS) await page.addInitScript(ident);
  await page.goto(URL);
  return page;
}

function save(png, name) {
  execFileSync("convert", [png, "-resize", "2000x", "-quality", "86", "-define", "webp:method=6", path.join(OUT, name + ".webp")]);
}

(async () => {
  const browser = await chromium.launch();
  const tmp = name => path.join(require("os").tmpdir(), `soulsync-${name}.png`);
  const shot = async (page, name) => { await page.screenshot({ path: tmp(name) }); save(tmp(name), name); };

  // The demo's scripted battle, both players at once: a taller window so
  // both cards with their catch chance fit.
  const fight = await open(browser, "light", 1180);
  await fight.locator(".wild2").first().waitFor({ timeout: 60000 });
  await fight.waitForTimeout(2900);
  await shot(fight, "battle");
  await fight.close();

  const page = await open(browser, "light");
  await page.locator(".wild2").first().waitFor({ timeout: 60000 });
  // The end of the script: every link made, one lost.
  await page.waitForTimeout(43000);
  await shot(page, "overview");
  for (const [tab, name] of [["Team", "team"], ["Encounters", "encounters"]]) {
    await page.getByRole("button", { name: tab }).first().click();
    await page.waitForTimeout(800);
    await shot(page, name);
  }

  const dark = await open(browser, "dark");
  await dark.waitForTimeout(64000);
  await shot(dark, "overview-dark");
  await browser.close();
})();
