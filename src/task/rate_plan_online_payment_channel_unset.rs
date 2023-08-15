use clap::ValueEnum;
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::types::Uuid;
use sqlx::Row;

use crate::types::env::Env;

#[derive(Debug, sqlx::FromRow)]
struct RatePlanRecord {
    project_uuid: Uuid,
    rate_plan_uuid: Uuid,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, ValueEnum)]
pub(crate) enum RatePlanOnlinePaymentChannel {
    Alipay,
    Wechat,
    Wallet,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, ValueEnum)]
pub(crate) enum Origin {
    App,
    Com,
    Console,
    Switch,
}

pub(crate) async fn rate_plan_online_payment_channel_unset(
    host: &String,
    origin: &Origin,
    channel: &RatePlanOnlinePaymentChannel,
    db_env: &Env,
) -> Result<(), sqlx::Error> {
    let env_db_url = match db_env {
        Env::Dev => "mysql://secadmin:dT7dfitUhqd0g4FsKueW@dev-mysql-01.mysql.database.chinacloudapi.cn:3306/stey_dc?useSSL=true",
        Env::Uat => "mysql://secadmin:PAa7PKwNUe505Dop200S@uat-mysql-01.mysql.database.chinacloudapi.cn:3306/stey_dc?useSSL=true",
        Env::Prod => "mysql://secadmin:X9ONgqR4W1rVwMGkQvAr@prod-mysql-01.mysql.database.chinacloudapi.cn:3306/stey_dc?useSSL=true",
    };

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(env_db_url)
        .await?;

    let rate_plan_info = sqlx::query(
        r#"SELECT project_uuid, rate_plan_uuid 
        FROM rate_plan_online_payment_channel 
        WHERE origin=? AND online_payment_channel=? "#,
    )
    .bind(match origin {
        Origin::App => "stey_app",
        Origin::Com => "stey_com",
        Origin::Console => "stey_console",
        Origin::Switch => "stey_switch",
    })
    .bind(match channel {
        RatePlanOnlinePaymentChannel::Alipay => "alipay",
        RatePlanOnlinePaymentChannel::Wechat => "wechat",
        RatePlanOnlinePaymentChannel::Wallet => "stey_wallet",
    })
    .map(|row: MySqlRow| RatePlanRecord {
        project_uuid: Uuid::from_slice(row.get("project_uuid")).unwrap(),
        rate_plan_uuid: Uuid::from_slice(row.get("rate_plan_uuid")).unwrap(),
    })
    .fetch_all(&pool)
    .await?;

    let commands = rate_plan_info.into_iter().map(|rate_plan_info| format!(
        "grpcurl -max-time 600 -d \'{{\"projectId\":\"{}\",\"ratePlanId\":\"{}\",\"origin\":\"{}\",\"RatePlanOnlinePaymentChannel\":\"{}\"}}\' --plaintext {}:9000 com.stey.dc.api.grpc.SteyDcService.RatePlanOnlinePaymentChannelUnset",
        rate_plan_info.project_uuid,
        rate_plan_info.rate_plan_uuid,
        match origin {
            Origin::App => "ORIGIN_STEY_APP",
            Origin::Com => "ORIGIN_STEY_COM",
            Origin::Console => "ORIGIN_STEY_CONSOLE",
            Origin::Switch => "ORIGIN_STEY_SWITCH",
        },
        match channel {
            RatePlanOnlinePaymentChannel::Alipay => "ONLINE_PAYMENT_CHANNEL_ALIPAY",
            RatePlanOnlinePaymentChannel::Wechat => "ONLINE_PAYMENT_CHANNEL_WECHAT",
            RatePlanOnlinePaymentChannel::Wallet => "ONLINE_PAYMENT_CHANNEL_STEY_WALLET",
        },
        host
    )).collect::<Vec<String>>();

    commands.iter().for_each(|s| {
        let (code, _output, error) = run_script::run_script!(s).unwrap();

        println!("Exit Code: {}", code);
        println!("Error: {}", error);
    });

    Ok(())
}
