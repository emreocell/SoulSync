# Roadmap

SoulSync reads game state **from the outside**, through save files and the DeSmuME Lua script, instead of reimplementing or forking an emulator. Each step below is useful on its own. Ideas and help are welcome in the [issues](https://github.com/hildlabs/SoulSync/issues).

## Done

- Generation 5 save parser and CLI (`soulsync inspect`, `watch`, `sync`)
- Run server with run codes, WebSocket sync, persistent runs and limits for a public instance
- Live tracking from DeSmuME: party, wild encounters, battles with catch chance, bag
- Rule engine: links, shared fate, level caps from the ROM, failed catches
- Desktop app for Windows with one-click updates; solo and two to four players
- Built-in randomizer: a Rust port of Universal Pokémon Randomizer ZX 4.6.1 for Black 2 and White 2, byte-identical to the original
- One-click new attempts with a fresh save and Rare Candies
- Run end, archive and real time played

## Next

- A code-signed Windows build
- OBS overlay and a read-only viewer link for streamers
- Testing other game languages, then turning the native randomizer on for them (until then the original randomizer is downloaded as a fallback)
- Black and White tested end to end
- A tested macOS build (see [mac.md](mac.md))

## Later

- More games: Platinum, HeartGold and SoulSilver
- More emulators: melonDS, BizHawk
- Chat bot for streams
- Opt-in leaderboards (last, since randomized runs are hard to compare)
