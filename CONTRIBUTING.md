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

## Good places to start

- Translations: the dashboard text is in `web/index.html` (English and German so far).
- Testing other game languages (French, Spanish, Italian, Japanese…) and reporting what does not match.
- Dashboard polish, following `docs/design.md`.
- Anything on the [roadmap](docs/ROADMAP.md).

## Rules

- **No game assets in the repository:** no ROMs, saves, RAM dumps, sprites or other Nintendo graphics. Data has to come from the player's own ROM at runtime. Never link to places to download ROMs, in code, docs or issues.
- The randomizer port must stay byte-identical to Universal Pokémon Randomizer ZX for the same settings and seed. If you change it, check against the original.
- Code, comments and docs are in English. `cargo fmt` and `cargo clippy` should be clean.
- By contributing you agree that your work is licensed under GPL-3.0-or-later, like the rest of the project.
