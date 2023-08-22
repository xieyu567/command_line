use clap::{Parser, Subcommand};

use crate::error::Error;
use crate::tasks::rate_plan_online_payment_channel_unset::{
    Origin, RatePlanOnlinePaymentChannel,
};
use crate::types::env::Env;

mod error;
mod tasks;
mod types;
mod utils;

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Parser)]
#[command(author = "xieyu", version = "1.0", about = "some tasks scripts", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    TrnCodeTagAdd(TrnCodeTagArgs),
    TrnCodeTagRemove(TrnCodeTagArgs),
    RoomAttributeAdd(RoomAttributeAddArgs),
    RatePlanOnlinePaymentChannelUnset(RatePlanOnlinePaymentChannelUnsetArgs),
    OperationReasonAdd(CommonArgs),
    OperationReasonDeleteAll(CommonArgs),
}

#[derive(Parser, Debug)]
#[command(author = "xieyu", version = "1.0", about = "common args which supply host and env", long_about = None)]
struct CommonArgs {
    #[arg(long)]
    host: String,

    #[arg(short, long, value_enum, default_value_t = Env::Dev)]
    db_env: Env,
}

/// Stey Inc. finance service trn code tag set and unset tasks
#[derive(Parser, Debug)]
#[command(author = "xieyu", version = "1.0", about = "help to set trn code tag", long_about = None)]
struct TrnCodeTagArgs {
    /// host of grpc server
    #[arg(long)]
    host: String,

    /// tag type
    #[arg(short, long)]
    tag_type: String,

    /// db env
    #[arg(short, long, value_enum, default_value_t = Env::Dev)]
    db_env: Env,
}

/// Stey Inc. crs service room attribute set tasks
#[derive(Parser, Debug)]
#[command(author = "xieyu", version = "1.0", about = "common args which supply host and env", long_about = None)]
struct RoomAttributeAddArgs {
    /// host of grpc server
    #[arg(long)]
    host: String,

    /// init data file path
    #[arg(long, short, default_value = "./init_data.csv")]
    csv_path: String,

    /// db env
    #[arg(short, long, value_enum, default_value_t = Env::Dev)]
    db_env: Env,
}

/// Stey Inc. dc rate plan online payment channel unset tasks
#[derive(Parser, Debug)]
#[command(author = "xieyu", version = "1.0", about = "common args which supply host and env", long_about = None)]
struct RatePlanOnlinePaymentChannelUnsetArgs {
    /// host of grpc server
    #[arg(long)]
    host: String,

    /// origin
    #[arg(long, short)]
    origin: Origin,

    /// online payment channel
    #[arg(long, short)]
    channel: RatePlanOnlinePaymentChannel,

    /// db env
    #[arg(short, long, value_enum, default_value_t = Env::Dev)]
    db_env: Env,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    match &args.command {
        Commands::TrnCodeTagAdd(trn_code_tag_add_args) => {
            tasks::tag_add::trn_code_tag_add(
                &trn_code_tag_add_args.host,
                &trn_code_tag_add_args.tag_type,
                &trn_code_tag_add_args.db_env,
            )
            .await?;
        }
        Commands::TrnCodeTagRemove(trn_code_tag_remove_args) => {
            tasks::tag_remove::trn_code_tag_remove(
                &trn_code_tag_remove_args.host,
                &trn_code_tag_remove_args.tag_type,
                &trn_code_tag_remove_args.db_env,
            )
            .await?;
        }
        Commands::RoomAttributeAdd(room_attribute_add_args) => {
            tasks::room_attribute_add::room_attribute_add(
                &room_attribute_add_args.host,
                &room_attribute_add_args.csv_path,
                &room_attribute_add_args.db_env,
            )
            .await?;
        }
        Commands::RatePlanOnlinePaymentChannelUnset(
            rate_plan_online_payment_channel_unset_args,
        ) => {
            tasks::rate_plan_online_payment_channel_unset::rate_plan_online_payment_channel_unset(
                &rate_plan_online_payment_channel_unset_args.host,
                &rate_plan_online_payment_channel_unset_args.origin,
                &rate_plan_online_payment_channel_unset_args.channel,
                &rate_plan_online_payment_channel_unset_args.db_env,
            )
                .await?;
        }
        Commands::OperationReasonAdd(common_args) => {
            tasks::operation_reason_add::operation_reason_add(
                &common_args.host,
                &common_args.db_env,
            )
            .await?;
        }
        Commands::OperationReasonDeleteAll(common_args) => {
            tasks::operation_reason_delete_all::operation_reason_delete_all(
                &common_args.host,
                &common_args.db_env,
            )
            .await?;
        }
    }

    Ok(())
}
