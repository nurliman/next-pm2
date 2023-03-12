use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Start(Start),
    Delete(Delete),
    List(List),
}

#[derive(Debug, Args)]
#[command(about = "Start and add a process to the pm2 process list:", long_about = None)]
#[command(arg_required_else_help = true)]
struct Start {
    /// <name|namespace|file|ecosystem|id...>
    app: String,
}

#[derive(Debug, Args)]
#[command(about = "Stop and delete a process from the pm2 process list:", long_about = None)]
#[command(arg_required_else_help = true)]
struct Delete {
    /// <name|id|namespace|script|all|json|stdin...>
    app: String,
}

#[derive(Debug, Args)]
#[command(about = "List all processes", long_about = None)]
#[command(arg_required_else_help = true)]
struct List {}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Start(start)) => {
            println!("Start command");
            println!("String: {:?}", start.app);
        }
        Some(Commands::Delete(delete)) => {
            println!("Delete command");
            println!("String: {:?}", delete.app);
        }
        Some(Commands::List(_)) => {
            println!("List command");
        }
        None => {}
    }
}
