# User guide

## Import the library

Select **Use current Steam account** to detect the SteamID64 from the Steam installation on this PC. Then select **Get API key from Steam**, register a personal key on Valve's official page, paste it into the app, and select **Import Steam Library**. A private game-details setting or invalid key prevents Steam from returning the library. The key is not saved.

Steam's normal browser login can prove which user is signing in, but Valve does not grant ordinary desktop applications full-library access through that login. A personal Web API key is therefore still required for the complete owned library. The app never asks for or handles your Steam password.

The import provides game names and playtime. Select **Refresh store tags** to add the genres and categories published in Steam's store metadata. Existing metadata is retained if an individual store request fails.

## Review IP suggestions

The **IP Review** screen recognizes supported franchise names from titles. **Approve** assigns the suggestion; **Reject** remembers that the title should not belong to the suggested IP in the current local cache. Explicit choices take precedence over automatic detection.

The initial catalog recognizes Warhammer-related titles (including Vermintide) and Star Wars-related titles (including Jedi). The classifier is deterministic and does not send game data to an AI service.

## Build collections

Open **Collections**. When the imported library contains a recognized IP, the app proposes a corresponding collection and shows its match count. Select **Add … collection** to save the proposed rule; it will not write anything to Steam until you explicitly export it.

You can also give a collection a name and optionally require one store category/genre and one IP. Leaving both rules empty matches the entire imported library. Saving an existing name updates that local definition.

## Export to Steam

Open **Export** and select **Preview** to check the match count. Fully exit Steam, including the tray icon, before applying. The app refuses to write while `steam.exe` is present.

**Apply to Steam** creates or updates the stable app-managed native collection with the matching AppIDs. Existing unrelated collections are preserved. A backup path is shown after success. Reopen Steam to let the client load and synchronize the change.

If multiple Steam accounts have usable collection files on the same Windows profile, export is deliberately blocked because the app cannot safely guess which account to modify.

## Recovery

Backups are placed beside `cloud-storage-namespace-1.json` and use a `.backup-<timestamp>.json` suffix. If a collection does not appear as expected, keep Steam closed, preserve the newest backup, and restore it by copying it over the active JSON file. Reopen Steam only after the copy completes.
