//! `site` — small helper for this website.
//!
//!   site new news "Gave a talk at ..." --kind talk --link https://...
//!   site new note "How I fit damping wings"  --tags jwst,python --math
//!   ADS_TOKEN=xxxx site papers sync          # rewrite data/papers.toml from NASA/ADS
//!
//! Run from anywhere inside the repository: it walks up to find `config.toml`.

use anyhow::{bail, Context, Result};
use chrono::{Local, NaiveDate};
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "site", version, about = "Helper for astroumeda.github.io")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Create a new News item or Note from a template
    New {
        #[command(subcommand)]
        what: New,
    },
    /// Publication list tools
    Papers {
        #[command(subcommand)]
        what: Papers,
    },
}

#[derive(Subcommand)]
enum New {
    /// One-line update shown in the News list (and the home page)
    News {
        /// The headline, e.g. "Invited talk at ..."
        title: String,
        #[arg(long, value_enum, default_value_t = Kind::Misc)]
        kind: Kind,
        /// Optional link (paper, slides, event page...)
        #[arg(long)]
        link: Option<String>,
        /// Date, YYYY-MM-DD (default: today)
        #[arg(long)]
        date: Option<NaiveDate>,
    },
    /// A short blog-style note (Markdown, with optional math)
    Note {
        title: String,
        /// Comma-separated tags, e.g. "jwst,python"
        #[arg(long, value_delimiter = ',')]
        tags: Vec<String>,
        /// Load KaTeX on this note
        #[arg(long)]
        math: bool,
        /// Folder name (default: made from the title)
        #[arg(long)]
        slug: Option<String>,
        /// Date, YYYY-MM-DD (default: today)
        #[arg(long)]
        date: Option<NaiveDate>,
    },
}

#[derive(Subcommand)]
enum Papers {
    /// Fetch your papers from NASA/ADS and rewrite data/papers.toml.
    /// Needs an ADS API token in $ADS_TOKEN (https://ui.adsabs.harvard.edu/user/settings/token).
    Sync {
        /// Print the result instead of writing the file
        #[arg(long)]
        dry_run: bool,
    },
}

#[derive(Copy, Clone, ValueEnum)]
enum Kind {
    Paper,
    Talk,
    Award,
    Life,
    Misc,
}

impl Kind {
    fn as_str(self) -> &'static str {
        match self {
            Kind::Paper => "paper",
            Kind::Talk => "talk",
            Kind::Award => "award",
            Kind::Life => "life",
            Kind::Misc => "misc",
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = find_root()?;
    match cli.cmd {
        Cmd::New { what } => match what {
            New::News { title, kind, link, date } => new_news(&root, &title, kind, link, date),
            New::Note { title, tags, math, slug, date } => new_note(&root, &title, &tags, math, slug, date),
        },
        Cmd::Papers { what: Papers::Sync { dry_run } } => sync_papers(&root, dry_run),
    }
}

/// Walk up from the current directory to the folder holding the Zola config.
fn find_root() -> Result<PathBuf> {
    let mut dir = std::env::current_dir()?;
    loop {
        if dir.join("config.toml").exists() && dir.join("content").is_dir() {
            return Ok(dir);
        }
        if !dir.pop() {
            bail!("could not find the site root (a folder with config.toml and content/)");
        }
    }
}

fn slugify(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars().flat_map(char::to_lowercase) {
        if c.is_ascii_alphanumeric() {
            out.push(c);
        } else if !out.ends_with('-') && !out.is_empty() {
            out.push('-');
        }
    }
    let out: String = out.trim_matches('-').chars().take(60).collect();
    out.trim_end_matches('-').to_string()
}

/// TOML-escape a string value (quotes included).
fn q(s: &str) -> String {
    toml::Value::String(s.to_string()).to_string()
}

fn write_new(path: &Path, body: &str) -> Result<()> {
    if path.exists() {
        bail!("{} already exists", path.display());
    }
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(path, body)?;
    println!("created {}", path.display());
    Ok(())
}

fn new_news(root: &Path, title: &str, kind: Kind, link: Option<String>, date: Option<NaiveDate>) -> Result<()> {
    let date = date.unwrap_or_else(|| Local::now().date_naive());
    let slug = match slugify(title) {
        s if s.is_empty() => "news".to_string(),
        s => s.split('-').take(6).collect::<Vec<_>>().join("-"),
    };
    let path = root.join("content/news").join(format!("{date}-{slug}.md"));
    let mut fm = format!(
        "+++\n# Optional: write a sentence or two of detail below the closing +++.\n\
         title = {}\ndate = {date}\n[extra]\nkind = \"{}\"  # paper | talk | award | life | misc\n",
        q(title),
        kind.as_str()
    );
    if let Some(l) = link {
        fm += &format!("link = {}\n", q(&l));
    }
    fm += "+++\n";
    write_new(&path, &fm)
}

fn new_note(
    root: &Path,
    title: &str,
    tags: &[String],
    math: bool,
    slug: Option<String>,
    date: Option<NaiveDate>,
) -> Result<()> {
    let date = date.unwrap_or_else(|| Local::now().date_naive());
    let slug = slug.unwrap_or_else(|| slugify(title));
    let slug = if slug.is_empty() { format!("note-{date}") } else { slug };
    let path = root.join("content/notes").join(&slug).join("index.md");
    let tags = tags.iter().map(|t| q(t.trim())).collect::<Vec<_>>().join(", ");
    let body = format!(
        "+++\ntitle = {}\ndate = {date}\ndescription = \"\"\ndraft = true  # remove this line to publish\n\
         [taxonomies]\ntags = [{tags}]\n[extra]\nmath = {math}\n+++\n\n\
         Write here. Put figures in this folder and link them like ![caption](figure.png).\n",
        q(title)
    );
    write_new(&path, &body)?;
    println!("preview drafts with: zola serve --drafts");
    Ok(())
}

// ------------------------------------------------------------------ ADS sync

#[derive(Serialize, Deserialize, Clone)]
struct Paper {
    authors: String,
    title: String,
    journal: String,
    year: i32,
    url: String,
}

#[derive(Serialize, Deserialize, Default)]
struct SyncCfg {
    /// ADS search query. Tip: `orcid:0000-...` is the most robust choice.
    query: String,
    /// Surname used to decide first-author vs co-authored, e.g. "Umeda"
    surname: String,
    /// Bibcodes to leave out (wrong person, errata, proceedings you don't want listed...)
    #[serde(default)]
    exclude: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct PapersFile {
    ads_query: String,
    sync: SyncCfg,
    #[serde(default)]
    first_author: Vec<Paper>,
    #[serde(default)]
    co_authored: Vec<Paper>,
}

#[derive(Deserialize)]
struct AdsResponse {
    response: AdsDocs,
}
#[derive(Deserialize)]
struct AdsDocs {
    docs: Vec<AdsDoc>,
}
#[derive(Deserialize)]
struct AdsDoc {
    bibcode: String,
    #[serde(default)]
    title: Vec<String>,
    #[serde(default)]
    author: Vec<String>,
    #[serde(default)]
    bibstem: Vec<String>,
    volume: Option<String>,
    #[serde(default)]
    page: Vec<String>,
    year: Option<String>,
}

/// Turn ADS title markup into plain text: Ly$\alpha$ → Lyα, <SUB>x</SUB> → x.
fn clean_title(t: &str) -> String {
    let mut s = t.to_string();
    for (from, to) in [
        ("$\\alpha$", "α"), ("{\\alpha}", "α"), ("\\alpha", "α"), ("$\\beta$", "β"), ("$\\sim$", "~"),
        ("$\\lesssim$", "≲"), ("$\\gtrsim$", "≳"), ("$<$", "<"), ("$>$", ">"), ("$\\odot$", "☉"),
        ("&lt;", "<"), ("&gt;", ">"), ("&amp;", "&"),
    ] {
        s = s.replace(from, to);
    }
    // drop simple HTML tags like <SUB>, <SUP>, <i>
    let mut out = String::with_capacity(s.len());
    let mut in_tag = false;
    for c in s.chars() {
        match c {
            '<' if !in_tag => in_tag = true,
            '>' if in_tag => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    out.replace('$', "").split_whitespace().collect::<Vec<_>>().join(" ")
}

fn surname(author: &str) -> &str {
    author.split(',').next().unwrap_or(author).trim()
}

/// "Umeda et al." for first-author papers, "Hatano, Ouchi, Umeda et al." otherwise.
fn author_line(authors: &[String], me: &str) -> String {
    let names: Vec<&str> = authors.iter().map(|a| surname(a)).collect();
    let upto = names.iter().position(|n| n.eq_ignore_ascii_case(me)).map_or(names.len().min(3), |i| i + 1);
    let mut s = names[..upto].join(", ");
    if names.len() > upto {
        s += " et al.";
    }
    s
}

fn sync_papers(root: &Path, dry_run: bool) -> Result<()> {
    let path = root.join("data/papers.toml");
    let text = fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    let mut file: PapersFile = toml::from_str(&text).context("parsing data/papers.toml")?;
    if file.sync.query.is_empty() || file.sync.surname.is_empty() {
        bail!("set [sync] query and surname in data/papers.toml first");
    }
    let token = std::env::var("ADS_TOKEN").context("set ADS_TOKEN (get one at https://ui.adsabs.harvard.edu/user/settings/token)")?;

    let resp: AdsResponse = ureq::get("https://api.adsabs.harvard.edu/v1/search/query")
        .set("Authorization", &format!("Bearer {token}"))
        .query("q", &file.sync.query)
        .query("fl", "bibcode,title,author,bibstem,volume,page,year")
        .query("sort", "date desc")
        .query("rows", "500")
        .call()
        .context("ADS request failed (check the token and the query)")?
        .into_json()?;

    let me = file.sync.surname.clone();
    let (mut first, mut co) = (Vec::new(), Vec::new());
    for d in resp.response.docs {
        if file.sync.exclude.contains(&d.bibcode) || d.author.is_empty() {
            continue;
        }
        let stem = d.bibstem.first().cloned().unwrap_or_default();
        let page = d.page.first().cloned().unwrap_or_default();
        let journal = if stem == "arXiv" {
            page.clone()
        } else {
            [stem, d.volume.unwrap_or_default(), page].into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>().join(", ")
        };
        let p = Paper {
            authors: author_line(&d.author, &me),
            title: clean_title(d.title.first().map(String::as_str).unwrap_or("(untitled)")),
            journal,
            year: d.year.as_deref().and_then(|y| y.parse().ok()).unwrap_or(0),
            url: format!("https://ui.adsabs.harvard.edu/abs/{}/abstract", d.bibcode),
        };
        if surname(&d.author[0]).eq_ignore_ascii_case(&me) {
            first.push(p);
        } else {
            co.push(p);
        }
    }
    println!("ADS: {} first-author, {} co-authored", first.len(), co.len());
    if first.is_empty() && co.is_empty() {
        bail!("ADS returned nothing; leaving data/papers.toml unchanged");
    }
    file.first_author = first;
    file.co_authored = co;

    let out = format!(
        "# Publications. Generated by `site papers sync` from NASA/ADS — edits to the paper lists\n\
         # are overwritten on the next sync. Change [sync] to control what is fetched.\n\
         # \"Umeda\" in `authors` is highlighted automatically.\n\n{}",
        toml::to_string_pretty(&file)?
    );
    if dry_run {
        print!("{out}");
    } else {
        fs::write(&path, out)?;
        println!("wrote {}", path.display());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authors() {
        let a = |v: &[&str]| v.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(author_line(&a(&["Umeda, Hiroya", "Ouchi, M."]), "Umeda"), "Umeda et al.");
        assert_eq!(author_line(&a(&["Umeda, Hiroya"]), "Umeda"), "Umeda");
        assert_eq!(
            author_line(&a(&["Hatano, S.", "Ouchi, M.", "Umeda, H.", "Xu, Y."]), "Umeda"),
            "Hatano, Ouchi, Umeda et al."
        );
    }

    #[test]
    fn titles() {
        assert_eq!(clean_title("Ly$\\alpha$ Emitters at z~7"), "Lyα Emitters at z~7");
        assert_eq!(clean_title("M<SUB>BH</SUB>  &amp; more"), "MBH & more");
    }

    #[test]
    fn slugs() {
        assert_eq!(slugify("How I fit Lyα damping wings!"), "how-i-fit-ly-damping-wings");
        assert_eq!(slugify("観測メモ"), "");
    }
}
