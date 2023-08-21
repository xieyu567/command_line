use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::types::Uuid;
use sqlx::Row;

use crate::types::env::Env;
use crate::utils::util::*;

#[derive(Debug, Clone)]
struct Reason {
    reason_type: String,
    code: String,
    title: String,
    description: Option<String>,
    is_internal: bool,
}

pub(crate) async fn operation_reason_add(
    host: &String,
    db_env: &Env,
) -> Result<(), sqlx::Error> {
    let init_data: Vec<Reason> = vec![
        Reason {
            reason_type: "OPERATION_REASON_TYPE_RESERVATION_CANCELLATION".to_string(),
            code: "{\"underlying\":\"TravelChange\"}".to_string(),
            title: "{\"underlying\":[{\"locale\":\"zh\",\"content\":\"行程变动\"},{\"locale\":\"en\",\"content\":\"Change of date or destination\"}]}".to_string(),
            description: None,
            is_internal: false,
        },
        Reason {
            reason_type: "OPERATION_REASON_TYPE_RESERVATION_CANCELLATION".to_string(),
            code: "{\"underlying\":\"BookingError\"}".to_string(),
            title: "{\"underlying\":[{\"locale\":\"zh\",\"content\":\"预订错误\"},{\"locale\":\"en\",\"content\":\"Booking errors\"}]}".to_string(),
            description: None,
            is_internal: false,
        },
        Reason {
            reason_type: "OPERATION_REASON_TYPE_RESERVATION_CANCELLATION".to_string(),
            code: "{\"underlying\":\"Personal\"}".to_string(),
            title: "{\"underlying\":[{\"locale\":\"zh\",\"content\":\"个人原因无法出行\"},{\"locale\":\"en\",\"content\":\"Personal reasons / unable to travel\"}]}".to_string(),
            description: None,
            is_internal: false,
        },
        Reason {
            reason_type: "OPERATION_REASON_TYPE_RESERVATION_CANCELLATION".to_string(),
            code: "{\"underlying\":\"OtherOption\"}".to_string(),
            title: "{\"underlying\":[{\"locale\":\"zh\",\"content\":\"酒店原因/选择其他住宿\"},{\"locale\":\"en\",\"content\":\"Hotel reason / found a different accommodation option\"}]}".to_string(),
            description: None,
            is_internal: false,
        },
        Reason {
            reason_type: "OPERATION_REASON_TYPE_RESERVATION_CANCELLATION".to_string(),
            code: "{\"underlying\":\"BPG\"}".to_string(),
            title: "{\"underlying\":[{\"locale\":\"zh\",\"content\":\"发现更低价格\"},{\"locale\":\"en\",\"content\":\"Found cheaper prices elsewhere\"}]}".to_string(),
            description: Option::from("{\"underlying\":[{\"locale\":\"zh\",\"content\":\"Stey提供最优价保证，如果您在其他地方找到更低价格，我们会将价格调至该低价。请将在其他订房网站找到的更低价格截图并在入住前出示给前台工作人员。\"},{\"locale\":\"en\",\"content\":\"When you book your stay directly on Stey app, you'll be guaranteed the best room rate. If a lower rate is available on another website, we will match the lower rate. Just take a screenshot and show it to our community team before check-in.\"}]}".to_string()),
            is_internal: false,
        },
        Reason {
            reason_type: "OPERATION_REASON_TYPE_RESERVATION_CANCELLATION".to_string(),
            code: "{\"underlying\":\"Transportation\"}".to_string(),
            title: "{\"underlying\":[{\"locale\":\"zh\",\"content\":\"交通延误或取消\"},{\"locale\":\"en\",\"content\":\"Flight / transportation delayed or canceled\"}]}".to_string(),
            description: None,
            is_internal: false,
        },
        Reason {
            reason_type: "OPERATION_REASON_TYPE_RESERVATION_CANCELLATION".to_string(),
            code: "{\"underlying\":\"Other\"}".to_string(),
            title: "{\"underlying\":[{\"locale\":\"zh\",\"content\":\"其他\"},{\"locale\":\"en\",\"content\":\"Other\"}]}".to_string(),
            description: None,
            is_internal: false,
        },
        Reason {
            reason_type: "OPERATION_REASON_TYPE_RESERVATION_CANCELLATION".to_string(),
            code: "{\"underlying\":\"NoShow\"}".to_string(),
            title: "{\"underlying\":[{\"locale\":\"zh\",\"content\":\"未到店\"},{\"locale\":\"en\",\"content\":\"No show\"}]}".to_string(),
            description: None,
            is_internal: true,
        },
        Reason {
            reason_type: "OPERATION_REASON_TYPE_RESERVATION_CANCELLATION".to_string(),
            code: "{\"underlying\":\"CancelByFinance\"}".to_string(),
            title: "{\"underlying\":[{\"locale\":\"zh\",\"content\":\"财务模块取消合同\"},{\"locale\":\"en\",\"content\":\"Cancel by finance\"}]}".to_string(),
            description: None,
            is_internal: true,
        },
        Reason {
            reason_type: "OPERATION_REASON_TYPE_UNDO_CHECK_IN".to_string(),
            code: "{\"underlying\":\"Unsatisfactory\"}".to_string(),
            title: "{\"underlying\":[{\"locale\":\"zh\",\"content\":\"不满意酒店服务/环境\"},{\"locale\":\"en\",\"content\":\"Not satisfied with the hotel service/environment\"}]}".to_string(),
            description: None,
            is_internal: false,
        },
        Reason {
            reason_type: "OPERATION_REASON_TYPE_UNDO_CHECK_IN".to_string(),
            code: "{\"underlying\":\"Test\"}".to_string(),
            title: "{\"underlying\":[{\"locale\":\"zh\",\"content\":\"测试订单\"},{\"locale\":\"en\",\"content\":\"Test reservation\"}]}".to_string(),
            description: None,
            is_internal: false,
        },
        Reason {
            reason_type: "OPERATION_REASON_TYPE_UNDO_CHECK_IN".to_string(),
            code: "{\"underlying\":\"Personal\"}".to_string(),
            title: "{\"underlying\":[{\"locale\":\"zh\",\"content\":\"顾客原因无法继续入住\"},{\"locale\":\"en\",\"content\":\"Customer is unable to continue check-in due to reasons\"}]}".to_string(),
            description: None,
            is_internal: false,
        },
        Reason {
            reason_type: "OPERATION_REASON_TYPE_UNDO_CHECK_IN".to_string(),
            code: "{\"underlying\":\"ChangeOccupant\"}".to_string(),
            title: "{\"underlying\":[{\"locale\":\"zh\",\"content\":\"更换主入住人\"},{\"locale\":\"en\",\"content\":\"Change the main occupant\"}]}".to_string(),
            description: None,
            is_internal: false,
        },
        Reason {
            reason_type: "OPERATION_REASON_TYPE_UNDO_CHECK_IN".to_string(),
            code: "{\"underlying\":\"BPG\"}".to_string(),
            title: "{\"underlying\":[{\"locale\":\"zh\",\"content\":\"发现更低价格重新下单\"},{\"locale\":\"en\",\"content\":\"Find a lower price and re-order\"}]}".to_string(),
            description: None,
            is_internal: false,
        },
        Reason {
            reason_type: "OPERATION_REASON_TYPE_UNDO_CHECK_IN".to_string(),
            code: "{\"underlying\":\"Misoperation\"}".to_string(),
            title: "{\"underlying\":[{\"locale\":\"zh\",\"content\":\"员工误操作\"},{\"locale\":\"en\",\"content\":\"Employee misoperation\"}]}".to_string(),
            description: None,
            is_internal: false,
        },
        Reason {
            reason_type: "OPERATION_REASON_TYPE_UNDO_CHECK_IN".to_string(),
            code: "{\"underlying\":\"Other\"}".to_string(),
            title: "{\"underlying\":[{\"locale\":\"zh\",\"content\":\"其他\"},{\"locale\":\"en\",\"content\":\"Others\"}]}".to_string(),
            description: None,
            is_internal: false,
        },
    ];

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(get_db_url(db_env).as_str())
        .await?;

    let project_uuid = sqlx::query(
        "SELECT project_uuid \
        FROM projection_project_project ",
    )
    .map(|row: MySqlRow| Uuid::from_slice(row.get("project_uuid")).unwrap())
    .fetch_all(&pool)
    .await?;

    let commands: Vec<String> = project_uuid.iter().flat_map(|project_uuid| init_data.iter().map(move |data| format!(
        "grpcurl -max-time 600 -d \'{{\"projectId\":\"{}\",\"operationReasonType\":\"{}\",\"code\":{},\"title\":{},\"isInternal\":{}{}}}\' --plaintext {}:9000 com.stey.crs.api.grpc.config.SteyCrsConfigService.ConfigOperationReasonCreate",
        project_uuid,
        data.reason_type,
        data.code,
        data.title,
        data.is_internal,
        match &data.description {
            Some(description) => format!(",\"description\":{}", description),
            None => "".to_string(),
        },
        host
    ))).collect();

    run_command(commands);

    Ok(())
}
