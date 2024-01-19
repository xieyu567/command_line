use crate::types::env::Env;
use crate::utils::util::run_command;

#[derive(Debug)]
struct Permission {
    portal: String,
    permission: String,
    description: String,
    is_applicable: bool,
    role_code: String,
}

pub(crate) async fn auth_permission_add(
    host: &String,
    _db_env: &Env,
) -> Result<(), sqlx::Error> {
    let data = vec![
        Permission {
            portal: "PORTAL_CONSOLE".to_string(),
            permission: "PERMISSION_MANAGE_WO_CLASSIFICATION".to_string(),
            description: "manage_wo_classification".to_string(),
            is_applicable: true,
            role_code: "ADMIN".to_string(),
        },
        Permission {
            portal: "PORTAL_CONSOLE".to_string(),
            permission: "PERMISSION_MANAGE_WO_TEMPLATE".to_string(),
            description: "manage_wo_template".to_string(),
            is_applicable: true,
            role_code: "ADMIN".to_string(),
        },
    ];

    let commands: Vec<String> = data.iter().map(|i| vec!(
        format!(
            "grpcurl -max-time 600 -d \'{{\"portal\":\"{}\",\"permission\":\"{}\",\"description\":\"{}\",\"isApplicable\":{}}}\' --plaintext {}:9000 com.stey.auth.api.grpc.permission.SteyAuthPermissionService.PermissionCreate",
            i.portal,
            i.permission,
            i.description,
            i.is_applicable,
            host
        ),
        format!("grpcurl -max-time 600 -d \'{{\"roleCode\":{{\"underlying\":\"{}\"}},\"portal\":\"{}\",\"permission\":\"{}\"}}\' --plaintext {}:9000 com.stey.auth.api.grpc.role.SteyAuthRoleService.RolePermissionCreate",
                i.role_code,
                i.portal,
                i.permission,
                host)
    )).flatten().collect();

    run_command(commands);

    Ok(())
}
