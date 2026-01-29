use anyhow::Result;
use codex_core::CodexAuth;
use codex_core::ThreadManager;
use codex_core::built_in_model_providers;
use codex_core::models_manager::manager::RefreshStrategy;
use codex_protocol::openai_models::ModelPreset;
use codex_protocol::openai_models::ReasoningEffort;
use codex_protocol::openai_models::ReasoningEffortPreset;
use core_test_support::load_default_config_for_test;
use pretty_assertions::assert_eq;
use tempfile::tempdir;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn list_models_returns_api_key_models() -> Result<()> {
    let codex_home = tempdir()?;
    let config = load_default_config_for_test(&codex_home).await;
    let manager = ThreadManager::with_models_provider(
        CodexAuth::from_api_key("sk-test"),
        built_in_model_providers()["openrouter"].clone(),
    );
    let models = manager
        .list_models(&config, RefreshStrategy::OnlineIfUncached)
        .await;

    let expected_models = expected_models_for_api_key();
    assert_eq!(expected_models, models);

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn list_models_returns_chatgpt_models() -> Result<()> {
    let codex_home = tempdir()?;
    let config = load_default_config_for_test(&codex_home).await;
    let manager = ThreadManager::with_models_provider(
        CodexAuth::create_dummy_chatgpt_auth_for_testing(),
        built_in_model_providers()["openrouter"].clone(),
    );
    let models = manager
        .list_models(&config, RefreshStrategy::OnlineIfUncached)
        .await;

    let expected_models = expected_models_for_chatgpt();
    assert_eq!(expected_models, models);

    Ok(())
}

fn expected_models_for_api_key() -> Vec<ModelPreset> {
    vec![
        openai_gpt_oss(),
        minimax_m2_1(),
        kimi_k2_5(),
        glm_4_7(),
    ]
}

fn expected_models_for_chatgpt() -> Vec<ModelPreset> {
    expected_models_for_api_key()
}

fn openai_gpt_oss() -> ModelPreset {
    ModelPreset {
        id: "openai/gpt-oss-120b:nitro".to_string(),
        model: "openai/gpt-oss-120b:nitro".to_string(),
        display_name: "gpt-oss".to_string(),
        description: "OpenRouter GPT-OSS 120B (nitro).".to_string(),
        default_reasoning_effort: ReasoningEffort::Medium,
        supported_reasoning_efforts: default_efforts(),
        supports_personality: true,
        is_default: true,
        upgrade: None,
        show_in_picker: true,
        supported_in_api: true,
    }
}

fn minimax_m2_1() -> ModelPreset {
    ModelPreset {
        id: "minimax/minimax-m2.1:nitro".to_string(),
        model: "minimax/minimax-m2.1:nitro".to_string(),
        display_name: "minimax-m2.1".to_string(),
        description: "OpenRouter Minimax M2.1 (nitro).".to_string(),
        default_reasoning_effort: ReasoningEffort::Medium,
        supported_reasoning_efforts: default_efforts(),
        supports_personality: true,
        is_default: false,
        upgrade: None,
        show_in_picker: true,
        supported_in_api: true,
    }
}

fn kimi_k2_5() -> ModelPreset {
    ModelPreset {
        id: "moonshotai/kimi-k2.5:nitro".to_string(),
        model: "moonshotai/kimi-k2.5:nitro".to_string(),
        display_name: "kimi-k2.5".to_string(),
        description: "OpenRouter Kimi K2.5 (nitro).".to_string(),
        default_reasoning_effort: ReasoningEffort::Medium,
        supported_reasoning_efforts: default_efforts(),
        supports_personality: true,
        is_default: false,
        upgrade: None,
        show_in_picker: true,
        supported_in_api: true,
    }
}

fn glm_4_7() -> ModelPreset {
    ModelPreset {
        id: "z-ai/glm-4.7:nitro".to_string(),
        model: "z-ai/glm-4.7:nitro".to_string(),
        display_name: "glm-4.7".to_string(),
        description: "OpenRouter GLM-4.7 (nitro).".to_string(),
        default_reasoning_effort: ReasoningEffort::Medium,
        supported_reasoning_efforts: default_efforts(),
        supports_personality: true,
        is_default: false,
        upgrade: None,
        show_in_picker: true,
        supported_in_api: true,
    }
}

fn default_efforts() -> Vec<ReasoningEffortPreset> {
    vec![
        effort(ReasoningEffort::Low, "Faster responses with lighter reasoning"),
        effort(ReasoningEffort::Medium, "Balanced speed and reasoning depth"),
        effort(ReasoningEffort::High, "More reasoning for complex problems"),
    ]
}

fn effort(effort: ReasoningEffort, description: &str) -> ReasoningEffortPreset {
    ReasoningEffortPreset {
        effort,
        description: description.to_string(),
    }
}
