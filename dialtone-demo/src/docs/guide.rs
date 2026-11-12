//! Dialtone staging component pages, rendered for the demo.
//!
//! Source: `apps/dialtone-documentation/docs/components` on `staging`.
//! Vue live demos are omitted. The prose, lists, tables, and non-demo
//! code samples stay.

const PORTED: &[&str] = &[
    "avatar",
    "badge",
    "banner",
    "box",
    "breadcrumbs",
    "button",
    "button-group",
    "card",
    "checkbox",
    "checkbox-group",
    "chip",
    "collapsible",
    "combobox",
    "combobox-multi-select",
    "combobox-with-popover",
    "datepicker",
    "description-list",
    "dropdown",
    "hovercard",
    "icon",
    "illustration",
    "image-viewer",
    "input",
    "input-group",
    "item-layout",
    "keyboard-shortcut",
    "lazy-show",
    "link",
    "list-item",
    "list-item-group",
    "loader",
    "modal",
    "mode-island",
    "motion-text",
    "notice",
    "pagination",
    "popover",
    "presence",
    "progress-circle",
    "prose",
    "radio",
    "radio-group",
    "root-layout",
    "skeleton",
    "slider",
    "table",
    "toast",
    "toggle",
    "tooltip",
];

pub(crate) fn summary(id: &str) -> Option<&'static str> {
    field(frontmatter(source(id)?), "description")
}

pub(crate) fn storybook(id: &str) -> Option<&'static str> {
    let meta = frontmatter(source(id)?);
    field(meta, "storybook").filter(|url| url.starts_with("http"))
}

pub(crate) fn body_html(id: &str) -> Option<String> {
    let raw = source(id)?;
    let body = after_frontmatter(raw);
    let html = render(body);
    if html.trim().is_empty() {
        None
    } else {
        Some(html)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct TocItem {
    pub level: u8,
    pub id: String,
    pub text: String,
}

/// Headings rendered into the component article, in order. Vue API and Classes
/// are omitted by the same skip the body uses.
pub(crate) fn toc(id: &str) -> Vec<TocItem> {
    body_html(id)
        .map(|html| toc_from_html(&html))
        .unwrap_or_default()
}

fn toc_from_html(html: &str) -> Vec<TocItem> {
    let mut items = Vec::new();
    let mut rest = html;
    while let Some(start) = rest.find("<h") {
        let slice = &rest[start..];
        let level_byte = slice.as_bytes().get(2).copied().unwrap_or(0);
        if !matches!(level_byte, b'2' | b'3' | b'4') {
            rest = &rest[start + 2..];
            continue;
        }
        let Some(tag_end) = slice.find('>') else {
            break;
        };
        let Some(id) = attr(&slice[..tag_end], "id") else {
            rest = &slice[tag_end + 1..];
            continue;
        };
        let inner = &slice[tag_end + 1..];
        let Some(close_at) = inner.find("</h") else {
            break;
        };
        let text = heading_text(&inner[..close_at]);
        if !text.is_empty() {
            items.push(TocItem {
                level: level_byte - b'0',
                id,
                text,
            });
        }
        rest = &inner[close_at..];
    }
    items
}

fn attr(open: &str, name: &str) -> Option<String> {
    let key = format!("{name}=\"");
    let (_, tail) = open.split_once(&key)?;
    let (value, _) = tail.split_once('"')?;
    Some(value.to_string())
}

fn heading_text(inner: &str) -> String {
    let mut plain = String::new();
    let mut in_tag = false;
    for ch in inner.chars() {
        if ch == '<' {
            in_tag = true;
            continue;
        }
        if in_tag {
            if ch == '>' {
                in_tag = false;
            }
            continue;
        }
        plain.push(ch);
    }
    decode_entities(&plain)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn source(id: &str) -> Option<&'static str> {
    Some(match id {
        "avatar" => include_str!("content/avatar.md"),
        "badge" => include_str!("content/badge.md"),
        "banner" => include_str!("content/banner.md"),
        "box" => include_str!("content/box.md"),
        "breadcrumbs" => include_str!("content/breadcrumbs.md"),
        "button" => include_str!("content/button.md"),
        "button-group" => include_str!("content/button-group.md"),
        "card" => include_str!("content/card.md"),
        "checkbox" => include_str!("content/checkbox.md"),
        "checkbox-group" => include_str!("content/checkbox-group.md"),
        "chip" => include_str!("content/chip.md"),
        "collapsible" => include_str!("content/collapsible.md"),
        "combobox" => include_str!("content/combobox.md"),
        "combobox-multi-select" => include_str!("content/combobox-multi-select.md"),
        "combobox-with-popover" => include_str!("content/combobox-with-popover.md"),
        "datepicker" => include_str!("content/datepicker.md"),
        "description-list" => include_str!("content/description-list.md"),
        "dropdown" => include_str!("content/dropdown.md"),
        "hovercard" => include_str!("content/hovercard.md"),
        "icon" => include_str!("content/icon.md"),
        "illustration" => include_str!("content/illustration.md"),
        "image-viewer" => include_str!("content/image-viewer.md"),
        "input" => include_str!("content/input.md"),
        "input-group" => include_str!("content/input-group.md"),
        "item-layout" => include_str!("content/item-layout.md"),
        "keyboard-shortcut" => include_str!("content/keyboard-shortcut.md"),
        "lazy-show" => include_str!("content/lazy-show.md"),
        "link" => include_str!("content/link.md"),
        "list-item" => include_str!("content/list-item.md"),
        "list-item-group" => include_str!("content/list-item-group.md"),
        "loader" => include_str!("content/loader.md"),
        "modal" => include_str!("content/modal.md"),
        "mode-island" => include_str!("content/mode-island.md"),
        "motion-text" => include_str!("content/motion-text.md"),
        "notice" => include_str!("content/notice.md"),
        "pagination" => include_str!("content/pagination.md"),
        "popover" => include_str!("content/popover.md"),
        "presence" => include_str!("content/presence.md"),
        "progress-circle" => include_str!("content/progress-circle.md"),
        "prose" => include_str!("content/prose.md"),
        "radio" => include_str!("content/radio.md"),
        "radio-group" => include_str!("content/radio-group.md"),
        "root-layout" => include_str!("content/root-layout.md"),
        "skeleton" => include_str!("content/skeleton.md"),
        "slider" => include_str!("content/slider.md"),
        "table" => include_str!("content/table.md"),
        "tab-group" => include_str!("content/tabs.md"),
        "toast" => include_str!("content/toast.md"),
        "toggle" => include_str!("content/toggle.md"),
        "tooltip" => include_str!("content/tooltip.md"),
        _ => return None,
    })
}

fn frontmatter(raw: &'static str) -> &'static str {
    let rest = raw.strip_prefix("---\n").unwrap_or(raw);
    rest.split_once("\n---").map(|(meta, _)| meta).unwrap_or("")
}

fn after_frontmatter(raw: &str) -> &str {
    let Some(rest) = raw.strip_prefix("---\n") else {
        return raw;
    };
    rest.split_once("\n---\n")
        .map(|(_, body)| body)
        .unwrap_or(rest)
}

fn field<'a>(meta: &'a str, key: &str) -> Option<&'a str> {
    let prefix = format!("{key}:");
    for line in meta.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix(&prefix) else {
            continue;
        };
        let rest = rest.trim();
        let rest = rest
            .strip_prefix('"')
            .and_then(|value| value.strip_suffix('"'))
            .unwrap_or(rest);
        let rest = rest
            .strip_prefix('\'')
            .and_then(|value| value.strip_suffix('\''))
            .unwrap_or(rest);
        if rest.is_empty() || rest == "planned" || rest == "null" || rest == "~" {
            return None;
        }
        return Some(rest);
    }
    None
}

fn render(body: &str) -> String {
    let body = cut_between(&body, "<script", "</script>");
    let body = cut_between(&body, "<!--", "-->");
    let (body, fences) = lift_fences(&body);
    let body = convert_usage(&body);
    let body = convert_html_tables(&body);
    let body = cut_custom_elements(&body);
    let body = strip_tags(&body);
    let body = decode_entities(&body);
    let body = restore_fences(&body, &fences);
    markdown(&body)
}

fn cut_between(input: &str, start: &str, end: &str) -> String {
    let mut out = String::new();
    let mut rest = input;
    while let Some(at) = rest.find(start) {
        out.push_str(&rest[..at]);
        let after = &rest[at + start.len()..];
        if let Some(stop) = after.find(end) {
            rest = &after[stop + end.len()..];
        } else {
            rest = "";
            break;
        }
    }
    out.push_str(rest);
    out
}

fn lift_fences(input: &str) -> (String, Vec<String>) {
    let mut out = String::new();
    let mut fences = Vec::new();
    let mut rest = input;
    while let Some(at) = rest.find("```") {
        out.push_str(&rest[..at]);
        let after = &rest[at + 3..];
        if let Some(stop) = after.find("\n```") {
            let block = &rest[at..at + 3 + stop + 4];
            let token = format!("\u{0}FENCE{} \u{0}", fences.len());
            fences.push(block.to_string());
            out.push_str(&token);
            rest = &after[stop + 4..];
        } else {
            out.push_str(&rest[at..]);
            break;
        }
    }
    out.push_str(rest);
    // Also protect inline code so tag stripping leaves `<button>` inside it.
    let mut protected = String::new();
    let mut inline = Vec::new();
    let mut chars = out.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '`' {
            let mut code = String::new();
            let mut closed = false;
            for next in chars.by_ref() {
                if next == '`' {
                    closed = true;
                    break;
                }
                if next == '\n' {
                    code.push('\n');
                    break;
                }
                code.push(next);
            }
            if closed {
                let token = format!("\u{0}CODE{} \u{0}", fences.len() + inline.len());
                inline.push(code);
                protected.push_str(&token);
            } else {
                protected.push('`');
                protected.push_str(&code);
            }
        } else {
            protected.push(ch);
        }
    }
    fences.extend(
        inline
            .into_iter()
            .map(|code| format!("\u{0}INLINE\u{0}{code}")),
    );
    (protected, fences)
}

fn restore_fences(input: &str, fences: &[String]) -> String {
    let mut out = input.to_string();
    for (index, block) in fences.iter().enumerate() {
        if let Some(code) = block.strip_prefix("\u{0}INLINE\u{0}") {
            let token = format!("\u{0}CODE{index} \u{0}");
            out = out.replace(&token, &format!("`{code}`"));
        } else {
            let token = format!("\u{0}FENCE{index} \u{0}");
            out = out.replace(&token, block);
        }
    }
    out
}

fn convert_usage(input: &str) -> String {
    let mut out = String::new();
    let mut rest = input;
    while let Some(at) = find_tag_start(rest, "dialtone-usage") {
        out.push_str(&rest[..at]);
        let after = &rest[at..];
        let Some(end_rel) = find_close(after, "dialtone-usage") else {
            break;
        };
        let block = &after[..end_rel];
        out.push('\n');
        if let Some(items) = template_body(block, "do") {
            out.push_str("**Do**\n\n");
            out.push_str(items.trim());
            out.push_str("\n\n");
        }
        if let Some(items) = template_body(block, "dont") {
            out.push_str("**Don't**\n\n");
            out.push_str(items.trim());
            out.push_str("\n\n");
        }
        rest = &after[end_rel..];
    }
    out.push_str(rest);
    out
}

fn template_body<'a>(block: &'a str, name: &str) -> Option<&'a str> {
    let needle = format!("<template #{name}");
    let start = block.find(&needle)?;
    let after = &block[start + needle.len()..];
    let content_at = after.find('>')? + 1;
    let content = &after[content_at..];
    let end = content.find("</template>")?;
    Some(&content[..end])
}

fn convert_html_tables(input: &str) -> String {
    let mut out = String::new();
    let mut rest = input;
    while let Some(at) = find_tag_start(rest, "table") {
        out.push_str(&rest[..at]);
        let after = &rest[at..];
        let Some(end_rel) = find_close(after, "table") else {
            out.push_str(after);
            return out;
        };
        let table = &after[..end_rel];
        out.push('\n');
        out.push_str(&html_table_to_markdown(table));
        out.push('\n');
        rest = &after[end_rel..];
    }
    out.push_str(rest);
    out
}

fn html_table_to_markdown(table: &str) -> String {
    let mut rows = Vec::new();
    let mut rest = table;
    while let Some(at) = find_tag_start(rest, "tr") {
        let after = &rest[at..];
        let Some(end_rel) = find_close(after, "tr") else {
            break;
        };
        let row = &after[..end_rel];
        let cells = row_cells(row);
        if cells.iter().any(|cell| !cell.is_empty()) {
            rows.push(cells);
        }
        rest = &after[end_rel..];
    }
    if rows.len() < 2 {
        return String::new();
    }
    let width = rows.iter().map(|row| row.len()).max().unwrap_or(0);
    let mut lines = Vec::new();
    for (index, row) in rows.iter().enumerate() {
        let mut cells = row.clone();
        cells.resize(width, String::new());
        lines.push(format!("| {} |", cells.join(" | ")));
        if index == 0 {
            lines.push(format!(
                "| {} |",
                (0..width).map(|_| "---").collect::<Vec<_>>().join(" | ")
            ));
        }
    }
    lines.join("\n")
}

fn row_cells(row: &str) -> Vec<String> {
    let mut cells = Vec::new();
    let mut rest = row;
    loop {
        let th = find_tag_start(rest, "th");
        let td = find_tag_start(rest, "td");
        let (at, name) = match (th, td) {
            (Some(th_at), Some(td_at)) if th_at < td_at => (th_at, "th"),
            (Some(_), Some(td_at)) => (td_at, "td"),
            (Some(th_at), None) => (th_at, "th"),
            (None, Some(td_at)) => (td_at, "td"),
            (None, None) => break,
        };
        let after = &rest[at..];
        let Some(end_rel) = find_close(after, name) else {
            break;
        };
        let cell = &after[..end_rel];
        let inner_at = cell.find('>').map(|index| index + 1).unwrap_or(0);
        let inner = cell.get(inner_at..).unwrap_or("");
        let inner = inner.strip_suffix(&format!("</{name}>")).unwrap_or(inner);
        cells.push(clean_cell(inner));
        rest = &after[end_rel..];
    }
    cells
}

fn clean_cell(value: &str) -> String {
    let without_mustache = cut_between(value, "{{", "}}");
    let text = strip_tags(&without_mustache);
    let text = decode_entities(&text);
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn cut_custom_elements(input: &str) -> String {
    let mut out = String::new();
    let mut rest = input;
    while let Some(at) = rest.find('<') {
        out.push_str(&rest[..at]);
        let after = &rest[at..];
        let name = tag_name(after);
        if is_custom(&name) {
            if let Some(end_rel) = find_close(after, &name) {
                rest = &after[end_rel..];
                continue;
            }
        }
        if let Some(end) = after.find('>') {
            if is_custom(&name) {
                rest = &after[end + 1..];
                continue;
            }
            out.push_str(&after[..=end]);
            rest = &after[end + 1..];
        } else {
            out.push_str(after);
            break;
        }
    }
    out.push_str(rest);
    out
}

fn is_custom(name: &str) -> bool {
    let name = name.trim_start_matches('/');
    name.starts_with("component-")
        || name.starts_with("example-")
        || name.starts_with("Button")
        || name.starts_with("Dt")
        || name == "dialtone-usage"
}

fn strip_tags(input: &str) -> String {
    let mut out = String::new();
    let mut rest = input;
    while let Some(at) = rest.find('<') {
        out.push_str(&rest[..at]);
        let after = &rest[at + 1..];
        let starts_tag = after
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_alphabetic() || ch == '/' || ch == '!');
        if starts_tag {
            if let Some(end) = after.find('>') {
                let name = tag_name(&rest[at..]);
                if matches!(
                    name.as_str(),
                    "div" | "p" | "section" | "br" | "li" | "ul" | "ol" | "dt-stack" | "h2"
                ) {
                    out.push('\n');
                }
                rest = &after[end + 1..];
                continue;
            }
        }
        out.push('<');
        rest = after;
    }
    out.push_str(rest);
    out
}

fn tag_name(at_tag: &str) -> String {
    let bytes = at_tag.trim_start_matches('<').trim_start_matches('/');
    bytes
        .chars()
        .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '-')
        .collect()
}

fn find_tag_start(input: &str, name: &str) -> Option<usize> {
    let patterns = [format!("<{name}"), format!("<{name}>"), format!("</{name}")];
    let mut best = None;
    for pattern in &patterns[..2] {
        if let Some(at) = input.find(pattern) {
            let boundary = input[at + pattern.len()..].chars().next();
            let ok = pattern.ends_with('>')
                || boundary.is_none()
                || boundary.is_some_and(|ch| !ch.is_ascii_alphanumeric() && ch != '-');
            if ok {
                best = Some(best.map_or(at, |current: usize| current.min(at)));
            }
        }
    }
    let _ = patterns;
    best
}

fn find_close(at_tag: &str, name: &str) -> Option<usize> {
    let close = format!("</{name}>");
    if let Some(end) = at_tag.find(&close) {
        return Some(end + close.len());
    }
    // Self-closing or void start tag.
    let start_end = at_tag.find('>')?;
    let head = &at_tag[..=start_end];
    if head.contains("/>") || !at_tag[start_end + 1..].contains(&format!("<{name}")) {
        // Only treat as the whole element when there is no close tag.
        if !at_tag.contains(&close) {
            return Some(start_end + 1);
        }
    }
    None
}

fn decode_entities(input: &str) -> String {
    input
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
}

fn markdown(input: &str) -> String {
    let mut html = String::new();
    let lines: Vec<&str> = input.lines().collect();
    let mut index = 0;
    let mut slugs = Vec::<String>::new();
    let mut skip = false;
    while index < lines.len() {
        let line = lines[index];
        let trimmed = line.trim();
        if trimmed.is_empty() {
            index += 1;
            continue;
        }
        if trimmed.starts_with("```") {
            let info = trimmed.trim_start_matches('`').trim();
            let mut block = Vec::new();
            index += 1;
            while index < lines.len() && !lines[index].trim().starts_with("```") {
                block.push(lines[index]);
                index += 1;
            }
            if index < lines.len() {
                index += 1;
            }
            if skip || info.contains("demo") {
                continue;
            }
            html.push_str("<div class=\"docs-code-shell\"><pre class=\"docs-code\"><code>");
            html.push_str(&super::highlight::code(&block.join("\n")));
            html.push_str("</code></pre>");
            html.push_str(super::copy::copy_button_markup());
            html.push_str("</div>\n");
            continue;
        }
        if let Some(heading) = heading(trimmed) {
            let (level, text) = heading;
            if level <= 2 {
                skip = matches!(text.trim(), "Vue API" | "Classes")
                    || text.trim().starts_with("image:");
            }
            index += 1;
            if skip {
                continue;
            }
            let slug = unique_slug(&mut slugs, text);
            html.push_str(&format!(
                "<h{level} id=\"doc-{slug}\">{}</h{level}>\n",
                inline(text)
            ));
            continue;
        }
        if skip {
            index += 1;
            continue;
        }
        if trimmed.starts_with('|') {
            let mut rows = Vec::new();
            while index < lines.len() && lines[index].trim().starts_with('|') {
                rows.push(lines[index].trim());
                index += 1;
            }
            html.push_str(&markdown_table(&rows));
            continue;
        }
        if trimmed.starts_with('>') {
            let mut quote = Vec::new();
            while index < lines.len() && lines[index].trim().starts_with('>') {
                quote.push(lines[index].trim().trim_start_matches('>').trim_start());
                index += 1;
            }
            html.push_str(&callout(&quote));
            continue;
        }
        if is_list_item(trimmed) {
            let ordered = trimmed.chars().next().is_some_and(|ch| ch.is_ascii_digit());
            let tag = if ordered { "ol" } else { "ul" };
            html.push_str(&format!("<{tag}>\n"));
            while index < lines.len() && is_list_item(lines[index].trim()) {
                html.push_str("<li>");
                html.push_str(&inline(list_text(lines[index].trim())));
                html.push_str("</li>\n");
                index += 1;
            }
            html.push_str(&format!("</{tag}>\n"));
            continue;
        }
        let mut para = vec![trimmed];
        index += 1;
        while index < lines.len() {
            let next = lines[index].trim();
            if next.is_empty()
                || next.starts_with("```")
                || heading(next).is_some()
                || next.starts_with('|')
                || next.starts_with('>')
                || is_list_item(next)
            {
                break;
            }
            para.push(next);
            index += 1;
        }
        html.push_str("<p>");
        html.push_str(&inline(&para.join(" ")));
        html.push_str("</p>\n");
    }
    html
}

fn heading(line: &str) -> Option<(usize, &str)> {
    let level = line.chars().take_while(|ch| *ch == '#').count();
    if (1..=6).contains(&level) && line.chars().nth(level) == Some(' ') {
        Some((level, line[level + 1..].trim()))
    } else {
        None
    }
}

fn is_list_item(line: &str) -> bool {
    let line = line.trim_start();
    if let Some(rest) = line.strip_prefix("- ").or_else(|| line.strip_prefix("* ")) {
        return !rest.is_empty();
    }
    let mut chars = line.chars();
    let mut saw_digit = false;
    while chars.clone().next().is_some_and(|ch| ch.is_ascii_digit()) {
        saw_digit = true;
        chars.next();
    }
    saw_digit && chars.next() == Some('.') && chars.next() == Some(' ')
}

fn list_text(line: &str) -> &str {
    let line = line.trim_start();
    if let Some(rest) = line.strip_prefix("- ").or_else(|| line.strip_prefix("* ")) {
        return rest;
    }
    line.split_once(". ").map(|(_, rest)| rest).unwrap_or(line)
}

fn unique_slug(seen: &mut Vec<String>, text: &str) -> String {
    let mut slug = String::new();
    let mut dash = false;
    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            dash = false;
        } else if !dash && !slug.is_empty() {
            slug.push('-');
            dash = true;
        }
    }
    let slug = slug.trim_matches('-').to_string();
    let slug = if slug.is_empty() {
        "section".to_string()
    } else {
        slug
    };
    let mut candidate = slug.clone();
    let mut n = 2;
    while seen.iter().any(|existing| existing == &candidate) {
        candidate = format!("{slug}-{n}");
        n += 1;
    }
    seen.push(candidate.clone());
    candidate
}

fn markdown_table(rows: &[&str]) -> String {
    let parsed: Vec<Vec<String>> = rows
        .iter()
        .filter(|row| !is_separator(row))
        .map(|row| split_row(row))
        .filter(|cells| cells.iter().any(|cell| !cell.is_empty()))
        .collect();
    if parsed.is_empty() {
        return String::new();
    }
    let mut html = String::from("<table>\n");
    for (index, cells) in parsed.iter().enumerate() {
        let tag = if index == 0 { "th" } else { "td" };
        html.push_str("<tr>");
        for cell in cells {
            html.push('<');
            html.push_str(tag);
            html.push('>');
            html.push_str(&inline(cell));
            html.push_str("</");
            html.push_str(tag);
            html.push('>');
        }
        html.push_str("</tr>\n");
    }
    html.push_str("</table>\n");
    html
}

fn is_separator(row: &str) -> bool {
    split_row(row)
        .iter()
        .all(|cell| !cell.is_empty() && cell.chars().all(|ch| ch == '-' || ch == ':' || ch == ' '))
}

fn split_row(row: &str) -> Vec<String> {
    row.trim()
        .trim_start_matches('|')
        .trim_end_matches('|')
        .split('|')
        .map(|cell| cell.trim().to_string())
        .collect()
}

fn callout(lines: &[&str]) -> String {
    if lines.is_empty() {
        return String::new();
    }
    let mut rest = lines;
    let mut kind = "note";
    let mut title = "";
    if let Some(stripped) = lines[0].strip_prefix("[!") {
        if let Some((name, label)) = stripped.split_once(']') {
            let lowered = name.trim().to_ascii_lowercase();
            kind = match lowered.as_str() {
                "warning" => "warning",
                "info" => "info",
                _ => "note",
            };
            title = label.trim();
            rest = &lines[1..];
        }
    }
    let mut html = format!("<aside class=\"docs-callout docs-callout--{kind}\">");
    if !title.is_empty() {
        html.push_str("<p class=\"docs-callout-title\">");
        html.push_str(&inline(title));
        html.push_str("</p>");
    }
    if !rest.is_empty() {
        html.push_str("<p>");
        html.push_str(&inline(&rest.join(" ")));
        html.push_str("</p>");
    }
    html.push_str("</aside>\n");
    html
}

fn inline(input: &str) -> String {
    let mut html = String::new();
    let chars: Vec<char> = input.chars().collect();
    let mut index = 0;
    while index < chars.len() {
        if chars[index] == '`' {
            if let Some(end) = chars[index + 1..].iter().position(|ch| *ch == '`') {
                let code: String = chars[index + 1..index + 1 + end].iter().collect();
                html.push_str("<code>");
                html.push_str(&escape(&code));
                html.push_str("</code>");
                index += end + 2;
                continue;
            }
        }
        if chars[index] == '*' && chars.get(index + 1) == Some(&'*') {
            if let Some(end) = find_seq(&chars, index + 2, &['*', '*']) {
                let inner: String = chars[index + 2..end].iter().collect();
                html.push_str("<strong>");
                html.push_str(&inline(&inner));
                html.push_str("</strong>");
                index = end + 2;
                continue;
            }
        }
        if chars[index] == '*' {
            if let Some(end) = chars[index + 1..].iter().position(|ch| *ch == '*') {
                let inner: String = chars[index + 1..index + 1 + end].iter().collect();
                if !inner.is_empty() && !inner.starts_with(' ') && !inner.ends_with(' ') {
                    html.push_str("<em>");
                    html.push_str(&inline(&inner));
                    html.push_str("</em>");
                    index += end + 2;
                    continue;
                }
            }
        }
        if chars[index] == '[' {
            if let Some((text, url, next)) = parse_link(&chars, index) {
                let href = rewrite_url(&url);
                html.push_str("<a href=\"");
                html.push_str(&escape(&href));
                html.push('"');
                if href.starts_with("http") {
                    html.push_str(" target=\"_blank\" rel=\"noreferrer\"");
                }
                html.push('>');
                html.push_str(&inline(&text));
                html.push_str("</a>");
                index = next;
                continue;
            }
        }
        if chars[index] == '!' && chars.get(index + 1) == Some(&'[') {
            if let Some((text, _url, next)) = parse_link(&chars, index + 1) {
                html.push_str(&inline(&text));
                index = next;
                continue;
            }
        }
        html.push_str(&escape(&chars[index].to_string()));
        index += 1;
    }
    html
}

fn find_seq(chars: &[char], from: usize, seq: &[char]) -> Option<usize> {
    chars[from..]
        .windows(seq.len())
        .position(|window| window == seq)
        .map(|offset| from + offset)
}

fn parse_link(chars: &[char], start: usize) -> Option<(String, String, usize)> {
    let close = chars[start + 1..].iter().position(|ch| *ch == ']')?;
    let text: String = chars[start + 1..start + 1 + close].iter().collect();
    let after = start + 1 + close + 1;
    if chars.get(after) != Some(&'(') {
        return None;
    }
    let end = chars[after + 1..].iter().position(|ch| *ch == ')')?;
    let mut url: String = chars[after + 1..after + 1 + end].iter().collect();
    if let Some((href, _)) = url.split_once(" \"") {
        url = href.to_string();
    }
    Some((text, url, after + 1 + end + 1))
}

fn rewrite_url(url: &str) -> String {
    let url = url.trim();
    if url.starts_with("http://")
        || url.starts_with("https://")
        || url.starts_with('#')
        || url.starts_with("mailto:")
    {
        return url.to_string();
    }
    let (path, hash) = url
        .split_once('#')
        .map(|(path, hash)| (path, Some(hash)))
        .unwrap_or((url, None));
    let file = path.rsplit('/').next().unwrap_or(path);
    let stem = file.trim_end_matches(".md").trim_end_matches(".html");
    if !path.contains("foundations")
        && PORTED.contains(&stem)
        && (file.ends_with(".md") || file.ends_with(".html"))
    {
        let mut href = format!("#/components/{stem}");
        if let Some(hash) = hash {
            href.push('#');
            href.push_str(hash);
        }
        return href;
    }
    let mut site = if path.starts_with('/') {
        format!("https://dialtone.dialpad.com{path}")
    } else if file.ends_with(".md") || file.ends_with(".html") {
        format!("https://dialtone.dialpad.com/components/{file}")
    } else {
        return url.to_string();
    };
    site = site.replace(".md", ".html");
    if let Some(hash) = hash {
        if !site.contains('#') {
            site.push('#');
            site.push_str(hash);
        }
    }
    site
}

fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn avatar_summary_and_sections() {
        assert_eq!(
            summary("avatar"),
            Some("Visual identifier for a user, object, or participant group.")
        );
        let html = body_html("avatar").expect("avatar");
        assert!(html.contains("id=\"doc-usage\""));
        assert!(html.contains("id=\"doc-accessibility\""));
        assert!(html.contains("<table>"));
        assert!(html.contains("16px"));
        assert!(!html.contains("dt-avatar"));
        assert!(!html.contains("Vue API"));
        assert!(html.contains("WCAG"));
    }

    #[test]
    fn badge_usage_lists_and_internal_links() {
        let html = body_html("badge").expect("badge");
        assert!(html.contains("<strong>Do</strong>"));
        assert!(html.contains("<strong>Don't</strong>"));
        assert!(html.contains("aria-label"));
    }

    #[test]
    fn button_keeps_guidelines_and_drops_live_demos() {
        let html = body_html("button").expect("button");
        assert!(html.contains("id=\"doc-writing-guidelines\""));
        assert!(html.contains("<strong>verb + noun</strong>") || html.contains("verb + noun"));
        assert!(!html.contains("Place Call"));
        assert!(html.contains("href=\"#/components/modal\"") || html.contains("Modal"));
    }

    #[test]
    fn deeper_headings_are_rendered() {
        let html = body_html("tooltip").expect("tooltip");
        assert!(html.contains("<h4 id=\"doc-default\">"));
        assert!(!html.contains("####"));
    }

    #[test]
    fn code_fences_have_a_copy_control() {
        let html = body_html("skeleton").expect("skeleton");
        assert!(html.contains("docs-code-shell"));
        assert!(html.contains("data-docs-copy"));
        assert!(html.contains("token tag\">dt-skeleton"));
        assert!(html.contains("Loaded content"));
        assert!(!html.contains("d-w-400"));
    }

    #[test]
    fn codeblock_has_no_page() {
        assert!(summary("codeblock").is_none());
        assert!(body_html("codeblock").is_none());
        assert!(toc("codeblock").is_empty());
    }

    #[test]
    fn checkbox_toc_follows_the_article() {
        let items = toc("checkbox");
        assert_eq!(
            items.first().map(|item| item.id.as_str()),
            Some("doc-usage")
        );
        assert_eq!(items.first().map(|item| item.text.as_str()), Some("Usage"));
        assert!(items
            .iter()
            .any(|item| item.level == 3 && item.text == "Best Practices"));
        assert!(!items
            .iter()
            .any(|item| item.text == "Vue API" || item.text == "Classes"));
    }
}
