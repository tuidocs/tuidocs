use std::io::Stdout;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use crate::{app::App, State};

pub fn ui(f: &mut Frame<CrosstermBackend<Stdout>>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let (msg, style) = match app.state {
        State::Reading => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("s", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start searching."),
                Span::raw(format!("{}", app.scroll)),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        State::Searching => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to cancel searching, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to open the selected page."),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_str())
        .style(match app.state {
            State::Reading => Style::default(),
            State::Searching => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[1]);

    match app.state {
        State::Reading =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        State::Searching => {
            // Make the cursor visible and ask ratatui to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1,
            )
        }
    }

    match app.state {
        State::Reading => f.render_widget(
            Paragraph::new(match app.page_manager.get_page(&app.input) {
                Some(page) => page.clone(),
                None => "This page doesn't exist".to_string(),
            })
            .scroll((app.scroll, 0))
            .wrap(Wrap { trim: true }),
            chunks[2],
        ),
        State::Searching => {
            let entries: Vec<ListItem> = app
                .page_manager
                .search(&app.input)
                .iter()
                .enumerate()
                .map(|v| {
                    let mut item = ListItem::new(v.1 .0.clone());
                    if v.0 == app.selected_entry {
                        item = item.style(Style::default().bg(Color::LightBlue));
                    }
                    item
                })
                .collect();
            f.render_widget(
                List::new(entries).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                ),
                chunks[2],
            );
        }
    }
}
