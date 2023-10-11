use crate::types::env::Env;
use csv;
use serde::de::DeserializeOwned;
use std::fs::File;

pub(crate) fn get_db_url(env: &Env, module: &str) -> String {
    match env {
        Env::Dev => format!("mysql://secadmin:dT7dfitUhqd0g4FsKueW@dev-mysql-01.mysql.database.chinacloudapi.cn:3306/stey_{}?useSSL=true", module),
        Env::Uat => format!("mysql://secadmin:PAa7PKwNUe505Dop200S@uat-mysql-01.mysql.database.chinacloudapi.cn:3306/stey_{}?useSSL=true", module),
        Env::Prod => format!("mysql://secadmin:X9ONgqR4W1rVwMGkQvAr@prod-mysql-01.mysql.database.chinacloudapi.cn:3306/stey_{}?useSSL=true", module),
    }
        .to_string()
}

pub(crate) fn run_command(commands: Vec<String>) {
    commands.iter().for_each(|s| {
        let (code, _output, error) = run_script::run_script!(s).unwrap();

        println!("Exit Code: {}", code);
        println!("Error: {}", error);
    });
}

pub(crate) fn parse_csv<T>(path: String) -> csv::Result<Vec<T>>
where
    T: DeserializeOwned,
{
    let file = File::open(path)?;
    let rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);
    rdr.into_deserialize().collect()
}
