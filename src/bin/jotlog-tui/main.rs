use std::io;

use failure;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Corner, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, SelectableList, Text, Widget};
use tui::Terminal;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

mod util;
use util::*;

fn main() -> Result<(), failure::Error> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut rng = thread_rng();

    let mut tags: Vec<String> = Vec::new();
    for _ in 0..10 {
        let len = rng.gen_range(7, 14);

        let tag: String = rng.sample_iter(&Alphanumeric).take(len).collect();
        tags.push(tag);
    }

    let events = Events::new();

    let mut selected_tag: Option<usize> = None;
    let mut prev_selected: Option<usize> = Some(0);

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
                .split(f.size());

            let style = Style::default().fg(Color::Green).bg(Color::Black);
            SelectableList::default()
                .block(Block::default().borders(Borders::ALL).title("Tags"))
                .items(&tags)
                .select(selected_tag)
                .style(style)
                .highlight_style(style.fg(Color::Yellow).modifier(Modifier::BOLD))
                .highlight_symbol(">")
                .render(&mut f, chunks[0]);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Left => {
                    if selected_tag.is_some() {
                        prev_selected = selected_tag.take();
                    }
                }
                Key::Right => {
                    if selected_tag.is_none() {
                        selected_tag = prev_selected.take();
                    }
                }
                Key::Down => {
                    selected_tag = if let Some(selected) = selected_tag {
                        if selected >= tags.len() - 1 {
                            Some(0)
                        } else {
                            Some(selected + 1)
                        }
                    } else {
                        Some(0)
                    }
                }
                Key::Up => {
                    selected_tag = if let Some(selected) = selected_tag {
                        if selected > 0 {
                            Some(selected - 1)
                        } else {
                            Some(tags.len() - 1)
                        }
                    } else {
                        Some(0)
                    }
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}
