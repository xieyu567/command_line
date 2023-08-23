use std::collections::HashMap;

use once_cell::sync::Lazy;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::types::Uuid;

use crate::types::env::Env;
use crate::utils::util::*;

#[derive(Debug, sqlx::FromRow)]
struct TrnCode {
    trn_code_uuid: Uuid,
    project_uuid: Uuid,
}

static TAG_TYPE: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut trn_code_map = HashMap::new();
    trn_code_map.insert("TRN_CODE_TAG_TYPE_NON_ROOM_CHARGE", "non_room_charge");
    trn_code_map
});

pub(crate) async fn trn_code_tag_remove(
    host: &String,
    tag_type: &String,
    db_env: &Env,
) -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(get_db_url(db_env, "finance").as_str())
        .await?;

    let trn_codes: Vec<TrnCode> = sqlx::query_as(
        r#"SELECT trn_code_uuid, project_uuid \
        FROM trn_code_tag \
        WHERE trn_code_tag_type = (?)"#,
    )
    .bind(TAG_TYPE.get(&tag_type.as_str()).unwrap())
    .fetch_all(&pool)
    .await?;

    let commands: Vec<String> = trn_codes.into_iter().map(|trn_code| format!(
        "grpcurl -max-time 600 -d \'{{\"projectId\":\"{}\",\"trnCodeId\":\"{}\",\"tagType\":\"{}\"}}\' --plaintext {}:9000 com.stey.finance.api.grpc.config.SteyFinanceConfigService/ConfigTrnCodeTagUnset",
        trn_code.project_uuid,
        trn_code.trn_code_uuid,
        tag_type,
        host
    )).collect();

    run_command(commands);

    Ok(())
}
