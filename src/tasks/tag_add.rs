use sqlx::mysql::MySqlPoolOptions;
use sqlx::types::Uuid;

use crate::types::env::Env;
use crate::utils::util::*;

#[derive(Debug, sqlx::FromRow)]
struct TrnCode {
    trn_code_uuid: Uuid,
    project_uuid: Uuid,
}

pub(crate) async fn trn_code_tag_add(
    host: &String,
    tag_type: &String,
    db_env: &Env,
) -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(get_db_url(db_env, "finance").as_str())
        .await?;

    let trn_codes: Vec<TrnCode> = sqlx::query_as(
        "SELECT c.trn_code_uuid, c.project_uuid \
        FROM trn_code_group a \
        JOIN trn_code_subgroup b ON a.trn_code_group_uuid = b.trn_code_group_uuid \
        JOIN trn_code c ON b.trn_code_subgroup_uuid = c.trn_code_subgroup_uuid \
        WHERE a.code NOT IN ('ROOMS_LS', 'ROOMS', 'ROOMS_TENANT', 'ROOMS_HOTEL')"
    )
        .fetch_all(&pool).await?;

    let commands: Vec<String> = trn_codes.into_iter().map(|trn_code| format!(
        "grpcurl -max-time 600 -d \'{{\"projectId\":\"{}\",\"trnCodeId\":\"{}\",\"tagType\":\"{}\"}}\' --plaintext {}:9000 com.stey.finance.api.grpc.config.SteyFinanceConfigService/ConfigTrnCodeTagSet",
        trn_code.project_uuid,
        trn_code.trn_code_uuid,
        tag_type,
        host
    )).collect();

    run_command(commands);

    Ok(())
}
