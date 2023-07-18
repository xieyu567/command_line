use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use csv;
use serde::Deserialize;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::types::Uuid;

use crate::types::env::Env;

#[derive(Debug, sqlx::FromRow)]
struct SpaceInfo {
    project_uuid: Uuid,
    // #[sqlx(rename = "code")]
    project_code: String,
    space_uuid: Uuid,
    space_code: String,
    room_type_uuid: Uuid,
    room_type_code: String,
    room_type_name: sqlx::types::Json<Vec<I18nText>>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct I18nText {
    locale: String,
    #[allow(dead_code)]
    localeC: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct CsvData {
    project_code: String,
    space_code: String,
    room_type_name: String,
    room_size: String,
    floor: String,
    orientation: String,
    accessible: Option<bool>,
    quiet: Option<bool>,
}

#[derive(Debug)]
struct ProtobufParams {
    project_uuid: String,
    space_uuid: String,
    room_type_uuid: String,
    room_size: String,
    floor: String,
    orientation: String,
    is_accessible: Option<bool>,
    is_quiet: Option<bool>,
}

pub(crate) async fn room_attribute_add(
    host: &String,
    csv_path: &String,
    db_env: &Env,
) -> Result<(), sqlx::Error> {
    let env_db_url = match db_env {
        Env::Dev => "mysql://secadmin:dT7dfitUhqd0g4FsKueW@dev-mysql-01.mysql.database.chinacloudapi.cn:3306/stey_crs?useSSL=true",
        Env::Uat => "mysql://secadmin:PAa7PKwNUe505Dop200S@uat-mysql-01.mysql.database.chinacloudapi.cn:3306/stey_crs?useSSL=true",
        Env::Prod => "mysql://secadmin:X9ONgqR4W1rVwMGkQvAr@prod-mysql-01.mysql.database.chinacloudapi.cn:3306/stey_crs?useSSL=true",
    };

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(env_db_url)
        .await?;

    let space_info = sqlx::query_as::<_, SpaceInfo>(
        "SELECT tmp2.*, r.room_type_uuid, r.code as room_type_code, r.name_t as room_type_name FROM room_type r JOIN \
                 (SELECT tmp.*, s.room_type_uuid FROM space s JOIN \
                 (SELECT b.project_uuid, b.code as project_code, a.space_uuid, a.code as space_code \
                 FROM projection_project_space a \
                 JOIN projection_project_project b ON a.project_uuid=b.project_uuid \
                 WHERE a.type_c='room') tmp ON s.space_uuid=tmp.space_uuid) tmp2 ON r.room_type_uuid=tmp2.room_type_uuid "
    ).fetch_all(&pool).await?;

    let room_type_name_map: HashMap<(String, Uuid), Uuid> = space_info
        .iter()
        .map(|item| {
            (
                (
                    format!(
                        "{} - {}",
                        item.room_type_code,
                        item.room_type_name
                            .iter()
                            .filter(|i| i.locale == "en")
                            .map(|i| i.content.clone())
                            .collect::<String>()
                    ),
                    item.project_uuid,
                ),
                item.room_type_uuid,
            )
        })
        .collect();

    let mut file = File::open(&csv_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut reader = csv::Reader::from_reader(contents.as_bytes());
    let mut records = Vec::new();
    for result in reader.records() {
        let record = result.expect("a CSV record");
        let project_code = record.get(0).unwrap().to_string();
        let space_code = record.get(1).unwrap().to_string();
        // let space_code = record.get(1).unwrap().parse::<u8>()?;
        let room_type_name = record.get(2).unwrap().to_string();
        let room_size = record.get(3).unwrap().to_string();
        let floor = record.get(4).unwrap().to_string();
        let orientation = record.get(5).unwrap().to_string();
        let accessible = match record.get(6) {
            Some("1") => Some(true),
            Some("0") => Some(false),
            _ => None,
        };
        let quiet = match record.get(7) {
            Some("1") => Some(true),
            Some("0") => Some(false),
            _ => None,
        };
        let record = CsvData {
            project_code,
            space_code,
            room_type_name,
            room_size,
            floor,
            orientation,
            accessible,
            quiet,
        };
        records.push(record);
    }

    let mut protobuf_params: Vec<ProtobufParams> = Vec::new();
    let _ = space_info.iter().for_each(|item| {
        let match_item = records.iter().find(|i| {
            i.space_code == item.space_code
                && i.project_code == item.project_code
        });
        match match_item {
            Some(match_item) => protobuf_params.push(ProtobufParams {
                project_uuid: item.project_uuid.to_string(),
                space_uuid: item.space_uuid.to_string(),
                room_type_uuid: room_type_name_map
                    .get(&(
                        match_item.room_type_name.clone(),
                        item.project_uuid.clone(),
                    ))
                    .unwrap_or(&item.room_type_uuid)
                    .to_string(),
                room_size: match_item.room_size.clone(),
                floor: match_item.floor.clone(),
                orientation: match_item.orientation.clone(),
                is_accessible: match_item.accessible.clone(),
                is_quiet: match_item.quiet.clone(),
            }),
            _ => protobuf_params.push(ProtobufParams {
                project_uuid: item.project_uuid.to_string(),
                space_uuid: item.space_uuid.to_string(),
                room_type_uuid: item.room_type_uuid.to_string(),
                room_size: "0.0".to_string(),
                floor: "high".to_string(),
                orientation: "east".to_string(),
                is_accessible: None,
                is_quiet: None,
            }),
        }
    });

    let commands = protobuf_params.into_iter().map(|param| format!(
        "grpcurl -max-time 600 -d \'{{\"projectId\":\"{}\",\"spaceId\":\"{}\",\"roomTypeId\":\"{}\",\"roomSize\":\"{}\",\"floor\":\"{}\",\"orientation\":\"{}\"{}{}}}\' --plaintext {}:9000 com.stey.crs.api.grpc.config.SteyCrsConfigService/ConfigSpaceRoomAttributeSet",
        param.project_uuid,
        param.space_uuid,
        param.room_type_uuid,
        param.room_size,
        match param.floor {
            value if value == "high" => "ROOM_ATTRIBUTE_FLOOR_TYPE_HIGH",
            value if value == "middle" => "ROOM_ATTRIBUTE_FLOOR_TYPE_MIDDLE",
            value if value == "low" => "ROOM_ATTRIBUTE_FLOOR_TYPE_LOW",
            _ => panic!("floor value is not valid")
        },
        match param.orientation {
            value if value == "east" => "ROOM_ATTRIBUTE_ORIENTATION_TYPE_EAST",
            value if value == "west" => "ROOM_ATTRIBUTE_ORIENTATION_TYPE_WEST",
            value if value == "south" => "ROOM_ATTRIBUTE_ORIENTATION_TYPE_SOUTH",
            value if value == "north" => "ROOM_ATTRIBUTE_ORIENTATION_TYPE_NORTH",
            value if value == "southWest" => "ROOM_ATTRIBUTE_ORIENTATION_TYPE_SOUTH_WEST",
            value if value == "southEast" => "ROOM_ATTRIBUTE_ORIENTATION_TYPE_SOUTH_EAST",
            value if value == "northWest" => "ROOM_ATTRIBUTE_ORIENTATION_TYPE_NORTH_WEST",
            value if value == "northEast" => "ROOM_ATTRIBUTE_ORIENTATION_TYPE_NORTH_EAST",
            _ => panic!("orientation value is not valid")
        },
        match param.is_accessible {
            Some(value) => format!(",\"isAccessible\":\"{}\"", value),
            _ => "".to_string()
        },
        match param.is_quiet {
            Some(value) => format!(",\"isQuiet\":\"{}\"", value),
            _ => "".to_string()
        },
        host
    )).collect::<Vec<String>>();

    commands.iter().for_each(|s| {
        let (code, _output, error) = run_script::run_script!(s).unwrap();
        // println!("{}", s);
        println!("Exit Code: {}", code);
        println!("Error: {}\n", error);
    });

    Ok(())
}
