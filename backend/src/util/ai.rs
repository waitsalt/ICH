use rig::{
    agent::Agent,
    providers::deepseek::{self, DeepSeekCompletionModel},
};

use crate::util::config::CONFIG;

pub fn new() -> Agent<DeepSeekCompletionModel> {
    let model = CONFIG.ai.model.clone();
    let api_key = CONFIG.ai.api_key.clone();
    let base_url = CONFIG.ai.base_url.clone();

    let agent = deepseek::Client::from_url(&api_key, &base_url);
    agent
        .agent(&model)
        .preamble("你是一个用于非遗物质文家的助手")
        .build()
}
