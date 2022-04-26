use std::io;
use std::io::Read;
use color_eyre::eyre::{eyre, Report, Result};
use std::sync::Once;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, execute};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Alignment, Constraint, Layout};
use tui::{Frame, Terminal};
use tui::layout::Direction::Vertical;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, BorderType};
// use tracing_subscriber::filter::
// use crossterm::

struct Rumodoro{
    // display:String,
    //
    // start_button: button::State,
    // stop_button: button::State,
}

static INIT: Once = Once::new();

fn setup(verbose:bool)->Result<()>{
    color_eyre::install()?;
    INIT.call_once(|| {
        let log_level = if verbose {
            Level::TRACE
        } else {
            Level::ERROR
        };
        //print our pretty stack traces by default
        if std::env::var("RUST_LIB_BACKTRACE").is_err() {
            std::env::set_var("RUST_LIB_BACKTRACE", "1")
        }
        //the library for our pretty stack traces


        let subs = FmtSubscriber::builder()
            .with_max_level(log_level)
            .finish();

        tracing::subscriber::set_global_default(subs).expect("setting logger failed, m'sieur");
    });

    Ok(())
}

// fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {

fn run_the_jewels<B: Backend>(terminal: &mut Terminal<B>) -> Result<()>{
   loop{
       terminal.draw(ui);

       if let Event::Key(key) = event::read()? {
          if let KeyCode::Char('q') = key.code{
              return Ok(());
          }
       }
   }
}

fn ui<B: Backend>(f: &mut Frame<B>){
    let size = f.size();

    let surround_block = Block::default()
        .borders(Borders::ALL)
        .title(" Rumodoro ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(Color::LightCyan));
    f.render_widget(surround_block, size);

    //we going to put the clock block and the button block in here
    let chunks = Layout::default()
        .direction(Vertical)
        .margin(4)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(f.size());


}

fn main() -> Result<()>  {
    //todo we gonna clap this!
    setup(true)?;

    //enable the app
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture )?;
    let backend = CrosstermBackend::new(stdout);
    // // let mut terminal = Terminal::new(backend)?;
    let mut terminal = Terminal::new(backend)?;
    // create app and run it
    let res = run_the_jewels(&mut terminal);
    //turn off the app, m'sieur
    disable_raw_mode();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res{
        println!("{:?}", err);
    }
    Ok(())
}
