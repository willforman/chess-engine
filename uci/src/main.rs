use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

use anyhow::{Context, Result};
use engine::{HyperbolaQuintessenceMoveGen, HYPERBOLA_QUINTESSENCE_MOVE_GEN};
use tracing::{debug, level_filters::LevelFilter, warn, Level};
use tracing_subscriber::{layer::SubscriberExt, prelude::*, util::SubscriberInitExt, Registry};

use uci::{LOGS_DIRECTORY, UCI};

static MOVE_GEN: HyperbolaQuintessenceMoveGen = HYPERBOLA_QUINTESSENCE_MOVE_GEN;

fn main() -> Result<()> {
    enable_logging()?;

    let mut uci = UCI::new(MOVE_GEN);

    //uci.handle_command("uci");
    //uci.handle_command("debug on");

    for line in io::stdin().lock().lines().map(|r| r.unwrap()) {
        debug!("{}", line);
        let cmd_res = uci.handle_command(&line);

        if let Err(err) = cmd_res {
            warn!("{}", err);
        }
    }
    Ok(())
}

fn enable_logging() -> Result<()> {
    let mut logs_dir = dirs::home_dir().context("Home directory not set")?;
    logs_dir.push(PathBuf::from(".local/state/chess"));

    let _ = LOGS_DIRECTORY.get_or_init(|| logs_dir.clone());

    let mut log_path = logs_dir;
    log_path.push("chess.log");

    let log_file = File::create(log_path)?;

    let stdout_layer = tracing_subscriber::fmt::layer()
        .without_time()
        .with_level(false)
        .with_target(false)
        .with_filter(LevelFilter::from_level(Level::INFO));

    let log_layer = tracing_subscriber::fmt::layer()
        .with_writer(log_file)
        .with_filter(LevelFilter::from_level(Level::DEBUG));

    Registry::default()
        .with(stdout_layer)
        .with(log_layer)
        .init();

    Ok(())
}
