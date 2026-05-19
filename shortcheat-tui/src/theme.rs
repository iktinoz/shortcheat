use ratatui::style::Color;

pub const SURFACE_CONTAINER_HIGHEST: Color = Color::Rgb(0x2e, 0x36, 0x3c);
pub const PRIMARY: Color = Color::Rgb(0xc2, 0xdc, 0x99);
pub const SECONDARY: Color = Color::Rgb(0xff, 0xb5, 0x95);
pub const ON_SURFACE: Color = Color::Rgb(0xdb, 0xe3, 0xeb);
pub const ON_SURFACE_VARIANT: Color = Color::Rgb(0x9f, 0xa8, 0xb0);
pub const TERTIARY: Color = Color::Rgb(0xd2, 0xb4, 0x78);
pub const OUTLINE_VARIANT: Color = Color::Rgb(0x45, 0x48, 0x3d);

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
