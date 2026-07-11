use clap::{Parser, Subcommand};
use rusb::Error;
mod quantum_lt;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    #[clap(arg_required_else_help = true)]
    Init {
        #[arg(short, long)]
        serial: String,
    },
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    
    match args.subcommand {
        SubCommands::Init { serial } => {
            let dev = quantum_lt::search(serial)?;
            quantum_lt::init(dev)?;
            println!("Success initialization.")
        },
    }
    
    Ok(())
}