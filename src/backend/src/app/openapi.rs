use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};

pub const AUTH_TAG: &str = "Auth";
pub const MONITORING_TAG: &str = "Monitoring";
pub const USER_TAG: &str = "User";
pub const SYSTEM_TAG: &str = "System";
pub const DATA_TAG: &str = "Data";
pub const PUBLIC_SYSTEM_TAG: &str = "Public systems";

#[derive(OpenApi)]
#[openapi(
    modifiers(&ApiDocSecurityAddon),
    tags(
        (name = AUTH_TAG, description = "Endpoints to authenticate users"),
        (name = MONITORING_TAG, description = "Endpoints to monitor the system (such as healthchecks)"),
        (name = USER_TAG, description = "Endpoints related to users and their accounts"),
        (name = SYSTEM_TAG, description = "Endpoints related to monitored systems"),
        (name = DATA_TAG, description = "Endpoints that must be connected to by the monitored systems"),
        (name = PUBLIC_SYSTEM_TAG, description = "Endpoints related to monitored systems that are public")
    )
)]
pub(super) struct ApiDoc;

struct ApiDocSecurityAddon;

impl Modify for ApiDocSecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "session",
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("monitor_id"))),
            )
        }
    }
}
