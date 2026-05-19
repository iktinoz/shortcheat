use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

use crate::filter::filter_apps;
use crate::model::{App, Shortcut};
use crate::theme::{
    icon_for, parse_chips, ON_SURFACE, ON_SURFACE_VARIANT, OUTLINE_VARIANT, PRIMARY, SECONDARY,
    SURFACE_CONTAINER_HIGHEST, TERTIARY,
};

pub const COLUMNS: usize = 4;

// Reserved chip column width = render width of "MOD + CTRL + SHFT".
// chip pill = label_chars + 2 (single-space padding each side);
// separator " + " = 3 chars.
//   " MOD "(5) + " + "(3) + " CTRL "(6) + " + "(3) + " SHFT "(6) = 23
const CHIP_AREA_WIDTH: usize = 23;
const CHIP_DESC_GAP: usize = 2;

pub struct State {
    pub query: String,
    pub scroll_y: u16,
    pub apps: Vec<App>,
}

pub fn draw(frame: &mut Frame, state: &State) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(area);

    draw_search(frame, chunks[0], state);
    draw_grid(frame, chunks[1], state);
    draw_footer(frame, chunks[2]);
}

fn draw_search(frame: &mut Frame, area: Rect, state: &State) {
    let title = Line::from(vec![
        Span::raw(" "),
        Span::styled(
            "SEARCH",
            Style::default().fg(PRIMARY).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(OUTLINE_VARIANT))
        .title(title);

    let mut spans = vec![Span::styled(
        " › ",
        Style::default().fg(PRIMARY).add_modifier(Modifier::BOLD),
    )];
    if state.query.is_empty() {
        spans.push(Span::styled("▎", Style::default().fg(PRIMARY)));
        spans.push(Span::styled(
            "Search shortcuts…",
            Style::default().fg(ON_SURFACE_VARIANT),
        ));
    } else {
        spans.push(Span::styled(
            state.query.clone(),
            Style::default().fg(ON_SURFACE),
        ));
        spans.push(Span::styled("▎", Style::default().fg(PRIMARY)));
    }

    let para = Paragraph::new(Line::from(spans)).block(block);
    frame.render_widget(para, area);
}

fn draw_footer(frame: &mut Frame, area: Rect) {
    let hint = Line::from(vec![
        Span::raw("  "),
        Span::styled("↑↓", Style::default().fg(PRIMARY)),
        Span::styled(" scroll   ", Style::default().fg(ON_SURFACE_VARIANT)),
        Span::styled("PgUp/PgDn", Style::default().fg(PRIMARY)),
        Span::styled(" page   ", Style::default().fg(ON_SURFACE_VARIANT)),
        Span::styled("Esc", Style::default().fg(PRIMARY)),
        Span::styled(" clear / quit   ", Style::default().fg(ON_SURFACE_VARIANT)),
        Span::styled("Ctrl-C", Style::default().fg(PRIMARY)),
        Span::styled(" quit", Style::default().fg(ON_SURFACE_VARIANT)),
    ]);
    let para = Paragraph::new(hint);
    frame.render_widget(para, area);
}

fn draw_grid(frame: &mut Frame, area: Rect, state: &State) {
    let filtered = filter_apps(&state.apps, &state.query);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(OUTLINE_VARIANT));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if filtered.is_empty() {
        let msg = Paragraph::new(Line::from(Span::styled(
            "No matches",
            Style::default().fg(ON_SURFACE_VARIANT),
        )))
        .alignment(Alignment::Center);
        frame.render_widget(msg, inner);
        return;
    }

    let constraints: Vec<Constraint> = (0..COLUMNS)
        .map(|_| Constraint::Percentage(100 / COLUMNS as u16))
        .collect();
    let column_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .horizontal_margin(1)
        .vertical_margin(1)
        .spacing(2)
        .split(inner);

    let col_height = column_areas
        .first()
        .map(|r| r.height as usize)
        .unwrap_or(0)
        .saturating_add(state.scroll_y as usize);
    let cols = fill_sequential(&filtered, col_height);

    for (i, col_apps) in cols.iter().enumerate() {
        let col_area = column_areas[i];
        let mut lines: Vec<Line<'static>> = Vec::new();
        for app in col_apps {
            lines.extend(build_app_lines(app, col_area.width as usize));
        }
        let para = Paragraph::new(lines).scroll((state.scroll_y, 0));
        frame.render_widget(para, col_area);
    }
}

fn build_app_lines(app: &App, col_width: usize) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();
    let icon = icon_for(&app.app);
    let title = if icon.is_empty() {
        app.app.clone()
    } else {
        format!("{}  {}", icon, app.app)
    };
    lines.push(Line::from(Span::styled(
        title,
        Style::default().fg(PRIMARY).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));

    for section in &app.sections {
        let icon = icon_for(&section.name);
        let header = if icon.is_empty() {
            section.name.to_uppercase()
        } else {
            format!("{}  {}", icon, section.name.to_uppercase())
        };
        lines.push(Line::from(Span::styled(
            header,
            Style::default().fg(SECONDARY).add_modifier(Modifier::BOLD),
        )));
        for shortcut in &section.shortcuts {
            lines.extend(shortcut_lines(shortcut, col_width));
        }
        lines.push(Line::from(""));
    }
    lines
}

fn shortcut_lines(s: &Shortcut, col_width: usize) -> Vec<Line<'static>> {
    let chips = parse_chips(&s.keys);
    let chip_spans = render_chip_spans(&chips);
    let chip_width = chip_render_width(&chips);

    let desc_indent = CHIP_AREA_WIDTH + CHIP_DESC_GAP;

    if chip_width <= CHIP_AREA_WIDTH && desc_indent < col_width {
        // Single line: chips left, padded to fixed area, then description.
        let pad = (CHIP_AREA_WIDTH - chip_width) + CHIP_DESC_GAP;
        let desc_max = col_width.saturating_sub(desc_indent);
        let desc = truncate(&s.description, desc_max);

        let mut spans = chip_spans;
        spans.push(Span::raw(" ".repeat(pad)));
        spans.push(Span::styled(desc, Style::default().fg(ON_SURFACE)));
        vec![Line::from(spans)]
    } else {
        // Chips overflow the fixed area → break: chips on row 1, indented desc on row 2.
        let line1 = Line::from(chip_spans);
        let indent_chars = desc_indent.min(col_width);
        let desc_max = col_width.saturating_sub(indent_chars);
        let desc = truncate(&s.description, desc_max);
        let line2 = Line::from(vec![
            Span::raw(" ".repeat(indent_chars)),
            Span::styled(desc, Style::default().fg(ON_SURFACE)),
        ]);
        vec![line1, line2]
    }
}

fn render_chip_spans(chips: &[String]) -> Vec<Span<'static>> {
    let chip_style = Style::default()
        .fg(TERTIARY)
        .bg(SURFACE_CONTAINER_HIGHEST)
        .add_modifier(Modifier::BOLD);
    let plus_style = Style::default().fg(ON_SURFACE_VARIANT);

    let mut spans = Vec::with_capacity(chips.len() * 2);
    for (i, chip) in chips.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled(" + ", plus_style));
        }
        spans.push(Span::styled(format!(" {} ", chip), chip_style));
    }
    spans
}

fn chip_render_width(chips: &[String]) -> usize {
    let chip_chars: usize = chips.iter().map(|c| c.chars().count() + 2).sum();
    let sep = 3 * chips.len().saturating_sub(1);
    chip_chars + sep
}

fn shortcut_height(s: &Shortcut) -> usize {
    if chip_render_width(&parse_chips(&s.keys)) <= CHIP_AREA_WIDTH {
        1
    } else {
        2
    }
}

fn truncate(s: &str, max_chars: usize) -> String {
    let count = s.chars().count();
    if max_chars == 0 {
        return String::new();
    }
    if count <= max_chars {
        s.to_string()
    } else if max_chars == 1 {
        "…".into()
    } else {
        let kept: String = s.chars().take(max_chars - 1).collect();
        format!("{}…", kept)
    }
}

fn measure_app(app: &App) -> usize {
    let mut h = 2;
    for section in &app.sections {
        h += 1;
        for shortcut in &section.shortcuts {
            h += shortcut_height(shortcut);
        }
        h += 1;
    }
    h
}

fn fill_sequential(apps: &[App], col_height: usize) -> Vec<Vec<App>> {
    let mut cols: Vec<Vec<App>> = (0..COLUMNS).map(|_| Vec::new()).collect();
    let cap = col_height.max(1);
    let mut col_idx = 0;
    let mut used = 0usize;

    for app in apps {
        let h = measure_app(app);
        if used > 0 && used + h > cap && col_idx + 1 < COLUMNS {
            col_idx += 1;
            used = 0;
        }
        cols[col_idx].push(app.clone());
        used += h + 1;
    }
    cols
}
