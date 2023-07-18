use sqlx::types::Uuid;

pub(crate) struct TrnCode {
    pub(crate) code_id: Uuid,
    pub(crate) project_id: Uuid,
}
