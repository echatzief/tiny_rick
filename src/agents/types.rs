use crate::tools::types::{Permission, Tool};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Agent {
    pub name: String,
    pub system_prompt: String,
    pub tools: Vec<Tool>,
    pub tool_permissions: Vec<Permission>,
}
