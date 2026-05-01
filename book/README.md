# sui-clippy book (mdBook)

This directory is an [mdBook](https://github.com/rust-lang/mdBook) project modeled after [rust-clippy’s `book/`](https://github.com/rust-lang/rust-clippy/tree/master/book).

## Build

```bash
cd book
mdbook build
```

Output: **`book/build/`** (see `[build] build-dir` in `book.toml`).

## Regenerate the lint table + searchable lint page

From the **workspace root** (parent of `book/`):

```bash
cargo run -p sui-clippy-docgen
```

That refreshes:

- `book/src/lints_generated.md` — Markdown lint table (a book chapter).
- `book/src/lints/index.html`, `style.css`, `script.js` — static assets; mdBook copies them into **`build/lints/`** so the site serves the book at `/` and the lint UI at **`/lints/`**.

Then run `mdbook build` again so the book includes the updates.

### Preview the lint page locally

```bash
cd book && mdbook build && python3 -m http.server 8765 --directory build
```

Open `http://127.0.0.1:8765/lints/` (serving `style.css` / `script.js` as sibling files requires HTTP, not `file://`).

## GitHub Pages

The workflow **`.github/workflows/pages.yml`** (on push to `main` or manual **Run workflow**) does the following:

1. Runs `sui-clippy-docgen`, then `mdbook build` with `MDBOOK_OUTPUT__HTML__SITE_URL` set to `/REPO_NAME/` so in-repo links work on `https://YOUR_USER.github.io/REPO_NAME/`.
2. Uploads **`book/build/`** as the site root (the lint list is already under **`build/lints/`** from mdBook).

**Repository settings:** **Settings → Pages → Build and deployment → Source:** choose **GitHub Actions** (not “Deploy from a branch”). The first successful run creates the `github-pages` environment.

Published URL shape: `https://YOUR_USER.github.io/REPO_NAME/`
