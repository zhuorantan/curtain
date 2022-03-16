use std::io::{self, Write};
use clap::{Parser, Subcommand};

mod curtain;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Lock physical screens and input devices.
    Lock { 
        /// Message to display on the lock screen
        #[clap(short, long)]
        message: Option<String>,

        /// Skip the confirmation prompt
        #[clap(short, long)]
        yes: bool,
    },

    /// Unlock physical screens
    Unlock,
}

fn lock(message: &Option<String>, yes: &bool) {
    if !*yes {
        print!("{}", "Are you sure you want to lock? Make sure you can access this device remotely, otherwise you won't be able to unlock. (y/n) ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() != "y" {
            return;
        }
    }

    curtain::lock_screen(message);
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Lock { message, yes } => lock(message, yes),
        Command::Unlock => curtain::unlock_screen(),
    }
}
