use crate::types::env::Env;

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
