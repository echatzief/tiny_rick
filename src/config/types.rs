use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub agent: AgentConfig,
}

#[derive(Deserialize)]
pub struct AgentConfig {
    pub provider: String,
    pub model: String,
    pub max_iterations: u32,
    pub system_prompt: String,
    pub permissions: Vec<Permission>,
}

#[derive(Deserialize)]
pub enum PermissionAction {
    Allow,
    Deny,
    Ask,
}

#[derive(Deserialize)]
pub struct Permission {
    pub name: String,
    pub action: PermissionAction,
}
