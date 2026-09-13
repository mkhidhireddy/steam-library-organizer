# Steam Library Organizer

A local-first Windows desktop app for turning a large Steam backlog into useful, native Steam collections. It imports owned games, enriches them with Steam store genres and categories, detects recognizable IPs such as Warhammer and Star Wars, and writes only the collections that it manages.

## What it does

- Imports your owned library through Steam's `GetOwnedGames` Web API.
- Keeps the API key only in memory for the import; it is never written to disk.
- Caches the imported library and collection rules on this computer.
- Suggests IP labels from game titles and lets you approve or reject them.
- Builds smart collections from a store tag/category and/or an IP.
- Previews collection size before export.
- Writes app-managed collections into Steam's native Library collections.
- Refuses to export while Steam is running and backs up Steam's collection file first.

## Install and use

1. Download and run the Windows installer from the [latest GitHub release](https://github.com/mkhidhireddy/steam-library-organizer/releases/latest).
2. Open the app and select **Use current Steam account**. The app detects the Steam account installed on this PC and fills in the SteamID64.
3. Select **Get API key from Steam**, register a personal key on Valve's page, paste it into the app, and select **Import Steam Library**. The Steam profile's game details must be public for the API to return the library.
4. Select **Refresh store tags**. Large libraries take time because Steam metadata is fetched per game.
5. Review detected IPs under **IP Review**.
6. Open **Collections** and add the suggested Warhammer or Star Wars collections with one click, or create a custom rule. Then inspect them under **Export**.
7. Fully exit Steam, including its system-tray process, and select **Apply to Steam**.
8. Reopen Steam. The generated collections appear in the Steam Library alongside existing collections.

The exporter preserves unrelated collection records. Every successful export creates a timestamped `.backup-<timestamp>.json` file beside Steam's cloud collection file.

## Privacy and safety

The application has no analytics, advertising, account service, or developer-operated server. Your Steam ID, cached games, IP decisions, and smart collection definitions stay on this computer. The API key is cleared from the interface after import and is not stored. See [Privacy and data](docs/privacy.md) for details.

Steam does not provide a public collection-management API. This app uses a guarded adapter for the observed Steam cloud collection format. If Steam changes that format, export fails rather than attempting an unsafe edit. Keep the generated backups until you have confirmed the collections in Steam.

## Development

Prerequisites are Node.js 22+, pnpm, Rust, and the Visual Studio 2022 C++ build tools.

```powershell
pnpm install --frozen-lockfile
pnpm lint
pnpm typecheck
pnpm test
cargo test --manifest-path src-tauri/Cargo.toml
pnpm tauri build
```

Rust commands on Windows must run in a Visual Studio Developer PowerShell or Developer Command Prompt.
