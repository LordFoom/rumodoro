use std::{fmt, io};
use std::collections::HashMap;
use std::fmt::Formatter;
use std::sync::Once;
use std::thread;
use std::time::{Duration, Instant};

use clap::Parser;
use color_eyre::eyre::Result;
use color_eyre::Report;
use iced::{alignment, Application, button, Button, Color, Column, Command, Container, Element, Executor, executor, Length, Row, Sandbox, Settings, Subscription, Text, time};
use iced::window::Mode;
use tracing::{info, Level};
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::FmtSubscriber;

use crate::alignment::Alignment;

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
    long_time: u64,
    ///This is the break time, in minutes
    #[clap(short, long, default_value = "5")]
    short_time: u64,
    ///verbose, means logs
    #[clap(short, long)]
    verbose: bool,
}

///Possible phases for the clock
#[derive(Debug, Clone, PartialEq, )]
enum Phase{
    Work,
    ShortRest,
    LongRest,
}

#[derive(Clone)]
enum State{
    Idle,
    Ticking{last_tick: Instant}
}

impl fmt::Display for Phase{

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result{
        write!(f, "{:?}", self)
    }
}

// #[derive(Clone, Data, Lens)]
#[derive(Clone)]
struct Rumodoro {
    current_phase: Phase,
    current_duration: Duration,
    current_time: String,
    current_phase_as_millis: u128,
    work_time: u64,
    short_rest_time: u64,
    long_rest_time: u64,
    ///How many times have we rested, resetting at 4
    rest_counter: u8,
    ///Ticking or Idle
    state: State,
    btn_toggle: button::State,
    btn_next: button::State,
    btn_reset: button::State,
    btn_quit: button::State,
}



#[derive(Clone, Copy, Debug)]
enum Message{
    Toggle,
    Next,
    Reset,
    Quit,
    Tick(Instant)
}

impl Application for Rumodoro {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self{
                current_phase: Phase::Work,
                state: State::Idle,
                current_duration: Duration::default(),
                work_time: 25,
                short_rest_time: 5,
                long_rest_time: 20,
                rest_counter: 0,
                current_time: "25.0000".into(),
                current_phase_as_millis: 25*60*1000,
                btn_next: button::State::new(),
                btn_toggle: button::State::new(),
                btn_reset: button::State::new(),
                btn_quit: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "R U M O D O R O".into()
    }

    fn update(&mut self,  message: Message)->Command<Message>{
        match message{
            Message::Toggle => {
                match self.state{
                    State::Idle => {
                        self.state = State::Ticking {
                            last_tick: Instant::now(),
                        }
                    },
                    State::Ticking {..} => self.state = State::Idle,
                }
            },
            Message::Tick(now) => match &mut self.state{
                State::Ticking{last_tick} =>{
                    //TIME LAPSED
                    self.current_duration += now - *last_tick;
                    //...why do we do this?? think!
                    *last_tick = now;
                    //if zero or less, change phase,
                    self.current_time = if self.current_duration.as_millis() > self.current_phase_as_millis {
                        self.current_duration = Duration::default();
                        match self.current_phase{
                            //TODO have to be able to generalize the below surely
                           Phase::Work => {
                               self.rest_counter += 1;
                               if self.rest_counter<4  {
                                   self.current_phase = Phase::ShortRest;
                                   self.current_phase_as_millis = self.short_rest_time as u128 * 1000;
                                   format!("{:.4}", self.short_rest_time)
                               }else{
                                   self.rest_counter = 0;
                                   self.current_phase = Phase::LongRest;
                                   self.current_phase_as_millis = self.long_rest_time as u128 * 1000;
                                   format!("{:.4}", self.long_rest_time)
                               }
                           },
                            Phase::ShortRest | Phase::LongRest=> {
                                self.current_phase =  Phase::Work;
                                self.current_phase_as_millis = self.work_time as u128 * 1000;
                                format!("{:.4}", self.work_time)
                            },
                        }
                    }else{
                        let remaining_time = self.current_phase_as_millis - self.current_duration.as_millis();
                        let min_in_millis = (60 * 1000);
                        let min= remaining_time/ min_in_millis;
                        let seconds = remaining_time % min_in_millis;

                        format!("{}:{}", min, seconds)
                    };
                    info!("current time getting updated? think it needs to go to view maybe...{}", self.current_time);

                },
                _ =>{}//if the cabin ain't ticking, no need for kicking
            },
            Message::Reset => self.current_duration = Duration::default(),
            Message::Next => {
                match self.current_phase{
                    Phase::Work => {
                        self.rest_counter += 1;
                        self.current_duration = Duration::default();
                        if self.rest_counter < 4{
                            self.current_phase = Phase::ShortRest;
                            self.current_phase_as_millis = self.short_rest_time as u128 * 1000;
                        }else{
                            self.current_phase = Phase::LongRest;
                            self.current_phase_as_millis = self.long_rest_time as u128 * 1000;
                            self.rest_counter = 0;
                        }
                    },
                    Phase::ShortRest | Phase::LongRest => {
                        self.current_phase = Phase::Work;
                        self.current_phase_as_millis = self.work_time as u128 * 1000;
                    }
                }

            },
            Message::Quit => {

            },
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        match self.state {
            State::Idle => Subscription::none(),
            State::Ticking { .. } => {
                time::every(Duration::from_millis(10)).map(Message::Tick)
            }
        }
    }

    fn view(&mut self) -> Element<Message>{
        let button = |state, label, style|{
            Button::new(state,
                        Text::new(label)
                .horizontal_alignment(alignment::Horizontal::Center)
            )
                .padding(10)
                .width(Length::Units(91))
                .style(style)
        };
        //
        let toggle_btn = {
            let (label, color) = match self.state{
                State::Idle => ("Go", style::Button::Primary),
                State::Ticking{..} => ("Pause", style::Button::Destructive),
            };

            button(&mut self.btn_toggle,label, color ).on_press(Message::Toggle)
        };

        let reset_btn = button(&mut self.btn_reset,
                               "Reset",
                               style::Button::Secondary).on_press(Message::Reset);

        let next_btn = button(&mut self.btn_next,
                              "Next",
                              style::Button::Secondary).on_press(Message::Next);
        let quit_btn = button(&mut self.btn_quit,
                              "Quit",
                              style::Button::Destructive).on_press(Message::Quit);
        //
        // // let
        let controls = Row::new()
            // .width(Length::Units(900))
            .spacing(20)
            .push(toggle_btn)
            .push(next_btn)
            .push(reset_btn)
            .push(quit_btn);
        ;

        let content = Column::new()
            // .width(Length::Units(1600))
            .push(Text::new(self.current_phase.to_string()).size(50))
            .push(
                Text::new(self.current_time.clone())
                    .size(150),
            )
            .push(
                    controls
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()

    }


}

mod style{
    use iced::{Background, button, Color, Vector};
    use iced::button::Style;

    pub enum Button{
        Primary,
        Secondary,
        Destructive
    }

    impl button::StyleSheet for Button{

        fn active(&self) -> Style {
            Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.17, 0.32, 0.91),
                    Button::Secondary => Color::from_rgb(0.4, 0.84, 0.8),
                    Button::Destructive => Color::from_rgb(0.77, 0.11, 0.04),
                })),
                border_radius: 11.0,
                shadow_offset: Vector::new(1.1, 1.1),
                text_color: Color::WHITE,
                ..Style::default()
            }
        }
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
    use iced::{Settings, window};

    //use the below to setup the settings
    //THIS IS HOW YOU RESIZE THE APP, DAMN IT TOOK ME LONG TO FIND
    // let settings = Settings {
    //     window: window::Settings {
    //         size: (300,500),
    //         resizable: true,
    //         decorations: true,
    //     },
    //     ..Default::default()
    // };
    Rumodoro::run(Settings::default())?;
    // Counter::run(Settings::default())?;
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

