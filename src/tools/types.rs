use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub enum Tool {
    Bash,
    Edit,
    Write,
    Read,
    Grep,
    Glob,
    Webfetch,
}

#[derive(Clone, Deserialize)]
pub enum PermissionAction {
    Allow,
    Deny,
    Ask,
}

#[derive(Clone, Deserialize)]
pub struct Permission {
    pub name: Tool,
    pub action: PermissionAction,
}
