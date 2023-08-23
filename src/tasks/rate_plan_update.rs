use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use csv;
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::types::Uuid;
use sqlx::Row;

use crate::types::env::Env;
use crate::utils::util::*;

#[derive(Debug, sqlx::FromRow)]
struct RatePlanRecord {
    project_uuid: Uuid,
    rate_plan_uuid: Uuid,
    code: String,
    name_t: String,
    source_uuid: Uuid,
    resv_type_uuid: Uuid,
    lead_time: i32,
    min_stay: i32,
    max_stay: i32,
    breakfast: i32,
    is_suppressed: bool,
}

#[derive(Debug)]
struct CsvRecord {
    project_code: String,
    rate_plan_code: String,
    market_code: String,
}

#[derive(Debug)]
struct RatePlanMarketRecord {
    rate_plan_uuid: Uuid,
    market_uuid: Uuid,
}

pub(crate) async fn rate_plan_update(
    host: &String,
    db_env: &Env,
) -> Result<(), anyhow::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(get_db_url(db_env, "crs").as_str())
        .await?;

    let mut file =
        File::open("/Users/peterxie/Desktop/scripts/rate_plan_market.csv")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut reader = csv::Reader::from_reader(contents.as_bytes());
    let mut records = Vec::new();
    for result in reader.records() {
        let record = result?;
        let project_code = record.get(0).unwrap().to_string();
        let rate_plan_code = record.get(1).unwrap().to_string();
        let market_code = record.get(2).unwrap().to_string();
        let record = CsvRecord {
            project_code,
            rate_plan_code,
            market_code,
        };
        records.push(record);
    }

    let project_map: Vec<(String, Uuid)> = sqlx::query(
        "SELECT project_uuid, code FROM projection_project_project",
    )
    .map(|row: MySqlRow| {
        (
            row.get("code"),
            Uuid::from_slice(row.get("project_uuid")).unwrap(),
        )
    })
    .fetch_all(&pool)
    .await?;
    let project_map: HashMap<String, Uuid> = project_map.into_iter().collect();

    let rate_plan_map: Vec<(Uuid, String, Uuid)> =
        sqlx::query("SELECT project_uuid, code, rate_plan_uuid FROM rate_plan")
            .map(|row: MySqlRow| {
                (
                    row.get("project_uuid"),
                    row.get("code"),
                    Uuid::from_slice(row.get("rate_plan_uuid")).unwrap(),
                )
            })
            .fetch_all(&pool)
            .await?;
    let rate_plan_map: HashMap<(Uuid, String), Uuid> = rate_plan_map
        .into_iter()
        .map(|(project_uuid, code, rate_plan_uuid)| {
            ((project_uuid, code), rate_plan_uuid)
        })
        .collect();

    let market_map: Vec<(Uuid, String, Uuid)> =
        sqlx::query("SELECT project_uuid, code, market_uuid FROM market")
            .map(|row: MySqlRow| {
                (
                    row.get("project_uuid"),
                    row.get("code"),
                    Uuid::from_slice(row.get("market_uuid")).unwrap(),
                )
            })
            .fetch_all(&pool)
            .await?;
    let market_map: HashMap<(Uuid, String), Uuid> = market_map
        .into_iter()
        .map(|(project_uuid, code, market_uuid)| {
            ((project_uuid, code), market_uuid)
        })
        .collect();

    let convert_data: Vec<RatePlanMarketRecord> = records
        .into_iter()
        .map(|record| {
            let project_uuid = project_map.get(&record.project_code).unwrap();
            let rate_plan_uuid = rate_plan_map
                .get(&(*project_uuid, record.rate_plan_code))
                .unwrap();
            let market_uuid = market_map
                .get(&(*project_uuid, record.market_code))
                .unwrap();
            RatePlanMarketRecord {
                rate_plan_uuid: *rate_plan_uuid,
                market_uuid: *market_uuid,
            }
        })
        .collect();

    let rate_plan_ids: Vec<Uuid> = convert_data
        .iter()
        .map(|item| item.rate_plan_uuid)
        .collect();

    let rate_plan_info: Vec<RatePlanRecord> =
        sqlx::query_as(&format!(r#"SELECT project_uuid, rate_plan_uuid, code, name_t, source_uuid, resv_type_uuid, lead_time, min_stay, max_stay, breakfast, is_suppressed FROM rate_plan WHERE rate_plan_uuid IN ({})"#, rate_plan_ids.iter().map(|i| format!("uuid_to_bin('{}')", i.to_string())).collect::<Vec<String>>().join(",")))
            .fetch_all(&pool)
            .await?;

    let convert_data: HashMap<Uuid, Uuid> = convert_data
        .into_iter()
        .map(|rate_plan_market_record| {
            (
                rate_plan_market_record.rate_plan_uuid,
                rate_plan_market_record.market_uuid,
            )
        })
        .collect();

    let commands = rate_plan_info.into_iter().map(|rate_plan_info| format!(
        "grpcurl -max-time 600 -d \'{{\"projectId\":\"{}\",\"ratePlanId\":\"{}\",\"code\":{},\"name\":{},\"sourceId\":\"{}\",\"marketId\":\"{}\",\"resvTypeId\":\"{}\",\"leadTime\":{},\"minStay\":{},\"maxStay\":{},\"breakfast\":{},\"isSuppressed\":{}}}\' --plaintext {}:9000 com.stey.crs.api.grpc.config.SteyCrsConfigService.ConfigRatePlanUpdate",
        rate_plan_info.project_uuid,
        rate_plan_info.rate_plan_uuid,
        format!("{{\"underlying\":\"{}\"}}", rate_plan_info.code),
        format!("{{\"underlying\":{}}}", rate_plan_info.name_t),
        rate_plan_info.source_uuid,
        convert_data.get(&rate_plan_info.rate_plan_uuid).unwrap(),
        rate_plan_info.resv_type_uuid,
        rate_plan_info.lead_time,
        rate_plan_info.min_stay,
        rate_plan_info.max_stay,
        rate_plan_info.breakfast,
        rate_plan_info.is_suppressed,
        host
    )).collect::<Vec<String>>();

    run_command(commands);

    Ok(())
}
