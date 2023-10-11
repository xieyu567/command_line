use std::collections::HashMap;

use serde::Deserialize;

use crate::types::env::Env;
use crate::utils::util::{parse_csv, run_command};

#[derive(Debug, Deserialize)]
struct UserIdentityRecord {
    user_id: String,
    user_identity_id: String,
    user_identity_type: String,
    nation_code: String,
    name: String,
    number: String,
}

#[derive(Debug, Deserialize)]
struct NationCodeRecord {
    nation_id: String,
    nation_code: String,
}

pub(crate) async fn user_identity_add(
    host: &String,
    _db_env: &Env,
) -> Result<(), anyhow::Error> {
    let mut user_identity_record = parse_csv::<UserIdentityRecord>(
        String::from("./userIdentity/table1.csv"),
    )?;

    let nation_code_record = parse_csv::<NationCodeRecord>(String::from(
        "./userIdentity/table2.csv",
    ))?;

    let convert_map: HashMap<String, String> =
        nation_code_record
            .iter()
            .fold(HashMap::new(), |mut acc, record| {
                acc.insert(
                    record.nation_code.clone(),
                    record.nation_id.clone(),
                );
                acc
            });

    for record in user_identity_record.iter_mut() {
        if let Some(new_code) = convert_map.get(&record.nation_code) {
            record.nation_code = new_code.clone();
        }
    }

    let commands: Vec<String> = user_identity_record.into_iter().map(|record| format!(
        "grpcurl -max-time 600 -d \'{{\"userId\":\"{}\",\"userIdentityId\":{},\"userIdentityType\":{},\"nationCode\":\"{}\",\"name\":\"{}\",\"number\":\"{}\"}}\' --plaintext {}:9000 com.stey.profile.api.grpc.user.SteyProfileUserService/UserIdentityUpdate",
        record.user_id,
        record.user_identity_id,
        record.user_identity_type,
        record.nation_code,
        record.name,
        record.number,
        host
    )).collect();

    run_command(commands);

    Ok(())
}
