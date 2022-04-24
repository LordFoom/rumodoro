use color_eyre::eyre::{eyre, Report, Result};
use std::sync::Once;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
// use tracing_subscriber::filter::

struct Rumodoro{
    // display:String,
    //
    // start_button: button::State,
    // stop_button: button::State,
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
        color_eyre::install()?;


        let subs = FmtSubscriber::builder()
            .with_max_level(log_level)
            .init();

        tracing::subscriber::set_global_default(subs).expect("setting logger failed, m'sieur");
    });
    Ok(())
}

fn main()  {


}
