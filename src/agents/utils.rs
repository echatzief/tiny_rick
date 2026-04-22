use crate::agents::types::Agent;
use crate::config::constants::DEFAULT_AGENTS;

pub fn merge_agents(config_agents: Vec<Agent>) -> Vec<Agent> {
    let mut result = DEFAULT_AGENTS.to_vec();

    for cfg in config_agents {
        if let Some(existing) = result.iter_mut().find(|a| a.name == cfg.name) {
            *existing = cfg;
        } else {
            result.push(cfg);
        }
    }

    result
}
