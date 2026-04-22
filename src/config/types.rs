use serde::Deserialize;

use crate::agents::types::Agent;
use crate::ui::types::UIConfig;
use crate::{providers::types::Provider, tools::types::Permission};

#[derive(Deserialize)]
pub struct Config {
    pub providers: Vec<Provider>,
    pub permissions: Vec<Permission>,
    pub agents: Vec<Agent>,
    pub ui: Vec<UIConfig>,
}
