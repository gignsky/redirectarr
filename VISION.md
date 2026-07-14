# redirectarr — Vision & Roadmap

> The friendly front door for every domain you own.

This document is the durable source of truth for what redirectarr is, who it's
for, and the order we build it in. GitHub issues are written against it. It will
evolve — treat it as living, not final.

---

## North star

A single self-hosted, [Nix](https://nixos.org/)-packaged service that you point
**every domain you own** at. For each incoming request it decides — per host,
per path — whether the visitor sees a delightful **waiting page**, gets
**redirected**, or is routed to your **real site**.

Configuration lives in version-controlled files (GitOps). redirectarr starts as
a personal homelab tool, stays open-source, and is architected with a clean
"open-core" seam so a management dashboard / hosted tier could later become a
paid product.

**The name fits the goal:** it's a member of the self-hosted *-arr family — a
service you run, not a site generator.

---

## Who it's for

1. **Me, first.** It has to be great for managing my own domains before anything
   else.
2. **The self-hosted / \*arr community**, via open source — good docs, sane
   defaults, NixOS + Docker support.
3. **Maybe a business, eventually.** If a hosted or "pro" tier can fund the work,
   great. We keep that door open by not hard-coupling the core to anything that
   would block multi-tenancy or a paid dashboard later.

---

## Design principles

1. **Config-as-code first.** TOML/YAML files are the source of truth,
   hot-reloaded. GitOps-friendly and NixOS-native. A dashboard is a *later*
   surface, never a dependency.
2. **Personality is a feature.** Multiple "vibes" ship as themes and **each
   domain picks its own**. Easter eggs and delight are first-class, not a coat
   of paint.
3. **One self-contained binary.** Embedded assets, reverse-proxy friendly,
   trivial to deploy.
4. **Open-core seam.** Keep a clean boundary between the free self-hosted core
   and anything that could become paid — don't hard-couple.
5. **State only when earned.** The core is stateless; storage (SQLite) is
   introduced only when a feature (analytics) genuinely needs it, behind a clear
   boundary.

---

## The mental model

```
request → match Host → match path rule → resolve ACTION
```

**Actions:**

| Action           | Behavior                                                                 |
| ---------------- | ------------------------------------------------------------------------ |
| `wait`           | Render a themed waiting page for the domain.                             |
| `redirect`       | Issue a 3xx to a target (status configurable, path-preserving optional). |
| `health-fallback`| If the real backend is up, route to it; if it's down, show a status/waiting page. |
| `shortlink`      | *(later)* Map a short path to a long URL.                                |

Each **domain** carries: a vibe/theme, branding, an optional launch date, and a
set of **path rules** with a default action.

Illustrative config (shape, not final):

```toml
[server]
bind = "0.0.0.0:8080"

[default]                      # fallback for unknown hosts
mode = "wait"
theme = "coming-soon"

[[domains]]
host = "coolproject.net"
theme = "terminal"
title = "Cool Project"
launch = "2026-12-01"          # drives a countdown
[[domains.rules]]
path = "/blog"
mode = "redirect"
target = "https://blog.coolproject.net"
status = 301

[[domains]]
host = "app.example.com"
mode = "health-fallback"
backend = "http://10.0.0.5:3000"
theme = "status"
```

---

## The three surfaces

redirectarr has three distinct front-ends. They serve different audiences and
use different tech.

1. **Marketing site — sells the product.**
   Lives at `redirectarr.net`. Feature showcase, live theme demos, install
   instructions, docs, "get it on GitHub." A **separate static site
   (Astro/11ty)** built in this monorepo, deployed to Cloudflare Pages. The
   current WIP placeholder in `site/` is the seed of this.

2. **Served pages — the product in action.**
   What the redirectarr binary renders for each domain: themed waiting pages,
   redirects, health/status pages. This is the **core engine's output** and
   where the per-domain personality lives.

3. **Management / admin dashboard — how I run my instance.**
   Starts as config files (GitOps). Later grows into a private admin dashboard
   for managing domains without editing files by hand. This is the most likely
   **paid surface**.

> The "two versions" framing: **surface #1 sells it**; my personal instance is
> **#2 (the live pages) + eventually #3 (the dashboard)**.

A nice bit of leverage: the marketing homepage can partly **dogfood** the engine
by cycling through the real vibes as a live demo — while docs/blog stay in the
static site.

---

## Repository layout (monorepo)

Everything lives in `gignsky/redirectarr` under subdirectories, sharing one Nix
flake:

```
redirectarr/
├── flake.nix, nix/           # Nix packaging & dev shell (exists today)
├── src/ (or crates/)         # Rust core engine + binary
├── themes/                   # built-in waiting-page vibes/themes
├── site/                     # → migrates to a web/ Astro marketing site
├── docs/                     # documentation
├── dashboard/                # (later) admin dashboard
└── VISION.md                 # this document
```

---

## Roadmap (phases)

Phases are milestones, not deadlines. Issues are cut from these.

### P0 · Foundations
Turn the Nix scaffold into a real project.
- Scaffold the Rust (axum) crate; wire it into the existing flake (crane).
- Minimal server: `--config`, `--help`, bind address, `/healthz`.
- CI (rewrite the commented-out nix workflow) + rustfmt in treefmt + release.

### P1 · Core front door — *first win*
- Host + path routing → action resolver.
- Config loading + **hot reload**.
- First waiting-page theme (port the WIP page into it).
- `redirect` action; default fallback for unknown hosts.
- Structured access logging.
- **🏁 First win: `redirectarr.net` served by the binary running on my infra.**

### P2 · Personality & theming
- Multiple built-in vibes (terminal / whimsical / sleek …).
- Per-domain theme selection + branding (logo, colors, links, copy).
- Launch countdown.
- Easter-egg framework (konami code, terminal mode, rotating facts).

### P3 · Health-based fallback *(priority feature)*
- Probe real backends; auto-switch live ↔ waiting/status pages.
- Status page theme.

### P4 · Analytics *(priority feature)*
- Introduce SQLite storage layer (first stateful feature).
- Privacy-friendly per-domain hit counts + referrers.

### P5 · Ops & polish
- NixOS module (systemd service).
- OCI image output from Nix.
- Prometheus metrics.
- Docs site build-out.

### P6+ · Monetization exploration
- Admin dashboard.
- Multi-tenant / hosted offering.

---

## Proposed technical defaults (open to change)

These are working defaults, not final commitments — revisit as we build:

- **Language / framework:** Rust + [axum](https://github.com/tokio-rs/axum).
- **Templating:** runtime templates (Tera / minijinja) so themes can be added
  without recompiling. *(Alternative: compile-time Askama.)*
- **Assets:** embedded in the binary (rust-embed) for a self-contained package.
- **TLS:** terminated upstream (Caddy / Traefik / Cloudflare Tunnel); redirectarr
  speaks plain HTTP behind a reverse proxy.

---

## Deliberately deferred questions

To revisit when the relevant phase arrives — not now:

- **Health-fallback depth:** full reverse-proxy passthrough vs. probe-and-switch-page?
- **Analytics:** built-in SQLite vs. pluggable (Plausible/umami) — or both?
- **The paid line:** exactly which capabilities sit behind a future pro/hosted tier?
