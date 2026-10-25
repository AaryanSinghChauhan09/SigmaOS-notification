# SigmaOS-notification

Notification daemon (d-Bus org.freedesktop.Notifications)

## SigmaOS multi-repository ecosystem

This repository is one of the per-component repositories of the SigmaOS
operating system, modelled on the Linux Mint & Omarchy Linux sub-repository
architecture (multiple focused repos instead of one monolithic repo).

Scaffold created. Source code, tests, CI, and releases are to be added
incrementally per component (see COMPONENTS.toml in the SigmaOS main repo).

## Principles

- **One main branch** — only `main` exists
- **Separation of concerns** — docs live in wiki, not monorepo
- **Modularity / Component isolation** — 150 repos (Linux Mint & Omarchy model)
- **Composition over inheritance** — sub-repos, not monolith
- **Discoverability** — manifest + READMEs
- **KISS / Gall's Law** — scaffold first, add code incrementally
- **Do-cracy** — everything possible done; rate-limit blocker honestly reported

## SigmaOS multi-repository ecosystem

- **Upstream inspirations**: - https://github.com/omacom/omapager
- **Engine**: notify
- **Language**: shell
- **Visibility**: public

