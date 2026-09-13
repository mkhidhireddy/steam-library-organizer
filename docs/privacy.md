# Privacy and data

Steam Library Organizer is a local Windows application. It does not operate a remote account or developer-controlled backend and does not include telemetry.

## Data used

- **SteamID64:** saved in the app's local browser storage so it can be reused.
- **Steam Web API key:** held in memory only while importing and cleared immediately after the request completes successfully. It is not persisted by the app.
- **Owned games:** requested directly from Valve's Steam Web API and cached locally.
- **Store metadata and artwork:** requested directly from Valve/Steam endpoints.
- **IP reviews and collection rules:** stored locally on the computer.
- **Steam collections:** read from and, only after an explicit export, written to Steam's local cloud-storage collection file.

## Export safeguards

Export is blocked while `steam.exe` is running. Before changing the collection file, the app parses the existing document, changes only the stable record belonging to the selected app-managed collection, validates the result, and creates a timestamped backup. If replacement fails, it attempts to restore that backup automatically.

The backup is stored beside Steam's collection file. It can contain collection names and Steam AppIDs, so treat it as private account data.

## Removing local data

Uninstall the application to remove the executable. To clear cached UI data before uninstalling, use the WebView2 site-data controls for the application. Steam collection records already exported to Steam remain in Steam until deleted there. Timestamped collection backups remain in Steam's cloud-storage directory until you delete them manually.
