use sqlx::mysql::MySqlPoolOptions;
use sqlx::types::Uuid;

use crate::types::env::Env;
use crate::utils::util::*;

#[derive(Debug, sqlx::FromRow)]
struct Reason {
    project_uuid: Uuid,
    operation_reason_uuid: Uuid,
}

pub(crate) async fn operation_reason_delete_all(
    host: &String,
    db_env: &Env,
) -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(get_db_url(db_env, "crs").as_str())
        .await?;

    let reasons: Vec<Reason> = sqlx::query_as(
        "SELECT project_uuid, operation_reason_uuid \
        FROM operation_reason ",
    )
    .fetch_all(&pool)
    .await?;

    let commands: Vec<String> = reasons.into_iter().map(|reason| format!(
        "grpcurl -max-time 600 -d \'{{\"projectId\":\"{}\",\"operationReasonId\":\"{}\"}}\' --plaintext {}:9000 com.stey.crs.api.grpc.config.SteyCrsConfigService.ConfigOperationReasonDelete",
        reason.project_uuid,
        reason.operation_reason_uuid,
        host
    )).collect();

    run_command(commands);

    Ok(())
}
