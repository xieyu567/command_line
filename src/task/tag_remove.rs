use std::collections::HashMap;

use once_cell::sync::Lazy;
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::types::Uuid;
use sqlx::Row;

use crate::types::env::Env;
use crate::types::trn_code::TrnCode;

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
    let env_db_url = match db_env {
        Env::Dev => "mysql://secadmin:dT7dfitUhqd0g4FsKueW@dev-mysql-01.mysql.database.chinacloudapi.cn:3306/stey_finance?useSSL=true",
        Env::Uat => "mysql://secadmin:PAa7PKwNUe505Dop200S@uat-mysql-01.mysql.database.chinacloudapi.cn:3306/stey_finance?useSSL=true",
        Env::Prod => "mysql://secadmin:X9ONgqR4W1rVwMGkQvAr@prod-mysql-01.mysql.database.chinacloudapi.cn:3306/stey_finance?useSSL=true",
    };

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(env_db_url)
        .await?;

    let trn_codes = sqlx::query(
        "SELECT trn_code_uuid, project_uuid \
        FROM trn_code_tag \
        WHERE trn_code_tag_type = (?)",
    )
    .bind(TAG_TYPE.get(&tag_type.as_str()).unwrap())
    .map(|row: MySqlRow| TrnCode {
        code_id: Uuid::from_slice(row.get("trn_code_uuid")).unwrap(),
        project_id: Uuid::from_slice(row.get("project_uuid")).unwrap(),
    })
    .fetch_all(&pool)
    .await?;

    let commands = trn_codes.into_iter().map(|trn_code| format!(
        "grpcurl -max-time 600 -d \'{{\"projectId\":\"{}\",\"trnCodeId\":\"{}\",\"tagType\":\"{}\"}}\' --plaintext {}:9000 com.stey.finance.api.grpc.config.SteyFinanceConfigService/ConfigTrnCodeTagUnset",
        trn_code.project_id,
        trn_code.code_id,
        tag_type,
        host
    )).collect::<Vec<String>>();

    commands.iter().for_each(|s| {
        let (code, _output, error) = run_script::run_script!(s).unwrap();

        println!("Exit Code: {}", code);
        println!("Error: {}", error);
    });

    Ok(())
}
