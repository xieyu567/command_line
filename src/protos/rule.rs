#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuleEngineAction {
    #[prost(oneof = "rule_engine_action::SealedValue", tags = "1, 2, 3")]
    pub sealed_value: ::core::option::Option<rule_engine_action::SealedValue>,
}
/// Nested message and enum types in `RuleEngineAction`.
pub mod rule_engine_action {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SealedValue {
        #[prost(message, tag = "1")]
        Ignore(super::Ignore),
        #[prost(message, tag = "2")]
        WriteLog(super::WriteLog),
        #[prost(message, tag = "3")]
        SendEmail(super::SendEmail),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ignore {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteLog {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendEmail {
    #[prost(string, repeated, tag = "1")]
    pub to: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigRuleEngineActionCreateRequest {
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub rule_engine_action: ::core::option::Option<RuleEngineAction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigRuleEngineActionCreateResponse {
    #[prost(string, tag = "1")]
    pub rule_engine_action_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuleEngineFactValue {
    #[prost(oneof = "rule_engine_fact_value::SealedValue", tags = "1, 2, 3, 4, 5")]
    pub sealed_value: ::core::option::Option<rule_engine_fact_value::SealedValue>,
}
/// Nested message and enum types in `RuleEngineFactValue`.
pub mod rule_engine_fact_value {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SealedValue {
        #[prost(message, tag = "1")]
        Int(super::Int),
        #[prost(message, tag = "2")]
        Double(super::Double),
        #[prost(message, tag = "3")]
        String(super::String),
        #[prost(message, tag = "4")]
        BigDecimal(super::BigDecimal),
        #[prost(message, tag = "5")]
        Boolean(super::Boolean),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int {
    #[prost(int32, tag = "1")]
    pub value: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Double {
    #[prost(double, tag = "1")]
    pub value: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct String {
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigDecimal {
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Boolean {
    #[prost(bool, tag = "1")]
    pub value: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigRuleEngineConditionCreateRequest {
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(enumeration = "RuleEngineSubject", tag = "2")]
    pub rule_engine_subject: i32,
    #[prost(enumeration = "RuleEnginePredicate", tag = "3")]
    pub rule_engine_predicate: i32,
    #[prost(message, optional, tag = "4")]
    pub rule_engine_fact_value: ::core::option::Option<RuleEngineFactValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigRuleEngineConditionCreateResponse {
    #[prost(string, tag = "1")]
    pub rule_engine_condition_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Code {
    #[prost(string, tag = "1")]
    pub underlying: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigRuleEngineRuleCreateRequest {
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub code: ::core::option::Option<Code>,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(enumeration = "RuleEngineTriggerFunction", tag = "4")]
    pub rule_engine_trigger_function: i32,
    #[prost(int32, tag = "5")]
    pub threshold: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigRuleEngineRuleCreateResponse {
    #[prost(string, tag = "1")]
    pub rule_engine_rule_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigRuleEngineRuleGroupCreateRequest {
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub code: ::core::option::Option<Code>,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigRuleEngineRuleGroupCreateResponse {
    #[prost(string, tag = "1")]
    pub rule_engine_rule_group_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigRuleEngineRuleActionAssociateRequest {
    /// UUID
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// UUID
    #[prost(string, tag = "2")]
    pub rule_engine_rule_id: ::prost::alloc::string::String,
    /// UUID
    #[prost(string, tag = "3")]
    pub rule_engine_action_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigRuleEngineRuleConditionAssociateRequest {
    /// UUID
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// UUID
    #[prost(string, tag = "2")]
    pub rule_engine_rule_id: ::prost::alloc::string::String,
    /// UUID
    #[prost(string, tag = "3")]
    pub rule_engine_condition_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigRuleEngineRuleGroupRuleAssociateRequest {
    /// UUID
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// UUID
    #[prost(string, tag = "2")]
    pub rule_engine_rule_id: ::prost::alloc::string::String,
    /// UUID
    #[prost(string, tag = "3")]
    pub rule_engine_rule_group_id: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RuleEnginePredicate {
    Equal = 0,
    Unequal = 1,
    GreaterOrEqual = 2,
    LessOrEqual = 3,
    Greater = 4,
    Less = 5,
}
impl RuleEnginePredicate {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RuleEnginePredicate::Equal => "RULE_ENGINE_PREDICATE_EQUAL",
            RuleEnginePredicate::Unequal => "RULE_ENGINE_PREDICATE_UNEQUAL",
            RuleEnginePredicate::GreaterOrEqual => {
                "RULE_ENGINE_PREDICATE_GREATER_OR_EQUAL"
            }
            RuleEnginePredicate::LessOrEqual => "RULE_ENGINE_PREDICATE_LESS_OR_EQUAL",
            RuleEnginePredicate::Greater => "RULE_ENGINE_PREDICATE_GREATER",
            RuleEnginePredicate::Less => "RULE_ENGINE_PREDICATE_LESS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RULE_ENGINE_PREDICATE_EQUAL" => Some(Self::Equal),
            "RULE_ENGINE_PREDICATE_UNEQUAL" => Some(Self::Unequal),
            "RULE_ENGINE_PREDICATE_GREATER_OR_EQUAL" => Some(Self::GreaterOrEqual),
            "RULE_ENGINE_PREDICATE_LESS_OR_EQUAL" => Some(Self::LessOrEqual),
            "RULE_ENGINE_PREDICATE_GREATER" => Some(Self::Greater),
            "RULE_ENGINE_PREDICATE_LESS" => Some(Self::Less),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RuleEngineSubject {
    DailyOccupancy = 0,
    DailyPickUp = 1,
    DailyRatePlanPickUp = 2,
    DailyRatePlanDatePickUp = 3,
}
impl RuleEngineSubject {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RuleEngineSubject::DailyOccupancy => "RULE_ENGINE_SUBJECT_DAILY_OCCUPANCY",
            RuleEngineSubject::DailyPickUp => "RULE_ENGINE_SUBJECT_DAILY_PICK_UP",
            RuleEngineSubject::DailyRatePlanPickUp => {
                "RULE_ENGINE_SUBJECT_DAILY_RATE_PLAN_PICK_UP"
            }
            RuleEngineSubject::DailyRatePlanDatePickUp => {
                "RULE_ENGINE_SUBJECT_DAILY_RATE_PLAN_DATE_PICK_UP"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RULE_ENGINE_SUBJECT_DAILY_OCCUPANCY" => Some(Self::DailyOccupancy),
            "RULE_ENGINE_SUBJECT_DAILY_PICK_UP" => Some(Self::DailyPickUp),
            "RULE_ENGINE_SUBJECT_DAILY_RATE_PLAN_PICK_UP" => {
                Some(Self::DailyRatePlanPickUp)
            }
            "RULE_ENGINE_SUBJECT_DAILY_RATE_PLAN_DATE_PICK_UP" => {
                Some(Self::DailyRatePlanDatePickUp)
            }
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RuleEngineTriggerFunction {
    ForAll = 0,
    Exists = 1,
}
impl RuleEngineTriggerFunction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RuleEngineTriggerFunction::ForAll => "RULE_ENGINE_TRIGGER_FUNCTION_FOR_ALL",
            RuleEngineTriggerFunction::Exists => "RULE_ENGINE_TRIGGER_FUNCTION_EXISTS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RULE_ENGINE_TRIGGER_FUNCTION_FOR_ALL" => Some(Self::ForAll),
            "RULE_ENGINE_TRIGGER_FUNCTION_EXISTS" => Some(Self::Exists),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod stey_rms_config_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct SteyRmsConfigServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SteyRmsConfigServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SteyRmsConfigServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SteyRmsConfigServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            SteyRmsConfigServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn config_rule_engine_action_create(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfigRuleEngineActionCreateRequest>,
        ) -> Result<
            tonic::Response<super::ConfigRuleEngineActionCreateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.stey.rms.api.grpc.config.SteyRmsConfigService/ConfigRuleEngineActionCreate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn config_rule_engine_condition_create(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ConfigRuleEngineConditionCreateRequest,
            >,
        ) -> Result<
            tonic::Response<super::ConfigRuleEngineConditionCreateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.stey.rms.api.grpc.config.SteyRmsConfigService/ConfigRuleEngineConditionCreate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn config_rule_engine_rule_create(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfigRuleEngineRuleCreateRequest>,
        ) -> Result<
            tonic::Response<super::ConfigRuleEngineRuleCreateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.stey.rms.api.grpc.config.SteyRmsConfigService/ConfigRuleEngineRuleCreate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn config_rule_engine_rule_group_create(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ConfigRuleEngineRuleGroupCreateRequest,
            >,
        ) -> Result<
            tonic::Response<super::ConfigRuleEngineRuleGroupCreateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.stey.rms.api.grpc.config.SteyRmsConfigService/ConfigRuleEngineRuleGroupCreate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn config_rule_engine_rule_action_associate(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ConfigRuleEngineRuleActionAssociateRequest,
            >,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.stey.rms.api.grpc.config.SteyRmsConfigService/ConfigRuleEngineRuleActionAssociate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn config_rule_engine_rule_condition_associate(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ConfigRuleEngineRuleConditionAssociateRequest,
            >,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.stey.rms.api.grpc.config.SteyRmsConfigService/ConfigRuleEngineRuleConditionAssociate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn config_rule_engine_rule_group_rule_associate(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ConfigRuleEngineRuleGroupRuleAssociateRequest,
            >,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.stey.rms.api.grpc.config.SteyRmsConfigService/ConfigRuleEngineRuleGroupRuleAssociate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
