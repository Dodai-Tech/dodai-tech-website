//! Builds the whole site into a temp directory and checks it as a visitor would see it.
use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
};

static N: AtomicUsize = AtomicUsize::new(0);

/// A fresh build in its own directory, so parallel tests do not delete each other's files.
fn build() -> PathBuf {
    let out = std::env::temp_dir().join(format!(
        "dodaitech-website-test-{}-{}",
        std::process::id(),
        N.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = fs::remove_dir_all(&out);
    dodaitech_website::copy_dir(&Path::new(env!("CARGO_MANIFEST_DIR")).join("static"), &out)
        .expect("copy static");
    dodaitech_website::site::build(&out);
    out
}

fn html_files(dir: &Path, found: &mut Vec<PathBuf>) {
    for e in fs::read_dir(dir).expect("read dir").flatten() {
        let p = e.path();
        if p.is_dir() {
            html_files(&p, found);
        } else if p.extension().is_some_and(|x| x == "html") {
            found.push(p);
        }
    }
}

/// Every value of `attr="…"` in `html`.
fn attr_values<'a>(html: &'a str, attr: &str) -> Vec<&'a str> {
    let needle = format!(" {attr}=\"");
    html.match_indices(&needle)
        .filter_map(|(i, m)| {
            let rest = &html[i + m.len()..];
            rest.find('"').map(|end| &rest[..end])
        })
        .collect()
}

#[test]
fn every_page_exists() {
    let out = build();
    for page in [
        "index.html",
        "design-system/index.html",
        "design-system/charts/index.html",
        "design-system/components/index.html",
        "design-system/showcase/index.html",
        "products/watchfaces/index.html",
        "privacy/index.html",
        "dodai.css",
        "charts.css",
        "site.css",
    ] {
        assert!(out.join(page).is_file(), "missing {page}");
    }
}

#[test]
fn every_local_link_and_image_resolves() {
    let out = build();
    let mut pages = Vec::new();
    html_files(&out, &mut pages);
    let mut broken = Vec::new();
    for page in &pages {
        let html = fs::read_to_string(page).expect("read page");
        for target in attr_values(&html, "href")
            .into_iter()
            .chain(attr_values(&html, "src"))
        {
            if target.is_empty()
                || target.starts_with(['#', '{'])
                || target.contains(':')
                || target.starts_with("//")
            {
                continue;
            }
            let clean = target.split(['#', '?']).next().unwrap_or("");
            let base = if clean.starts_with('/') {
                out.clone()
            } else {
                page.parent().expect("page dir").to_path_buf()
            };
            let path = base.join(clean.trim_start_matches('/'));
            if !(path.is_file() || path.join("index.html").is_file()) {
                broken.push(format!(
                    "{} -> {target}",
                    page.strip_prefix(&out).unwrap_or(page).display()
                ));
            }
        }
    }
    assert!(broken.is_empty(), "broken links:\n{}", broken.join("\n"));
}

#[test]
fn the_home_page_names_what_is_sold_and_what_is_open() {
    let html = fs::read_to_string(build().join("index.html")).expect("home");
    assert!(html.contains("SynthWave"));
    assert!(html.contains("Dodai UI"));
    // Regression: Instruments and watch face development are not shown (not public, not open source).
    assert!(!html.contains("Instruments"));
    assert!(!html.contains("Watch face development"));
}

#[test]
fn the_theme_switch_is_on_every_page() {
    let out = build();
    let mut pages = Vec::new();
    html_files(&out, &mut pages);
    for page in pages.iter().filter(|p| {
        !p.ends_with("design-system/showcase/index.html") && !p.starts_with(out.join("privacy"))
    }) {
        assert!(
            fs::read_to_string(page)
                .expect("read")
                .contains("data-mode=\"system\""),
            "{}",
            page.display()
        );
    }
}

#[test]
fn copy_dir_keeps_the_tree_and_replaces_files() {
    let a = std::env::temp_dir().join(format!("dodai-copy-a-{}", std::process::id()));
    let b = std::env::temp_dir().join(format!("dodai-copy-b-{}", std::process::id()));
    let _ = fs::remove_dir_all(&a);
    let _ = fs::remove_dir_all(&b);
    fs::create_dir_all(a.join("x/y")).expect("mkdir");
    fs::write(a.join("x/y/f.txt"), "new").expect("write");
    fs::create_dir_all(b.join("x/y")).expect("mkdir");
    fs::write(b.join("x/y/f.txt"), "old").expect("write");
    dodaitech_website::copy_dir(&a, &b).expect("copy");
    assert_eq!(
        fs::read_to_string(b.join("x/y/f.txt")).expect("read"),
        "new"
    );
    assert!(dodaitech_website::copy_dir(&a.join("missing"), &b).is_err());
}

#[test]
fn the_watch_face_page_links_the_garmin_store_listing() {
    let html = fs::read_to_string(build().join("products/watchfaces/index.html")).expect("page");
    assert!(html.contains("https://apps.garmin.com/apps/bc5757e4-365b-4714-b4a7-b5d720cdc2d6"));
}
