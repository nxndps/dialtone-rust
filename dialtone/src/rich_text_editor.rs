//! Dialtone Vue `DtRichTextEditor` (`dialtone/v10.7.0`).
//!
//! The Vue component hosts a TipTap editor. This port is a `contenteditable`
//! surface with the same shell: `d-rich-text-editor` and `data-qa="dt-rich-text-editor"`
//! around a `ProseMirror` element (`inputClass`, `role="textbox"`, `aria-multiline`,
//! `aria-label` from `inputAriaLabel`). Output is `text`, `json`, `html`, or
//! `markdown` (`outputFormat`, default `html`).
//!
//! Bold, italic, underline, strike, code, lists, blockquote, and code block use
//! the browser editing commands when the matching `allow*` prop is true. Enter
//! emits `enter` unless `allowLineBreaks` is set. Shift+Enter inserts a hard
//! break in that default mode. `link` autolinks a URL once a space follows it,
//! opens the Edit / Open link / Remove bubble, and emits `link-click`.
//! [`force_linkify_pending_text`] links a trailing URL. It does nothing when
//! `link` is off or `customLink` is on.
//!
//! Not applied: TipTap mention, channel, and slash-command suggestions,
//! `linkPhoneNumbers`, variables, font color, background, family, size, line
//! height, image resize, custom link phone and IP marks, `additionalExtensions`,
//! and the ProseMirror undo history. `preserveWhitespace` `false` collapses
//! whitespace outside `pre`. `full` and `true` keep it. Table and image tags are
//! kept only when `allowTables` or `allowInlineImages` is set. `autoFocus`
//! `start`, `end`, `all`, `true`, and a character offset are applied once.

use leptos::html;
use leptos::portal::Portal;
use leptos::prelude::*;
use serde_json::{Map, Value};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, KeyboardEvent, MouseEvent, Node};

use crate::button::DtButton;
use crate::stack::DtStack;

/// Vue `RICH_TEXT_EDITOR_OUTPUT_FORMATS`.
pub const RICH_TEXT_EDITOR_OUTPUT_FORMATS: &[&str] = &["text", "json", "html", "markdown"];

/// Vue `RICH_TEXT_EDITOR_AUTOFOCUS_TYPES`. `true` focuses the start. A number focuses that character.
pub const RICH_TEXT_EDITOR_AUTOFOCUS_TYPES: &[&str] = &["start", "end", "all"];

/// Vue `RICH_TEXT_EDITOR_SUPPORTED_LINK_PROTOCOLS`.
pub const RICH_TEXT_EDITOR_SUPPORTED_LINK_PROTOCOLS: &[&str] =
    &["https", "http", "ftp", "ftps", "sftp", "mailto"];

const EDITOR_SHELL_CLASS: &str = "d-rich-text-editor";
const EDITOR_QA: &str = "dt-rich-text-editor";
const PROSE_CLASS: &str = "ProseMirror";
const LINK_CLASS: &str = "d-link d-wb-break-all";
const CODE_BLOCK_CLASS: &str = "d-rich-text-editor__code-block";
/// Tooltip overlays in this crate use 400. The Vue bubble menu does not set a z-index.
const BUBBLE_Z_INDEX: i32 = 400;

/// Payload for `edit-link`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DtRichTextEditorLink {
    pub href: String,
    pub text: String,
}

/// Payload for `link-click`. The Vue event also carries the `MouseEvent`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DtRichTextEditorLinkClick {
    pub href: String,
    pub text: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Marks {
    bold: bool,
    italic: bool,
    strike: bool,
    underline: bool,
    code: bool,
    codeblock: bool,
    blockquote: bool,
    list: bool,
    link: bool,
    inline_images: bool,
    tables: bool,
    text_align: bool,
    use_div: bool,
    collapse_whitespace: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Autofocus {
    Off,
    Start,
    End,
    All,
    Position(u32),
}

#[derive(Clone, Debug)]
struct HtmlNode {
    name: String,
    attrs: Vec<(String, String)>,
    children: Vec<HtmlNode>,
    text: String,
}

impl HtmlNode {
    fn text(text: String) -> Self {
        Self {
            name: String::new(),
            attrs: Vec::new(),
            children: Vec::new(),
            text,
        }
    }

    fn elem(name: &str, attrs: Vec<(String, String)>, children: Vec<HtmlNode>) -> Self {
        Self {
            name: name.to_string(),
            attrs,
            children,
            text: String::new(),
        }
    }

    fn is_text(&self) -> bool {
        self.name.is_empty()
    }
}

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn prose_mirror_class(input_class: &str) -> String {
    join_class([PROSE_CLASS, input_class])
}

fn block_tag(use_div: bool) -> &'static str {
    if use_div {
        "div"
    } else {
        "p"
    }
}

fn empty_block(use_div: bool) -> String {
    let tag = block_tag(use_div);
    format!("<{tag}></{tag}>")
}

fn normalize_output_format(format: &str) -> &'static str {
    match format {
        "" | "html" => "html",
        "text" => "text",
        "json" => "json",
        "markdown" => "markdown",
        other => {
            leptos::logging::warn!(
                "DtRichTextEditor output_format `{other}` is not text, json, html, or markdown"
            );
            "html"
        }
    }
}

fn autofocus_kind(value: &str) -> Autofocus {
    match value {
        "" | "false" => Autofocus::Off,
        "true" | "start" => Autofocus::Start,
        "end" => Autofocus::End,
        "all" => Autofocus::All,
        other => {
            if let Ok(offset) = other.parse::<u32>() {
                Autofocus::Position(offset)
            } else {
                leptos::logging::warn!(
                    "DtRichTextEditor auto_focus `{other}` is not false, true, start, end, all, or a number"
                );
                Autofocus::Off
            }
        }
    }
}

fn typing_is_blocked(allow_typing: bool, key: &str, shift: bool, allow_line_breaks: bool) -> bool {
    if allow_typing {
        return false;
    }
    if key == "Backspace" {
        return false;
    }
    if key == "Enter" && !allow_line_breaks && !shift {
        return false;
    }
    true
}

fn enter_submits(allow_line_breaks: bool, shift: bool) -> bool {
    !allow_line_breaks && !shift
}

fn shift_enter_breaks(allow_line_breaks: bool, shift: bool) -> bool {
    shift && !allow_line_breaks
}

/// Prepend `default_prefix` when `input` has no supported protocol. Empty input stays empty.
pub fn rich_text_editor_link_href(input: &str, default_prefix: &str) -> String {
    let input = input.trim();
    if input.is_empty() {
        return String::new();
    }
    if input.starts_with("www.") {
        return format!("http://{input}");
    }
    if protocol_at(input) {
        return input.to_string();
    }
    format!("{default_prefix}{input}")
}

fn protocol_at(value: &str) -> bool {
    const PREFIXES: &[&str] = &[
        "https://", "http://", "ftps://", "ftp://", "sftp://", "mailto:",
    ];
    PREFIXES.iter().any(|prefix| value.starts_with(prefix))
}

/// Link a trailing URL in `html`. Vue's `forceLinkifyPendingText` is a no-op when
/// `link` is false or `customLink` is true. A `www.` host gets an `http://` href.
pub fn force_linkify_pending_text(html: &str, link: bool, custom_link: bool) -> String {
    if !link || custom_link {
        return html.to_string();
    }
    serialize(&linkify_nodes(parse_html(html), false))
}

fn linkify_nodes(nodes: Vec<HtmlNode>, inside_anchor: bool) -> Vec<HtmlNode> {
    let mut out = Vec::new();
    for node in nodes {
        if node.is_text() {
            if inside_anchor {
                out.push(node);
            } else {
                out.extend(linkify_text(&node.text));
            }
            continue;
        }
        let inside = inside_anchor || node.name == "a";
        let mut node = node;
        node.children = linkify_nodes(node.children, inside);
        out.push(node);
    }
    out
}

fn linkify_text(text: &str) -> Vec<HtmlNode> {
    let mut nodes = Vec::new();
    let mut cursor = 0;
    while let Some(hit) = find_url(text, cursor) {
        if hit.start > cursor {
            nodes.push(HtmlNode::text(text[cursor..hit.start].to_string()));
        }
        let label = text[hit.start..hit.end].to_string();
        nodes.push(HtmlNode::elem(
            "a",
            vec![
                ("href".to_string(), hit.href),
                ("class".to_string(), LINK_CLASS.to_string()),
            ],
            vec![HtmlNode::text(label)],
        ));
        cursor = hit.end;
    }
    if cursor < text.len() {
        nodes.push(HtmlNode::text(text[cursor..].to_string()));
    }
    if nodes.is_empty() && !text.is_empty() {
        nodes.push(HtmlNode::text(text.to_string()));
    }
    nodes
}

struct UrlHit {
    start: usize,
    end: usize,
    href: String,
}

fn find_url(text: &str, from: usize) -> Option<UrlHit> {
    let mut index = from;
    while index < text.len() {
        if let Some(hit) = url_at(text, index) {
            return Some(hit);
        }
        index += text[index..]
            .chars()
            .next()
            .map(|ch| ch.len_utf8())
            .unwrap_or(1);
    }
    None
}

fn url_at(text: &str, index: usize) -> Option<UrlHit> {
    if index > 0 {
        let prev = text[..index].chars().next_back()?;
        if !prev.is_whitespace() {
            return None;
        }
    }
    let rest = &text[index..];
    let www = rest.starts_with("www.");
    if !www && !protocol_at(rest) {
        return None;
    }
    let token_end = rest
        .find(|ch: char| ch.is_whitespace() || matches!(ch, '<' | '>' | '"'))
        .unwrap_or(rest.len());
    let mut token = &rest[..token_end];
    token =
        token.trim_end_matches(|ch: char| matches!(ch, '.' | ',' | ';' | ':' | '!' | '?' | ')'));
    if token.is_empty() || token == "www." {
        return None;
    }
    if !www && !protocol_at(token) {
        return None;
    }
    let href = if www {
        rich_text_editor_link_href(token, "http://")
    } else {
        token.to_string()
    };
    Some(UrlHit {
        start: index,
        end: index + token.len(),
        href,
    })
}

fn autolink_span(text: &str) -> Option<(usize, usize, String)> {
    let boundary = text.chars().next_back()?;
    if !boundary.is_whitespace() {
        return None;
    }
    let body_end = text.trim_end().len();
    let body = &text[..body_end];
    if body.is_empty() {
        return None;
    }
    let start = body
        .char_indices()
        .rev()
        .find(|(_, ch)| ch.is_whitespace())
        .map(|(index, ch)| index + ch.len_utf8())
        .unwrap_or(0);
    let hit = url_at(body, start)?;
    if hit.start != start || hit.end != body_end {
        return None;
    }
    Some((hit.start, hit.end, hit.href))
}

fn escape_text(text: &str) -> String {
    let mut out = String::new();
    for ch in text.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(ch),
        }
    }
    out
}

fn decode_entities(text: &str) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(start) = rest.find('&') {
        out.push_str(&rest[..start]);
        rest = &rest[start + 1..];
        let Some(end) = rest.find(';') else {
            out.push('&');
            break;
        };
        let name = &rest[..end];
        rest = &rest[end + 1..];
        match name {
            "amp" => out.push('&'),
            "lt" => out.push('<'),
            "gt" => out.push('>'),
            "quot" => out.push('"'),
            "apos" | "#39" => out.push('\''),
            "nbsp" => out.push('\u{00a0}'),
            other if other.starts_with('#') => {
                let code = if let Some(hex) = other
                    .strip_prefix("#x")
                    .or_else(|| other.strip_prefix("#X"))
                {
                    u32::from_str_radix(hex, 16).ok()
                } else {
                    other[1..].parse().ok()
                };
                if let Some(ch) = code.and_then(char::from_u32) {
                    out.push(ch);
                }
            }
            other => {
                out.push('&');
                out.push_str(other);
                out.push(';');
            }
        }
    }
    out.push_str(rest);
    out
}

fn is_void(name: &str) -> bool {
    matches!(
        name,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "source"
            | "track"
            | "wbr"
    )
}

fn parse_html(input: &str) -> Vec<HtmlNode> {
    let mut pos = 0;
    parse_nodes(input, &mut pos, None)
}

fn parse_nodes(input: &str, pos: &mut usize, until: Option<&str>) -> Vec<HtmlNode> {
    let mut nodes = Vec::new();
    while *pos < input.len() {
        if until.is_some_and(|tag| starts_with_ignore_ascii_case(&input[*pos..], tag)) {
            break;
        }
        if input[*pos..].starts_with("<!--") {
            if let Some(end) = input[*pos..].find("-->") {
                *pos += end + 3;
                continue;
            }
            break;
        }
        if input[*pos..].starts_with("</") {
            if until.is_some() {
                break;
            }
            if let Some(end) = input[*pos..].find('>') {
                *pos += end + 1;
                continue;
            }
            break;
        }
        if input[*pos..].starts_with('<') {
            match parse_element(input, pos) {
                Some(node) => nodes.push(node),
                None => break,
            }
            continue;
        }
        let start = *pos;
        let end = input[*pos..]
            .find('<')
            .map(|index| *pos + index)
            .unwrap_or(input.len());
        if end == start {
            break;
        }
        let text = decode_entities(&input[start..end]);
        *pos = end;
        if !text.is_empty() {
            nodes.push(HtmlNode::text(text));
        }
    }
    nodes
}

fn starts_with_ignore_ascii_case(text: &str, prefix: &str) -> bool {
    text.len() >= prefix.len()
        && text
            .as_bytes()
            .iter()
            .zip(prefix.as_bytes())
            .all(|(left, right)| left.to_ascii_lowercase() == *right)
}

fn parse_element(input: &str, pos: &mut usize) -> Option<HtmlNode> {
    if !input[*pos..].starts_with('<') {
        return None;
    }
    let start = *pos + 1;
    let rel = input[start..].find('>')?;
    let raw = &input[start..start + rel];
    *pos = start + rel + 1;
    let self_closing = raw.trim_end().ends_with('/');
    let raw = raw.trim().trim_end_matches('/').trim();
    let (name, attrs) = split_tag(raw);
    let name = name.to_ascii_lowercase();
    if name.is_empty() {
        return None;
    }
    if self_closing || is_void(&name) {
        return Some(HtmlNode::elem(&name, attrs, Vec::new()));
    }
    let close = format!("</{name}");
    let children = parse_nodes(input, pos, Some(&close));
    if starts_with_ignore_ascii_case(&input[*pos..], &close) {
        if let Some(end) = input[*pos..].find('>') {
            *pos += end + 1;
        }
    }
    Some(HtmlNode::elem(&name, attrs, children))
}

fn split_tag(raw: &str) -> (String, Vec<(String, String)>) {
    let name_end = raw.find(char::is_whitespace).unwrap_or(raw.len());
    let name = raw[..name_end].to_string();
    let attrs = parse_attrs(raw[name_end..].trim());
    (name, attrs)
}

fn parse_attrs(input: &str) -> Vec<(String, String)> {
    let mut attrs = Vec::new();
    let mut rest = input;
    while !rest.is_empty() {
        rest = rest.trim_start();
        if rest.is_empty() {
            break;
        }
        let name_end = rest
            .find(|ch: char| ch == '=' || ch.is_whitespace())
            .unwrap_or(rest.len());
        let name = rest[..name_end].to_ascii_lowercase();
        rest = &rest[name_end..];
        if !rest.starts_with('=') {
            if !name.is_empty() {
                attrs.push((name, String::new()));
            }
            continue;
        }
        rest = &rest[1..];
        let quote = rest.chars().next();
        if matches!(quote, Some('"' | '\'')) {
            let quote = quote.unwrap();
            rest = &rest[1..];
            let end = rest.find(quote).unwrap_or(rest.len());
            attrs.push((name, decode_entities(&rest[..end])));
            rest = rest.get(end + 1..).unwrap_or("");
        } else {
            let end = rest.find(char::is_whitespace).unwrap_or(rest.len());
            attrs.push((name, decode_entities(&rest[..end])));
            rest = &rest[end..];
        }
    }
    attrs
}

fn attr<'a>(node: &'a HtmlNode, name: &str) -> &'a str {
    node.attrs
        .iter()
        .find(|(key, _)| key == name)
        .map(|(_, value)| value.as_str())
        .unwrap_or("")
}

fn serialize(nodes: &[HtmlNode]) -> String {
    let mut out = String::new();
    for node in nodes {
        write_node(&mut out, node);
    }
    out
}

fn write_node(out: &mut String, node: &HtmlNode) {
    if node.is_text() {
        out.push_str(&escape_text(&node.text));
        return;
    }
    out.push('<');
    out.push_str(&node.name);
    for (key, value) in &node.attrs {
        out.push(' ');
        out.push_str(key);
        out.push_str("=\"");
        out.push_str(&escape_text(value));
        out.push('"');
    }
    if is_void(&node.name) {
        out.push('>');
        return;
    }
    out.push('>');
    for child in &node.children {
        write_node(out, child);
    }
    out.push_str("</");
    out.push_str(&node.name);
    out.push('>');
}

fn normalize_nodes(nodes: Vec<HtmlNode>, marks: &Marks, in_pre: bool) -> Vec<HtmlNode> {
    let mut out = Vec::new();
    for node in nodes {
        if node.is_text() {
            let text = if marks.collapse_whitespace && !in_pre {
                collapse_ws(&node.text)
            } else {
                node.text
            };
            if !text.is_empty() {
                out.push(HtmlNode::text(text));
            }
            continue;
        }
        out.extend(normalize_elem(node, marks, in_pre));
    }
    out
}

fn normalize_elem(mut node: HtmlNode, marks: &Marks, in_pre: bool) -> Vec<HtmlNode> {
    let name = std::mem::take(&mut node.name);
    let attrs = std::mem::take(&mut node.attrs);
    let next_pre = in_pre || name == "pre";
    let children = normalize_nodes(std::mem::take(&mut node.children), marks, next_pre);
    let attr_value = |key: &str| {
        attrs
            .iter()
            .find(|(name, _)| name == key)
            .map(|(_, value)| value.clone())
            .unwrap_or_default()
    };
    match name.as_str() {
        "b" | "strong" => wrap_or_flatten("strong", children, marks.bold && !in_pre),
        "i" | "em" => wrap_or_flatten("em", children, marks.italic && !in_pre),
        "u" => wrap_or_flatten("u", children, marks.underline && !in_pre),
        "s" | "strike" | "del" => wrap_or_flatten("s", children, marks.strike && !in_pre),
        "code" if in_pre => vec![HtmlNode::elem("code", Vec::new(), children)],
        "code" => wrap_or_flatten("code", children, marks.code),
        "br" | "hr" => vec![HtmlNode::elem(&name, Vec::new(), Vec::new())],
        "a" => {
            let href = attr_value("href");
            if !marks.link || !href_allowed(&href) {
                return children;
            }
            vec![HtmlNode::elem(
                "a",
                vec![
                    ("href".to_string(), href),
                    ("class".to_string(), LINK_CLASS.to_string()),
                ],
                children,
            )]
        }
        "img" => {
            if !marks.inline_images {
                return Vec::new();
            }
            let mut kept = Vec::new();
            for key in ["src", "alt", "width", "height"] {
                let value = attr_value(key);
                if !value.is_empty() {
                    kept.push((key.to_string(), value));
                }
            }
            vec![HtmlNode::elem("img", kept, Vec::new())]
        }
        "pre" => {
            if !marks.codeblock {
                return children;
            }
            let children = if children.iter().any(|child| child.name == "code") {
                children
            } else {
                vec![HtmlNode::elem("code", Vec::new(), children)]
            };
            vec![HtmlNode::elem(
                "pre",
                vec![("class".to_string(), CODE_BLOCK_CLASS.to_string())],
                children,
            )]
        }
        "blockquote" => wrap_or_flatten("blockquote", children, marks.blockquote),
        "ul" | "ol" | "li" => wrap_or_flatten(&name, children, marks.list),
        "table" | "thead" | "tbody" | "tr" | "td" | "th" => {
            wrap_or_flatten(&name, children, marks.tables)
        }
        "p" | "div" => {
            let tag = block_tag(marks.use_div);
            vec![HtmlNode::elem(tag, Vec::new(), drop_lone_break(children))]
        }
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
            vec![HtmlNode::elem(
                block_tag(marks.use_div),
                Vec::new(),
                children,
            )]
        }
        "script" | "style" => Vec::new(),
        _ => children,
    }
}

fn drop_lone_break(children: Vec<HtmlNode>) -> Vec<HtmlNode> {
    if children.len() == 1 && children[0].name == "br" {
        Vec::new()
    } else {
        children
    }
}

fn wrap_or_flatten(name: &str, children: Vec<HtmlNode>, keep: bool) -> Vec<HtmlNode> {
    if keep {
        vec![HtmlNode::elem(name, Vec::new(), children)]
    } else {
        children
    }
}

fn href_allowed(href: &str) -> bool {
    let href = href.trim();
    !href.is_empty()
        && (href.starts_with('/')
            || href.starts_with('#')
            || href.starts_with('?')
            || protocol_at(href))
}

fn collapse_ws(text: &str) -> String {
    let mut out = String::new();
    let mut pending = false;
    for ch in text.chars() {
        if ch.is_whitespace() {
            pending = true;
            continue;
        }
        if pending && !out.is_empty() {
            out.push(' ');
        }
        pending = false;
        out.push(ch);
    }
    if pending && !out.is_empty() {
        out.push(' ');
    }
    out
}

fn semantic_html(html: &str, marks: &Marks) -> String {
    let nodes = normalize_nodes(parse_html(html), marks, false);
    let html = serialize(&nodes);
    if html.is_empty() {
        empty_block(marks.use_div)
    } else {
        html
    }
}

fn html_to_output(html: &str, format: &str, marks: &Marks) -> String {
    let semantic = semantic_html(html, marks);
    match normalize_output_format(format) {
        "text" => html_to_text(&semantic),
        "json" => html_to_json(&semantic, marks.text_align),
        "markdown" => html_to_markdown(&semantic),
        _ => semantic,
    }
}

fn model_to_html(value: &str, format: &str, marks: &Marks) -> String {
    let raw = match normalize_output_format(format) {
        "text" => text_to_html(value, marks.use_div),
        "json" => json_model_to_html(value).unwrap_or_else(|| text_to_html(value, marks.use_div)),
        "markdown" => markdown_to_html(value),
        _ => {
            if value.trim().is_empty() {
                empty_block(marks.use_div)
            } else if value.contains('<') {
                value.to_string()
            } else {
                text_to_html(value, marks.use_div)
            }
        }
    };
    semantic_html(&raw, marks)
}

fn text_to_html(value: &str, use_div: bool) -> String {
    if value.is_empty() {
        return empty_block(use_div);
    }
    let tag = block_tag(use_div);
    value
        .split('\n')
        .map(|line| format!("<{tag}>{}</{tag}>", escape_text(line)))
        .collect()
}

fn display_html(semantic: &str, placeholder: &str, use_div: bool) -> String {
    if html_to_text(semantic).trim().is_empty() {
        let tag = block_tag(use_div);
        format!(
            "<{tag} class=\"is-editor-empty\" data-placeholder=\"{}\"><br></{tag}>",
            escape_text(placeholder)
        )
    } else {
        semantic.to_string()
    }
}

fn html_to_text(html: &str) -> String {
    let mut blocks = Vec::new();
    push_text_blocks(&parse_html(html), &mut blocks);
    blocks.join("\n")
}

fn push_text_blocks(nodes: &[HtmlNode], blocks: &mut Vec<String>) {
    let mut inline = String::new();
    let flush = |inline: &mut String, blocks: &mut Vec<String>| {
        if !inline.is_empty() {
            blocks.push(std::mem::take(inline));
        }
    };
    for node in nodes {
        match node.name.as_str() {
            "" => inline.push_str(&node.text),
            "br" => inline.push('\n'),
            "p" | "div" | "blockquote" | "li" | "pre" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6"
            | "tr" => {
                flush(&mut inline, blocks);
                let mut nested = Vec::new();
                push_text_blocks(&node.children, &mut nested);
                blocks.push(nested.join("\n"));
            }
            "ul" | "ol" | "table" | "thead" | "tbody" => {
                flush(&mut inline, blocks);
                push_text_blocks(&node.children, blocks);
            }
            _ => push_inline_text(&node.children, &mut inline),
        }
    }
    flush(&mut inline, blocks);
}

fn push_inline_text(nodes: &[HtmlNode], out: &mut String) {
    for node in nodes {
        if node.is_text() {
            out.push_str(&node.text);
        } else if node.name == "br" {
            out.push('\n');
        } else {
            push_inline_text(&node.children, out);
        }
    }
}

fn html_to_markdown(html: &str) -> String {
    markdown_blocks(&parse_html(html))
}

fn markdown_blocks(nodes: &[HtmlNode]) -> String {
    let mut parts = Vec::new();
    for node in nodes {
        match node.name.as_str() {
            "" => {
                if !node.text.is_empty() {
                    parts.push(node.text.clone());
                }
            }
            "p" | "div" => parts.push(inline_md(&node.children)),
            "blockquote" => {
                let inner = markdown_blocks(&node.children);
                let quoted = inner
                    .lines()
                    .map(|line| format!("> {line}"))
                    .collect::<Vec<_>>()
                    .join("\n");
                parts.push(quoted);
            }
            "ul" => parts.push(list_md(&node.children, false)),
            "ol" => parts.push(list_md(&node.children, true)),
            "pre" => {
                let text = html_to_text(&serialize(std::slice::from_ref(node)));
                parts.push(format!("```\n{text}\n```"));
            }
            "br" => {}
            _ => {
                let inner = markdown_blocks(&node.children);
                if !inner.is_empty() {
                    parts.push(inner);
                }
            }
        }
    }
    parts.join("\n\n")
}

fn inline_md(nodes: &[HtmlNode]) -> String {
    let mut out = String::new();
    for node in nodes {
        match node.name.as_str() {
            "" => out.push_str(&node.text),
            "br" => out.push('\n'),
            "strong" | "b" => out.push_str(&format!("**{}**", inline_md(&node.children))),
            "em" | "i" => out.push_str(&format!("*{}*", inline_md(&node.children))),
            "s" | "strike" | "del" => out.push_str(&format!("~~{}~~", inline_md(&node.children))),
            "u" => out.push_str(&format!("<u>{}</u>", inline_md(&node.children))),
            "code" => out.push_str(&format!("`{}`", inline_md(&node.children))),
            "a" => out.push_str(&format!(
                "[{}]({})",
                inline_md(&node.children),
                attr(node, "href")
            )),
            _ => out.push_str(&inline_md(&node.children)),
        }
    }
    out
}

fn list_md(items: &[HtmlNode], ordered: bool) -> String {
    items
        .iter()
        .filter(|node| node.name == "li")
        .enumerate()
        .map(|(index, item)| {
            let text = item_text(item);
            if ordered {
                format!("{}. {text}", index + 1)
            } else {
                format!("- {text}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn item_text(item: &HtmlNode) -> String {
    let blocks: Vec<&HtmlNode> = item
        .children
        .iter()
        .filter(|node| node.name == "p" || node.name == "div")
        .collect();
    if blocks.is_empty() {
        inline_md(&item.children)
    } else {
        blocks
            .iter()
            .map(|node| inline_md(&node.children))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn html_to_json(html: &str, text_align: bool) -> String {
    let content = nodes_to_json(&parse_html(html), text_align);
    let mut doc = Map::new();
    doc.insert("type".to_string(), Value::String("doc".to_string()));
    doc.insert("content".to_string(), Value::Array(content));
    Value::Object(doc).to_string()
}

fn nodes_to_json(nodes: &[HtmlNode], text_align: bool) -> Vec<Value> {
    let mut content = Vec::new();
    let mut inline = Vec::new();
    let flush = |inline: &mut Vec<Value>, content: &mut Vec<Value>, text_align: bool| {
        if !inline.is_empty() {
            content.push(paragraph_json(std::mem::take(inline), text_align));
        }
    };
    for node in nodes {
        match node.name.as_str() {
            "" => {
                if !node.text.is_empty() {
                    inline.push(text_json(&node.text, &[]));
                }
            }
            "br" => inline.push(json_type("hardBreak")),
            "p" | "div" => {
                flush(&mut inline, &mut content, text_align);
                content.push(paragraph_json(inline_json(&node.children), text_align));
            }
            "blockquote" => {
                flush(&mut inline, &mut content, text_align);
                content.push(container_json(
                    "blockquote",
                    nodes_to_json(&node.children, text_align),
                ));
            }
            "ul" | "ol" => {
                flush(&mut inline, &mut content, text_align);
                let kind = if node.name == "ol" {
                    "orderedList"
                } else {
                    "bulletList"
                };
                content.push(container_json(kind, list_json(&node.children, text_align)));
            }
            "pre" => {
                flush(&mut inline, &mut content, text_align);
                content.push(code_block_json(node));
            }
            _ => inline.extend(inline_json(std::slice::from_ref(node))),
        }
    }
    flush(&mut inline, &mut content, text_align);
    content
}

fn list_json(nodes: &[HtmlNode], text_align: bool) -> Vec<Value> {
    nodes
        .iter()
        .filter(|node| node.name == "li")
        .map(|item| container_json("listItem", nodes_to_json(&item.children, text_align)))
        .collect()
}

fn inline_json(nodes: &[HtmlNode]) -> Vec<Value> {
    let mut out = Vec::new();
    for node in nodes {
        match node.name.as_str() {
            "" => {
                if !node.text.is_empty() {
                    out.push(text_json(&node.text, &[]));
                }
            }
            "br" => out.push(json_type("hardBreak")),
            "strong" | "b" => out.extend(marked(&node.children, "bold")),
            "em" | "i" => out.extend(marked(&node.children, "italic")),
            "s" | "strike" | "del" => out.extend(marked(&node.children, "strike")),
            "u" => out.extend(marked(&node.children, "underline")),
            "code" => out.extend(marked(&node.children, "code")),
            "a" => out.extend(link_marked(node)),
            _ => out.extend(inline_json(&node.children)),
        }
    }
    out
}

fn marked(nodes: &[HtmlNode], mark: &str) -> Vec<Value> {
    inline_json(nodes)
        .into_iter()
        .map(|mut value| {
            if value.get("type").and_then(|kind| kind.as_str()) == Some("text") {
                let marks = value
                    .as_object_mut()
                    .and_then(|map| map.get_mut("marks"))
                    .and_then(|marks| marks.as_array_mut());
                let mark_value = json_type(mark);
                if let Some(marks) = marks {
                    marks.push(mark_value);
                } else if let Some(map) = value.as_object_mut() {
                    map.insert("marks".to_string(), Value::Array(vec![mark_value]));
                }
            }
            value
        })
        .collect()
}

fn link_marked(node: &HtmlNode) -> Vec<Value> {
    let href = attr(node, "href");
    let class = attr(node, "class");
    inline_json(&node.children)
        .into_iter()
        .map(|mut value| {
            if value.get("type").and_then(|kind| kind.as_str()) == Some("text") {
                let mut mark = Map::new();
                mark.insert("type".to_string(), Value::String("link".to_string()));
                let mut attrs = Map::new();
                attrs.insert("href".to_string(), Value::String(href.to_string()));
                if !class.is_empty() {
                    attrs.insert("class".to_string(), Value::String(class.to_string()));
                }
                mark.insert("attrs".to_string(), Value::Object(attrs));
                if let Some(map) = value.as_object_mut() {
                    map.entry("marks".to_string())
                        .or_insert_with(|| Value::Array(Vec::new()))
                        .as_array_mut()
                        .map(|marks| marks.push(Value::Object(mark)));
                }
            }
            value
        })
        .collect()
}

fn paragraph_json(content: Vec<Value>, text_align: bool) -> Value {
    let mut map = Map::new();
    map.insert("type".to_string(), Value::String("paragraph".to_string()));
    if text_align {
        let mut attrs = Map::new();
        attrs.insert("textAlign".to_string(), Value::Null);
        map.insert("attrs".to_string(), Value::Object(attrs));
    }
    if !content.is_empty() {
        map.insert("content".to_string(), Value::Array(content));
    }
    Value::Object(map)
}

fn text_json(text: &str, marks: &[&str]) -> Value {
    let mut map = Map::new();
    map.insert("type".to_string(), Value::String("text".to_string()));
    map.insert("text".to_string(), Value::String(text.to_string()));
    if !marks.is_empty() {
        map.insert(
            "marks".to_string(),
            Value::Array(marks.iter().copied().map(json_type).collect()),
        );
    }
    Value::Object(map)
}

fn json_type(kind: &str) -> Value {
    let mut map = Map::new();
    map.insert("type".to_string(), Value::String(kind.to_string()));
    Value::Object(map)
}

fn container_json(kind: &str, content: Vec<Value>) -> Value {
    let mut map = Map::new();
    map.insert("type".to_string(), Value::String(kind.to_string()));
    if !content.is_empty() {
        map.insert("content".to_string(), Value::Array(content));
    }
    Value::Object(map)
}

fn code_block_json(node: &HtmlNode) -> Value {
    let text = html_to_text(&serialize(std::slice::from_ref(node)));
    let mut map = Map::new();
    map.insert("type".to_string(), Value::String("codeBlock".to_string()));
    let mut attrs = Map::new();
    attrs.insert(
        "class".to_string(),
        Value::String(CODE_BLOCK_CLASS.to_string()),
    );
    map.insert("attrs".to_string(), Value::Object(attrs));
    if !text.is_empty() {
        map.insert(
            "content".to_string(),
            Value::Array(vec![text_json(&text, &[])]),
        );
    }
    Value::Object(map)
}

fn json_model_to_html(value: &str) -> Option<String> {
    let value: Value = serde_json::from_str(value).ok()?;
    let mut out = String::new();
    write_json_html(&value, &mut out);
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

fn write_json_html(value: &Value, out: &mut String) {
    let Some(kind) = value.get("type").and_then(|kind| kind.as_str()) else {
        return;
    };
    let content = value.get("content").and_then(|content| content.as_array());
    match kind {
        "doc" | "listItem" => {
            if let Some(content) = content {
                for child in content {
                    write_json_html(child, out);
                }
            }
        }
        "paragraph" => {
            out.push_str("<p>");
            write_json_children(content, out);
            out.push_str("</p>");
        }
        "text" => {
            let text = value
                .get("text")
                .and_then(|text| text.as_str())
                .unwrap_or("");
            let html = escape_text(text);
            let marks = value
                .get("marks")
                .and_then(|marks| marks.as_array())
                .map(|marks| marks.as_slice())
                .unwrap_or(&[]);
            out.push_str(&wrap_marks(&html, marks));
        }
        "hardBreak" => out.push_str("<br>"),
        "bulletList" => wrap_children(out, "ul", content),
        "orderedList" => wrap_children(out, "ol", content),
        "blockquote" => wrap_children(out, "blockquote", content),
        "codeBlock" => {
            let text = content
                .and_then(|nodes| nodes.first())
                .and_then(|node| node.get("text"))
                .and_then(|text| text.as_str())
                .unwrap_or("");
            out.push_str("<pre class=\"");
            out.push_str(CODE_BLOCK_CLASS);
            out.push_str("\"><code>");
            out.push_str(&escape_text(text));
            out.push_str("</code></pre>");
        }
        _ => {
            if let Some(content) = content {
                for child in content {
                    write_json_html(child, out);
                }
            }
        }
    }
}

fn write_json_children(content: Option<&Vec<Value>>, out: &mut String) {
    if let Some(content) = content {
        for child in content {
            write_json_html(child, out);
        }
    }
}

fn wrap_children(out: &mut String, tag: &str, content: Option<&Vec<Value>>) {
    out.push('<');
    out.push_str(tag);
    out.push('>');
    if tag == "ul" || tag == "ol" {
        if let Some(content) = content {
            for child in content {
                out.push_str("<li>");
                if let Some(grand) = child.get("content").and_then(|nodes| nodes.as_array()) {
                    for node in grand {
                        write_json_html(node, out);
                    }
                }
                out.push_str("</li>");
            }
        }
    } else {
        write_json_children(content, out);
    }
    out.push_str("</");
    out.push_str(tag);
    out.push('>');
}

fn wrap_marks(html: &str, marks: &[Value]) -> String {
    let mut out = html.to_string();
    for mark in marks.iter().rev() {
        let kind = mark
            .get("type")
            .and_then(|kind| kind.as_str())
            .unwrap_or("");
        out = match kind {
            "bold" => format!("<strong>{out}</strong>"),
            "italic" => format!("<em>{out}</em>"),
            "strike" => format!("<s>{out}</s>"),
            "underline" => format!("<u>{out}</u>"),
            "code" => format!("<code>{out}</code>"),
            "link" => {
                let href = mark
                    .get("attrs")
                    .and_then(|attrs| attrs.get("href"))
                    .and_then(|href| href.as_str())
                    .unwrap_or("");
                format!(
                    "<a href=\"{}\" class=\"{LINK_CLASS}\">{out}</a>",
                    escape_text(href)
                )
            }
            _ => out,
        };
    }
    out
}

fn markdown_to_html(markdown: &str) -> String {
    if markdown.is_empty() {
        return empty_block(false);
    }
    let lines: Vec<&str> = markdown.split('\n').collect();
    let mut html = String::new();
    let mut index = 0;
    while index < lines.len() {
        let line = lines[index];
        if line.starts_with("```") {
            let mut body = String::new();
            index += 1;
            while index < lines.len() && !lines[index].starts_with("```") {
                if !body.is_empty() {
                    body.push('\n');
                }
                body.push_str(lines[index]);
                index += 1;
            }
            html.push_str("<pre class=\"");
            html.push_str(CODE_BLOCK_CLASS);
            html.push_str("\"><code>");
            html.push_str(&escape_text(&body));
            html.push_str("</code></pre>");
            if index < lines.len() {
                index += 1;
            }
            continue;
        }
        if let Some(item) = line.strip_prefix("- ") {
            html.push_str("<ul>");
            let mut item = Some(item);
            while let Some(text) = item {
                html.push_str("<li><p>");
                html.push_str(&inline_markdown_html(text));
                html.push_str("</p></li>");
                index += 1;
                item = lines.get(index).and_then(|line| line.strip_prefix("- "));
            }
            html.push_str("</ul>");
            continue;
        }
        if let Some(text) = line.strip_prefix("> ") {
            html.push_str("<blockquote><p>");
            html.push_str(&inline_markdown_html(text));
            html.push_str("</p></blockquote>");
            index += 1;
            continue;
        }
        if line.is_empty() {
            index += 1;
            continue;
        }
        html.push_str("<p>");
        html.push_str(&inline_markdown_html(line));
        html.push_str("</p>");
        index += 1;
    }
    if html.is_empty() {
        empty_block(false)
    } else {
        html
    }
}

fn inline_markdown_html(line: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = line.chars().collect();
    let mut index = 0;
    while index < chars.len() {
        if chars[index] == '`' {
            if let Some(end) = chars[index + 1..].iter().position(|ch| *ch == '`') {
                let text: String = chars[index + 1..index + 1 + end].iter().collect();
                out.push_str("<code>");
                out.push_str(&escape_text(&text));
                out.push_str("</code>");
                index += end + 2;
                continue;
            }
        }
        if chars[index..].starts_with(&['*', '*']) {
            if let Some(end) = find_marker(&chars, index + 2, &['*', '*']) {
                let text: String = chars[index + 2..end].iter().collect();
                out.push_str("<strong>");
                out.push_str(&inline_markdown_html(&text));
                out.push_str("</strong>");
                index = end + 2;
                continue;
            }
        }
        if chars[index..].starts_with(&['~', '~']) {
            if let Some(end) = find_marker(&chars, index + 2, &['~', '~']) {
                let text: String = chars[index + 2..end].iter().collect();
                out.push_str("<s>");
                out.push_str(&inline_markdown_html(&text));
                out.push_str("</s>");
                index = end + 2;
                continue;
            }
        }
        if chars[index] == '*' {
            if let Some(end) = chars[index + 1..].iter().position(|ch| *ch == '*') {
                let text: String = chars[index + 1..index + 1 + end].iter().collect();
                out.push_str("<em>");
                out.push_str(&inline_markdown_html(&text));
                out.push_str("</em>");
                index += end + 2;
                continue;
            }
        }
        if chars[index] == '[' {
            if let Some(html) = markdown_link(&chars, index) {
                out.push_str(&html.0);
                index = html.1;
                continue;
            }
        }
        out.push_str(&escape_text(&chars[index].to_string()));
        index += 1;
    }
    out
}

fn find_marker(chars: &[char], from: usize, marker: &[char]) -> Option<usize> {
    chars[from..]
        .windows(marker.len())
        .position(|window| window == marker)
        .map(|index| from + index)
}

fn markdown_link(chars: &[char], start: usize) -> Option<(String, usize)> {
    let end_label = chars[start + 1..].iter().position(|ch| *ch == ']')? + start + 1;
    if chars.get(end_label + 1) != Some(&'(') {
        return None;
    }
    let end_href = chars[end_label + 2..].iter().position(|ch| *ch == ')')? + end_label + 2;
    let label: String = chars[start + 1..end_label].iter().collect();
    let href: String = chars[end_label + 2..end_href].iter().collect();
    if !href_allowed(&href) && !href.starts_with("www.") {
        return None;
    }
    let href = if href.starts_with("www.") {
        rich_text_editor_link_href(&href, "http://")
    } else {
        href
    };
    Some((
        format!(
            "<a href=\"{}\" class=\"{LINK_CLASS}\">{}</a>",
            escape_text(&href),
            escape_text(&label)
        ),
        end_href + 1,
    ))
}

fn bubble_style(top: f64, left: f64) -> String {
    format!(
        "position:fixed;top:{top}px;left:{left}px;transform:translateY(-100%);margin-top:-8px;z-index:{BUBBLE_Z_INDEX}"
    )
}

fn editor_element(editor_ref: NodeRef<html::Div>) -> Option<HtmlElement> {
    editor_ref.get().map(|node| node.into())
}

fn exec_command(command: &str) {
    exec_command_value(command, "");
}

fn exec_command_value(command: &str, value: &str) {
    // `Document::execCommand` is not in web-sys. The browser still implements it for contenteditable.
    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        return;
    };
    let Ok(function) =
        js_sys::Reflect::get(&document, &wasm_bindgen::JsValue::from_str("execCommand"))
    else {
        return;
    };
    let Ok(function) = function.dyn_into::<js_sys::Function>() else {
        return;
    };
    let _ = function.call3(
        &document,
        &wasm_bindgen::JsValue::from_str(command),
        &wasm_bindgen::JsValue::FALSE,
        &wasm_bindgen::JsValue::from_str(value),
    );
}

fn selected_string() -> String {
    web_sys::window()
        .and_then(|window| window.get_selection().ok().flatten())
        .map(|selection| selection.to_string().into())
        .unwrap_or_default()
}

fn selection_anchor(root: &HtmlElement) -> Option<Element> {
    let window = web_sys::window()?;
    let selection = window.get_selection().ok().flatten()?;
    if selection.range_count() == 0 {
        return None;
    }
    let range = selection.get_range_at(0).ok()?;
    let container = range.start_container().ok()?;
    let element = if container.node_type() == Node::ELEMENT_NODE {
        container.dyn_into::<Element>().ok()?
    } else {
        container.parent_element()?
    };
    let anchor = element.closest("a").ok().flatten()?;
    if !root.contains(Some(anchor.as_ref())) {
        return None;
    }
    Some(anchor)
}

fn current_link(root: &HtmlElement) -> DtRichTextEditorLink {
    if let Some(anchor) = selection_anchor(root) {
        return DtRichTextEditorLink {
            href: anchor.get_attribute("href").unwrap_or_default(),
            text: anchor.text_content().unwrap_or_default(),
        };
    }
    DtRichTextEditorLink {
        href: String::new(),
        text: selected_string(),
    }
}

fn unwrap_anchor(anchor: &Element) {
    let Some(parent) = anchor.parent_node() else {
        return;
    };
    while let Some(child) = anchor.first_child() {
        if parent.insert_before(&child, Some(anchor.as_ref())).is_err() {
            break;
        }
    }
    let _ = parent.remove_child(anchor);
}

fn sync_empty_marker(element: &HtmlElement, placeholder: &str, use_div: bool) {
    let text = element.text_content().unwrap_or_default();
    let empty = text.trim().is_empty();
    if element.inner_html().trim().is_empty() {
        element.set_inner_html(&display_html("", placeholder, use_div));
        return;
    }
    let Some(first) = element.first_element_child() else {
        if empty {
            element.set_inner_html(&display_html("", placeholder, use_div));
        }
        return;
    };
    if empty {
        let _ = first.class_list().add_1("is-editor-empty");
        let _ = first.set_attribute("data-placeholder", placeholder);
    } else {
        let _ = first.class_list().remove_1("is-editor-empty");
        let _ = first.remove_attribute("data-placeholder");
    }
}

fn prefix_by_utf16(text: &str, offset: u32) -> Option<&str> {
    let mut units = 0u32;
    for (index, ch) in text.char_indices() {
        if units == offset {
            return Some(&text[..index]);
        }
        if units > offset {
            return None;
        }
        units += ch.len_utf16() as u32;
    }
    if units == offset {
        Some(text)
    } else {
        None
    }
}

fn byte_to_utf16(text: &str, byte: usize) -> u32 {
    text[..byte].chars().map(|ch| ch.len_utf16() as u32).sum()
}

fn autolink_caret(root: &HtmlElement) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Ok(Some(selection)) = window.get_selection() else {
        return;
    };
    if selection.range_count() == 0 || !selection.is_collapsed() {
        return;
    }
    let Ok(range) = selection.get_range_at(0) else {
        return;
    };
    let Ok(container) = range.start_container() else {
        return;
    };
    if container.node_type() != Node::TEXT_NODE || !root.contains(Some(&container)) {
        return;
    }
    if container
        .parent_element()
        .and_then(|element| element.closest("a").ok().flatten())
        .is_some()
    {
        return;
    }
    let Ok(offset) = range.start_offset() else {
        return;
    };
    let Some(text) = container.text_content() else {
        return;
    };
    let Some(before) = prefix_by_utf16(&text, offset) else {
        return;
    };
    let Some((start, end, href)) = autolink_span(before) else {
        return;
    };
    let Ok(text_node) = container.dyn_into::<web_sys::Text>() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Ok(url_node) = text_node.split_text(byte_to_utf16(before, start)) else {
        return;
    };
    let Ok(after) = url_node.split_text(byte_to_utf16(&before[start..], end - start)) else {
        return;
    };
    let Ok(anchor) = document.create_element("a") else {
        return;
    };
    let _ = anchor.set_attribute("href", &href);
    let _ = anchor.set_attribute("class", LINK_CLASS);
    let Some(parent) = url_node.parent_node() else {
        return;
    };
    if parent
        .insert_before(anchor.as_ref(), Some(url_node.as_ref()))
        .is_err()
    {
        return;
    }
    if anchor.append_child(&url_node).is_err() {
        return;
    }
    let Ok(caret) = document.create_range() else {
        return;
    };
    let after_len = after
        .text_content()
        .unwrap_or_default()
        .chars()
        .map(|ch| ch.len_utf16() as u32)
        .sum();
    if caret.set_start(after.as_ref(), after_len).is_err() {
        return;
    }
    let _ = caret.collapse_with_to_start(true);
    let _ = selection.remove_all_ranges();
    let _ = selection.add_range(&caret);
}

fn apply_autofocus(element: &HtmlElement, kind: &str) {
    let kind = autofocus_kind(kind);
    if kind == Autofocus::Off {
        return;
    }
    let _ = element.focus();
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Ok(Some(selection)) = window.get_selection() else {
        return;
    };
    let Ok(range) = document.create_range() else {
        return;
    };
    match kind {
        Autofocus::Off => {}
        Autofocus::Start | Autofocus::Position(_) => {
            if let Autofocus::Position(offset) = kind {
                if place_caret(element, offset, &range) {
                    let _ = selection.remove_all_ranges();
                    let _ = selection.add_range(&range);
                    return;
                }
            }
            let _ = range.select_node_contents(element);
            let _ = range.collapse_with_to_start(true);
        }
        Autofocus::End => {
            let _ = range.select_node_contents(element);
            let _ = range.collapse_with_to_start(false);
        }
        Autofocus::All => {
            let _ = range.select_node_contents(element);
        }
    }
    let _ = selection.remove_all_ranges();
    let _ = selection.add_range(&range);
}

enum CaretPlace {
    Placed,
    Rest(u32),
}

fn place_caret(element: &HtmlElement, offset: u32, range: &web_sys::Range) -> bool {
    let Some(child) = element.first_child() else {
        return false;
    };
    matches!(place_caret_node(&child, offset, range), CaretPlace::Placed)
}

fn place_caret_node(node: &Node, offset: u32, range: &web_sys::Range) -> CaretPlace {
    if node.node_type() == Node::TEXT_NODE {
        let text = node.text_content().unwrap_or_default();
        let len = text.chars().map(|ch| ch.len_utf16() as u32).sum();
        if offset <= len && range.set_start(node, offset).is_ok() {
            range.collapse_with_to_start(true);
            return CaretPlace::Placed;
        }
        return CaretPlace::Rest(offset.saturating_sub(len));
    }
    let mut remaining = offset;
    let children = node.child_nodes();
    for index in 0..children.length() {
        let Some(child) = children.item(index) else {
            continue;
        };
        match place_caret_node(&child, remaining, range) {
            CaretPlace::Placed => return CaretPlace::Placed,
            CaretPlace::Rest(next) => remaining = next,
        }
    }
    CaretPlace::Rest(remaining)
}

fn open_link(href: &str) {
    if href.is_empty() {
        return;
    }
    if let Some(window) = web_sys::window() {
        let _ = window.open_with_url_and_target(href, "_blank");
    }
}

#[derive(Clone)]
struct LinkBubble {
    href: String,
    text: String,
    top: f64,
    left: f64,
}

#[derive(Clone, Copy)]
struct EditorState {
    editor_ref: NodeRef<html::Div>,
    marks: StoredValue<Marks>,
    output_format: StoredValue<String>,
    placeholder: StoredValue<String>,
    auto_focus: StoredValue<String>,
    link: StoredValue<bool>,
    custom_link: StoredValue<bool>,
    paste_rich_text: StoredValue<bool>,
    allow_typing: StoredValue<bool>,
    allow_line_breaks: StoredValue<bool>,
    show_bubble: StoredValue<bool>,
    emitted: StoredValue<Option<String>>,
    did_focus: StoredValue<bool>,
    keep_bubble: StoredValue<bool>,
    bubble: RwSignal<Option<LinkBubble>>,
    on_change: Option<Callback<String>>,
    on_json: Option<Callback<String>>,
    on_html: Option<Callback<String>>,
    on_text: Option<Callback<String>>,
    on_markdown: Option<Callback<String>>,
    on_enter: Option<Callback<()>>,
    on_edit_link: Option<Callback<DtRichTextEditorLink>>,
    on_selected: Option<Callback<String>>,
    on_link_click: Option<Callback<DtRichTextEditorLinkClick>>,
    on_focus: Option<Callback<()>>,
    on_blur: Option<Callback<()>>,
}

fn publish(state: EditorState) {
    let Some(element) = editor_element(state.editor_ref) else {
        return;
    };
    let html = element.inner_html();
    let marks = state.marks.get_value();
    let format = state.output_format.get_value();
    let output = html_to_output(&html, &format, &marks);
    state.emitted.set_value(Some(output.clone()));
    if let Some(callback) = state.on_change {
        callback.run(output);
    }
    if let Some(callback) = state.on_html {
        callback.run(html_to_output(&html, "html", &marks));
    }
    if let Some(callback) = state.on_text {
        callback.run(html_to_output(&html, "text", &marks));
    }
    if let Some(callback) = state.on_json {
        callback.run(html_to_output(&html, "json", &marks));
    }
    if let Some(callback) = state.on_markdown {
        callback.run(html_to_output(&html, "markdown", &marks));
    }
}

fn refresh_bubble(state: EditorState) {
    let Some(element) = editor_element(state.editor_ref) else {
        return;
    };
    let next = if state.link.get_value() && state.show_bubble.get_value() {
        selection_anchor(&element).map(|anchor| {
            let rect = anchor.get_bounding_client_rect();
            LinkBubble {
                href: anchor.get_attribute("href").unwrap_or_default(),
                text: anchor.text_content().unwrap_or_default(),
                top: rect.top(),
                left: rect.left(),
            }
        })
    } else {
        None
    };
    state.bubble.set(next);
}

fn on_editor_input(state: EditorState) {
    let Some(element) = editor_element(state.editor_ref) else {
        return;
    };
    let marks = state.marks.get_value();
    if state.link.get_value() && !state.custom_link.get_value() {
        autolink_caret(&element);
    }
    sync_empty_marker(&element, &state.placeholder.get_value(), marks.use_div);
    publish(state);
    refresh_bubble(state);
}

fn on_editor_key(event: KeyboardEvent, state: EditorState) {
    let key = event.key();
    let marks = state.marks.get_value();
    if typing_is_blocked(
        state.allow_typing.get_value(),
        &key,
        event.shift_key(),
        state.allow_line_breaks.get_value(),
    ) {
        event.prevent_default();
        return;
    }
    if key == "Enter" {
        if enter_submits(state.allow_line_breaks.get_value(), event.shift_key()) {
            event.prevent_default();
            if let Some(callback) = state.on_enter {
                callback.run(());
            }
            return;
        }
        if shift_enter_breaks(state.allow_line_breaks.get_value(), event.shift_key()) {
            event.prevent_default();
            exec_command("insertLineBreak");
            return;
        }
    }
    let modified = event.meta_key() || event.ctrl_key();
    if !modified {
        return;
    }
    let lower = key.to_ascii_lowercase();
    let code = event.code();
    if lower == "k" && state.link.get_value() {
        event.prevent_default();
        if let Some(element) = editor_element(state.editor_ref) {
            if let Some(callback) = state.on_edit_link {
                callback.run(current_link(&element));
            }
        }
        return;
    }
    let command = shortcut_command(&lower, &code, event.shift_key(), event.alt_key(), &marks);
    let Some(command) = command else {
        return;
    };
    event.prevent_default();
    if command.allowed {
        match command.action {
            EditAction::Command(name) => exec_command(name),
            EditAction::Value(name, value) => exec_command_value(name, value),
            EditAction::Code => {
                let text = selected_string();
                if !text.is_empty() {
                    exec_command_value(
                        "insertHTML",
                        &format!("<code>{}</code>", escape_text(&text)),
                    );
                }
            }
            EditAction::CodeBlock => {
                let text = selected_string();
                exec_command_value(
                    "insertHTML",
                    &format!(
                        "<pre class=\"{CODE_BLOCK_CLASS}\"><code>{}</code></pre>",
                        escape_text(&text)
                    ),
                );
            }
        }
        on_editor_input(state);
    }
}

struct Shortcut {
    allowed: bool,
    action: EditAction,
}

enum EditAction {
    Command(&'static str),
    Value(&'static str, &'static str),
    Code,
    CodeBlock,
}

fn shortcut_command(
    key: &str,
    code: &str,
    shift: bool,
    alt: bool,
    marks: &Marks,
) -> Option<Shortcut> {
    if alt && !shift && key == "c" {
        return Some(Shortcut {
            allowed: marks.codeblock,
            action: EditAction::CodeBlock,
        });
    }
    if shift && (key == "8" || key == "*" || code == "Digit8") {
        return Some(Shortcut {
            allowed: marks.list,
            action: EditAction::Command("insertUnorderedList"),
        });
    }
    if shift && (key == "7" || key == "&" || code == "Digit7") {
        return Some(Shortcut {
            allowed: marks.list,
            action: EditAction::Command("insertOrderedList"),
        });
    }
    if shift && key == "b" {
        return Some(Shortcut {
            allowed: marks.blockquote,
            action: EditAction::Value("formatBlock", "blockquote"),
        });
    }
    if shift && key == "s" {
        return Some(Shortcut {
            allowed: marks.strike,
            action: EditAction::Command("strikeThrough"),
        });
    }
    if shift {
        return None;
    }
    match key {
        "b" => Some(Shortcut {
            allowed: marks.bold,
            action: EditAction::Command("bold"),
        }),
        "i" => Some(Shortcut {
            allowed: marks.italic,
            action: EditAction::Command("italic"),
        }),
        "u" => Some(Shortcut {
            allowed: marks.underline,
            action: EditAction::Command("underline"),
        }),
        "e" => Some(Shortcut {
            allowed: marks.code,
            action: EditAction::Code,
        }),
        _ => None,
    }
}

fn on_editor_paste(event: web_sys::ClipboardEvent, state: EditorState) {
    if state.paste_rich_text.get_value() {
        return;
    }
    event.prevent_default();
    let text = event
        .clipboard_data()
        .and_then(|data| data.get_data("text/plain").ok())
        .unwrap_or_default();
    exec_command_value("insertText", &text);
}

fn on_editor_click(event: MouseEvent, state: EditorState) {
    if !state.link.get_value() {
        return;
    }
    let Some(element) = editor_element(state.editor_ref) else {
        return;
    };
    let Some(target) = event.target() else {
        return;
    };
    let Ok(target) = target.dyn_into::<Element>() else {
        return;
    };
    let Some(anchor) = target.closest("a").ok().flatten() else {
        return;
    };
    if !element.contains(Some(anchor.as_ref())) {
        return;
    }
    if let Some(callback) = state.on_link_click {
        callback.run(DtRichTextEditorLinkClick {
            href: anchor.get_attribute("href").unwrap_or_default(),
            text: anchor.text_content().unwrap_or_default(),
        });
    }
}

fn remove_link(state: EditorState) {
    let Some(element) = editor_element(state.editor_ref) else {
        return;
    };
    if let Some(anchor) = selection_anchor(&element) {
        unwrap_anchor(&anchor);
    }
    state.bubble.set(None);
    on_editor_input(state);
}

fn warn_omitted(
    allow_font_color: bool,
    allow_background_color: bool,
    allow_font_size: bool,
    allow_font_family: bool,
    allow_line_height: bool,
    allow_variable: bool,
    allow_image_resize: bool,
    variable_items: &[String],
    link_phone_numbers: Option<&Vec<String>>,
) {
    let phones = link_phone_numbers.is_some_and(|numbers| !numbers.is_empty());
    if allow_font_color
        || allow_background_color
        || allow_font_size
        || allow_font_family
        || allow_line_height
        || allow_variable
        || allow_image_resize
        || !variable_items.is_empty()
        || phones
    {
        leptos::logging::warn!(
            "DtRichTextEditor ignores font style, variables, phone-number marks, and image resize"
        );
    }
}

/// Dialtone Vue `DtRichTextEditor`.
///
/// `value` is Vue `modelValue`. `on_change` is `update:modelValue` in `outputFormat`.
/// `input_aria_label` is required `inputAriaLabel`. `input_class` is `inputClass` on the
/// `ProseMirror` element. `auto_focus` is `autoFocus`.
#[component]
pub fn DtRichTextEditor(
    /// Current value in `output_format` (`modelValue`).
    #[prop(optional, into)]
    value: Option<Signal<String>>,
    /// `update:modelValue`.
    #[prop(optional, into)]
    on_change: Option<Callback<String>>,
    /// `json-input`. Always JSON, separate from `output_format`.
    #[prop(optional, into)]
    on_json_input: Option<Callback<String>>,
    /// `html-input`.
    #[prop(optional, into)]
    on_html_input: Option<Callback<String>>,
    /// `text-input`.
    #[prop(optional, into)]
    on_text_input: Option<Callback<String>>,
    /// `markdown-input`.
    #[prop(optional, into)]
    on_markdown_input: Option<Callback<String>>,
    /// `focus`. The `FocusEvent` is not forwarded.
    #[prop(optional, into)]
    on_focus: Option<Callback<()>>,
    /// `blur`. The `FocusEvent` is not forwarded.
    #[prop(optional, into)]
    on_blur: Option<Callback<()>>,
    /// `enter`. Vue emits this with no payload.
    #[prop(optional, into)]
    on_enter: Option<Callback<()>>,
    /// `edit-link`.
    #[prop(optional, into)]
    on_edit_link: Option<Callback<DtRichTextEditorLink>>,
    /// `selected`.
    #[prop(optional, into)]
    on_selected: Option<Callback<String>>,
    /// `link-click`. Requires `link`.
    #[prop(optional, into)]
    on_link_click: Option<Callback<DtRichTextEditorLinkClick>>,
    /// `editable`.
    #[prop(optional, default = true)]
    editable: bool,
    /// `allowTyping`. When false, only Backspace and Enter (without Shift, when line breaks are off) change the text.
    #[prop(optional, default = true)]
    allow_typing: bool,
    /// `pasteRichText`. When false, the paste is plain text.
    #[prop(optional, default = true)]
    paste_rich_text: bool,
    /// `allowLineBreaks`. Enter submits unless this is set.
    #[prop(optional)]
    allow_line_breaks: bool,
    /// `inputAriaLabel`.
    #[prop(into)]
    input_aria_label: String,
    /// `inputClass` on the editable element.
    #[prop(optional, into)]
    input_class: String,
    /// `autoFocus`: `false`, `true`, `start`, `end`, `all`, or a character offset.
    #[prop(optional, into)]
    auto_focus: String,
    /// `outputFormat`. Default `html`. A signal republishes the current text in
    /// that format and leaves the caret where it is.
    #[prop(into, default = Signal::stored(String::new()))]
    output_format: Signal<String>,
    /// `placeholder`.
    #[prop(optional, into)]
    placeholder: String,
    /// `link`. Shows the bubble menu and autolinks URLs after a space.
    #[prop(optional)]
    link: bool,
    /// `customLink`. Disables autolink and [`force_linkify_pending_text`]. The custom link extension is not ported.
    #[prop(optional)]
    custom_link: bool,
    /// `linkPhoneNumbers`. Accepted and not applied.
    #[prop(optional)]
    link_phone_numbers: Option<Vec<String>>,
    /// `showLinkBubbleMenu`.
    #[prop(optional, default = true)]
    show_link_bubble_menu: bool,
    /// `allowBlockquote`.
    #[prop(optional, default = true)]
    allow_blockquote: bool,
    /// `allowBold`.
    #[prop(optional, default = true)]
    allow_bold: bool,
    /// `allowBulletList`. Also allows the ordered list.
    #[prop(optional, default = true)]
    allow_bullet_list: bool,
    /// `allowItalic`.
    #[prop(optional, default = true)]
    allow_italic: bool,
    /// `allowStrike`.
    #[prop(optional, default = true)]
    allow_strike: bool,
    /// `allowUnderline`.
    #[prop(optional, default = true)]
    allow_underline: bool,
    /// `allowCode`.
    #[prop(optional, default = true)]
    allow_code: bool,
    /// `allowCodeblock`.
    #[prop(optional, default = true)]
    allow_codeblock: bool,
    /// `allowInlineImages`.
    #[prop(optional)]
    allow_inline_images: bool,
    /// `allowFontColor`. Accepted and not applied.
    #[prop(optional)]
    allow_font_color: bool,
    /// `allowBackgroundColor`. Accepted and not applied.
    #[prop(optional)]
    allow_background_color: bool,
    /// `allowFontSize`. Accepted and not applied.
    #[prop(optional)]
    allow_font_size: bool,
    /// `allowFontFamily`. Accepted and not applied.
    #[prop(optional)]
    allow_font_family: bool,
    /// `allowLineHeight`. Accepted and not applied.
    #[prop(optional)]
    allow_line_height: bool,
    /// `allowVariable`. Accepted and not applied.
    #[prop(optional)]
    allow_variable: bool,
    /// `variableItems`. Accepted and not applied.
    #[prop(optional)]
    variable_items: Vec<String>,
    /// `preserveWhitespace`: `full` (default), `true`, or `false`.
    #[prop(optional, into)]
    preserve_whitespace: String,
    /// `useDivTags`. Blocks are `div` instead of `p`.
    #[prop(optional)]
    use_div_tags: bool,
    /// `allowTables`. Keeps table tags. Resize handles are not ported.
    #[prop(optional)]
    allow_tables: bool,
    /// `allowTextAlign`. JSON paragraphs include `textAlign: null`. Alignment commands are not ported.
    #[prop(optional, default = true)]
    allow_text_align: bool,
    /// `allowImageResize`. Accepted and not applied.
    #[prop(optional)]
    allow_image_resize: bool,
) -> impl IntoView {
    if input_aria_label.is_empty() {
        leptos::logging::warn!("DtRichTextEditor input_aria_label is required");
    }
    if !preserve_whitespace.is_empty()
        && !matches!(preserve_whitespace.as_str(), "full" | "true" | "false")
    {
        leptos::logging::warn!(
            "DtRichTextEditor preserve_whitespace `{preserve_whitespace}` is not full, true, or false"
        );
    }
    warn_omitted(
        allow_font_color,
        allow_background_color,
        allow_font_size,
        allow_font_family,
        allow_line_height,
        allow_variable,
        allow_image_resize,
        &variable_items,
        link_phone_numbers.as_ref(),
    );
    let _ = (
        RICH_TEXT_EDITOR_OUTPUT_FORMATS,
        RICH_TEXT_EDITOR_AUTOFOCUS_TYPES,
        RICH_TEXT_EDITOR_SUPPORTED_LINK_PROTOCOLS,
    );
    let marks = Marks {
        bold: allow_bold,
        italic: allow_italic,
        strike: allow_strike,
        underline: allow_underline,
        code: allow_code,
        codeblock: allow_codeblock,
        blockquote: allow_blockquote,
        list: allow_bullet_list,
        link,
        inline_images: allow_inline_images,
        tables: allow_tables,
        text_align: allow_text_align,
        use_div: use_div_tags,
        collapse_whitespace: preserve_whitespace == "false",
    };
    let incoming = value.unwrap_or_else(|| Signal::derive(|| String::new()));
    let state = EditorState {
        editor_ref: NodeRef::<html::Div>::new(),
        marks: StoredValue::new(marks),
        output_format: StoredValue::new(output_format.get_untracked()),
        placeholder: StoredValue::new(placeholder),
        auto_focus: StoredValue::new(auto_focus),
        link: StoredValue::new(link),
        custom_link: StoredValue::new(custom_link),
        paste_rich_text: StoredValue::new(paste_rich_text),
        allow_typing: StoredValue::new(allow_typing),
        allow_line_breaks: StoredValue::new(allow_line_breaks),
        show_bubble: StoredValue::new(show_link_bubble_menu),
        emitted: StoredValue::new(None),
        did_focus: StoredValue::new(false),
        keep_bubble: StoredValue::new(false),
        bubble: RwSignal::new(None),
        on_change,
        on_json: on_json_input,
        on_html: on_html_input,
        on_text: on_text_input,
        on_markdown: on_markdown_input,
        on_enter,
        on_edit_link,
        on_selected,
        on_link_click,
        on_focus,
        on_blur,
    };
    Effect::new(move |_| {
        let format = output_format.get();
        if state.output_format.try_get_value().as_deref() == Some(format.as_str()) {
            return;
        }
        // `try_set_value` returns the value when the store is already disposed.
        if state.output_format.try_set_value(format).is_some() {
            return;
        }
        if editor_element(state.editor_ref).is_some() {
            publish(state);
        }
    });
    Effect::new(move |_| {
        let next = incoming.get();
        let Some(element) = editor_element(state.editor_ref) else {
            return;
        };
        if state.emitted.get_value().as_ref() == Some(&next) {
            return;
        }
        let marks = state.marks.get_value();
        let semantic = model_to_html(&next, &state.output_format.get_value(), &marks);
        let html = display_html(&semantic, &state.placeholder.get_value(), marks.use_div);
        element.set_inner_html(&html);
        state.emitted.set_value(Some(next));
        if !state.did_focus.get_value() {
            apply_autofocus(&element, &state.auto_focus.get_value());
            state.did_focus.set_value(true);
        }
    });
    let prose_class = prose_mirror_class(&input_class);
    let content_editable = if editable { "true" } else { "false" };
    let aria_label = (!input_aria_label.is_empty()).then_some(input_aria_label);
    let aria_readonly = (!editable).then_some("true");
    let bubble_state = state;
    view! {
        <div>
            {move || {
                bubble_state.bubble.get().map(|item| {
                    let top = item.top;
                    let left = item.left;
                    view! {
                        <Portal>
                            <div
                                class="d-popover__dialog"
                                style=bubble_style(top, left)
                                on:mousedown=move |event: MouseEvent| {
                                    event.prevent_default();
                                    bubble_state.keep_bubble.set_value(true);
                                }
                            >
                                <DtStack
                                    direction="row"
                                    gap="0"
                                    class="d-rich-text-editor-bubble-menu__button-stack"
                                >
                                    <DtButton
                                        kind="muted"
                                        importance="clear"
                                        on_click=move || {
                                            let Some(item) = bubble_state.bubble.get_untracked()
                                            else {
                                                return;
                                            };
                                            if let Some(callback) = bubble_state.on_edit_link {
                                                callback.run(DtRichTextEditorLink {
                                                    href: item.href,
                                                    text: item.text,
                                                });
                                            }
                                        }
                                    >
                                        "Edit"
                                    </DtButton>
                                    <DtButton
                                        kind="muted"
                                        importance="clear"
                                        on_click=move || {
                                            if let Some(item) =
                                                bubble_state.bubble.get_untracked()
                                            {
                                                open_link(&item.href);
                                            }
                                        }
                                    >
                                        "Open link"
                                    </DtButton>
                                    <DtButton
                                        kind="critical"
                                        importance="clear"
                                        class="d-btn--critical"
                                        on_click=move || remove_link(bubble_state)
                                    >
                                        "Remove"
                                    </DtButton>
                                </DtStack>
                            </div>
                        </Portal>
                    }
                })
            }}
            <div class=EDITOR_SHELL_CLASS data-qa=EDITOR_QA>
                <div
                    node_ref=state.editor_ref
                    class=prose_class
                    contenteditable=content_editable
                    role="textbox"
                    aria-multiline="true"
                    aria-label=aria_label
                    aria-readonly=aria_readonly
                    on:input=move |_| on_editor_input(state)
                    on:keydown=move |event: KeyboardEvent| on_editor_key(event, state)
                    on:paste=move |event| on_editor_paste(event, state)
                    on:click=move |event: MouseEvent| on_editor_click(event, state)
                    on:mouseup=move |_| {
                        refresh_bubble(state);
                        if let Some(callback) = state.on_selected {
                            callback.run(selected_string());
                        }
                    }
                    on:keyup=move |_| {
                        refresh_bubble(state);
                        if let Some(callback) = state.on_selected {
                            callback.run(selected_string());
                        }
                    }
                    on:focus=move |_| {
                        if let Some(callback) = state.on_focus {
                            callback.run(());
                        }
                    }
                    on:blur=move |_| {
                        let Some(keep) = state.keep_bubble.try_get_value() else {
                            return;
                        };
                        if keep {
                            state.keep_bubble.try_set_value(false);
                        } else {
                            state.bubble.try_set(None);
                        }
                        if let Some(callback) = state.on_blur {
                            callback.run(());
                        }
                    }
                />
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    fn sample_marks() -> Marks {
        Marks {
            bold: true,
            italic: true,
            strike: true,
            underline: true,
            code: true,
            codeblock: true,
            blockquote: true,
            list: true,
            link: true,
            inline_images: false,
            tables: false,
            text_align: true,
            use_div: false,
            collapse_whitespace: false,
        }
    }

    #[test]
    fn prose_class_appends_input_class() {
        assert_eq!(prose_mirror_class(""), "ProseMirror");
        assert_eq!(prose_mirror_class("qa-editor"), "ProseMirror qa-editor");
        assert_eq!(EDITOR_SHELL_CLASS, "d-rich-text-editor");
        assert_eq!(EDITOR_QA, "dt-rich-text-editor");
    }

    #[test]
    fn html_output_wraps_plain_text() {
        let marks = sample_marks();
        assert_eq!(
            model_to_html("initial value", "html", &marks),
            "<p>initial value</p>"
        );
        assert_eq!(
            html_to_output("<p>new value</p>", "html", &marks),
            "<p>new value</p>"
        );
        assert_eq!(
            html_to_output("<div>new value</div>", "html", &marks),
            "<p>new value</p>"
        );
    }

    #[test]
    fn text_output_drops_marks_and_joins_blocks() {
        let marks = sample_marks();
        assert_eq!(
            html_to_output("<p><strong>new</strong> value</p>", "text", &marks),
            "new value"
        );
        assert_eq!(html_to_output("<p>a</p><p>b</p>", "text", &marks), "a\nb");
        assert_eq!(html_to_output("<p>a<br>b</p>", "text", &marks), "a\nb");
    }

    #[test]
    fn json_output_includes_text_align() {
        let marks = sample_marks();
        let value: Value =
            serde_json::from_str(&html_to_output("<p>new value</p>", "json", &marks)).unwrap();
        assert_eq!(value["type"], "doc");
        assert_eq!(value["content"][0]["type"], "paragraph");
        assert!(value["content"][0]["attrs"]["textAlign"].is_null());
        assert_eq!(value["content"][0]["content"][0]["text"], "new value");
    }

    #[test]
    fn markdown_marks_lists_and_code_blocks() {
        let marks = sample_marks();
        assert_eq!(
            html_to_output("<p><strong>bold text</strong></p>", "markdown", &marks),
            "**bold text**"
        );
        assert_eq!(
            html_to_output("<p><em>italic text</em></p>", "markdown", &marks),
            "*italic text*"
        );
        assert_eq!(
            html_to_output("<p><s>strikethrough text</s></p>", "markdown", &marks),
            "~~strikethrough text~~"
        );
        assert_eq!(
            html_to_output(
                "<ul><li><p>one</p></li><li><p>two</p></li></ul>",
                "markdown",
                &marks
            ),
            "- one\n- two"
        );
        assert_eq!(
            html_to_output(
                "<pre class=\"d-rich-text-editor__code-block\"><code>line1\nline2</code></pre>",
                "markdown",
                &marks
            ),
            "```\nline1\nline2\n```"
        );
    }

    #[test]
    fn disallowed_bold_unwraps() {
        let mut marks = sample_marks();
        marks.bold = false;
        assert_eq!(
            html_to_output("<p><strong>x</strong></p>", "html", &marks),
            "<p>x</p>"
        );
    }

    #[test]
    fn force_linkify_www_and_existing_anchors() {
        let linked = force_linkify_pending_text("www.google.com", true, false);
        assert!(linked.contains("href=\"http://www.google.com\""));
        let existing = "<p><a href=\"https://www.google.com\" class=\"d-link d-wb-break-all\">www.google.com</a></p>";
        assert_eq!(force_linkify_pending_text(existing, true, false), existing);
        assert_eq!(
            force_linkify_pending_text("www.google.com", true, true),
            "www.google.com"
        );
        assert_eq!(
            force_linkify_pending_text("www.google.com", false, false),
            "www.google.com"
        );
    }

    #[test]
    fn autolink_requires_a_boundary() {
        let hit = autolink_span("see https://dialpad.com ").unwrap();
        assert_eq!(hit.2, "https://dialpad.com");
        assert!(autolink_span("see https://dialpad.com").is_none());
    }

    #[test]
    fn link_href_adds_the_default_prefix() {
        assert_eq!(
            rich_text_editor_link_href("example.com", "https://"),
            "https://example.com"
        );
        assert_eq!(
            rich_text_editor_link_href("mailto:a@b.c", "https://"),
            "mailto:a@b.c"
        );
        assert_eq!(
            rich_text_editor_link_href("https://example.com", "https://"),
            "https://example.com"
        );
        assert_eq!(rich_text_editor_link_href("", "https://"), "");
    }

    #[test]
    fn unknown_output_format_falls_back_to_html() {
        let marks = sample_marks();
        assert_eq!(
            html_to_output("<p>new value</p>", "rtf", &marks),
            "<p>new value</p>"
        );
    }

    #[test]
    fn typing_and_enter_follow_the_vue_keys() {
        assert!(!typing_is_blocked(false, "Backspace", false, false));
        assert!(typing_is_blocked(false, "a", false, false));
        assert!(!typing_is_blocked(false, "Enter", false, false));
        assert!(typing_is_blocked(false, "Enter", true, false));
        assert!(!typing_is_blocked(true, "a", false, false));
        assert!(enter_submits(false, false));
        assert!(!enter_submits(true, false));
        assert!(!enter_submits(false, true));
        assert!(shift_enter_breaks(false, true));
        assert!(!shift_enter_breaks(true, true));
    }

    #[test]
    fn autofocus_tokens() {
        assert_eq!(autofocus_kind("false"), Autofocus::Off);
        assert_eq!(autofocus_kind(""), Autofocus::Off);
        assert_eq!(autofocus_kind("true"), Autofocus::Start);
        assert_eq!(autofocus_kind("start"), Autofocus::Start);
        assert_eq!(autofocus_kind("end"), Autofocus::End);
        assert_eq!(autofocus_kind("all"), Autofocus::All);
        assert_eq!(autofocus_kind("3"), Autofocus::Position(3));
    }

    #[test]
    fn json_model_round_trips_a_paragraph() {
        let marks = sample_marks();
        let json = r#"{"type":"doc","content":[{"type":"paragraph","content":[{"type":"text","text":"new value","marks":[{"type":"bold"}]}]}]}"#;
        assert_eq!(
            model_to_html(json, "json", &marks),
            "<p><strong>new value</strong></p>"
        );
    }

    #[test]
    fn bubble_style_uses_the_tooltip_z_index() {
        let style = bubble_style(12.0, 4.0);
        assert!(style.contains("z-index:400"));
        assert!(style.contains("top:12"));
        assert!(style.contains("left:4"));
    }
}
