//! Prism-style tokens for Leptos, Rust, Vue, HTML, and JavaScript samples.
//!
//! The classes match Dialtone's docs syntax theme (`token keyword`, `token tag`,
//! and the rest). The demo paints them in `tailwind.css` so light and dark stay
//! readable. Every character of `source` is escaped, so the result is safe to
//! assign as `innerHTML`.

pub(super) fn code(source: &str) -> String {
    let mut highlighter = Highlighter {
        src: source,
        index: 0,
        out: String::with_capacity(source.len().saturating_mul(2)),
    };
    highlighter.run();
    highlighter.out
}

struct Highlighter<'a> {
    src: &'a str,
    index: usize,
    out: String,
}

impl Highlighter<'_> {
    fn run(&mut self) {
        while self.index < self.src.len() {
            let before = self.index;
            self.scan_token();
            if self.index == before {
                break;
            }
        }
    }

    /// Returns the bracket depth change: `1` for an opener, `-1` for a closer.
    fn scan_token(&mut self) -> i32 {
        if self.index >= self.src.len() {
            return 0;
        }
        if self.scan_whitespace()
            || self.scan_comment()
            || self.scan_string("string")
            || self.scan_operator()
            || self.scan_tag()
        {
            return 0;
        }
        let ch = self.peek().unwrap_or('\0');
        if ch.is_ascii_digit() {
            self.scan_number();
            return 0;
        }
        if is_ident_start(ch) {
            self.scan_ident();
            return 0;
        }
        if matches!(ch, '(' | '[' | '{') {
            self.emit_len("punctuation", ch.len_utf8());
            return 1;
        }
        if matches!(ch, ')' | ']' | '}') {
            self.emit_len("punctuation", ch.len_utf8());
            return -1;
        }
        if matches!(ch, '<' | '>' | '.' | ',' | ';' | ':') {
            self.emit_len("punctuation", ch.len_utf8());
            return 0;
        }
        self.emit_raw_len(ch.len_utf8());
        0
    }

    fn scan_whitespace(&mut self) -> bool {
        let rest = self.rest();
        let len = rest
            .chars()
            .take_while(|ch| ch.is_whitespace())
            .map(char::len_utf8)
            .sum();
        if len == 0 {
            return false;
        }
        self.emit_raw_len(len);
        true
    }

    fn scan_comment(&mut self) -> bool {
        let rest = self.rest();
        if rest.starts_with("//") {
            let len = rest.find('\n').unwrap_or(rest.len());
            self.emit_len("comment", len);
            return true;
        }
        if rest.starts_with("/*") {
            let len = rest.find("*/").map(|at| at + 2).unwrap_or(rest.len());
            self.emit_len("comment", len);
            return true;
        }
        if rest.starts_with("<!--") {
            let len = rest.find("-->").map(|at| at + 3).unwrap_or(rest.len());
            self.emit_len("comment", len);
            return true;
        }
        false
    }

    fn scan_string(&mut self, class: &str) -> bool {
        let Some(quote) = self.peek() else {
            return false;
        };
        if quote != '"' && quote != '\'' && quote != '`' {
            return false;
        }
        if quote == '\'' && self.scan_lifetime() {
            return true;
        }
        let start = self.index;
        self.index += quote.len_utf8();
        while self.index < self.src.len() {
            let ch = self.peek().unwrap();
            self.index += ch.len_utf8();
            if ch == '\\' {
                if let Some(next) = self.peek() {
                    self.index += next.len_utf8();
                }
                continue;
            }
            if ch == quote {
                break;
            }
        }
        self.push_span(class, start);
        true
    }

    /// `'static` is a lifetime. `'a'` and `'Loading'` stay strings.
    fn scan_lifetime(&mut self) -> bool {
        let rest = self.rest();
        let mut chars = rest.chars();
        if chars.next() != Some('\'') {
            return false;
        }
        let Some(first) = chars.next() else {
            return false;
        };
        if !is_ident_start(first) {
            return false;
        }
        let mut len = 1 + first.len_utf8();
        for ch in chars {
            if is_ident_char(ch) {
                len += ch.len_utf8();
                continue;
            }
            if ch == '\'' {
                return false;
            }
            break;
        }
        self.emit_len("punctuation", 1);
        self.emit_len("class-name", len - 1);
        true
    }

    fn scan_operator(&mut self) -> bool {
        let rest = self.rest();
        for op in ["::", "->", "=>", "..", "<=", ">=", "==", "!=", "&&", "||"] {
            if rest.starts_with(op) {
                let class = if op == "::" {
                    "punctuation"
                } else {
                    "operator"
                };
                self.emit_len(class, op.len());
                return true;
            }
        }
        false
    }

    fn scan_number(&mut self) {
        let rest = self.rest();
        let mut len = rest
            .chars()
            .take_while(|ch| ch.is_ascii_digit())
            .map(char::len_utf8)
            .sum();
        let after = &rest[len..];
        if after.starts_with('.') {
            let fraction: usize = after[1..]
                .chars()
                .take_while(|ch| ch.is_ascii_digit())
                .map(char::len_utf8)
                .sum();
            if fraction > 0 {
                len += 1 + fraction;
            }
        }
        self.emit_len("number", len);
    }

    fn scan_ident(&mut self) {
        let len = self.ident_len();
        let word = &self.src[self.index..self.index + len];
        let class = if is_boolean(word) {
            "boolean"
        } else if is_keyword(word) {
            "keyword"
        } else if word.chars().next().is_some_and(|ch| ch.is_uppercase())
            && self.pascal_is_type(len)
        {
            "class-name"
        } else if self.after_name_is(len, '(')
            || (self.after_name_is(len, '!') && !self.rest()[len..].starts_with("!="))
        {
            "function"
        } else {
            ""
        };
        if class.is_empty() {
            self.emit_raw_len(len);
        } else {
            self.emit_len(class, len);
        }
    }

    fn scan_tag(&mut self) -> bool {
        if !self.starts_tag() {
            return false;
        }
        self.emit_len("punctuation", 1);
        if self.rest().starts_with('/') {
            self.emit_len("punctuation", 1);
        }
        if self.tag_name_len() > 0 {
            self.emit_len("tag", self.tag_name_len());
        }
        while self.index < self.src.len() {
            if self.rest().starts_with("/>") {
                self.emit_len("punctuation", 2);
                break;
            }
            if self.rest().starts_with('>') {
                self.emit_len("punctuation", 1);
                break;
            }
            if self.scan_whitespace() {
                continue;
            }
            if let Some(attr) = self.attr_name_len() {
                self.emit_len("attr-name", attr);
                if self.rest().starts_with('=') {
                    self.emit_len("operator", 1);
                    if self.peek().is_some_and(|ch| matches!(ch, '"' | '\'' | '`')) {
                        self.scan_string("attr-value");
                    } else {
                        self.scan_attr_expr();
                    }
                }
                continue;
            }
            let before = self.index;
            self.scan_token();
            if self.index == before {
                break;
            }
        }
        true
    }

    fn scan_attr_expr(&mut self) {
        let mut depth = 0i32;
        while self.index < self.src.len() {
            if depth == 0 {
                let rest = self.rest();
                if rest.starts_with("/>") || rest.starts_with('>') {
                    return;
                }
                if rest.starts_with('\n') && self.looking_at_next_attr() {
                    return;
                }
            }
            let before = self.index;
            depth += self.scan_token();
            if depth < 0 {
                depth = 0;
            }
            if self.index == before {
                break;
            }
        }
    }

    fn looking_at_next_attr(&self) -> bool {
        let bytes = self.rest().as_bytes();
        if bytes.first() != Some(&b'\n') {
            return false;
        }
        let mut at = 1;
        while at < bytes.len() && matches!(bytes[at], b' ' | b'\t') {
            at += 1;
        }
        if matches!(bytes.get(at), Some(b':' | b'@' | b'#')) {
            at += 1;
        }
        if at >= bytes.len() || !is_ident_start(bytes[at] as char) {
            return false;
        }
        while at < bytes.len() && is_tag_char(bytes[at] as char) {
            at += 1;
        }
        while at < bytes.len() && matches!(bytes[at], b' ' | b'\t') {
            at += 1;
        }
        at < bytes.len() && bytes[at] == b'='
    }

    fn starts_tag(&self) -> bool {
        let rest = self.rest();
        if !rest.starts_with('<') || rest.starts_with("<!--") || rest.starts_with("<=") {
            return false;
        }
        let after = &rest[1..];
        if after.starts_with('/') {
            return after[1..].chars().next().is_some_and(is_ident_start);
        }
        // `Vec<T>` and `foo::<T>` are generics. A tag follows whitespace or punctuation.
        if self.prev_is_ident() {
            return false;
        }
        after.chars().next().is_some_and(is_ident_start)
    }

    fn prev_is_ident(&self) -> bool {
        self.src[..self.index]
            .chars()
            .next_back()
            .is_some_and(|ch| is_ident_char(ch) || ch == ':')
    }

    fn attr_name_len(&self) -> Option<usize> {
        let rest = self.rest();
        let mut chars = rest.chars();
        let first = chars.next()?;
        let mut len = 0;
        if matches!(first, '@' | ':' | '#' | '.') {
            len += first.len_utf8();
            let next = chars.next()?;
            if !is_ident_start(next) {
                return None;
            }
            len += next.len_utf8();
        } else if is_ident_start(first) {
            len += first.len_utf8();
        } else {
            return None;
        }
        for ch in chars {
            if is_tag_char(ch) || ch == ':' {
                len += ch.len_utf8();
            } else {
                break;
            }
        }
        Some(len)
    }

    fn ident_len(&self) -> usize {
        self.rest()
            .chars()
            .take_while(|ch| is_ident_char(*ch))
            .map(char::len_utf8)
            .sum()
    }

    fn tag_name_len(&self) -> usize {
        self.rest()
            .chars()
            .take_while(|ch| is_tag_char(*ch))
            .map(char::len_utf8)
            .sum()
    }

    /// `Vec<String>` and `RwSignal::new` name a type. `Loaded content` does not.
    fn pascal_is_type(&self, len: usize) -> bool {
        let next = self.src[self.index + len..]
            .chars()
            .find(|ch| !ch.is_whitespace());
        if matches!(next, Some(':' | '<' | '(' | ';' | ',')) {
            return true;
        }
        let prev = self.src[..self.index]
            .chars()
            .rev()
            .find(|ch| !ch.is_whitespace());
        matches!(prev, Some(':' | '<' | '(' | ','))
    }

    fn after_name_is(&self, name_len: usize, expected: char) -> bool {
        self.src[self.index + name_len..]
            .chars()
            .find(|ch| !ch.is_whitespace())
            == Some(expected)
    }

    fn peek(&self) -> Option<char> {
        self.rest().chars().next()
    }

    fn rest(&self) -> &str {
        &self.src[self.index..]
    }

    fn emit_len(&mut self, class: &str, len: usize) {
        let start = self.index;
        self.index += len;
        self.push_span(class, start);
    }

    fn emit_raw_len(&mut self, len: usize) {
        let end = self.index + len;
        push_escape(&mut self.out, &self.src[self.index..end]);
        self.index = end;
    }

    fn push_span(&mut self, class: &str, start: usize) {
        self.out.push_str("<span class=\"token ");
        self.out.push_str(class);
        self.out.push_str("\">");
        push_escape(&mut self.out, &self.src[start..self.index]);
        self.out.push_str("</span>");
    }
}

fn push_escape(out: &mut String, value: &str) {
    for ch in value.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            other => out.push(other),
        }
    }
}

fn is_ident_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_ident_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

fn is_tag_char(ch: char) -> bool {
    is_ident_char(ch) || ch == '-'
}

fn is_boolean(word: &str) -> bool {
    matches!(word, "true" | "false")
}

fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "as" | "async"
            | "await"
            | "break"
            | "case"
            | "catch"
            | "class"
            | "const"
            | "continue"
            | "crate"
            | "default"
            | "delete"
            | "do"
            | "dyn"
            | "else"
            | "enum"
            | "export"
            | "extends"
            | "extern"
            | "fn"
            | "for"
            | "from"
            | "function"
            | "if"
            | "impl"
            | "import"
            | "in"
            | "instanceof"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "null"
            | "of"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "switch"
            | "this"
            | "throw"
            | "trait"
            | "try"
            | "type"
            | "typeof"
            | "undefined"
            | "unsafe"
            | "use"
            | "var"
            | "void"
            | "where"
            | "while"
            | "yield"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highlights_a_leptos_tag_and_a_rust_attribute_expression() {
        let html = code(
            "<DtDescriptionList\n    items=vec![\n        DtDescriptionListItem::new(\"Local time\", \"10:36 AM\"),\n    ]\n/>",
        );
        assert!(html.contains("token tag\">DtDescriptionList<"));
        assert!(html.contains("token attr-name\">items<"));
        assert!(html.contains("token function\">vec<"));
        assert!(html.contains("token class-name\">DtDescriptionListItem<"));
        assert!(html.contains("token function\">new<"));
        assert!(html.contains("token string\">&quot;Local time&quot;<"));
        assert!(html.contains("token punctuation\">/&gt;<"));
    }

    #[test]
    fn generics_and_comparisons_are_not_tags() {
        let generic = code("let value = Vec<String>::new();");
        assert!(generic.contains("token keyword\">let<"));
        assert!(generic.contains("token class-name\">Vec<"));
        assert!(generic.contains("token class-name\">String<"));
        assert!(!generic.contains("token tag"));
        let compared = code("if a < b { }");
        assert!(compared.contains("token keyword\">if<"));
        assert!(!compared.contains("token tag"));
    }

    #[test]
    fn keeps_text_between_vue_tags() {
        let html = code(
            "<dt-skeleton\n  v-if=\"loading\"\n  :aria-label=\"$i18n('Loading')\"\n/>\n<p\n  v-else\n  aria-live=\"polite\"\n  aria-busy=\"false\"\n>\n  Loaded content.\n</p>",
        );
        assert!(html.contains("Loaded content"), "missing text in {html}");
    }

    #[test]
    fn vue_attributes_and_markup_are_escaped() {
        let html = code("<dt-skeleton\n  v-if=\"loading\"\n  :aria-label=\"$i18n('Loading')\"\n/>");
        assert!(html.contains("token tag\">dt-skeleton<"));
        assert!(html.contains("token attr-name\">v-if<"));
        assert!(html.contains("token attr-name\">:aria-label<"));
        assert!(html.contains("token attr-value\">&quot;$i18n('Loading')&quot;<"));
        let escaped = code("let s = \"<script>\";");
        assert!(!escaped.contains("<script"));
        assert!(escaped.contains("&lt;script&gt;"));
    }
}
