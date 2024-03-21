use sqlx::mysql::MySqlPoolOptions;
use uuid::Uuid;

use crate::types::env::Env;
use crate::utils::util::{get_db_url, run_command};

#[derive(Debug, sqlx::FromRow)]
struct RoomTypeRecord {
    room_type_uuid: Uuid,
    project_uuid: Uuid,
}

pub(crate) async fn base_rate_limit_add(host: &String, db_env: &Env) -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(get_db_url(db_env, "crs").as_str())
        .await?;

    let room_type_records: Vec<RoomTypeRecord> = sqlx::query_as("SELECT room_type_uuid, project_uuid FROM room_type ")
        .fetch_all(&pool)
        .await?;

    let commands: Vec<String> = room_type_records.iter().map(|i|
        format!(
            "grpcurl -max-time 600 -d \'{{\"baseRateLimits\":[{{\"roomTypeId\":\"{}\",\"projectId\":\"{}\",\"minBaseRateLimit\":\"300\",\"maxBaseRateLimit\":\"3000\"}}]}}\' --plaintext {}:9000 com.stey.crs.api.grpc.rate.SteyCrsRateService.RateBaseRateLimitSet",
            i.room_type_uuid,
            i.project_uuid,
            host
        )
    ).collect();

    run_command(commands);

    Ok(())
}
