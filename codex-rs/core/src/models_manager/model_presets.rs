use codex_app_server_protocol::AuthMode;
use codex_protocol::openai_models::ModelPreset;
use codex_protocol::openai_models::ReasoningEffort;
use codex_protocol::openai_models::ReasoningEffortPreset;
use once_cell::sync::Lazy;

pub const HIDE_GPT5_1_MIGRATION_PROMPT_CONFIG: &str = "hide_gpt5_1_migration_prompt";
pub const HIDE_GPT_5_1_CODEX_MAX_MIGRATION_PROMPT_CONFIG: &str =
    "hide_gpt-5.1-codex-max_migration_prompt";

fn default_reasoning_efforts() -> Vec<ReasoningEffortPreset> {
    vec![
        ReasoningEffortPreset {
            effort: ReasoningEffort::Low,
            description: "Faster responses with lighter reasoning".to_string(),
        },
        ReasoningEffortPreset {
            effort: ReasoningEffort::Medium,
            description: "Balanced speed and reasoning depth".to_string(),
        },
        ReasoningEffortPreset {
            effort: ReasoningEffort::High,
            description: "More reasoning for complex problems".to_string(),
        },
    ]
}

static PRESETS: Lazy<Vec<ModelPreset>> = Lazy::new(|| {
    vec![
        ModelPreset {
            id: "openai/gpt-oss-120b:nitro".to_string(),
            model: "openai/gpt-oss-120b:nitro".to_string(),
            display_name: "openai/gpt-oss-120b:nitro".to_string(),
            description: "OpenRouter GPT-OSS 120B (nitro).".to_string(),
            default_reasoning_effort: ReasoningEffort::Medium,
            supported_reasoning_efforts: default_reasoning_efforts(),
            supports_personality: true,
            is_default: true,
            upgrade: None,
            show_in_picker: true,
            supported_in_api: true,
        },
        ModelPreset {
            id: "minimax/minimax-m2.1:nitro".to_string(),
            model: "minimax/minimax-m2.1:nitro".to_string(),
            display_name: "minimax/minimax-m2.1:nitro".to_string(),
            description: "OpenRouter Minimax M2.1 (nitro).".to_string(),
            default_reasoning_effort: ReasoningEffort::Medium,
            supported_reasoning_efforts: default_reasoning_efforts(),
            supports_personality: true,
            is_default: false,
            upgrade: None,
            show_in_picker: true,
            supported_in_api: true,
        },
        ModelPreset {
            id: "moonshotai/kimi-k2.5:nitro".to_string(),
            model: "moonshotai/kimi-k2.5:nitro".to_string(),
            display_name: "moonshotai/kimi-k2.5:nitro".to_string(),
            description: "OpenRouter Kimi K2.5 (nitro).".to_string(),
            default_reasoning_effort: ReasoningEffort::Medium,
            supported_reasoning_efforts: default_reasoning_efforts(),
            supports_personality: true,
            is_default: false,
            upgrade: None,
            show_in_picker: true,
            supported_in_api: true,
        },
        ModelPreset {
            id: "z-ai/glm-4.7:nitro".to_string(),
            model: "z-ai/glm-4.7:nitro".to_string(),
            display_name: "z-ai/glm-4.7:nitro".to_string(),
            description: "OpenRouter GLM-4.7 (nitro).".to_string(),
            default_reasoning_effort: ReasoningEffort::Medium,
            supported_reasoning_efforts: default_reasoning_efforts(),
            supports_personality: true,
            is_default: false,
            upgrade: None,
            show_in_picker: true,
            supported_in_api: true,
        },
    ]
});

pub(super) fn builtin_model_presets(_auth_mode: Option<AuthMode>) -> Vec<ModelPreset> {
    PRESETS.iter().cloned().collect()
}

#[cfg(any(test, feature = "test-support"))]
pub fn all_model_presets() -> &'static Vec<ModelPreset> {
    &PRESETS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_one_default_model_is_configured() {
        let default_models = PRESETS.iter().filter(|preset| preset.is_default).count();
        assert!(default_models == 1);
    }
}
