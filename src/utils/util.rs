use crate::types::env::Env;

pub(crate) fn get_db_url(env: &Env) -> String {
    match env {
        Env::Dev => "mysql://secadmin:dT7dfitUhqd0g4FsKueW@dev-mysql-01.mysql.database.chinacloudapi.cn:3306/stey_dc?useSSL=true",
        Env::Uat => "mysql://secadmin:PAa7PKwNUe505Dop200S@uat-mysql-01.mysql.database.chinacloudapi.cn:3306/stey_dc?useSSL=true",
        Env::Prod => "mysql://secadmin:X9ONgqR4W1rVwMGkQvAr@prod-mysql-01.mysql.database.chinacloudapi.cn:3306/stey_dc?useSSL=true",
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
