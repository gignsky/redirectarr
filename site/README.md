# site/

A minimal static "work in progress" placeholder page for `redirectarr.net`. This
is step 1 of the redirectarr project: get *something* live on the domain while
the full waiting-page service (the Rust/Nix project in the rest of this repo)
is built out.

## Deploying to Cloudflare Pages

1. In the Cloudflare dashboard: **Workers & Pages → Create → Pages → Connect to Git**.
2. Select the `gignsky/redirectarr` repository.
3. Build settings:
   - Framework preset: `None`
   - Build command: *(leave empty)*
   - Build output directory: `site`
4. Deploy. Cloudflare will give you a `*.pages.dev` URL first.
5. Add the custom domain: in the new Pages project, go to **Custom domains →
   Add a domain** and enter `redirectarr.net` (and `www.redirectarr.net` if
   desired). Since the domain's nameservers are already on Cloudflare, the DNS
   records are created automatically.

Every push to `main` that touches `site/` will redeploy automatically once the
Pages project is connected — no GitHub Actions or API tokens needed.
