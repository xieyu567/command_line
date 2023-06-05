use clap::{Parser, Subcommand};

use crate::types::env::Env;

mod task;
mod types;

#[derive(Parser)]
#[command(author = "xieyu", version = "1.0", about = "some task scripts", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    TrnCodeTagAdd(TrnCodeTagAddArgs),
}

#[derive(Parser, Debug)]
#[command(author = "xieyu", version = "1.0", about = "help to set trn code tag", long_about = None)]
struct TrnCodeTagAddArgs {
    #[arg(long)]
    host: String,

    #[arg(short, long)]
    tag_type: String,

    #[arg(short, long, value_enum, default_value_t = Env::Dev)]
    db_env: Env,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Cli::parse();
    match &args.command {
        Commands::TrnCodeTagAdd(trn_code_tag_add_args) => {
            task::tag_add::trn_code_tag_add(&trn_code_tag_add_args.host, &trn_code_tag_add_args.tag_type, &trn_code_tag_add_args.db_env).await?;
        }
    }

    Ok(())
}
