use pumpkin_plugin_api::text::TextComponent;
use serde_json::{json, Value};

#[derive(Clone, Default, Debug)]
struct StyleState {
    color: Option<String>,
    bold: Option<bool>,
    italic: Option<bool>,
    underlined: Option<bool>,
    strikethrough: Option<bool>,
    obfuscated: Option<bool>,
    click_event: Option<(String, String)>,
    hover_event: Option<String>,
}

pub fn parse_minimessage(input: &str) -> TextComponent {
    if !input.contains('<') {
        if input.contains('§') || input.contains('&') {
            return TextComponent::from_legacy_string(input);
        }
        return TextComponent::text(input);
    }

    let json_str = minimessage_to_json(input);
    TextComponent::from_json(&json_str).unwrap_or_else(|_| {
        TextComponent::from_legacy_string(input)
    })
}

pub fn minimessage_to_json(input: &str) -> String {
    let mut extra: Vec<Value> = Vec::new();
    let mut style_stack: Vec<StyleState> = vec![StyleState::default()];
    let mut chars = input.chars().peekable();
    let mut buffer = String::new();

    let flush_buffer = |buf: &mut String, extra: &mut Vec<Value>, stack: &[StyleState]| {
        if buf.is_empty() {
            return;
        }
        let current_style = stack.last().cloned().unwrap_or_default();
        let mut obj = json!({
            "text": buf.as_str()
        });

        if let Some(c) = current_style.color {
            obj["color"] = json!(c);
        }
        if let Some(b) = current_style.bold {
            obj["bold"] = json!(b);
        }
        if let Some(i) = current_style.italic {
            obj["italic"] = json!(i);
        }
        if let Some(u) = current_style.underlined {
            obj["underlined"] = json!(u);
        }
        if let Some(s) = current_style.strikethrough {
            obj["strikethrough"] = json!(s);
        }
        if let Some(o) = current_style.obfuscated {
            obj["obfuscated"] = json!(o);
        }
        if let Some((action, val)) = current_style.click_event {
            obj["clickEvent"] = json!({
                "action": action,
                "command": val,
                "value": val
            });
        }
        if let Some(hover) = current_style.hover_event {
            obj["hoverEvent"] = json!({
                "action": "show_text",
                "contents": hover,
                "value": hover,
                "text": hover
            });
        }

        extra.push(obj);
        buf.clear();
    };

    while let Some(ch) = chars.next() {
        if ch == '<' {
            let mut tag_content = String::new();
            let mut closed = false;
            while let Some(&next_ch) = chars.peek() {
                if next_ch == '>' {
                    chars.next();
                    closed = true;
                    break;
                }
                tag_content.push(next_ch);
                chars.next();
            }

            if closed && !tag_content.is_empty() {
                let mut temp_stack = style_stack.clone();
                if handle_tag(&tag_content, &mut temp_stack) {
                    flush_buffer(&mut buffer, &mut extra, &style_stack);
                    style_stack = temp_stack;
                } else {
                    buffer.push('<');
                    buffer.push_str(&tag_content);
                    buffer.push('>');
                }
            } else {
                buffer.push('<');
                buffer.push_str(&tag_content);
            }
        } else {
            buffer.push(ch);
        }
    }

    flush_buffer(&mut buffer, &mut extra, &style_stack);

    json!({
        "text": "",
        "extra": extra
    })
    .to_string()
}

fn is_known_closing_tag(tag: &str) -> bool {
    let t = tag.to_ascii_lowercase();
    t.is_empty()
        || t == "color"
        || t == "colour"
        || t == "c"
        || t == "bold"
        || t == "b"
        || t == "italic"
        || t == "i"
        || t == "em"
        || t == "underlined"
        || t == "u"
        || t == "strikethrough"
        || t == "st"
        || t == "obfuscated"
        || t == "obf"
        || t == "reset"
        || t == "r"
        || t == "click"
        || t == "hover"
}

fn handle_tag(tag: &str, stack: &mut Vec<StyleState>) -> bool {
    if let Some(rest) = tag.strip_prefix('/') {
        if is_known_closing_tag(rest) {
            if stack.len() > 1 {
                stack.pop();
            } else if let Some(first) = stack.first_mut() {
                *first = StyleState::default();
            }
            return true;
        }
        return false;
    }

    let mut current = stack.last().cloned().unwrap_or_default();

    if let Some(hex) = extract_color(tag) {
        current.color = Some(hex);
        stack.push(current);
        return true;
    }

    if tag.eq_ignore_ascii_case("bold") || tag.eq_ignore_ascii_case("b") {
        current.bold = Some(true);
        stack.push(current);
        return true;
    }

    if tag.eq_ignore_ascii_case("italic")
        || tag.eq_ignore_ascii_case("i")
        || tag.eq_ignore_ascii_case("em")
    {
        current.italic = Some(true);
        stack.push(current);
        return true;
    }

    if tag.eq_ignore_ascii_case("underlined") || tag.eq_ignore_ascii_case("u") {
        current.underlined = Some(true);
        stack.push(current);
        return true;
    }

    if tag.eq_ignore_ascii_case("strikethrough") || tag.eq_ignore_ascii_case("st") {
        current.strikethrough = Some(true);
        stack.push(current);
        return true;
    }

    if tag.eq_ignore_ascii_case("obfuscated") || tag.eq_ignore_ascii_case("obf") {
        current.obfuscated = Some(true);
        stack.push(current);
        return true;
    }

    if tag.eq_ignore_ascii_case("reset") || tag.eq_ignore_ascii_case("r") {
        stack.push(StyleState::default());
        return true;
    }

    if let Some(rest) = tag.strip_prefix("click:") {
        if let Some(cmd) = rest.strip_prefix("run_command:") {
            let clean = cmd.trim().trim_matches('\'').trim_matches('"');
            current.click_event = Some(("run_command".to_string(), clean.to_string()));
            stack.push(current);
            return true;
        }
        if let Some(cmd) = rest.strip_prefix("suggest_command:") {
            let clean = cmd.trim().trim_matches('\'').trim_matches('"');
            current.click_event = Some(("suggest_command".to_string(), clean.to_string()));
            stack.push(current);
            return true;
        }
        if let Some(url) = rest.strip_prefix("open_url:") {
            let clean = url.trim().trim_matches('\'').trim_matches('"');
            current.click_event = Some(("open_url".to_string(), clean.to_string()));
            stack.push(current);
            return true;
        }
    }

    if let Some(text) = tag.strip_prefix("hover:show_text:") {
        let clean = text.trim().trim_matches('\'').trim_matches('"');
        current.hover_event = Some(clean.to_string());
        stack.push(current);
        return true;
    }

    false
}

fn extract_color(tag: &str) -> Option<String> {
    if let Some(rest) = tag.strip_prefix("color:") {
        return normalize_hex(rest);
    }
    if let Some(rest) = tag.strip_prefix("colour:") {
        return normalize_hex(rest);
    }
    if let Some(rest) = tag.strip_prefix("c:") {
        return normalize_hex(rest);
    }
    if tag.starts_with('#') {
        return normalize_hex(tag);
    }

    match tag.to_ascii_lowercase().as_str() {
        "black" => Some("#000000".to_string()),
        "dark_blue" => Some("#0000AA".to_string()),
        "dark_green" => Some("#00AA00".to_string()),
        "dark_aqua" => Some("#00AAAA".to_string()),
        "dark_red" => Some("#AA0000".to_string()),
        "dark_purple" => Some("#AA00AA".to_string()),
        "gold" => Some("#FFAA00".to_string()),
        "gray" | "grey" => Some("#AAAAAA".to_string()),
        "dark_gray" | "dark_grey" => Some("#555555".to_string()),
        "blue" => Some("#5555FF".to_string()),
        "green" => Some("#55FF55".to_string()),
        "aqua" => Some("#55FFFF".to_string()),
        "red" => Some("#FF5555".to_string()),
        "light_purple" => Some("#FF55FF".to_string()),
        "yellow" => Some("#FFFF55".to_string()),
        "white" => Some("#FFFFFF".to_string()),
        _ => None,
    }
}

fn normalize_hex(s: &str) -> Option<String> {
    let clean = s.trim().trim_start_matches('#');
    if clean.len() == 6 && clean.chars().all(|c| c.is_ascii_hexdigit()) {
        Some(format!("#{}", clean.to_uppercase()))
    } else {
        None
    }
}

pub fn strip_minimessage(input: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for ch in input.chars() {
        if ch == '<' {
            in_tag = true;
        } else if ch == '>' && in_tag {
            in_tag = false;
        } else if !in_tag {
            out.push(ch);
        }
    }
    out
}

pub fn minimessage_to_legacy(input: &str) -> String {
    let mut out = String::new();
    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '<' {
            let mut tag = String::new();
            let mut closed = false;
            while let Some(&next_ch) = chars.peek() {
                if next_ch == '>' {
                    chars.next();
                    closed = true;
                    break;
                }
                tag.push(next_ch);
                chars.next();
            }
            if closed {
                let tag_lower = tag.to_ascii_lowercase();
                if tag_lower.starts_with('/') {
                    out.push_str("§r");
                } else if tag_lower == "bold" || tag_lower == "b" {
                    out.push_str("§l");
                } else if tag_lower == "italic" || tag_lower == "i" || tag_lower == "em" {
                    out.push_str("§o");
                } else if tag_lower == "underlined" || tag_lower == "u" {
                    out.push_str("§n");
                } else if tag_lower == "strikethrough" || tag_lower == "st" {
                    out.push_str("§m");
                } else if tag_lower == "obfuscated" || tag_lower == "obf" {
                    out.push_str("§k");
                } else if tag_lower == "reset" || tag_lower == "r" {
                    out.push_str("§r");
                } else if let Some(hex) = extract_color(&tag) {
                    let clean = hex.trim_start_matches('#');
                    out.push_str("§x");
                    for c in clean.chars() {
                        out.push('§');
                        out.push(c);
                    }
                }
            } else {
                out.push('<');
                out.push_str(&tag);
            }
        } else {
            out.push(ch);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimessage_color_parsing() {
        let json_output = minimessage_to_json("<color:#A0A0A0>normal</color>");
        let val: Value = serde_json::from_str(&json_output).unwrap();
        assert_eq!(val["extra"][0]["text"], "normal");
        assert_eq!(val["extra"][0]["color"], "#A0A0A0");
    }

    #[test]
    fn test_nested_tags_and_formatting() {
        let json_output = minimessage_to_json(
            "<color:#E2B700><bold>Duels</bold></color> <color:#00E262>Won!</color>",
        );
        let val: Value = serde_json::from_str(&json_output).unwrap();
        assert_eq!(val["extra"][0]["text"], "Duels");
        assert_eq!(val["extra"][0]["color"], "#E2B700");
        assert_eq!(val["extra"][0]["bold"], true);
        assert_eq!(val["extra"][1]["text"], " ");
        assert_eq!(val["extra"][2]["text"], "Won!");
        assert_eq!(val["extra"][2]["color"], "#00E262");
    }

    #[test]
    fn test_click_and_hover_parsing() {
        let json_output = minimessage_to_json(
            "<click:run_command:'/duel accept Notch'><hover:show_text:'Click to accept!'>[ACCEPT]</hover></click>",
        );
        let val: Value = serde_json::from_str(&json_output).unwrap();
        assert_eq!(val["extra"][0]["text"], "[ACCEPT]");
        assert_eq!(val["extra"][0]["clickEvent"]["action"], "run_command");
        assert_eq!(val["extra"][0]["clickEvent"]["value"], "/duel accept Notch");
        assert_eq!(val["extra"][0]["clickEvent"]["command"], "/duel accept Notch");
        assert_eq!(val["extra"][0]["hoverEvent"]["action"], "show_text");
        assert_eq!(val["extra"][0]["hoverEvent"]["contents"], "Click to accept!");
    }

    #[test]
    fn test_strip_and_legacy_minimessage() {
        let input = "<color:#00E262><bold>NoDebuff</bold></color>";
        assert_eq!(strip_minimessage(input), "NoDebuff");
        assert_eq!(minimessage_to_legacy(input), "§x§0§0§E§2§6§2§lNoDebuff§r§r");
    }
}
