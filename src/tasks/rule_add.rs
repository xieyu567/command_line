use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::types::Uuid;
use sqlx::{MySql, Pool, Row};
use tonic::transport::Channel;

use crate::protos::rule;
use crate::protos::rule::stey_rms_config_service_client::SteyRmsConfigServiceClient;
use crate::types::env::Env;
use crate::utils::util::get_db_url;

#[derive(Debug)]
struct Rule {
    project_id: Uuid,
    rule_code: String,
    rule_description: String,
    rule_engine_trigger_function: rule::RuleEngineTriggerFunction,
    threshold: i32,
    rule_group: RuleGroup,
    rule_condition: RuleCondition,
    rule_action: RuleAction,
}

#[derive(Debug)]
struct RuleGroup {
    code: String,
    description: String,
}

#[derive(Debug)]
struct RuleCondition {
    rule_engine_subject: rule::RuleEngineSubject,
    rule_engine_predicate: rule::RuleEnginePredicate,
    rule_engine_fact_value: i32,
}

#[derive(Debug)]
struct RuleAction {
    rule_engine_action: rule::SendEmail,
}

#[derive(Debug)]
struct ProjectInfo {
    email_list: Vec<String>,
    project_name: String,
}

impl Default for ProjectInfo {
    fn default() -> Self {
        Self {
            email_list: vec!["peter.xie@stey.com".to_string()],
            project_name: "".to_string(),
        }
    }
}

impl std::fmt::Display for rule::RuleEngineAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.sealed_value {
            Some(rule::rule_engine_action::SealedValue::Ignore(_)) => write!(f, "{{\"_type\":\"ignore\"}}"),
            Some(rule::rule_engine_action::SealedValue::WriteLog(_)) => write!(f, "{{\"_type\":\"writeLog\", \"title\":\"N.A.\"}}"),
            Some(rule::rule_engine_action::SealedValue::SendEmail(params)) => write!(f, "{{\"_type\":\"sendEmail\", \"to\":[\"{:?}\"], \"title\":\"{}\"}}", params.to, params.title, ),
            _ => write!(f, "{{\"_type\":\"ignore\"}}"),
        }
    }
}

#[tracing::instrument]
pub(crate) async fn rule_add(
    host: &String,
    db_env: &Env,
) -> anyhow::Result<()> {
    let pool = Arc::new(
        MySqlPoolOptions::new()
            .max_connections(5)
            .connect(get_db_url(db_env, "rms").as_str())
            .await?,
    );
    let host_arc = Arc::new(host.clone());

    let project_ids =
        sqlx::query("SELECT project_uuid FROM projection_project_project ")
            .map(|row: MySqlRow| {
                Uuid::from_slice(row.get("project_uuid")).unwrap()
            })
            .fetch_all(pool.as_ref())
            .await?;

    let project_infos: HashMap<Uuid, ProjectInfo> = HashMap::from([
        (
            Uuid::from_str("17D28EC3-B0E8-4488-ABCF-2EC580B48D32").unwrap(),
            ProjectInfo {
                email_list: vec![
                    "Caitlin.Gao@stey.com",
                    "ella.yu@stey.com",
                    "Susan.zhao@stey.com",
                    "hou.chen@stey.com",
                    "liang.liao@stey.com",
                ]
                .iter()
                .map(|e| e.to_string())
                .collect(),
                project_name: "798".to_string(),
            },
        ),
        (
            Uuid::from_str("C9D67D0A-DA6F-EA11-AA77-0003FF0198C8").unwrap(),
            ProjectInfo {
                email_list: vec![
                    "tony.na@stey.com",
                    "ella.yu@stey.com",
                    "Susan.zhao@stey.com",
                    "hou.chen@stey.com",
                    "liang.liao@stey.com",
                ]
                .iter()
                .map(|e| e.to_string())
                .collect(),
                project_name: "WFJ".to_string(),
            },
        ),
        (
            Uuid::from_str("CAD67D0A-DA6F-EA11-AA77-0003FF0198C8").unwrap(),
            ProjectInfo {
                email_list: vec![
                    "catherine.zheng@stey.com",
                    "ella.yu@stey.com",
                    "Susan.zhao@stey.com",
                    "hou.chen@stey.com",
                    "liang.liao@stey.com",
                ]
                .iter()
                .map(|e| e.to_string())
                .collect(),
                project_name: "SLT".to_string(),
            },
        ),
    ]);

    let rules: Vec<Rule> = project_ids
        .into_iter()
        .map(|project_id| Rule {
            project_id,
            rule_code: "PURP".to_string(),
            rule_description: "pickup in rate plan less than 0".to_string(),
            rule_engine_trigger_function:
                rule::RuleEngineTriggerFunction::Exists,
            threshold: 1,
            rule_group: RuleGroup {
                code: "PUDAILY".to_string(),
                description: "pickup check daily".to_string(),
            },
            rule_condition: RuleCondition {
                rule_engine_subject:
                    rule::RuleEngineSubject::DailyRatePlanPickUp,
                rule_engine_predicate: rule::RuleEnginePredicate::LessOrEqual,
                rule_engine_fact_value: 0,
            },
            rule_action: RuleAction {
                rule_engine_action: rule::SendEmail {
                    to: project_infos
                        .get(&project_id)
                        .unwrap_or(&Default::default())
                        .email_list
                        .clone(),
                    title: format!(
                        "Pickup in {} less than 0",
                        &project_infos
                            .get(&project_id)
                            .unwrap_or(&Default::default())
                            .project_name
                    ),
                },
            },
        })
        .collect();

    let tasks = rules
        .into_iter()
        .map(|rule| {
            tracing::info!("create rule {:?} start", rule);

            let pool_copy = pool.clone();
            let host = host_arc.clone();
            tokio::spawn(async move {
                create_rule(&host, pool_copy.as_ref(), rule).await
            })
        })
        .collect::<Vec<_>>();
    futures::future::join_all(tasks).await;

    Ok(())
}

async fn create_rule(
    host: &String,
    pool: &Pool<MySql>,
    rule: Rule,
) -> anyhow::Result<()> {
    let channel = Channel::from_shared(format!("https://{}:9000", host))?
        .connect()
        .await?;
    let mut client = SteyRmsConfigServiceClient::new(channel);

    let rule_create_request =
        tonic::Request::new(rule::ConfigRuleEngineRuleCreateRequest {
            project_id: rule.project_id.to_string(),
            code: Some(rule::Code {
                underlying: rule.rule_code,
            }),
            description: rule.rule_description,
            rule_engine_trigger_function: rule
                .rule_engine_trigger_function
                .into(),
            threshold: rule.threshold,
        });

    let rule_create_response = client
        .config_rule_engine_rule_create(rule_create_request)
        .await?;

    let rule_id = rule_create_response.into_inner().rule_engine_rule_id;

    let rule_group_id = sqlx::query_scalar::<_, Uuid>(
        "SELECT rule_engine_rule_group_uuid FROM rule_engine_rule_group WHERE project_uuid = ? AND code = ?",
    )
        .bind(&rule.project_id)
        .bind(&rule.rule_group.code)
        .fetch_optional(pool)
        .await?;

    match rule_group_id {
        Some(rule_group_id) => {
            let rule_group_associate_request = tonic::Request::new(
                rule::ConfigRuleEngineRuleGroupRuleAssociateRequest {
                    project_id: rule.project_id.to_string(),
                    rule_engine_rule_id: rule_id.clone(),
                    rule_engine_rule_group_id: rule_group_id.to_string(),
                },
            );

            client
                .config_rule_engine_rule_group_rule_associate(
                    rule_group_associate_request,
                )
                .await?;
        }

        None => {
            let rule_group_create_request = tonic::Request::new(
                rule::ConfigRuleEngineRuleGroupCreateRequest {
                    project_id: rule.project_id.to_string(),
                    code: Some(rule::Code {
                        underlying: rule.rule_group.code,
                    }),
                    description: rule.rule_group.description,
                },
            );

            let rule_group_create_response = client
                .config_rule_engine_rule_group_create(rule_group_create_request)
                .await?;

            let rule_group_id = rule_group_create_response
                .into_inner()
                .rule_engine_rule_group_id;

            let rule_group_associate_request = tonic::Request::new(
                rule::ConfigRuleEngineRuleGroupRuleAssociateRequest {
                    project_id: rule.project_id.to_string(),
                    rule_engine_rule_id: rule_id.clone(),
                    rule_engine_rule_group_id: rule_group_id,
                },
            );

            client
                .config_rule_engine_rule_group_rule_associate(
                    rule_group_associate_request,
                )
                .await?;
        }
    }

    let condition_create_request =
        tonic::Request::new(rule::ConfigRuleEngineConditionCreateRequest {
            project_id: rule.project_id.to_string(),
            rule_engine_subject: rule.rule_condition.rule_engine_subject.into(),
            rule_engine_predicate: rule
                .rule_condition
                .rule_engine_predicate
                .into(),
            rule_engine_fact_value: rule::RuleEngineFactValue {
                sealed_value: rule::rule_engine_fact_value::SealedValue::Int {
                    0: rule::Int {
                        value: rule.rule_condition.rule_engine_fact_value,
                    },
                }
                .into(),
            }
            .into(),
        });

    let condition_create_response = client
        .config_rule_engine_condition_create(condition_create_request)
        .await?;

    let condition_id = condition_create_response
        .into_inner()
        .rule_engine_condition_id;

    let rule_condition_associate_request = tonic::Request::new(
        rule::ConfigRuleEngineRuleConditionAssociateRequest {
            project_id: rule.project_id.to_string(),
            rule_engine_rule_id: rule_id.clone(),
            rule_engine_condition_id: condition_id,
        },
    );

    client
        .config_rule_engine_rule_condition_associate(
            rule_condition_associate_request,
        )
        .await?;

    let log_action_id = sqlx::query_scalar::<_, Uuid>(
        "SELECT rule_engine_action_uuid FROM rule_engine_action WHERE project_uuid = ? AND rule_engine_action_type = ? AND rule_engine_action = ?",
    )
        .bind(&rule.project_id)
        .bind("write_log")
        .bind("{\"_type\":\"write_log\", \"title\":\"N.A.\"}")
        .fetch_optional(pool)
        .await?;

    if let Some(log_action_id) = log_action_id {
        let rule_action_log_associate_request = tonic::Request::new(
            rule::ConfigRuleEngineRuleActionAssociateRequest {
                project_id: rule.project_id.to_string(),
                rule_engine_rule_id: rule_id.clone(),
                rule_engine_action_id: log_action_id.to_string(),
            },
        );
        client
            .config_rule_engine_rule_action_associate(
                rule_action_log_associate_request,
            )
            .await?;
    };

    let action_create_email_request =
        tonic::Request::new(rule::ConfigRuleEngineActionCreateRequest {
            project_id: rule.project_id.to_string(),
            rule_engine_action: rule::RuleEngineAction {
                sealed_value: rule::rule_engine_action::SealedValue::SendEmail(
                    rule.rule_action.rule_engine_action,
                )
                .into(),
            }
            .into(),
        });

    let action_create_email_response = client
        .config_rule_engine_action_create(action_create_email_request)
        .await?;

    let action_email_id = action_create_email_response
        .into_inner()
        .rule_engine_action_id;

    let rule_action_email_associate_request =
        tonic::Request::new(rule::ConfigRuleEngineRuleActionAssociateRequest {
            project_id: rule.project_id.to_string(),
            rule_engine_rule_id: rule_id,
            rule_engine_action_id: action_email_id,
        });

    client
        .config_rule_engine_rule_action_associate(
            rule_action_email_associate_request,
        )
        .await?;

    Ok(())
}
