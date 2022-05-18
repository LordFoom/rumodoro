use std::{fmt, io};
use std::collections::HashMap;
use std::fmt::Formatter;
use std::sync::Once;
use std::thread;
use std::time::{Duration, Instant};

use clap::Parser;
use color_eyre::eyre::Result;
use color_eyre::Report;
use iced::{Application, button, Button, Color, Column, Command, Element, Executor, Sandbox, Settings, Subscription, Text};
use iced::window::Mode;
use tracing::{info, Level};
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::FmtSubscriber;

// use tracing_subscriber::filter::
// use crossterm::

//check out this guy....https://github.com/nagy135/pomodorust/blob/master/src/main.rs
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
    long_time: u64,
    ///This is the break time, in minutes
    #[clap(short, long, default_value = "5")]
    short_time: u64,
    ///verbose, means logs
    #[clap(short, long)]
    verbose: bool,
}

///Possible phases for the clock
#[derive(Debug, Clone, )]
enum Phase{
    Work,
    Rest,
}

impl fmt::Display for Phase{

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result{
        write!(f, "{:?}", self)
    }
}

// #[derive(Clone, Data, Lens)]
#[derive(Clone)]
struct RumodoroState{
    current_phase: Phase,
    current_start_moment: Instant,
    current_time: String,
    current_seconds_remaining: u64,
    work_time: u64,
    short_rest_time: u64,
    long_rest_time: u64,
    ///Tracking time?
    running: bool,
}


#[derive(Default)]
struct Counter{
    value: i32,

    //local state of the two buttons
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Clone, Copy, Debug)]
enum Message{
    Start,
    Stop,
    Next,
    Reset,
    Tick
}

impl Application for RumodoroState{
    type Executor = Executor::default;
    type Message = Message;
    type Flags = ();
    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self{
                current_phase: Phase::Work,
                running: false,
                current_start_moment: Instant::now(),
                work_time: 25,
                short_rest_time: 5,
                long_rest_time: 20,
                current_time: "".into(),
                current_seconds_remaining: 25*60,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        todo!()
    }

    fn update(&mut self,  message: Message){
        match message{
            Message::IncrementPressed =>{
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        todo!()
    }

    fn view(&mut self) -> Element<Message>{
        Column::new()
            .push(
                //produce message when pressed
                Button::new(&mut self.increment_button, Text::new("+"))
                    .on_press(Message::IncrementPressed),
            )
            .push(
                //show the vvalue of the counter here
               Text::new(self.value.to_string()).size(50),
            )
            .push(
                Button::new(&mut self.decrement_button, Text::new("-"))
                    .on_press(Message::DecrementPressed),
            )
            .into()
    }

    fn mode(&self) -> Mode {
        todo!()
    }

    fn background_color(&self) -> Color {
        todo!()
    }

    fn scale_factor(&self) -> f64 {
        todo!()
    }

    fn should_exit(&self) -> bool {
        todo!()
    }

    fn run(settings: Settings<Self::Flags>) -> iced::Result where Self: 'static {
        todo!()
    }
}

///We default to to 25 minutes work, to 5 minute break
impl Default for RumodoroConfig {
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
            Level::INFO
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


    Counter::run(Settings::default())?;
    // let state = RumodoroState {
    //     current_phase: Phase::Work,
    //     //when we started running - the start, after a pause, etc
    //     current_start_moment: Instant::now(),
    //     current_time: format!("{:.4}",rmd.long_time),
    //     ///how many we want in the current phase
    //     current_seconds_needed:  rmd.long_time * 60,
    //     ///seconds remaining in current phase
    //     current_seconds_remaining:  rmd.long_time * 60,
    //     long_time: rmd.long_time,
    //     short_time: rmd.short_time,
    //     running: false,
    // };


    Ok(())
}

