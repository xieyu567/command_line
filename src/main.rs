use clap::{Parser, Subcommand};

use crate::error::Error;
use crate::types::env::Env;

mod task;
mod types;
mod error;

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Parser)]
#[command(author = "xieyu", version = "1.0", about = "some task scripts", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    TrnCodeTagAdd(TrnCodeTagArgs),
    TrnCodeTagRemove(TrnCodeTagArgs),
}

#[derive(Parser, Debug)]
#[command(author = "xieyu", version = "1.0", about = "help to set trn code tag", long_about = None)]
struct TrnCodeTagArgs {
    #[arg(long)]
    host: String,

    #[arg(short, long)]
    tag_type: String,

    #[arg(short, long, value_enum, default_value_t = Env::Dev)]
    db_env: Env,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    match &args.command {
        Commands::TrnCodeTagAdd(trn_code_tag_add_args) => {
            task::tag_add::trn_code_tag_add(&trn_code_tag_add_args.host, &trn_code_tag_add_args.tag_type, &trn_code_tag_add_args.db_env).await?;
        }
        Commands::TrnCodeTagRemove(trn_code_tag_remove_args) => {
            task::tag_remove::trn_code_tag_remove(&trn_code_tag_remove_args.host, &trn_code_tag_remove_args.tag_type, &trn_code_tag_remove_args.db_env).await?;
        }
    }

    Ok(())
}
