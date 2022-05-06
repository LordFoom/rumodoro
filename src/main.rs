use std::io;
use color_eyre::eyre::{Result};
use std::sync::Once;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use clap::Parser;
use druid::{Data, Lens, AppLauncher, Env, Widget, WidgetExt, WindowDesc};
use druid::widget::{Align, Flex, Label, TextBox};
use tracing_subscriber::fmt::writer::MakeWriterExt;
// use tracing_subscriber::filter::
// use crossterm::

///Command line struct
#[derive(Parser, Clone)]
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

///Possible phases for the clock
//TODO create phases
enum Phases{
    Paused, Work, Break,

}

#[derive(Clone, Data, Lens)]
struct RumodoroState{
    current_phase: String,
}

///We default to to 25 minutes work, to 5 minute break
/// TODO put in the extra long push and break
impl Default for Rumodoro{
    fn default() -> Self {
        Self{
            long_time: 25,
            short_time: 5,
            verbose: false,
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

// fn run_the_jewels<B: Backend>(terminal: &mut Terminal<B>) -> Result<()>{
//     info!("About to draw...");
//    loop{
//        terminal.draw(ui)?;
//
//        if let Event::Key(key) = event::read()? {
//           if let KeyCode::Char('q') = key.code{
//               return Ok(());
//           }
//        }
//    }
// }

// fn ui<B: Backend>(f: &mut Frame<B>) {
//     let size = f.size();
//
//     let surround_block = Block::default()
//         .borders(Borders::ALL)
//         .title(" Rumodoro ")
//         .title_alignment(Alignment::Center)
//         .border_type(BorderType::Double)
//         .border_style(Style::default().fg(Color::LightCyan));
//     f.render_widget(surround_block, size);
//
//     //we going to put the clock block and the button block in here
//     let chunks = Layout::default()
//         .direction(Vertical)
//         .margin(1)
//         .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
//         .split(f.size());
//
//     let clock_block = Block::default()
//         .borders(Borders::ALL)
//         // .title("")
//         .border_style(Style::default().fg(Color::Green))
//         .border_type(BorderType::Rounded);
//     f.render_widget(clock_block, chunks[0]);
//
//     let btn_clr = Color::LightYellow;
//     // let buttons_text = ["Start", "Stop", "Reset", "Pause"];//need better mnemonics than just the first one
//     let mut start_btn = vec![
//                 Span::styled("[ St", Style::default().fg(btn_clr)),
//                 Span::styled("a", Style::default().fg(btn_clr).add_modifier(Modifier::UNDERLINED)),
//                 Span::styled("rt ]", Style::default().fg(btn_clr)),
//             ];
//     let mut stop_btn = vec![
//         Span::styled("[ St", Style::default().fg(btn_clr)),
//         Span::styled("o", Style::default().fg(btn_clr).add_modifier(Modifier::UNDERLINED)),
//         Span::styled("p ]", Style::default().fg(btn_clr)),
//     ];
//
//     let mut reset_btn = vec![
//         Span::styled("[ ", Style::default().fg(btn_clr)),
//         Span::styled("R", Style::default().fg(btn_clr).add_modifier(Modifier::UNDERLINED)),
//         Span::styled("eset ]", Style::default().fg(btn_clr)),
//
//     ];
//     let mut quit_btn = vec![
//         Span::styled("[ ", Style::default().fg(btn_clr)),
//         Span::styled("Q", Style::default().fg(btn_clr).add_modifier(Modifier::UNDERLINED)),
//         Span::styled("uit ]", Style::default().fg(btn_clr)),
//
//     ];
//     let mut buttons = Vec::new();
//     buttons.append(&mut start_btn);
//     buttons.append(&mut stop_btn);
//     buttons.append(&mut reset_btn);
//     buttons.append(&mut quit_btn);
//
//     let button_spans = Spans::from(buttons);
//     //a for start
//     //o for stop
//     //r for reset
//     //p for pause
//     //and mouse support?
//     let button_bar = Paragraph::new(button_spans)
//         .alignment(Alignment::Center)
//         .block(
//             Block::default()
//                 .borders(Borders::ALL)
//                 .border_style(Style::default().fg(Color::LightRed))
//                 .border_type(BorderType::Rounded)
//         );
//     f.render_widget(button_bar, chunks[1]);
//
// }

fn main() -> Result<()>  {
    color_eyre::install()?;
    let rmd = Rumodoro::parse();
    setup(rmd.verbose)?;

    let main_window = WindowDesc::new(build_root_widget())
        .title("R U M O D O R O")
        .window_size((400.0, 400.0));


    let state = RumodoroState { current_phase: "Paused".into() };
    AppLauncher::with_window(main_window)
        .launch(state)
        .expect("Failed to launch window, m'sieur");


    //enable the app
    // enable_raw_mode()?;
    // let mut stdout = io::stdout();
    // execute!(stdout, EnterAlternateScreen, EnableMouseCapture )?;
    // let backend = CrosstermBackend::new(stdout);
    // // // let mut terminal = Terminal::new(backend)?;
    // let mut terminal = Terminal::new(backend)?;
    // // create app and run it
    // let res = run_the_jewels(&mut terminal);
    // //turn off the app, m'sieur
    // disable_raw_mode()?;
    // execute!(
    //     terminal.backend_mut(),
    //     LeaveAlternateScreen,
    //     DisableMouseCapture,
    // )?;
    // terminal.show_cursor()?;

    // if let Err(err) = res{
    //     println!("{:?}", err);
    // }
    Ok(())
}

fn build_root_widget() -> impl Widget<RumodoroState>{
    //a label that will determine its text based on the current app data
    let label = Label::new(|data: &RumodoroState, _env: &Env| format!("{}!", data.current_phase));
    //a textbox that modifies `name`
    let textbox = TextBox::new()
        .with_placeholder("What phase are we in?")
        .fix_width(200.0)
        .lens(RumodoroState::current_phase);

    //vertical with padding
    let layout = Flex::column()
        .with_child(label)
        .with_spacer(20.0)
        .with_child(textbox);

    Align::centered(layout)
}