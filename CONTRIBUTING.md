# Contributing to SoulSync

Thanks for wanting to help. Bug reports, ideas and pull requests are all welcome.

## Reporting a bug

Open an [issue](https://github.com/hildlabs/SoulSync/issues/new/choose). The app can collect what helps: *Settings › Report a problem* puts the logs into one zip file, which you can attach. It contains no ROM and no save. Please say which game and language you play and whether you play solo or with others.

## Building

You need [Rust](https://rustup.rs) (stable).

```sh
cargo build --release
cargo test
cargo run -p soulsync-server   # then open http://localhost:3000/demo
```

The demo plays a scripted run, so most dashboard work needs no game at all. `/demo?players=1` shows a solo run. The desktop app builds on Windows (`cargo run -p soulsync-app`); CI builds it for every push to `main`.

## Where things live

| Path | What |
| --- | --- |
| `crates/soulsync-core` | Save and ROM parsing, rules, the randomizer port (`rando/`) |
| `crates/soulsync-server` | Run server; the whole dashboard is `web/index.html`, no build step |
| `crates/soulsync-app` | Desktop app, setup, DeSmuME Lua script |
| `crates/soulsync-cli` | The `soulsync` command-line tool |
| `docs/design.md` | Design rules for the dashboard: tokens, components, motion |

## How it fits together

```
DeSmuME + soulsync.lua  ──file──▶  desktop app  ──WebSocket──▶  run server  ──▶  dashboard (browser or app window)
   (reads game memory)            (also reads save + ROM)        (one run, all players)
```

- `crates/soulsync-app/src/soulsync.lua` runs inside DeSmuME. It copies the party, the player's position and the Pokémon met in battle to a file. The format is described at the top of `crates/soulsync-core/src/live.rs`.
- The app reads that file, the save and the player's own ROM (names, catch rates, level caps), turns them into a snapshot and sends it to the run server.
- The server keeps the run, works out links and rule breaks, and pushes the state to every dashboard. Without a server address the app runs the same server locally.
- The dashboard is one HTML file, `crates/soulsync-server/web/index.html`. `web/demo.js` feeds it a scripted run at `/demo`.

## Good places to start

Look for issues labelled [**good first issue**](https://github.com/hildlabs/SoulSync/labels/good%20first%20issue) or [**help wanted**](https://github.com/hildlabs/SoulSync/labels/help%20wanted). Each one says which files to look at and how to check your change. Many need no copy of the game at all: the demo and the tests are enough.

- **Translations:** dashboard text is in the `T` object in `web/index.html`, command-line text in `crates/soulsync-cli/src/i18n.rs` (English, German and Spanish so far).
- **Testing:** play a game or language that has not been tested yet and report what works and what does not. No code needed.
- **Dashboard polish**, following `docs/design.md`.
- Anything on the [roadmap](docs/ROADMAP.md). For bigger features, open an issue first so we can agree on the approach before you spend time on it.

## Sending a pull request

1. Fork the repository and create a branch from `main`.
2. Make your change. Keep a pull request to one topic; small is easier to review.
3. Run `cargo fmt --all`, `cargo clippy --all-targets -- -D warnings` and `cargo test --all`. CI runs the same three on every pull request.
4. For dashboard changes, check `/demo` and `/demo?players=1` in light and dark mode, and add a screenshot to the pull request.
5. Open the pull request and say what you changed and how you checked it. If it fixes an issue, write `Fixes #123`.

Not sure about something? Ask in the issue, open a draft pull request early, or come to the [Discord](https://discord.gg/DKhuhJS8tJ).

## Rules

- **No game assets in the repository:** no ROMs, saves, RAM dumps, sprites or other Nintendo graphics. Data has to come from the player's own ROM at runtime. Never link to places to download ROMs, in code, docs or issues.
- The randomizer port must stay byte-identical to Universal Pokémon Randomizer ZX for the same settings and seed. If you change it, check against the original.
- A change players will notice gets a line under **Unreleased** in [CHANGELOG.md](CHANGELOG.md); releases turn it into the patch notes on GitHub and Discord.
- Code, comments and docs are in English. `cargo fmt` and `cargo clippy` should be clean.
- By contributing you agree that your work is licensed under GPL-3.0-or-later, like the rest of the project.
