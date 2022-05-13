use std::{fmt, io};
use std::collections::HashMap;
use std::fmt::Formatter;
use color_eyre::eyre::{Result};
use std::sync::Once;
use std::time::Instant;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use clap::Parser;
use color_eyre::Report;
use druid::{Data, Lens, AppLauncher, Env, Widget, WidgetExt, WindowDesc, FontDescriptor, FontFamily, FontWeight, LocalizedString, Command, commands};
use druid::widget::{Align, Button, Flex, Label};
use druid::widget::LabelText::Localized;
use tracing_subscriber::fmt::writer::MakeWriterExt;
// use tracing_subscriber::filter::
// use crossterm::

///Command line struct
#[derive(Parser,Debug, Clone)]
#[clap(
name = "Rumodoro",
author = "Foom",
version = "1.0",
about = "Pomodoro in the terminal, written in rust",
long_about = None, )]
struct RumodoroConfig {
    ///This is the working time, in minutes
    #[clap(short, long, default_value = "25")]
    long_time: f64,
    ///This is the break time, in minutes
    #[clap(short, long, default_value = "5")]
    short_time: f64,
    ///verbose, means logs
    #[clap(short, long)]
    verbose: bool,
}

///Possible phases for the clock
#[derive(Debug, Clone, PartialEq, Data)]
enum Phase{
    Work, Break,
}

impl fmt::Display for Phase{

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result{
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Data, Lens)]
struct RumodoroState{
    current_phase: Phase,
    current_start_moment: Instant,
    current_time: String,
    ///Tracking time?
    running: bool,
}

impl RumodoroState{

    pub fn start(&mut self){
       //this will start the time running down....
        match self.current_phase{
            Phase::Work => self.work(),
            Phase::Break => self.take_break(),
        }

        self.running = true;
        while(self.running){

        }
    }

    pub fn take_break(&mut self){
        self.current_phase = Phase::Break;
    }
    pub fn work(&mut self){
       self.current_phase = Phase::Work;
    }
    pub fn pause(&mut self){
        self.current_phase = Phase::Paused;
        self.running = false;
    }
    pub fn reset(&mut self){
        //we go to pause
        self.current_phase = Phase::Paused;
        //reset the display string
    }

    pub fn quit(&mut self){
        self.current_phase = Phase::Break;
    }

    fn calc_remaining_time(&self) -> String{
        //get the current moment
        //get the current time
        //subtract the one from the other
        "UNKNOWN".to_string()
    }
    fn display_time(&self) ->String{
        match self.current_phase{
            Phase::Paused => format!("{:?}", self.current_start_moment),
            Phase::Work | Phase::Break => self.calc_remaining_time(),
        }
    }
}

///We default to to 25 minutes work, to 5 minute break
/// TODO put in the extra long push and break
impl Default for RumodoroConfig {
    fn default() -> Self {
        Self{
            long_time: 25.,
            short_time: 5.,
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


fn main() -> Result<()>  {
    color_eyre::install()?;
    let rmd = RumodoroConfig::parse();
    setup(rmd.verbose)?;

    let main_window = WindowDesc::new(build_root_widget())
        .title("R U M O D O R O")
        .window_size((400.0, 400.0));


    let state = RumodoroState {
        current_phase: Phase::Paused,
        current_start_moment: Instant::now(),
        current_time: format!("{:.4}",rmd.long_time),
        running: false,
    };

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
    let phase_font = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_weight(FontWeight::BOLD)
        .with_size(30.);
    let phase_label = Label::new(|data: &RumodoroState, _env: &Env| format!("{}!", data.current_phase))
    .with_font(phase_font);


    //TODO  put fonts into a config file - also a good excuse to sneak in serde
    let time_font = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_weight(FontWeight::BOLD)
        .with_size(90.);
    let time_label = Label::new(|data: &RumodoroState, _env: &Env| format!("{}!", data.current_time))
        .with_font(time_font);
    //a textbox that modifies `name`
    // let textbox = TextBox::new()
    //     .with_placeholder("What phase are we in?")
    //     .fix_width(200.0)
    //     .lens(RumodoroState::current_phase);


    let padding = 1.;
    let btn_1 = Button::new("Hello");
    let btn_start = Button::new("Start")
        .padding(padding)
        .on_click(|_ctx, data:&mut RumodoroState, _env| data.start());
    let btn_stop = Button::new("Stop").padding(padding)
    .on_click(|_ctx, data:&mut RumodoroState, _env| data.pause());
    let btn_reset = Button::new("Reset").padding(padding)
        .on_click(|_ctx, data:&mut RumodoroState, _env| data.reset());
    let btn_quit = Button::new("Quit").padding(padding)
        .on_click(|_ctx, data:&mut RumodoroState, _env| _ctx.submit_command(commands::QUIT_APP));

    //single column with rows with padding
    let layout = Flex::column()
        .with_child(
            Flex::row()
            .with_child(phase_label))
        .with_child(
            Flex::row()
                .with_child(time_label)
        )
        .with_spacer(20.0)
        .with_child(
            Flex::row()
            .with_child(btn_start)
                .with_child(btn_stop)
                .with_child(btn_reset)
                .with_child(btn_quit));

    Align::centered(layout)
}

// fn create_buttons() -> Result<HashMap<String, dyn Widget<Button<String>>>, Report>{
//     let padding = 1;
//     let mut buttons = HashMap::new();
//     let btn_start = Button::new("Start")
//         .padding(padding)
//         .on_click(|_ctx, data:&mut RumodoroState, _env| data.work());
//     let btn_stop = Button::new("Stop").padding(padding)
//                                       .on_click(|_ctx, data:&mut RumodoroState, _env| data.pause());
//     let btn_reset = Button::new("Reset").padding(padding)
//                                         .on_click(|_ctx, data:&mut RumodoroState, _env| data.reset());
//     let btn_quit = Button::new("Quit").padding(padding)
//                                       .on_click(|_ctx, data:&mut RumodoroState, _env| _ctx.submit_command(commands::QUIT_APP));
//
//
//     let mut btn_hash = HashMap::new();
//     btn_hash.set("btn_start".into(), &btn_start);
//     btn_hash.set("btn_stop".into(), &btn_stop);
//     btn_hash.set("btn_reset".into(),&btn_reset);
//     btn_hash.set("btn_quit".into(), &btn_quit);
//     Ok(buttons)
// }