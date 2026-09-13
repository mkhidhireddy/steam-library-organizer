import { useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";
import "./App.css";

type Game = { appId: number; name: string; playtimeMinutes: number; installed: boolean; tags: string[]; approvedIp?: string | null };
type SmartCollection = { name: string; tag: string; ip: string };
type View = "library" | "ips" | "collections" | "export";

const detectIp = (name: string) => {
  const value = name.toLowerCase();
  if (value.includes("warhammer") || value.includes("vermintide")) return "Warhammer";
  if (value.includes("star wars") || value.includes("jedi")) return "Star Wars";
  return "";
};

function App() {
  const [view, setView] = useState<View>("library");
  const [steamId, setSteamId] = useState(() => localStorage.getItem("steam-id") ?? "");
  const [apiKey, setApiKey] = useState("");
  const [games, setGames] = useState<Game[]>(() => JSON.parse(localStorage.getItem("games") ?? "[]"));
  const [collections, setCollections] = useState<SmartCollection[]>(() => JSON.parse(localStorage.getItem("collections") ?? "[]"));
  const [name, setName] = useState("");
  const [tag, setTag] = useState("");
  const [ip, setIp] = useState("");
  const [search, setSearch] = useState("");
  const [busy, setBusy] = useState("");
  const [notice, setNotice] = useState("");

  const saveGames = (next: Game[]) => { setGames(next); localStorage.setItem("games", JSON.stringify(next)); };
  const saveCollections = (next: SmartCollection[]) => { setCollections(next); localStorage.setItem("collections", JSON.stringify(next)); };
  const filtered = games.filter((game) => game.name.toLowerCase().includes(search.toLowerCase()));
  const suggestions = games.filter((game) => !game.approvedIp && detectIp(game.name));
  const matches = (collection: SmartCollection) => games.filter((game) => (!collection.tag || game.tags.some((value) => value.toLowerCase() === collection.tag.toLowerCase())) && (!collection.ip || (game.approvedIp || detectIp(game.name)) === collection.ip));
  const tags = useMemo(() => [...new Set(games.flatMap((game) => game.tags))].sort(), [games]);

  async function importLibrary() {
    setBusy("Importing library…"); setNotice("");
    try {
      const imported = await invoke<Game[]>("sync_library", { steamId, apiKey });
      saveGames(imported); localStorage.setItem("steam-id", steamId); setApiKey("");
      setNotice(`Imported ${imported.length} games. Your API key was not stored.`);
    } catch (error) { setNotice(String(error)); } finally { setBusy(""); }
  }

  async function detectSteamAccount() {
    setBusy("Finding your current Steam account…"); setNotice("");
    try {
      const detected = await invoke<string>("detect_steam_account");
      setSteamId(detected); localStorage.setItem("steam-id", detected);
      setNotice("Current Steam account found. Now paste your Web API key and import.");
    } catch (error) { setNotice(String(error)); } finally { setBusy(""); }
  }

  async function refreshTags() {
    setBusy("Fetching Steam store metadata…"); setNotice("");
    try { const enriched = await invoke<Game[]>("refresh_metadata", { games }); saveGames(enriched); setNotice("Store metadata refreshed."); }
    catch (error) { setNotice(String(error)); } finally { setBusy(""); }
  }

  function approve(game: Game, approvedIp: string) { saveGames(games.map((item) => item.appId === game.appId ? { ...item, approvedIp } : item)); }
  function addCollection() { if (!name.trim()) return; saveCollections([...collections.filter((item) => item.name !== name.trim()), { name: name.trim(), tag, ip }]); setName(""); setTag(""); setIp(""); }
  async function preview(collection: SmartCollection, apply = false) {
    const appIds = matches(collection).map((game) => game.appId); setBusy(apply ? "Applying to Steam…" : "Preparing preview…");
    try {
      const command = apply ? "apply_export" : "preview_export";
      const result = await invoke(command, { name: collection.name, appIds });
      setNotice(apply ? `Applied ${appIds.length} games. Backup: ${result}` : `${collection.name}: ${appIds.length} games will be written. Close Steam before applying.`);
    } catch (error) { setNotice(String(error)); } finally { setBusy(""); }
  }

  return <main className="app-shell">
    <aside><p className="eyebrow">STEAM / CURATED</p><h1>Steam Library <span>Organizer</span></h1><nav>{(["library","ips","collections","export"] as View[]).map((item) => <button className={view === item ? "active" : ""} onClick={() => setView(item)} key={item}>{item === "ips" ? "IP Review" : item[0].toUpperCase() + item.slice(1)}</button>)}</nav><div className="status"><strong>{games.length}</strong><span>owned games</span><strong>{collections.length}</strong><span>smart collections</span></div></aside>
    <section className="workspace">
      {notice && <div className="notice" role="status">{notice}</div>}{busy && <div className="busy">{busy}</div>}
      {view === "library" && <><header className="page-head"><div><p className="kicker">THE SHELF</p><h2>Your Library</h2></div><button onClick={refreshTags} disabled={!games.length || !!busy}>Refresh store tags</button></header>
        <section className="connection"><div className="credential"><label>SteamID64<input value={steamId} onChange={(event) => setSteamId(event.target.value)} placeholder="7656119…"/></label><button className="quiet" onClick={detectSteamAccount} disabled={!!busy}>Use current Steam account</button></div><div className="credential"><label>Web API key<input type="password" value={apiKey} onChange={(event) => setApiKey(event.target.value)} placeholder="Not stored"/></label><a href="https://steamcommunity.com/dev/apikey" onClick={(event) => { event.preventDefault(); void openUrl("https://steamcommunity.com/dev/apikey"); }}>Get API key from Steam ↗</a></div><button onClick={importLibrary} disabled={!!busy}>Import Steam Library</button></section>
        <input className="search" aria-label="Search games" value={search} onChange={(event) => setSearch(event.target.value)} placeholder="Search your library…"/>
        <div className="game-grid">{filtered.map((game) => <article className="game" key={game.appId}><img src={`https://cdn.cloudflare.steamstatic.com/steam/apps/${game.appId}/header.jpg`} alt=""/><div><p className="playtime">{Math.round(game.playtimeMinutes / 60)}h played</p><h3>{game.name}</h3><div className="chips">{game.tags.slice(0,4).map((value) => <span key={value}>{value}</span>)}{(game.approvedIp || detectIp(game.name)) && <span className="ip">{game.approvedIp || detectIp(game.name)}</span>}</div></div></article>)}</div>{!games.length && <div className="empty"><h3>Bring your backlog into focus.</h3><p>Enter your Steam details above to import every owned game.</p></div>}</>}
      {view === "ips" && <><header className="page-head"><div><p className="kicker">FRANCHISE SIGNALS</p><h2>IP Review</h2></div><span>{suggestions.length} suggestions</span></header><div className="review-list">{suggestions.map((game) => <article key={game.appId}><div><h3>{game.name}</h3><p>Suggested from title evidence</p></div><b>{detectIp(game.name)}</b><button onClick={() => approve(game, detectIp(game.name))}>Approve</button><button className="quiet" onClick={() => approve(game, "None")}>Reject</button></article>)}</div>{!suggestions.length && <div className="empty"><h3>Review queue clear.</h3><p>Import games to generate franchise suggestions.</p></div>}</>}
      {view === "collections" && <><header className="page-head"><div><p className="kicker">RULE-DRIVEN</p><h2>Smart Collections</h2></div></header><section className="builder"><label>Collection name<input value={name} onChange={(event) => setName(event.target.value)} placeholder="Co-op RPGs"/></label><label>Required store tag<select value={tag} onChange={(event) => setTag(event.target.value)}><option value="">Any tag</option>{tags.map((value) => <option key={value}>{value}</option>)}</select></label><label>IP<select value={ip} onChange={(event) => setIp(event.target.value)}><option value="">Any IP</option><option>Warhammer</option><option>Star Wars</option></select></label><button onClick={addCollection}>Save collection</button></section><div className="collection-list">{collections.map((collection) => <article key={collection.name}><div><h3>{collection.name}</h3><p>{[collection.tag, collection.ip].filter(Boolean).join(" + ") || "All games"}</p></div><strong>{matches(collection).length}</strong><span>matches</span><button className="quiet" onClick={() => saveCollections(collections.filter((item) => item.name !== collection.name))}>Delete</button></article>)}</div></>}
      {view === "export" && <><header className="page-head"><div><p className="kicker">BACKUP-FIRST</p><h2>Export to Steam</h2></div></header><div className="safety">Steam must be fully closed. The app backs up and validates Steam's cloud collection file before replacing it; unrelated collections are preserved.</div><div className="collection-list">{collections.map((collection) => <article key={collection.name}><div><h3>{collection.name}</h3><p>{matches(collection).length} games ready</p></div><button className="quiet" onClick={() => preview(collection)}>Preview</button><button onClick={() => preview(collection, true)}>Apply to Steam</button></article>)}</div></>}
    </section>
  </main>;
}

export default App;
