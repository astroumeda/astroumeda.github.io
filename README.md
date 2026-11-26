# astroumeda.github.io

Personal website of Hiroya Umeda, built with [Zola](https://www.getzola.org) (a static site generator written in Rust),
plus a small Rust helper CLI in `tools/site`.

## Everyday updates

### Add a News item

```bash
site new news "Invited talk at XYZ Workshop, Kyoto" --kind talk --link https://example.org
```

`--kind` is one of `paper`, `talk`, `award`, `life`, `misc` (it sets the coloured dot).
This creates `content/news/<date>-<slug>.md`; push it and it appears on the home page and on `/news/`.

### Write a Note (blog)

```bash
site new note "Fitting damping wings with emcee" --tags jwst,python --math
```

This creates `content/notes/<slug>/index.md` as a **draft**. Write Markdown, put figures in the same folder
(`![caption](figure.png)`), then delete the `draft = true` line to publish. Preview drafts with `zola serve --drafts`.

- Inline math: `$x_\mathrm{HI}$`
- Display math: wrap LaTeX in a math block so Markdown leaves it alone:
  ```
  {% <math> %}
  \tau = \int n_\mathrm{HI}\, \sigma\, dl
  {% </math> %}
  ```
- Code blocks get syntax highlighting automatically.

### Update papers

Either edit `data/papers.toml` by hand, or fetch everything from NASA/ADS:

```bash
ADS_TOKEN=... site papers sync     # add --dry-run to preview
```

What is fetched is set in the `[sync]` table of `data/papers.toml` (ORCID query recommended).
A GitHub Action (`update-papers.yml`) also does this on the 1st of every month once the `ADS_TOKEN` secret is set.

### Edit from the browser

On GitHub, press <kbd>.</kbd> in the repository to open the web editor, edit a file, and commit —
the site redeploys automatically. No local setup needed for small fixes.

## Where things live

| What | File |
|---|---|
| Name, position, links, email, address | `config.toml` (`[extra]`) |
| About text, tagline, "I'm a …" words | `content/_index.md` |
| News | `content/news/*.md` |
| Notes | `content/notes/<slug>/index.md` |
| Research topics and figures | `data/research.toml` (+ images in `static/img/`) |
| Publications | `data/papers.toml` |
| Positions, education, talks, grants, awards | `data/cv.toml` |
| Full CV PDF | `static/assets/pdf/CV_Umeda.pdf` |
| Design | `templates/` (Tera v2), `sass/style.scss`, `static/js/site.js` |

Images are resized to WebP automatically at build time. News and Notes are published as an Atom feed at `/atom.xml`.

## Setup

```bash
brew install zola
cargo install --path tools/site     # installs the `site` command
zola serve                          # http://127.0.0.1:1111, reloads on save
```

## Deploy

Pushing to `astroumeda-no_blogs` runs `.github/workflows/deploy.yml`, which builds the site and publishes it to GitHub Pages.
One-time setup: **Settings → Pages → Build and deployment → Source: GitHub Actions**.

## Easter eggs

Press <kbd>z</kbd> to redshift the page (<kbd>0</kbd> to come home), or enter the Konami code to watch the universe reionize.
