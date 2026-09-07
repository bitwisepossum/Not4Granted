# Not4Granted

Not4Granted is a small desktop application for tracking research grant applications and manuscripts.

The program is intended as a local, single-user tool for keeping track of grant deadlines, submission status, funding decisions, manuscript progress, and related notes.

The project is also used as a learning project for Rust, Tauri, Svelte, and SQLite.

## Planned features

* Track grant applications and their status
* Track requested and received funding
* Track deadlines and submission dates
* Track manuscripts and publication status
* Show simple statistics for submitted, accepted, rejected, and active applications
* Export function with filtering
* Local SQLite database
* No cloud sync or external account requirements

## TODO

### Grants

- [x] Add grants
- [x] Load grants from SQLite
- [x] Update grant details
- [x] Quick update grant status
- [ ] Delete grants
- [ ] View full grant details
- [ ] Filter and sort grants
- [ ] Search grants

### Manuscripts

- [x] Add manuscripts
- [x] Load manuscripts from SQLite
- [x] Update manuscript details
- [ ] Quick update manuscript status
- [ ] Delete manuscripts
- [ ] View full manuscript details
- [ ] Filter and sort manuscripts
- [ ] Search manuscripts

### Grant–manuscript relationships

- [ ] Link manuscripts to grants
- [ ] Remove grant–manuscript links
- [ ] Show linked manuscripts on grant details
- [ ] Show linked grants on manuscript details

### Dashboard

- [x] Show grant status summary
- [x] Show manuscript status summary
- [ ] Statistics view
- [ ] Show upcoming grant deadlines
- [ ] Show manuscript next actions

### Application

- [ ] Add database migrations
- [ ] Improve error handling and user feedback
- [ ] Add confirmation for destructive actions
- [ ] Add settings/preferences

## Stack

* Rust
* Tauri
* Svelte 5
* TypeScript
* SQLite

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
