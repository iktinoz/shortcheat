pub fn normalize_key(part: &str) -> String {
    let lower = part.trim().to_lowercase();
    match lower.as_str() {
        "super" | "meta" => "MOD".into(),
        "ctrl" | "control" => "CTRL".into(),
        "shift" => "SHFT".into(),
        "alt" => "ALT".into(),
        "return" | "enter" => "RET".into(),
        "backspace" => "BKSP".into(),
        "escape" => "ESC".into(),
        "tab" => "TAB".into(),
        "space" => "SPC".into(),
        "up" => "↑".into(),
        "down" => "↓".into(),
        "left" => "←".into(),
        "right" => "→".into(),
        "print" => "PRT".into(),
        "delete" => "DEL".into(),
        "home" => "HOM".into(),
        "end" => "END".into(),
        "pageup" => "PG↑".into(),
        "pagedown" => "PG↓".into(),
        "mouse scroll" => "SCR".into(),
        s if s.starts_with('f')
            && s.len() <= 3
            && s[1..].chars().all(|c| c.is_ascii_digit())
            && !s[1..].is_empty() =>
        {
            s.to_uppercase()
        }
        s if part.len() <= 5 => s.to_uppercase(),
        _ => part.trim().to_string(),
    }
}

pub fn parse_chips(keys: &str) -> Vec<String> {
    let chips: Vec<String> = keys
        .split('+')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(normalize_key)
        .collect();
    if chips.is_empty() {
        vec![keys.chars().take(6).collect::<String>().to_uppercase()]
    } else {
        chips
    }
}

const ICONS: &[(&str, &str)] = &[
    ("hyprland", "◈"),
    ("kde", "⬡"),
    ("firefox", "◎"),
    ("vs code", "⌥"),
    ("vscode", "⌥"),
    ("window", "⬜"),
    ("workspace", "◱"),
    ("virtual desktop", "◱"),
    ("navigation", "⇥"),
    ("system", "⚙"),
    ("application", "▦"),
    ("terminal", "▶"),
    ("shell", "▶"),
    ("developer", "⎇"),
    ("editing", "✎"),
    ("page", "☰"),
    ("git", "⎇"),
    ("session", "⏻"),
];

pub fn icon_for(name: &str) -> &'static str {
    let lower = name.to_lowercase();
    for (key, icon) in ICONS {
        if lower.contains(key) {
            return icon;
        }
    }
    ""
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modifiers_are_normalized() {
        assert_eq!(parse_chips("Super + Shift + Q"), ["MOD", "SHFT", "Q"]);
        assert_eq!(parse_chips("Control+Alt+Delete"), ["CTRL", "ALT", "DEL"]);
        assert_eq!(parse_chips("Return"), ["RET"]);
    }

    #[test]
    fn arrows_and_function_keys() {
        assert_eq!(parse_chips("Alt + Left"), ["ALT", "←"]);
        assert_eq!(parse_chips("f5"), ["F5"]);
        assert_eq!(parse_chips("F12"), ["F12"]);
    }

    #[test]
    fn short_keys_are_uppercased_long_keys_kept() {
        assert_eq!(normalize_key("x"), "X");
        assert_eq!(normalize_key("Mouse Scroll"), "SCR");
        assert_eq!(normalize_key("Backtick"), "Backtick");
    }

    #[test]
    fn empty_input_falls_back_to_raw_text() {
        assert_eq!(parse_chips(" + "), [" + "]);
        assert_eq!(parse_chips(""), [""]);
    }

    #[test]
    fn icon_lookup_is_case_insensitive_substring() {
        assert_eq!(icon_for("Hyprland"), "◈");
        assert_eq!(icon_for("VS Code"), "⌥");
        assert_eq!(icon_for("unknown"), "");
    }
}
