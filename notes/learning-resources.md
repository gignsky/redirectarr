# Redirectarr Learning Resources

A curated collection of documentation, tutorials, and references for building redirectarr.

---

## RUST FUNDAMENTALS

### Official Documentation
- **The Rust Programming Language Book** (The Book)
  - URL: https://doc.rust-lang.org/book/
  - Focus: Chapters 12-20 for project structure, I/O, web server basics
  - Essential: Chapter 20 (Building a Multithreaded Web Server)

- **Rust By Example**
  - URL: https://doc.rust-lang.org/rust-by-example/
  - Great for quick syntax reference and patterns

- **The Cargo Book**
  - URL: https://doc.rust-lang.org/cargo/
  - Understanding Cargo.toml, dependencies, workspaces

- **Rust Standard Library Documentation**
  - URL: https://doc.rust-lang.org/std/
  - Reference for core types and traits

### Books (Recommended)
- **"Zero To Production In Rust"** by Luca Palmieri
  - URL: https://www.zero2prod.com/
  - Highly relevant: Building production web services in Rust
  - Covers async, testing, deployment, observability

- **"Programming Rust"** by Jim Blandy & Jason Orendorff (O'Reilly)
  - URL: https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/
  - Deep dive into ownership, traits, and advanced patterns

---

## ASYNC RUST & WEB FRAMEWORKS

### Tokio (Async Runtime)
- **Tokio Tutorial**
  - URL: https://tokio.rs/tokio/tutorial
  - Essential: Sections on spawning, channels, I/O
  - Focus: Understanding `#[tokio::main]` and task spawning

- **Tokio API Documentation**
  - URL: https://docs.rs/tokio
  - Reference for runtime, net, signal modules

- **Async Book**
  - URL: https://rust-lang.github.io/async-book/
  - Official guide to async/await in Rust

### Axum (Web Framework)
- **Axum Documentation**
  - URL: https://docs.rs/axum/latest/axum/
  - Start: "Getting Started" section
  - Key topics: Routing, handlers, extractors, middleware

- **Axum Examples Repository**
  - URL: https://github.com/tokio-rs/axum/tree/main/examples
  - Real-world patterns and code snippets
  - Check: hello-world, static-file-server, testing

- **Axum Community Resources**
  - Reddit: https://www.reddit.com/r/rust/search?q=axum
  - Discord: Tokio Discord server (has Axum channel)

### Alternative Frameworks (For Reference)
- **Actix-web**
  - URL: https://actix.rs/
  - High performance, mature ecosystem

- **Warp**
  - URL: https://docs.rs/warp
  - Filter-based composition (more functional)

---

## LOGGING & OBSERVABILITY

### Tracing
- **Tracing Documentation**
  - URL: https://docs.rs/tracing
  - Structured, contextual logging

- **Tracing Subscriber**
  - URL: https://docs.rs/tracing-subscriber
  - Formatting and outputting logs

- **Tracing Guide**
  - URL: https://tokio.rs/tokio/topics/tracing
  - Integration with Tokio and async

---

## CONFIGURATION & SERIALIZATION

### Serde
- **Serde Documentation**
  - URL: https://serde.rs/
  - Serialization/deserialization framework

- **Serde Derive**
  - URL: https://serde.rs/derive.html
  - Using `#[derive(Serialize, Deserialize)]`

### TOML
- **TOML Specification**
  - URL: https://toml.io/en/
  - Understanding TOML format

- **toml crate**
  - URL: https://docs.rs/toml
  - Rust TOML parser/encoder

### Config Management
- **config crate**
  - URL: https://docs.rs/config
  - Layered configuration (files, env vars, CLI)

---

## ERROR HANDLING

### Patterns & Libraries
- **anyhow**
  - URL: https://docs.rs/anyhow
  - Easy error handling for applications

- **thiserror**
  - URL: https://docs.rs/thiserror
  - Derive macros for custom error types

- **Error Handling in Rust (Blog Post)**
  - URL: https://blog.burntsushi.net/rust-error-handling/
  - Comprehensive guide by Andrew Gallant

---

## NIX & NIXOS

### Official Documentation
- **NixOS Manual**
  - URL: https://nixos.org/manual/nixos/stable/
  - Chapter 50: Writing NixOS Modules

- **Nixpkgs Manual**
  - URL: https://nixos.org/manual/nixpkgs/stable/
  - Section on Rust: https://nixos.org/manual/nixpkgs/stable/#rust

- **Nix Flakes**
  - URL: https://nixos.wiki/wiki/Flakes
  - Understanding flake.nix structure

- **nixos.asia Flakes Tutorial**
  - URL: https://nixos.asia/en/flakes
  - Beginner-friendly flakes guide

### Rust-Specific Nix Resources
- **Packaging Rust Projects with Nix**
  - URL: https://nixos.wiki/wiki/Rust#Packaging_Rust_projects_with_nix
  - Essential for Sprint 5

- **buildRustPackage Documentation**
  - URL: https://github.com/NixOS/nixpkgs/blob/master/doc/languages-frameworks/rust.section.md
  - Official nixpkgs Rust packaging guide

### Nix Tools
- **flake-parts**
  - URL: https://flake.parts/
  - Modular flake composition

- **treefmt-nix**
  - URL: https://github.com/numtide/treefmt-nix
  - Multi-language formatting

### Learning by Example
- **Study these well-packaged Rust projects in nixpkgs:**
  - ripgrep: Search "ripgrep" in nixpkgs
  - bat: Search "bat" in nixpkgs
  - fd-find: Search "fd" in nixpkgs

---

## DOCKER & CONTAINERS

### Nix Docker Tools
- **dockerTools in Nixpkgs**
  - URL: https://nixos.org/manual/nixpkgs/stable/#sec-pkgs-dockerTools
  - buildImage, buildLayeredImage

- **Nix Docker Tutorial**
  - URL: https://nix.dev/tutorials/nixos/building-and-running-docker-images
  - Step-by-step guide

### General Docker
- **Docker Documentation**
  - URL: https://docs.docker.com/
  - Understanding containers, volumes, networking

- **Docker Best Practices**
  - URL: https://docs.docker.com/develop/dev-best-practices/
  - Production-ready container patterns

---

## TRUENAS & HELM

### TrueNAS SCALE
- **TrueCharts Documentation**
  - URL: https://truecharts.org/
  - Community app catalog

- **Creating a TrueNAS App**
  - URL: https://truecharts.org/manual/development/creating-an-app/
  - Step-by-step app development guide

### Helm
- **Helm Documentation**
  - URL: https://helm.sh/docs/
  - Chart structure, templates, values

- **Helm Chart Best Practices**
  - URL: https://helm.sh/docs/chart_best_practices/
  - Production-ready charts

---

## STATIC SITE GENERATORS (For redirectarr.net)

### Zola (Recommended)
- **Zola Documentation**
  - URL: https://www.getzola.org/documentation/
  - Rust-based, fast, Nix-friendly

- **Zola Themes**
  - URL: https://www.getzola.org/themes/
  - Pre-built themes to customize

### Alternatives
- **Hugo**
  - URL: https://gohugo.io/
  - Popular, fast, large ecosystem

- **mdBook**
  - URL: https://rust-lang.github.io/mdBook/
  - Rust documentation style (like The Rust Book)

---

## SYSTEMD & DAEMON PATTERNS

### Systemd
- **systemd Documentation**
  - URL: https://www.freedesktop.org/software/systemd/man/
  - Official reference

- **systemd.service Man Page**
  - URL: https://www.freedesktop.org/software/systemd/man/systemd.service.html
  - Service unit configuration

### Signal Handling
- **UNIX Signal Handling in Rust**
  - Tokio signal module: https://docs.rs/tokio/latest/tokio/signal/
  - Understanding SIGTERM, SIGINT, SIGHUP

- **Graceful Shutdown Patterns**
  - Blog post: https://tokio.rs/tokio/topics/shutdown
  - Proper cleanup on termination

---

## COMMUNITY & SUPPORT

### Forums & Chat
- **Rust Users Forum**
  - URL: https://users.rust-lang.org/
  - Ask questions, search existing threads

- **Rust Subreddit**
  - URL: https://www.reddit.com/r/rust/
  - News, discussions, help

- **Rust Discord**
  - URL: https://discord.gg/rust-lang
  - Real-time help (use #beginners channel)

- **NixOS Discourse**
  - URL: https://discourse.nixos.org/
  - Nix-specific questions and discussions

### Example Projects to Study
- **microserver** (Rust web server)
  - URL: https://crates.io/crates/microserver
  - Simple HTTP server implementation

- **static-web-server**
  - URL: https://github.com/static-web-server/static-web-server
  - Production-ready static file server in Rust

- **miniserve**
  - URL: https://github.com/svenstaro/miniserve
  - CLI file server, good Rust patterns

---

## ADDITIONAL TOOLS & CRATES

### Useful Crates for Future Features
- **tera** (Template engine)
  - URL: https://docs.rs/tera
  - Jinja2-like templating

- **askama** (Alternative templates)
  - URL: https://docs.rs/askama
  - Compile-time template checking

- **notify** (File watching)
  - URL: https://docs.rs/notify
  - Hot reload templates

- **clap** (CLI parsing)
  - URL: https://docs.rs/clap
  - Command-line argument parsing

- **tower-http** (HTTP middleware)
  - URL: https://docs.rs/tower-http
  - CORS, compression, tracing, etc.

### Testing
- **Rust Testing Guide**
  - URL: https://doc.rust-lang.org/book/ch11-00-testing.html
  - Unit tests, integration tests

- **reqwest** (HTTP client for tests)
  - URL: https://docs.rs/reqwest
  - Making test requests to your server

---

## NIXPKGS CONTRIBUTION

### Process & Guidelines
- **Nixpkgs Contributing Guide**
  - URL: https://github.com/NixOS/nixpkgs/blob/master/CONTRIBUTING.md
  - Required reading before upstreaming

- **RFC 140: pkgs/by-name structure**
  - URL: https://github.com/NixOS/rfcs/blob/master/rfcs/0140-simple-package-paths.md
  - New package organization

- **Package Naming Conventions**
  - URL: https://nixos.org/manual/nixpkgs/stable/#sec-package-naming
  - Proper naming for acceptance

### Review Process
- **How to Get Your PR Merged**
  - Search recent merged package PRs on GitHub
  - See: https://github.com/NixOS/nixpkgs/pulls?q=is%3Apr+is%3Amerged+label%3A%228.has%3A+package+%28new%29%22

---

## GENERAL UNIX/LINUX

### Daemon Concepts
- **"Advanced Programming in the UNIX Environment"** (APUE)
  - Chapter 13: Daemon Processes
  - Classic reference (library book or PDF)

- **Linux System Programming** by Robert Love
  - O'Reilly book
  - Signals, I/O, processes

---

## BOOKMARKS CHEAT SHEET

**Quick Links (Bookmark These):**

1. https://docs.rs/ - All Rust crate documentation
2. https://doc.rust-lang.org/book/ - The Rust Book
3. https://tokio.rs/tokio/tutorial - Tokio Tutorial
4. https://docs.rs/axum/latest/axum/ - Axum Docs
5. https://nixos.org/manual/nixpkgs/stable/#rust - Nix+Rust Guide
6. https://users.rust-lang.org/ - Rust Forum
7. https://serde.rs/ - Serde Guide
8. https://github.com/tokio-rs/axum/tree/main/examples - Axum Examples

---

## SEARCH STRATEGIES

### When Stuck:
1. **Read error message completely** - Rust errors are helpful!
2. **Search docs.rs** for the type/trait mentioned
3. **GitHub issue search**: `repo:tokio-rs/axum <your error>`
4. **Rust Forum search**: Site-specific Google search
5. **crates.io**: Browse similar projects for patterns

### Example Searches:
- "axum extract host header"
- "tokio graceful shutdown"
- "serde deserialize toml"
- "nix buildRustPackage example"

---

*Last Updated: 2026-05-05*
*Commander Data standing by for technical support.*
