#!/usr/bin/env python3
"""Patch notes for a release, and the post that announces it on Discord.

    release_notes.py notes VERSION
        Prints the "What's new" part of the release notes as Markdown.
        Source, first match wins:
          1. the section "## VERSION" in CHANGELOG.md
          2. the section "## Unreleased" in CHANGELOG.md, if it has entries
          3. the titles of the pull requests merged since the last v* tag,
             leaving out ones that only touch the README, CI or docs

    release_notes.py discord VERSION RELEASE_URL
        Posts the patch notes to the Discord webhook in DISCORD_WEBHOOK.
        DISCORD_ROLE (optional) is the id of a role to ping, such as
        "Release Pings". Without DISCORD_WEBHOOK it does nothing.

Only the Python standard library is used.
"""

import json
import os
import re
import subprocess
import sys
import urllib.error
import urllib.request

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
REPO = os.environ.get("GITHUB_REPOSITORY", "hildlabs/SoulSync")
HEADER = f"https://raw.githubusercontent.com/{REPO}/main/docs/discord/release-header.png"
ORANGE = 0xFF8D4D

# Pull requests whose title starts like this are left out of the fallback list.
SKIP = re.compile(r"^(readme|ci|docs?|demo|repo|chore|build|tests?)\b", re.I)


def changelog_section(name):
    path = os.path.join(ROOT, "CHANGELOG.md")
    if not os.path.exists(path):
        return ""
    with open(path, encoding="utf-8") as f:
        text = f.read()
    m = re.search(rf"^##\s+\[?v?{re.escape(name)}\]?[^\n]*\n(.*?)(?=^##\s|\Z)", text, re.M | re.S | re.I)
    if not m:
        return ""
    body = re.sub(r"<!--.*?-->", "", m.group(1), flags=re.S).strip()
    return body


def git(*args):
    return subprocess.run(["git", *args], cwd=ROOT, capture_output=True, text=True, encoding="utf-8").stdout.strip()


def merged_prs():
    last = git("describe", "--tags", "--abbrev=0", "--match", "v*", "HEAD")
    rng = f"{last}..HEAD" if last else "HEAD"
    out = []
    for block in git("log", rng, "--first-parent", "--format=%s%x1f%b%x1e").split("\x1e"):
        if not block.strip():
            continue
        subject, _, body = block.strip().partition("\x1f")
        m = re.match(r"Merge pull request #(\d+)", subject)
        if m:
            title = body.strip().splitlines()[0].strip() if body.strip() else ""
            num = m.group(1)
        else:
            m = re.match(r"(.*) \(#(\d+)\)$", subject)
            if not m:
                continue
            title, num = m.group(1), m.group(2)
        if title and not SKIP.match(title):
            out.append((title, num))
    return out


def notes(version):
    for name in (version, "Unreleased"):
        body = changelog_section(name)
        if body:
            return body
    prs = merged_prs()
    if not prs:
        return "Small fixes and polish."
    return "\n".join(f"- {t} ([#{n}](https://github.com/{REPO}/pull/{n}))" for t, n in prs)


def for_discord(md):
    # Discord shows no link previews inside posts; keep links short: "(#123)" instead of full URLs
    md = re.sub(r"\(\[#(\d+)\]\([^)]+\)\)", r"(#\1)", md)
    md = re.sub(r"^\s*[-*]\s+", "• ", md, flags=re.M)
    if len(md) > 3000:
        md = md[:3000].rsplit("\n", 1)[0] + "\n…and more in the full release notes."
    return md


def post(url, payload):
    req = urllib.request.Request(url, data=json.dumps(payload).encode("utf-8"), method="POST",
                                 headers={"Content-Type": "application/json",
                                          "User-Agent": "SoulSync release (https://github.com/hildlabs/SoulSync)"})
    with urllib.request.urlopen(req, timeout=30) as r:
        return r.status


def discord(version, release_url):
    hook = os.environ.get("DISCORD_WEBHOOK", "").strip()
    if not hook:
        print("No DISCORD_WEBHOOK secret, skipping the Discord post.")
        return
    role = os.environ.get("DISCORD_ROLE", "").strip()
    changes = for_discord(notes(version))
    download = f"https://github.com/{REPO}/releases/latest"
    title = f"# SoulSync {version} is out"
    intro = "Update from inside the app (top bar), or download it fresh:"
    ping = f"<@&{role}>" if role else ""
    mentions = {"parse": [], "roles": [role] if role else []}
    buttons = [
        {"type": 2, "style": 5, "label": "Download", "url": download, "emoji": {"name": "⬇️"}},
        {"type": 2, "style": 5, "label": "Full release notes", "url": release_url, "emoji": {"name": "📄"}},
    ]
    footer = "-# Windows may warn on the first start because the app isn't code-signed yet: More info › Run anyway."

    v2 = {
        "username": "SoulSync",
        "flags": 1 << 15,
        "allowed_mentions": mentions,
        "components": [{"type": 17, "accent_color": ORANGE, "components": [
            {"type": 12, "items": [{"media": {"url": HEADER}}]},
            {"type": 10, "content": f"{title}\n{intro}" + (f"\n{ping}" if ping else "")},
            {"type": 14, "divider": True, "spacing": 2},
            {"type": 10, "content": "## ✨ What's new\n" + changes},
            {"type": 14, "divider": True, "spacing": 2},
            {"type": 10, "content": footer},
            {"type": 1, "components": buttons},
        ]}],
    }
    try:
        post(hook + ("&" if "?" in hook else "?") + "with_components=true", v2)
        print("Posted to Discord.")
        return
    except urllib.error.HTTPError as e:
        print(f"New-style post refused ({e.code}: {e.read()[:300]!r}), sending an embed instead.")

    embed = {
        "username": "SoulSync",
        "content": ping or None,
        "allowed_mentions": mentions,
        "embeds": [{
            "color": ORANGE,
            "title": f"SoulSync {version} is out",
            "url": release_url,
            "description": f"{intro} [Download]({download})\n\n**✨ What's new**\n{changes}",
            "image": {"url": HEADER},
            "footer": {"text": "Not code-signed yet: on the first start choose More info › Run anyway."},
        }],
    }
    post(hook, embed)
    print("Posted to Discord.")


def main():
    if len(sys.argv) >= 3 and sys.argv[1] == "notes":
        sys.stdout.reconfigure(encoding="utf-8")
        print(notes(sys.argv[2]))
    elif len(sys.argv) >= 4 and sys.argv[1] == "discord":
        try:
            discord(sys.argv[2], sys.argv[3])
        except Exception as e:  # the release is out either way; never fail it over Discord
            print(f"::warning::Discord post failed: {e}")
    else:
        sys.exit(__doc__)


if __name__ == "__main__":
    main()
