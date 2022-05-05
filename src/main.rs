use std::io;
use color_eyre::eyre::{Result};
use std::sync::Once;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, execute};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Alignment, Constraint, Layout};
use tui::{Frame, Terminal};
use tui::layout::Direction::{Vertical};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, BorderType, Paragraph};
use clap::Parser;
use tui::text::{Span, Spans};
// use tracing_subscriber::filter::
// use crossterm::

///Command line struct
/// TODO Add flags and properties
#[derive(Parser)]
#[clap(
name = "Rumodoro",
author = "Foom",
version = "1.0",
about = "Pomodoro in the terminal, written in rust",
long_about = None,


)]
struct Rumodoro{
    ///This is the working time, in minutes
    #[clap(short, long, default_value = "25")]
    long_time: u8,
    ///This is the break time, in minutes
    #[clap(short, long, default_value = "5")]
    short_time: u8,
    ///verbose, means logs
    #[clap(short, long)]
    verbose: bool,
}

///We default to to 25 minutes work, to 5 minute break
/// TODO put in the extra long push and break
impl Default for Rumodoro{
    fn default() -> Self {
        Self{
            long_time:25,
            short_time:5,
            verbose:false,
        }
    }
}

static INIT: Once = Once::new();

fn setup(verbose:bool)->Result<()>{
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
    info!("About to draw...");
   loop{
       terminal.draw(ui)?;

       if let Event::Key(key) = event::read()? {
          if let KeyCode::Char('q') = key.code{
              return Ok(());
          }
       }
   }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
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
        .margin(1)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(f.size());

    let clock_block = Block::default()
        .borders(Borders::ALL)
        // .title("")
        .border_style(Style::default().fg(Color::Green))
        .border_type(BorderType::Thick);
    f.render_widget(clock_block, chunks[0]);

    // let buttons_text = ["Start", "Stop", "Reset", "Pause"];//need better mnemonics than just the first one
    let mut start_btn = vec![
                Span::styled("[ St", Style::default().fg(Color::LightYellow)),
                Span::styled("a", Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
                Span::styled("rt ]", Style::default().fg(Color::LightYellow)),
            ];
    let mut stop_btn = vec![
        Span::styled("[ St", Style::default().fg(Color::LightYellow)),
        Span::styled("o", Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
        Span::styled("p ]", Style::default().fg(Color::LightYellow)),
    ];

    let mut buttons = Vec::new();
    buttons.append(&mut start_btn);
    buttons.append(&mut stop_btn);

    let button_spans = Spans::from(buttons);
    //a for start
    //o for stop
    //r for reset
    //p for pause
    //and mouse support?
    let button_bar = Paragraph::new(button_spans)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::LightRed))
                .border_type(BorderType::Rounded)
        );
    f.render_widget(button_bar, chunks[1]);

}

fn main() -> Result<()>  {
    //todo we gonna clap this!
    color_eyre::install()?;
    let rmd = Rumodoro::parse();
    setup(rmd.verbose)?;

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
    disable_raw_mode()?;
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
