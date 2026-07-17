use clap::{Parser, Subcommand};
use quantum_lt::{self, Error};

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
    #[clap(arg_required_else_help = true)]
    Query {
        #[arg(short, long)]
        serial: String,
    },
    List,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    
    match args.subcommand {
        SubCommands::Init { serial } => {
            let ctx = quantum_lt::search(Some(&serial))?
                .into_iter()
                .next();
            if let Some(mut ctx) = ctx {
                quantum_lt::init(&mut ctx)?;
                println!("Success initialization.")
            } else {
                eprintln!("No such device. {}", serial)
            }
        },
        SubCommands::Query { serial } => {
            let ctx = quantum_lt::search(Some(&serial))?
                .into_iter()
                .next();
            if let Some(mut ctx) = ctx {
                let res = quantum_lt::query(&mut ctx)?;
                for i in 0 .. 16 {
                    println!("in  {:2}: {:.3}", i + 1, res.input[i]);
                }
                for i in 0 .. 10 {
                    println!("daw {:2}: {:.3}", i + 1, res.daw[i]);
                }
                for i in 0 .. 10 {
                    println!("out {:2}: {:.3}", i + 1, res.output[i]);
                }
                println!("Opt1: 0x{:02x}", res.opt1);
                println!("Opt2: 0x{:02x}", res.opt2);
                let version = str::from_utf8(&res.version)
                    .map_err(|_| Error::PacketParse)?;
                println!("Version: {}", version);
            } else {
                eprintln!("No such device. {}", serial)
            }
        },
        SubCommands::List => {
            let list = quantum_lt::search(None)?;
            for info in &list {
                println!("{}:{}:{} {}", info.bus(), info.port(), info.address(), info.serial());
            }
            if list.len() == 0 {
                eprintln!("Not found.");
            }
        },
    }
    
    Ok(())
}