//! Trunk post-build: split the pre-bindgen demo wasm and replace the files
//! Trunk already named in `index.html`.

use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    if let Err(err) = run() {
        eprintln!("dialtone-demo wasm-split failed: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let staging = PathBuf::from(std::env::var("TRUNK_STAGING_DIR").map_err(|_| {
        "TRUNK_STAGING_DIR is unset; run this from the Trunk post_build hook".to_string()
    })?);
    let profile = std::env::var("TRUNK_PROFILE").unwrap_or_else(|_| "debug".to_string());
    let html = fs::read_to_string(staging.join("index.html")).map_err(|err| err.to_string())?;
    // The boot screen fetches the stylesheet, the JS loader, and the main wasm
    // itself so each bar can count bytes. A preload would finish untracked.
    // The module script stays inert until that JS fetch completes.
    let html = hold_module_script(&defer_stylesheets(&strip_modulepreload(
        &strip_main_wasm_preload(&html),
    )));
    let js_name = asset_name(&html, &["from '/", "from \"./"], ".js")?;
    let wasm_name = asset_name(
        &html,
        &["module_or_path: '/", "module_or_path: \"./"],
        ".wasm",
    )?;
    let input = workspace_wasm(&profile)?;
    let input_bytes = fs::read(&input).map_err(|err| format!("read {}: {err}", input.display()))?;
    let input_len = input_bytes.len();

    let split_dir = std::env::temp_dir().join("dt-demo-wasm-split");
    let _ = fs::remove_dir_all(&split_dir);
    fs::create_dir_all(&split_dir).map_err(|err| err.to_string())?;
    let main_split = split_dir.join("main.wasm");
    let main_module = format!("./{js_name}");
    let mut opts = wasm_split_cli_support::Options::new(&input_bytes);
    opts.output_dir = &split_dir;
    opts.main_out_path = &main_split;
    opts.main_module = &main_module;
    opts.link_name = "./__wasm_split.js";
    let split = wasm_split_cli_support::transform(opts)
        .map_err(|err| format!("wasm-split failed: {err:#}"))?;
    if split.split_modules.is_empty() {
        return Err("wasm-split produced no chunks; #[lazy] split points were not found".into());
    }

    let bindgen_out = split_dir.join("bindgen");
    fs::create_dir_all(&bindgen_out).map_err(|err| err.to_string())?;
    let mut bindgen = wasm_bindgen_cli_support::Bindgen::new();
    bindgen
        .input_path(&main_split)
        .web(true)
        .map_err(|err| err.to_string())?
        .out_name("dialtone-demo")
        .keep_lld_exports(true)
        .demangle(false)
        .generate(&bindgen_out)
        .map_err(|err| format!("wasm-bindgen failed: {err}"))?;

    for entry in fs::read_dir(&split_dir).map_err(|err| err.to_string())? {
        let entry = entry.map_err(|err| err.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        let name = entry.file_name();
        if name == "main.wasm" {
            continue;
        }
        fs::copy(&path, staging.join(&name)).map_err(|err| err.to_string())?;
    }
    let snippets = bindgen_out.join("snippets");
    if snippets.is_dir() {
        copy_dir(&snippets, &staging.join("snippets"))?;
    }
    fs::copy(bindgen_out.join("dialtone-demo.js"), staging.join(&js_name))
        .map_err(|err| err.to_string())?;
    fs::copy(
        bindgen_out.join("dialtone-demo_bg.wasm"),
        staging.join(&wasm_name),
    )
    .map_err(|err| err.to_string())?;

    let served = fs::read(staging.join(&wasm_name)).map_err(|err| err.to_string())?;
    if served
        .windows(b"__wasm_split_placeholder__".len())
        .any(|window| window == b"__wasm_split_placeholder__")
    {
        return Err(
            "split wasm still imports __wasm_split_placeholder__; the link rewrite did not run"
                .into(),
        );
    }
    let style_bytes = style_byte_len(&staging, &html);
    let ui_bytes = fs::metadata(staging.join(&js_name))
        .map(|meta| meta.len())
        .unwrap_or(0);
    let html = stamp_boot_sizes(&html, style_bytes, ui_bytes, served.len() as u64);
    fs::write(staging.join("index.html"), html).map_err(|err| err.to_string())?;
    println!(
        "wasm-split: {} chunks, main {} KiB (cargo wasm {} KiB), loader {}",
        split.split_modules.len(),
        served.len() / 1024,
        input_len / 1024,
        js_name
    );
    Ok(())
}

fn filter_links(html: &str, drop: impl Fn(&str) -> bool) -> String {
    let mut out = String::with_capacity(html.len());
    let mut rest = html;
    while let Some(start) = rest.find("<link") {
        out.push_str(&rest[..start]);
        let after = &rest[start..];
        let Some(end) = after.find('>') else {
            out.push_str(after);
            return out;
        };
        let tag = &after[..=end];
        if !drop(tag) {
            out.push_str(tag);
        }
        rest = &after[end + 1..];
    }
    out.push_str(rest);
    out
}

/// Drop Trunk's preload of the main `*_bg.wasm`. The boot script fetches it.
fn strip_main_wasm_preload(html: &str) -> String {
    filter_links(html, |tag| {
        tag.contains("_bg.wasm") && tag.contains("rel=\"preload\"")
    })
}

/// Drop Trunk's JS modulepreload. The boot script fetches that file for the UI bar.
fn strip_modulepreload(html: &str) -> String {
    filter_links(html, |tag| tag.contains("rel=\"modulepreload\""))
}

/// Keep the module script in the page without running it. The boot script
/// starts it after the UI file has been fetched.
fn hold_module_script(html: &str) -> String {
    if html.contains("id=\"dt-boot-module\"") {
        return html.to_string();
    }
    let opening = "<script type=\"module\">";
    let Some(start) = html.find(opening) else {
        return html.to_string();
    };
    let after = &html[start..];
    let Some(end) = after.find("</script>") else {
        return html.to_string();
    };
    if !after[..end].contains("module_or_path") {
        return html.to_string();
    }
    let mut out = String::with_capacity(html.len() + 32);
    out.push_str(&html[..start]);
    out.push_str("<script type=\"text/plain\" id=\"dt-boot-module\">");
    out.push_str(&html[start + opening.len()..]);
    out
}

/// A `rel="stylesheet"` link blocks first paint until the file arrives.
/// The boot script fetches it and applies the text. A sheet already inside
/// `<noscript>` is left alone.
fn defer_stylesheets(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut rest = html;
    while let Some(start) = rest.find("<link") {
        out.push_str(&rest[..start]);
        let after = &rest[start..];
        let Some(end) = after.find('>') else {
            out.push_str(after);
            return out;
        };
        let tag = &after[..=end];
        let inside_noscript = out.trim_end().ends_with("<noscript>");
        if tag.contains("rel=\"stylesheet\"") && !inside_noscript {
            if let Some(href) = attr(tag, "href") {
                out.push_str(&format!(
                    "<meta id=\"dt-boot-style\" data-href=\"{href}\"><noscript><link rel=\"stylesheet\" href=\"{href}\"></noscript>"
                ));
            } else {
                out.push_str(tag);
            }
        } else {
            out.push_str(tag);
        }
        rest = &after[end + 1..];
    }
    out.push_str(rest);
    out
}

fn style_href(html: &str) -> Option<&str> {
    let id = "id=\"dt-boot-style\"";
    let at = html.find(id)?;
    let tag = &html[at..];
    let end = tag.find('>')?;
    attr(&tag[..=end], "data-href")
}

fn style_byte_len(staging: &Path, html: &str) -> u64 {
    let Some(href) = style_href(html) else {
        return 0;
    };
    let name = href.rsplit('/').next().unwrap_or(href);
    fs::metadata(staging.join(name))
        .map(|meta| meta.len())
        .unwrap_or(0)
}

/// Uncompressed sizes for the boot bars. Caddy's encode drops Content-Length,
/// and the decoded body matches these file sizes. `X-Asset-Size` is the same
/// number when the proxy copied the upstream length.
fn stamp_boot_sizes(html: &str, style_bytes: u64, ui_bytes: u64, app_bytes: u64) -> String {
    let marker = "id=\"dt-boot\"";
    if html.contains("data-style-bytes=") {
        return replace_boot_sizes(html, style_bytes, ui_bytes, app_bytes);
    }
    let Some(at) = html.find(marker) else {
        return html.to_string();
    };
    let mut out = String::with_capacity(html.len() + 80);
    out.push_str(&html[..at]);
    out.push_str(marker);
    out.push_str(&format!(
        " data-style-bytes=\"{style_bytes}\" data-ui-bytes=\"{ui_bytes}\" data-app-bytes=\"{app_bytes}\""
    ));
    out.push_str(&html[at + marker.len()..]);
    out
}

fn replace_boot_sizes(html: &str, style_bytes: u64, ui_bytes: u64, app_bytes: u64) -> String {
    let mut html = replace_attr(html, "data-style-bytes", style_bytes);
    html = replace_attr(&html, "data-ui-bytes", ui_bytes);
    replace_attr(&html, "data-app-bytes", app_bytes)
}

fn replace_attr(html: &str, name: &str, value: u64) -> String {
    let key = format!("{name}=\"");
    let Some(start) = html.find(&key) else {
        return html.to_string();
    };
    let value_at = start + key.len();
    let Some(end) = html[value_at..].find('"') else {
        return html.to_string();
    };
    let mut out = String::with_capacity(html.len());
    out.push_str(&html[..value_at]);
    out.push_str(&value.to_string());
    out.push_str(&html[value_at + end..]);
    out
}

fn attr<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let key = format!("{name}=\"");
    let start = tag.find(&key)? + key.len();
    let rest = &tag[start..];
    let end = rest.find('"')?;
    Some(&rest[..end])
}

fn asset_name(html: &str, starts: &[&str], suffix: &str) -> Result<String, String> {
    for start in starts {
        if let Some(at) = html.find(start) {
            let rest = &html[at + start.len()..];
            let Some(stop) = rest.find(suffix) else {
                continue;
            };
            let raw = &rest[..stop + suffix.len()];
            let name = raw.rsplit('/').next().unwrap_or(raw);
            if !name.is_empty() {
                return Ok(name.to_string());
            }
        }
    }
    Err(format!("index.html is missing a *{suffix} asset url"))
}

fn workspace_wasm(profile: &str) -> Result<PathBuf, String> {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        let manifest = dir.join("Cargo.toml");
        if manifest.is_file() {
            let text = fs::read_to_string(&manifest).unwrap_or_default();
            if text.contains("[workspace]") {
                let path = dir
                    .join("target/wasm32-unknown-unknown")
                    .join(profile)
                    .join("dialtone-demo.wasm");
                if path.is_file() {
                    return Ok(path);
                }
                return Err(format!("cargo wasm not found at {}", path.display()));
            }
        }
        if !dir.pop() {
            break;
        }
    }
    Err("workspace Cargo.toml not found from the split tool".into())
}

#[cfg(test)]
mod tests {
    use super::{
        defer_stylesheets, hold_module_script, stamp_boot_sizes, strip_main_wasm_preload,
        strip_modulepreload,
    };

    #[test]
    fn strip_drops_the_main_wasm_preload_and_keeps_the_js_modulepreload() {
        let html = r#"<head>
<link rel="modulepreload" href="/dialtone-demo-abc.js" crossorigin="anonymous">
<link rel="preload" href="/dialtone-demo-abc_bg.wasm" crossorigin="anonymous" as="fetch" type="application/wasm">
<script type="module">const wasm = await init({ module_or_path: '/dialtone-demo-abc_bg.wasm' });</script>
</head>"#;
        let stripped = strip_main_wasm_preload(html);
        assert!(!stripped.contains("rel=\"preload\""));
        assert!(stripped.contains("dialtone-demo-abc_bg.wasm"));
        assert!(stripped.contains("modulepreload"));
        assert!(stripped.contains("dialtone-demo-abc.js"));
    }

    #[test]
    fn strip_drops_the_js_modulepreload() {
        let html =
            r#"<link rel="modulepreload" href="/dialtone-demo-abc.js" crossorigin="anonymous">"#;
        let stripped = strip_modulepreload(html);
        assert!(!stripped.contains("modulepreload"));
        assert!(!stripped.contains("dialtone-demo-abc.js"));
    }

    #[test]
    fn hold_keeps_the_module_source_inert_and_is_stable() {
        let html = r#"<script type="module">import init from '/dialtone-demo-abc.js';
const wasm = await init({ module_or_path: '/dialtone-demo-abc_bg.wasm' });
</script>"#;
        let once = hold_module_script(html);
        let twice = hold_module_script(&once);
        assert_eq!(once, twice);
        assert!(once.contains("<script type=\"text/plain\" id=\"dt-boot-module\">"));
        assert!(!once.contains("type=\"module\""));
        assert!(once.contains("module_or_path: '/dialtone-demo-abc_bg.wasm'"));
    }

    #[test]
    fn defer_makes_the_main_stylesheet_non_blocking_and_is_stable() {
        let html = r#"<head>
<link rel="stylesheet" href="/tailwind-abc.css"/>
<link rel="modulepreload" href="/dialtone-demo-abc.js" crossorigin="anonymous">
</head>"#;
        let once = defer_stylesheets(html);
        let twice = defer_stylesheets(&once);
        assert_eq!(once, twice);
        assert!(once.contains("<meta id=\"dt-boot-style\" data-href=\"/tailwind-abc.css\">"));
        assert!(once
            .contains("<noscript><link rel=\"stylesheet\" href=\"/tailwind-abc.css\"></noscript>"));
        assert!(once.contains("modulepreload"));
        assert!(!once.contains("rel=\"preload\""));
        assert_eq!(once.matches("rel=\"stylesheet\"").count(), 1);
    }

    #[test]
    fn stamp_records_boot_sizes_and_replaces_them() {
        let html = r#"<div id="dt-boot" role="status"></div>"#;
        let once = stamp_boot_sizes(html, 10, 20, 30);
        let twice = stamp_boot_sizes(&once, 11, 21, 31);
        assert!(once.contains(
            "id=\"dt-boot\" data-style-bytes=\"10\" data-ui-bytes=\"20\" data-app-bytes=\"30\""
        ));
        assert!(twice.contains("data-style-bytes=\"11\""));
        assert!(twice.contains("data-ui-bytes=\"21\""));
        assert!(twice.contains("data-app-bytes=\"31\""));
        assert!(!twice.contains("data-style-bytes=\"10\""));
    }
}

fn copy_dir(from: &Path, to: &Path) -> Result<(), String> {
    fs::create_dir_all(to).map_err(|err| err.to_string())?;
    for entry in fs::read_dir(from).map_err(|err| err.to_string())? {
        let entry = entry.map_err(|err| err.to_string())?;
        let dest = to.join(entry.file_name());
        if entry.path().is_dir() {
            copy_dir(&entry.path(), &dest)?;
        } else {
            fs::copy(entry.path(), dest).map_err(|err| err.to_string())?;
        }
    }
    Ok(())
}
