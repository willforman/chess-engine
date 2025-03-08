use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

use anyhow::{Context, Result};
use engine::HYPERBOLA_QUINTESSENCE_MOVE_GEN;

use uci::{LOGS_DIRECTORY, UCI};

fn main() -> Result<()> {
    let mut logs_dir = dirs::home_dir().expect("Home directory is unset");
    logs_dir.push(PathBuf::from(".local/state/chess"));

    let _ = LOGS_DIRECTORY.get_or_init(|| logs_dir.clone());

    let mut log_path = logs_dir;
    log_path.push("chess.log");

    //simplelog::WriteLogger::init(
    //    LevelFilter::Trace,
    //    simplelog::Config::default(),
    //    File::create(&log_path).context(format!("Failed to open file at: {:?}", log_path))?,
    //)
    //.context("Couldn't create WriteLogger")?;

    let move_gen = HYPERBOLA_QUINTESSENCE_MOVE_GEN;

    let mut uci = UCI::new(move_gen);

    //uci.handle_command("uci");
    //uci.handle_command("debug on");

    for line in io::stdin().lock().lines().map(|r| r.unwrap()) {
        let cmd_res = uci.handle_command(&line);

        if let Err(err) = cmd_res {
            println!("{}", err);
        }
    }
    Ok(())
}
