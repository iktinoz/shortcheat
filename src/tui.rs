use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::backend::Backend;
use ratatui::Terminal;

use crate::{model, ui};

/// Run the interactive viewer until the user quits.
pub fn start(apps: Vec<model::App>) -> Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal, apps);
    ratatui::restore();
    result
}

fn run<B: Backend>(terminal: &mut Terminal<B>, apps: Vec<model::App>) -> Result<()> {
    let mut state = ui::State {
        query: String::new(),
        scroll_y: 0,
        apps,
    };

    loop {
        terminal.draw(|f| ui::draw(f, &state))?;

        if !event::poll(Duration::from_millis(100))? {
            continue;
        }
        if let Event::Key(k) = event::read()? {
            if k.kind != KeyEventKind::Press {
                continue;
            }
            if !handle_key(k, &mut state) {
                return Ok(());
            }
        }
    }
}

fn handle_key(k: KeyEvent, state: &mut ui::State) -> bool {
    match (k.code, k.modifiers) {
        (KeyCode::Char('c'), KeyModifiers::CONTROL)
        | (KeyCode::Char('d'), KeyModifiers::CONTROL) => return false,

        (KeyCode::Esc, _) => {
            if state.query.is_empty() {
                return false;
            }
            state.query.clear();
            state.scroll_y = 0;
        }

        (KeyCode::Up, _) => state.scroll_y = state.scroll_y.saturating_sub(1),
        (KeyCode::Down, _) => state.scroll_y = state.scroll_y.saturating_add(1),
        (KeyCode::PageUp, _) => state.scroll_y = state.scroll_y.saturating_sub(10),
        (KeyCode::PageDown, _) => state.scroll_y = state.scroll_y.saturating_add(10),
        (KeyCode::Home, _) => state.scroll_y = 0,

        (KeyCode::Backspace, _) => {
            state.query.pop();
            state.scroll_y = 0;
        }

        (KeyCode::Char(c), m) if m.is_empty() || m == KeyModifiers::SHIFT => {
            state.query.push(c);
            state.scroll_y = 0;
        }

        _ => {}
    }
    true
}
