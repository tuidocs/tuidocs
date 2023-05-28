use std::io::Stdout;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{backend::CrosstermBackend, widgets::Paragraph, Frame, Terminal};

pub mod error;

mod app;
use app::App;

mod page_manager;
pub use page_manager::PageManager;

mod state;
pub use state::State;

pub fn run(page_manager: Box<dyn PageManager>) {
    let app = App::new(page_manager);
    let mut terminal = create_terminal().unwrap();

    run_app(&mut terminal, app).unwrap();

    destroy_terminal(terminal).unwrap();
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    mut app: App,
) -> Result<(), error::Error> {
    loop {
        terminal
            .draw(|f| ui(f, &app))
            .or(Err(error::Error::DrawError))?;

        if let Event::Key(key) = event::read().or(Err(error::Error::EventReadError))? {
            match app.state {
                State::Reading => match key.code {
                    KeyCode::Char('s') => app.state = State::Searching,
                    KeyCode::Char('q') => break,
                    _ => {}
                },
                State::Searching if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char(c) => app.input.push(c),
                    KeyCode::Enter => {
                        app.last_input = app.input.clone();
                        app.state = State::Reading;
                    },
                    KeyCode::Esc => {
                        app.input = app.last_input.clone();
                        app.state = State::Reading;
                    },
                    _ => {}
                },
                _ => {},
            }
        }
    }

    Ok(())
}

fn ui(f: &mut Frame<CrosstermBackend<Stdout>>, app: &App) {
    f.render_widget(
        Paragraph::new(app.input.clone()),
        f.size(),
    );
}

fn create_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, error::Error> {
    use crossterm::{
        event::EnableMouseCapture,
        execute,
        terminal::{enable_raw_mode, EnterAlternateScreen},
    };

    enable_raw_mode().or(Err(error::Error::EnableRawModeError))?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).or(Err(
        error::Error::UnknownError("execute! call failed".to_string()),
    ))?; // Lookup error return from execute!
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).or(Err(error::Error::TerminalCreationError))?;

    Ok(terminal)
}

fn destroy_terminal(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<(), error::Error> {
    use crossterm::{
        event::DisableMouseCapture,
        execute,
        terminal::{disable_raw_mode, LeaveAlternateScreen},
    };

    disable_raw_mode().or(Err(error::Error::DisableRawModeError))?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .or(Err(error::Error::UnknownError(
        "execute! call failed".to_string(),
    )))?; // Lookup error return from execute!
    terminal
        .show_cursor()
        .or(Err(error::Error::ShowCursorError))?;

    Ok(())
}
