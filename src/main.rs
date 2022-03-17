use std::io::{self, Write};
use clap::{Args, Parser, Subcommand};

mod curtain;
mod auto;
mod notify;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Lock physical screens and input devices
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

    /// Automatically lock physical screens and input devices
    Auto { 
        #[clap(subcommand)]
        command: AutoCommand,
    },
}

#[derive(Subcommand, Debug)]
enum AutoCommand {
    /// Lock physical screens and input devices when a Screen Sharing session established
    Run(AutoArgs),

    /// Enable automatic locking on startup
    Enable(AutoArgs),

    /// Disable automatic locking
    Disable,
}

#[derive(Args, Debug)]
struct AutoArgs {
    /// Message to display on the lock screen
    #[clap(short, long)]
    message: Option<String>,

    /// Notification duration in seconds. Specify 0 to disable notification
    #[clap(long, default_value = "5")]
    notify_duration: u64,
}

fn lock(message: Option<&str>, yes: bool) {
    if !yes {
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
        Command::Lock { message, yes } => lock(message.as_deref(), *yes),
        Command::Unlock => curtain::unlock_screen(),
        Command::Auto { command } => match command {
            AutoCommand::Run(args) => auto::run(args.message.as_deref(), args.notify_duration),
            AutoCommand::Enable(args) => auto::enable(args.message.as_deref(), args.notify_duration),
            AutoCommand::Disable => auto::disable(),
        },
    }
}
