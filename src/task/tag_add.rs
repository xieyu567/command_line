use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::Row;
use sqlx::types::Uuid;

use crate::types::env::Env;
use crate::types::trn_code::TrnCode;

pub(crate) async fn trn_code_tag_add(host: &String, tag_type: &String, db_env: &Env) -> Result<(), sqlx::Error> {
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
        "SELECT c.trn_code_uuid, c.project_uuid \
        FROM trn_code_group a \
        JOIN trn_code_subgroup b ON a.trn_code_group_uuid = b.trn_code_group_uuid \
        JOIN trn_code c ON b.trn_code_subgroup_uuid = c.trn_code_subgroup_uuid \
        WHERE a.code NOT IN ('ROOMS_LS', 'ROOMS', 'ROOMS_TENANT')"
    )
        .map(|row: MySqlRow| TrnCode { code_id: Uuid::from_slice(row.get("trn_code_uuid")).unwrap(), project_id: Uuid::from_slice(row.get("project_uuid")).unwrap() }).fetch_all(&pool).await?;

    let commands = trn_codes.into_iter().map(|trn_code| format!(
        "grpcurl -max-time 600 -d \'{{\"projectId\":\"{}\",\"trnCodeId\":\"{}\",\"tagType\":\"{}\"}}\' --plaintext {}:9000 com.stey.finance.api.grpc.config.SteyFinanceConfigService/ConfigTrnCodeTagSet",
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

#[cfg(test)]
mod tests {
    use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
    use sqlx::Row;
    use sqlx::types::Uuid;

    use crate::task::tag_add::TrnCode;

    #[tokio::test]
    async fn it_works() {
        let pool = MySqlPoolOptions::new()
            .max_connections(3)
            .connect("mysql://secadmin:dT7dfitUhqd0g4FsKueW@dev-mysql-01.mysql.database.chinacloudapi.cn:3306/stey_finance?useSSL=true")
            .await.expect("Failed to connect to MySQL");

        let trn_codes = sqlx::query(
            "SELECT c.trn_code_uuid, c.project_uuid \
        FROM trn_code_group a \
        JOIN trn_code_subgroup b ON a.trn_code_group_uuid = b.trn_code_group_uuid \
        JOIN trn_code c ON b.trn_code_subgroup_uuid = c.trn_code_subgroup_uuid \
        WHERE a.code IN ('ROOMS_LS', 'ROOMS_HOTEL', 'ROOMS_TENANT') \
        ORDER BY c.project_uuid"
        ).map(|row: MySqlRow| TrnCode { code_id: Uuid::from_slice(row.get("trn_code_uuid")).unwrap(), project_id: Uuid::from_slice(row.get("project_uuid")).unwrap() })
            .fetch_all(&pool).await.expect("Failed to fetch trn_codes");

        assert_eq!(trn_codes.len(), 41);
    }
}