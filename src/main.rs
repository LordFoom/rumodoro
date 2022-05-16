use std::{fmt, io};
use std::collections::HashMap;
use std::fmt::Formatter;
use color_eyre::eyre::{Result};
use std::sync::Once;
use std::time::{Duration, Instant};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use clap::Parser;
use color_eyre::Report;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use std::thread;
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
#[derive(Debug, Clone, )]
enum Phase{
    Work, Break,
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
    current_seconds_needed: u64,
    current_seconds_remaining: u64,
    long_time: u64,
    short_time: u64,
    ///Tracking time?
    running: bool,
}

impl RumodoroState{

    pub fn start(&mut self){
        //this will start the time running down....
        //  match self.current_phase{
        //      Phase::Work => self.work(),
        //      Phase::Break => self.take_break(),
        //  }

        self.current_start_moment = Instant::now();
        self.running = true;
        loop{
            self.calc_remaining_time();
            thread::sleep(Duration::from_millis(300));
            if !self.running {
                break;
            }
        }
    }

    pub fn take_break(&mut self){
        self.current_phase = Phase::Break;
    }
    pub fn work(&mut self){
       self.current_phase = Phase::Work;
    }
    pub fn pause(&mut self){
        self.running = false;
    }
    pub fn reset(&mut self){
        //we go to pause
        // self.current_phase = Phase::Paused;
        //reset the display string
    }

    pub fn quit(&mut self){
        self.current_phase = Phase::Break;
    }

    fn calc_remaining_time(&mut self) -> String{
        info!("Calcing that time!");
        // let ceil = self.current_ceiling;
        if self.running {
            let elapsed_secs = self.current_start_moment.elapsed().as_secs();
            info!("Current remaining Elapsed seconds: {}", elapsed_secs);
            let current = self.current_seconds_needed.clone();
            //now we need to subtrack elapsed seconds
            let mut new_curr =  if  elapsed_secs > current {
                0
            }else{
                current - elapsed_secs
            };
            info!("New current: {}", new_curr);
            if new_curr <= 0 {
                info!("We stopping?");
                new_curr = 0;
               self.running = false;
            }

            self.current_seconds_remaining = new_curr;
            self.current_time = self.format_time(new_curr);
        }
        return self.current_time.clone();
        // if self
        //get the current moment
        //get the current time
        //subtract the one from the other
    }

    ///Hours, minutes, seconds
    fn format_time(&self, seconds: u64) -> String{
       //hours
        //let's assume no hours...for now
       //  let hours = seconds/3600;
       //  let remainder = seconds % 3600;
       //  let minutes = remainder/60;
        // let rem_secs = remainder %60;
        let minutes = seconds/60;
        let rem_secs = seconds%60;
        format!("{minutes:0>width$}:{seconds:0>width$}", minutes=minutes, width=2, seconds=rem_secs)
    }

    fn display_time(&mut self) ->String{
        // if self.running {
        //     return format!("{:.4}", self.current_ceiling);
        // }
        self.calc_remaining_time()
        // match self.current_phase{
        //     Phase::Paused => format!("{:?}", self.current_start_moment),
        //     Phase::Work | Phase::Break => self.calc_remaining_time(),
        // }
    }
}

///We default to to 25 minutes work, to 5 minute break
/// TODO put in the extra long push and break
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

    MainWindow::new().run();

    Ok(())
}

slint::slint!{
   MemoryTile := Rectangle {
    width: 64px;
    height: 64px;
    background: #3960D5;

    Image {
        source: @image-url("icons/bus.png");
        width: parent.width;
        height: parent.height;
    }
}
    MainWindow := Window{
        MemoryTile {}
    }
}
