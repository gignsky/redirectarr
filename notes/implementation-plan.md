# Redirectarr Implementation Plan

**Project Goal:** Build a simple daemon web server in Rust that serves domain-specific landing pages for unused domains.

**Core Principles:**
- SIMPLE first, features later
- Learn by doing
- MVP before polish
- Configurable, not complex

---

## PHASE 0: FOUNDATIONAL SETUP

### Sprint 0: Nix + Rust Development Environment

**Goal:** Setup flake-based Rust development environment with cargo available in devShell

**Tasks:**
1. Copy Rust flake infrastructure from ~/local_repos/rust-nix-template/
2. Adapt to redirectarr project structure
3. Initialize cargo project within Nix context
4. Verify `nix develop` provides cargo, rustc, rust-analyzer
5. Test basic `cargo build` works

**Learning Focus:**
- How Nix provides Rust toolchain
- Relationship between flake.nix and Cargo.toml
- DevShell environment variables

**Success Criteria:**
- [ ] `nix develop` works without errors
- [ ] `cargo --version` works inside devShell
- [ ] `cargo init` creates project structure
- [ ] Pre-commit hooks active

---

## PHASE 1: MINIMAL VIABLE DAEMON (MVD)

### Sprint 1: Hello World Web Server

**Goal:** A Rust binary that listens on port 8080 and returns "Hello World"

**Tasks:**
1. Add dependencies to Cargo.toml:
   - `axum` (web framework)
   - `tokio` (async runtime with "full" feature)
   - `tower` (middleware utilities)
   - `tracing` (structured logging)
   - `tracing-subscriber` (log formatting/output)

2. Write minimal `src/main.rs`:
   - Create async main with `#[tokio::main]`
   - Setup tracing subscriber
   - Create basic Axum router with single route
   - Bind to 0.0.0.0:8080
   - Start server with logging

3. Test locally:
   ```bash
   cargo run
   # In another terminal:
   curl http://localhost:8080
   ```

4. Add basic error handling

**Learning Focus:**
- Async/await in Rust
- Axum's Router and handler functions
- Tokio runtime setup
- Basic tracing usage

**Success Criteria:**
- [ ] Server starts without panicking
- [ ] Responds to HTTP requests
- [ ] Logs requests to stdout
- [ ] Can Ctrl+C to stop gracefully

---

### Sprint 2: Configuration System

**Goal:** Load server settings from a TOML file instead of hardcoded values

**Tasks:**
1. Add dependencies:
   - `serde` with derive feature
   - `toml` (TOML parsing)
   - `anyhow` or `thiserror` (better error handling)

2. Create `config.toml` in repo root:
   ```toml
   [server]
   host = "0.0.0.0"
   port = 8080
   
   [[mappings]]
   domain = "redirectarr.net"
   template_dir = "./templates/redirectarr"
   
   [[mappings]]
   domain = "example.com"
   template_dir = "./templates/example"
   ```

3. Implement configuration structs in `src/config.rs`:
   - `ServerConfig` struct
   - `DomainMapping` struct
   - `AppConfig` struct (root)
   - Derive Deserialize for all

4. Load config on startup:
   - Read config.toml file
   - Parse with serde/toml
   - Validate template directories exist
   - Return errors for missing files

5. Use config values to bind server

**Learning Focus:**
- Serde derive macros
- TOML format
- File I/O and error handling
- Structuring Rust projects (modules)

**Success Criteria:**
- [ ] Server reads port from config.toml
- [ ] Invalid config shows helpful error
- [ ] Missing config file shows helpful error
- [ ] Config struct accessible in handlers

---

### Sprint 3: Domain-Based Routing

**Goal:** Serve different HTML files based on the `Host` header in requests

**Tasks:**
1. Create template directories and sample HTML:
   ```bash
   mkdir -p templates/redirectarr
   mkdir -p templates/example
   # Create simple index.html in each
   ```

2. Modify Axum router to catch-all route:
   - Single handler for all paths
   - Extract `Host` header from request

3. Implement domain matching logic:
   - Parse Host header (handle port if present)
   - Match against configured domains
   - Look up corresponding template_dir

4. Serve static HTML files:
   - Read index.html from matched template_dir
   - Return as HTTP response with correct Content-Type
   - Handle file not found

5. Implement fallback for unknown domains:
   - Return generic "Redirectarr" landing page
   - Or 404 with message

**Learning Focus:**
- HTTP header extraction in Axum
- Pattern matching in Rust
- String parsing (remove :port from Host)
- Serving files as HTTP responses

**Success Criteria:**
- [ ] Different domains serve different HTML
- [ ] Host header correctly parsed
- [ ] Missing template shows error
- [ ] Unknown domain handled gracefully
- [ ] Can test with `curl -H "Host: example.com" http://localhost:8080`

---

### Sprint 4: Daemon Behavior

**Goal:** Transform from script to proper daemon with signal handling and graceful operations

**Tasks:**
1. Implement graceful shutdown on SIGTERM/SIGINT:
   - Use `tokio::signal::ctrl_c()`
   - Gracefully stop accepting new connections
   - Wait for in-flight requests to complete
   - Log shutdown message

2. Upgrade logging to proper structured logging:
   - Replace println! with `tracing` macros (info!, debug!, error!)
   - Add spans for request handling
   - Include useful context (domain, path, status code)
   - Format for journald consumption

3. Add health check endpoint:
   - `/health` returns 200 OK with "healthy" message
   - Useful for monitoring and container orchestration

4. Add basic request logging:
   - Log each request: method, path, host, response status
   - Time request duration

5. (Optional) Configuration reload on SIGHUP:
   - Can defer to later sprint if complex

**Learning Focus:**
- Signal handling in async Rust
- Tracing spans and structured logging
- Graceful shutdown patterns
- Production-ready daemon patterns

**Success Criteria:**
- [ ] Ctrl+C triggers graceful shutdown
- [ ] No more println!, only tracing macros
- [ ] Logs show request details
- [ ] /health endpoint works
- [ ] Server doesn't crash on errors

---

## PHASE 2: PACKAGING & DISTRIBUTION

### Sprint 5: Nix Package Integration

**Goal:** Build redirectarr binary via `nix build`

**Tasks:**
1. Create `nix/flake/modules/package.nix`:
   - Use `rustPlatform.buildRustPackage`
   - Set `pname`, `version`, `src`
   - Generate `cargoHash` (use `lib.fakeHash` first, then update)
   - Define `meta` attributes (description, license, maintainers)

2. Export package in flake outputs:
   - Add to `packages.default`
   - Ensure cross-system support

3. Test Nix build:
   ```bash
   nix build
   ./result/bin/redirectarr --version
   ```

4. Update devShell to include built package:
   - Makes binary available in development

5. Update README with build instructions

**Learning Focus:**
- How Nix builds Rust projects
- Cargo.lock integration with Nix
- Flake package outputs

**Success Criteria:**
- [ ] `nix build` succeeds
- [ ] Binary runs from ./result/bin/
- [ ] Version info correct
- [ ] Can run `nix run .` to execute

---

### Sprint 6: NixOS Module (Optional Initially)

**Goal:** Allow declarative NixOS configuration for redirectarr service

**Tasks:**
1. Create `nix/flake/modules/nixos-module.nix`:
   - Define options: `services.redirectarr.enable`
   - Configuration options for port, config file path
   - systemd service unit definition

2. Systemd service configuration:
   - ExecStart pointing to redirectarr binary
   - Restart on failure
   - User/group (dedicated unprivileged user)
   - Proper dependencies (network.target)

3. Export module in flake outputs

4. Test on local NixOS system (if applicable)

**Learning Focus:**
- NixOS module system
- systemd service units in Nix
- Declarative service management

**Success Criteria:**
- [ ] Module defines valid options
- [ ] Service starts with `systemctl start redirectarr`
- [ ] Logs visible in journald
- [ ] Service survives reboot (if wanted)

**Note:** Can be deferred until MVP is stable

---

### Sprint 7: Docker Image

**Goal:** Create OCI container image for redirectarr

**Tasks:**
1. Add Docker image to flake outputs:
   ```nix
   packages.docker = pkgs.dockerTools.buildImage {
     name = "redirectarr";
     tag = "latest";
     config = {
       Cmd = [ "${packages.default}/bin/redirectarr" ];
       ExposedPorts = { "8080/tcp" = {}; };
       WorkingDir = "/app";
     };
     # Copy default config and templates
   };
   ```

2. Build and test:
   ```bash
   nix build .#docker
   docker load < result
   docker run -p 8080:8080 redirectarr:latest
   ```

3. Add volume mounts for config and templates

4. Document Docker usage in README

**Learning Focus:**
- Nix dockerTools
- Container best practices
- Volume mounting for configuration

**Success Criteria:**
- [ ] Image builds with Nix
- [ ] Container runs and serves content
- [ ] Can mount custom config
- [ ] Image size reasonable

---

### Sprint 8: TrueNAS SCALE App

**Goal:** Create Helm chart for TrueNAS catalog submission

**Tasks:**
1. Research TrueNAS app structure:
   - Read TrueCharts documentation
   - Study existing simple apps

2. Create Helm chart structure:
   - Chart.yaml (metadata)
   - values.yaml (default config)
   - templates/deployment.yaml
   - templates/service.yaml
   - questions.yaml (TrueNAS UI schema)

3. Test locally:
   - Deploy in TrueNAS SCALE VM or instance
   - Verify UI configuration works

4. Submit to catalog (when ready):
   - Follow contribution guidelines
   - Provide screenshots and documentation

**Learning Focus:**
- Helm chart structure
- TrueNAS app conventions
- Kubernetes basics

**Note:** Defer until Sprint 7 complete and tested

---

### Sprint 9: Nixpkgs Upstreaming

**Goal:** Get redirectarr accepted into official nixpkgs

**Tasks:**
1. Polish Nix package:
   - Clean derivation code
   - Comprehensive meta attributes
   - Add yourself as maintainer
   - Ensure follows nixpkgs conventions

2. Test on multiple systems if possible:
   - x86_64-linux (primary)
   - aarch64-linux (if available)

3. Fork NixOS/nixpkgs

4. Add package following RFC 140:
   - Location: `pkgs/by-name/re/redirectarr/package.nix`
   - Follow naming conventions

5. Submit PR:
   - Use template
   - Link to GitHub repo
   - Explain what it does

6. Respond to review feedback

**Learning Focus:**
- Nixpkgs contribution process
- Code review collaboration
- Nix community standards

**Note:** Only when project is stable and well-tested

---

## PHASE 3: PROJECT WEBSITE & MARKETING

### Sprint 10: redirectarr.net Website

**Goal:** Professional landing page explaining the project

**Suggested Stack:**
- Static site generator: Zola (Rust-based, Nix-friendly)
- Or: Hugo, plain HTML, mdBook
- Hosting: GitHub Pages or Cloudflare Pages

**Tasks:**
1. Initialize site project:
   ```bash
   mkdir website
   cd website
   zola init
   # Or create plain HTML structure
   ```

2. Choose/customize theme:
   - Zola themes: getzola.org/themes
   - Keep it simple and clean

3. Write content:
   - Homepage (hero, features, quick start)
   - Installation guide
   - Configuration examples
   - Screenshots or demos

4. Setup deployment:
   - GitHub Action for builds
   - Deploy to GitHub Pages
   - Configure redirectarr.net DNS

**Learning Focus:**
- Static site generators
- CI/CD for websites
- DNS configuration

**Success Criteria:**
- [ ] Site loads at redirectarr.net
- [ ] Installation instructions clear
- [ ] Professional appearance
- [ ] Mobile responsive

**Note:** Can be done in parallel with other sprints

---

## PHASE 4: ITERATION & POLISH

### Future Feature Ideas

**After MVP is working, consider:**

1. **Template Engine Support:**
   - Add `tera` or `askama` crate
   - Variables like {{domain}}, {{year}}, custom metadata
   - More flexible than static HTML

2. **Metrics/Observability:**
   - Prometheus metrics endpoint at /metrics
   - Track requests per domain, response times
   - Grafana dashboard examples

3. **Hot Reload:**
   - Watch template directories with `notify` crate
   - Reload templates without restart
   - Watch config file for changes

4. **Web UI for Configuration:**
   - Admin panel at /admin (password protected)
   - Edit domain mappings via web interface
   - Save changes back to config.toml

5. **Let's Encrypt Integration:**
   - ACME client for automatic HTTPS
   - Challenge handling (HTTP-01)
   - Certificate renewal

6. **Multiple Template Files:**
   - Serve full directories, not just index.html
   - Static assets (CSS, JS, images)
   - Proper MIME type detection

**Prioritization:**
- Keep "FUTURE.md" file for ideas
- Mark all as "post-1.0"
- Gather user feedback first
- Don't scope creep!

---

## LEARNING PHILOSOPHY

### When to Use AI vs Manual Resources:

**Use Manual Resources (Primary Learning):**
- Understanding fundamentals
- Following tutorials start to finish
- Exploring API documentation
- Reading error messages completely

**Use AI (Commander Data):**
- Explaining concepts after reading
- Code review and suggestions
- Debugging specific blockers
- Design decision discussions
- Template/boilerplate generation

**Best Approach:**
1. Try independently first
2. Search docs and examples
3. Use AI to verify understanding
4. Ask for explanation of "why" not just "how"

---

## SUCCESS METRICS

### Phase 1 Complete When:
- [ ] Binary runs as daemon (doesn't exit immediately)
- [ ] Different domains show different HTML content
- [ ] Configuration loads from TOML file
- [ ] Logs go to stdout (structured with tracing)
- [ ] Graceful shutdown on Ctrl+C
- [ ] Basic error handling (doesn't crash on bad requests)

### Phase 2 Complete When:
- [ ] `nix build` produces working binary
- [ ] `nix run github:gignsky/redirectarr` works
- [ ] Docker image builds and runs
- [ ] Can deploy in container environment

### Phase 3 Complete When:
- [ ] redirectarr.net website live
- [ ] Installation docs complete
- [ ] At least 2-3 example templates provided
- [ ] Project looks professional to outsiders

---

## DECISION LOG

### Questions Answered (2026-05-05):

**Q1: Default port?**
**A:** 8080 (unprivileged, container-friendly, configurable later)

**Q2: HTTPS in MVP?**
**A:** No, start with HTTP only. Add encryption later (Phase 4).

**Q3: Templates in MVP?**
**A:** Static HTML files. Template engine (tera/askama) is Phase 4.

**Q4: Unknown domain fallback behavior?**
**A:** TBD - Generic Redirectarr landing page is good UX. Decide in Sprint 3.

**Q5: Config file location?**
**A:** Start with `./config.toml` (development-friendly). Revisit for production (Docker: `/app/config.toml`, NixOS service: `/etc/redirectarr/config.toml` or `/var/lib/redirectarr/config.toml`).

**Q6: CLI arguments for MVP?**
**A:** Defer until Sprint 1 starts. Basic: `--help`, `--version`, `--config <path>`

---

## IMMEDIATE NEXT STEPS

### Sprint 0 Starts Now:

1. **Copy Rust+Nix infrastructure from rust-nix-template**
2. **Adapt flake.nix for redirectarr project**
3. **Test devShell provides cargo**
4. **Initialize cargo project**
5. **Commit infrastructure changes**

### After Sprint 0:

1. **Read Axum getting started guide** (30-60 min)
2. **Skim Tokio tutorial** (first 2 sections)
3. **Begin Sprint 1: Hello World Server**

---

## SIMPLICITY ENFORCEMENT

**Rules to Prevent Scope Creep:**

1. No database until proven necessary
2. No GraphQL/gRPC/WebSocket - HTTP only
3. Single binary - no microservices
4. Flat file config - no complex DSL
5. Standard library preferred over crates
6. If you can't explain it simply, it's too complex

**Before Adding Anything:**
- Is this needed for MVP?
- Does this solve an actual problem?
- Can it wait until 1.0 is shipped?

---

## TIMELINE ESTIMATES

| Sprint | Focus | Estimated Time |
|--------|-------|----------------|
| 0 | Nix+Rust Setup | 1-2 days |
| 1 | Hello World Server | 1-2 days |
| 2 | Configuration | 2-3 days |
| 3 | Domain Routing | 2-3 days |
| 4 | Daemon Behavior | 3-4 days |
| 5 | Nix Package | 1-2 days |
| 6 | NixOS Module | 2-3 days (optional) |
| 7 | Docker Image | 1 day |
| 10 | Website | 3-5 days |

**Total MVP (Sprints 0-5):** ~2-3 weeks part-time  
**Full production ready:** ~4-6 weeks part-time

---

*This plan is a living document. Update as you learn and priorities change.*

*Last Updated: 2026-05-05*
