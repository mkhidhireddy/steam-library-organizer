# Steam Library Organizer Implementation Plan

> **For Agent:** REQUIRED: Read and follow `executing-plans/SKILL.md` task-by-task.

**Goal:** Deliver a safe Windows Steam library organizer that can export app-managed native Steam collections.

**Architecture:** React/TypeScript calls a narrow typed Tauri command layer. Rust owns Steam access, storage, rules, backups, and export. The renderer never receives the API key after it is stored.

**Tech Stack:** Tauri 2, React, TypeScript, Vite, Rust, SQLite/sqlx, Windows Credential Manager, Vitest, Testing Library, Cargo tests, WebdriverIO, pnpm.

**Skills to Reference:** `superpowers:executing-plans`, `superpowers:test-driven-development`, `frontend-design`, `superpowers:systematic-debugging`, `superpowers:verification-before-completion`.

---

## Task 1: Project foundation — complete

- Scaffold Tauri React/TypeScript application.
- Add Vitest, Testing Library, ESLint, Prettier, and quality scripts.
- Add a test-first app heading.
- Verify lint, typecheck, frontend tests, Rust tests, and formatting.
- Commit: `chore: scaffold Tauri desktop application`.

## Task 2: Steam collection adapter feasibility gate

**Files:** `docs/steam-collection-format.md`, `src-tauri/src/steam/{mod.rs,discovery.rs,collections/{mod.rs,model.rs,parser.rs}}`, `src-tauri/tests/{collection_parser.rs,fixtures/steam/*}`.

1. Close Steam and copy the smallest relevant portion of the active collection configuration into a sanitized fixture; record the client build, source pattern, schema keys, and sanitization method.
2. Write a failing parser test for named collections and AppID memberships.
3. Implement typed read-only parsing that preserves unknown document nodes.
4. Test discovery for explicit Steam roots, missing userdata, multiple accounts, and configured-account selection. Never guess between accounts.
5. Run Cargo tests and commit only after the fixture contains no SteamID, account name, or private collection data.

## Task 3: SQLite domain storage

Create migrations and typed repository modules for games, store tags, IP assignments/suggestions/rejections, smart collections/rules, export mappings, sync runs, and backups. Test round trips against in-memory SQLite. Reconciliation must preserve user decisions.

## Task 4: Secure setup

Store SteamID in SQLite and the API key in Windows Credential Manager through a secret-store abstraction. Return only `has_api_key`, never the key. Add setup UI tests using mocked Tauri IPC.

## Task 5: Owned-game import

Implement a mocked HTTP-contract-tested Steam client and transactional reconciliation. Handle credential errors, privacy, rate limiting, timeouts, malformed data, and retained local decisions.

## Task 6: Cached store metadata

Use a provider abstraction with cache freshness, bounded concurrency, retry/backoff, and preserved last successful metadata. Test all failure states without live Steam calls.

## Task 7: IP classifier

Load an editable alias catalog. Use deterministic weighted title/publisher/description matching, confidence/evidence, explicit approvals, and remembered rejections. No external AI service.

## Task 8: Rule engine

Build a pure, table-tested evaluator for all/any/exclude tags, IP, and installed status. Return structured explanations. Persist rules and expose collection CRUD/preview commands.

## Task 9: Library UI

Build a responsive, accessible search/filter/sort library interface with loading, empty, cached-warning, and failure states. Test user-visible behavior.

## Task 10: IP review and collection editor

Implement batch IP review, manual overrides, an accessible rule builder, live debounced match previews, unsaved-change protection, and tested save/delete flows.

## Task 11: Safe Steam mutation

Implement a diff, backup, temporary-file validation, atomic replacement, post-write verification, and automatic rollback. Only stable app-owned mappings may change; unknown schemas and a running Steam process block export.

## Task 12: Export and backups UI

Require a preview revision for apply. Explain additions/removals and unrelated-collection preservation. Provide typed blocker messages, progress, backup list, and restore confirmation. Prove a two-game controlled export and restore manually.

## Task 13: End-to-end tests and packaging

Add deterministic fixture mode, WebdriverIO workflow coverage, redacted diagnostics, user/privacy documentation, and an NSIS Windows installer. Build and test without accessing live Steam.

## Final verification

Run from a clean checkout:

```powershell
pnpm install --frozen-lockfile
pnpm lint
pnpm typecheck
pnpm test
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml
pnpm tauri build --debug
pnpm wdio run e2e/wdio.conf.ts
pnpm tauri build
```

Do not ship if the installed Steam collection format differs from the supported fixture or rollback has not been proven for that format.
