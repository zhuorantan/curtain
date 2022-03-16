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
    Lock { 
        #[clap(short, long)]
        message: Option<String> 
    },
    Unlock,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Lock { message } => curtain::lock_screen(&message.as_deref()),
        Command::Unlock => curtain::unlock_screen(),
    }

    println!("{:?}", cli);

    // curtain::lock_screen("foobar");
    // curtain::unlock_screen();
}
