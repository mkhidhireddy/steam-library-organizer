# Steam Library Organizer Design

## Goal

Create a local-first Windows desktop application that imports a complete Steam library, enriches it with store tags, suggests reviewable top-level IP groups (for example Warhammer and Star Wars), evaluates smart collections, and safely exports app-managed collections into Steam's Library UI.

## Agreed scope

- Windows-only MVP using Tauri 2, React, TypeScript, Rust, and SQLite.
- Full owned-library import through Steam's documented owned-games API.
- Store tags are cached locally and refreshed incrementally.
- IP suggestions are automatic but require user approval; explicit approvals and rejections override future suggestions.
- IPs are flat top-level labels, not a franchise hierarchy.
- Smart collections use include, any, and exclude rules over tags, IP, and library fields.
- Export is explicit and one-way. It previews changes, requires Steam to be closed, creates a backup, validates output, and restores automatically on failure.

## Boundaries

Steam exposes no documented public API to manage Library Collections. The exporter is therefore isolated behind a versioned adapter, with sanitized fixtures, read/parse verification, minimal mutations, and an unsupported-format safe failure. It must never silently change unrelated Steam collections.

## Main areas

1. Library search and filtering.
2. IP suggestion review and manual assignment.
3. Smart collection rule authoring and match explanations.
4. Steam export preview and guarded application.
5. Settings, diagnostics, and backup restoration.

## Success criteria

The MVP imports a real full library, displays cached store tags, preserves IP decisions, produces repeatable smart collections, previews accurate native Steam changes, and creates/restores managed Steam collections without altering unrelated collections.
