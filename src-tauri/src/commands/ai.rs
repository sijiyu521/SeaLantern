use crate::models::ai::{AIRequest, AIResponse, AIConfig, GeneratedCommand, LogAnalysis};
use crate::services::{ai_service, global};
use tauri::command;

/// 发送消息到 AI 服务
#[command]
pub async fn send_ai_message(
    prompt: String,
    context: Option<Vec<String>>,
    user: Option<String>,
) -> Result<AIResponse, String> {
    let settings = global::settings_manager().get();

    // 检查 AI 功能是否启用
    if !settings.ai_config.enabled {
        return Err("AI 功能未启用，请在设置中启用".to_string());
    }

    let request = AIRequest {
        prompt,
        context: context.unwrap_or_default(),
        user: user.unwrap_or_default(),
    };

    // 创建 AI 服务实例并调用
    let ai_service = ai_service::AIService::new(settings.ai_config.clone());
    match ai_service.send_message(request).await {
        Ok(response) => Ok(response),
        Err(e) => Err(e.to_string()),
    }
}

/// 生成 Minecraft 命令
#[command]
pub async fn generate_minecraft_command(
    description: String,
    version: Option<String>,
) -> Result<GeneratedCommand, String> {
    let settings = global::settings_manager().get();

    // 检查 AI 功能是否启用
    if !settings.ai_config.enabled {
        return Err("AI 功能未启用，请在设置中启用".to_string());
    }

    // 检查命令生成功能是否启用
    if !settings.ai_config.command_generation_enabled {
        return Err("命令生成功能未启用，请在 AI 设置中启用".to_string());
    }

    // TODO: 调用 AI 服务生成命令
    // 暂时返回模拟响应
    Ok(GeneratedCommand {
        command: format!("/say 这是一个由 AI 生成的命令示例"),
        explanation: format!("根据你的描述 '{}' 生成的命令。Minecraft 版本：{}",
            description,
            version.unwrap_or_else(|| "未知".to_string())
        ),
        warning: Some("这是一个示例命令，实际使用时请根据你的服务器配置进行调整。".to_string()),
    })
}

/// 分析服务器日志
#[command]
pub async fn analyze_server_logs(
    log_path: String,
    _server_id: Option<String>,
) -> Result<LogAnalysis, String> {
    let settings = global::settings_manager().get();

    // 检查 AI 功能是否启用
    if !settings.ai_config.enabled {
        return Err("AI 功能未启用，请在设置中启用".to_string());
    }

    // 检查日志分析功能是否启用
    if !settings.ai_config.log_analysis_enabled {
        return Err("日志分析功能未启用，请在 AI 设置中启用".to_string());
    }

    // TODO: 读取和分析日志文件
    // 暂时返回模拟响应
    Ok(LogAnalysis {
        summary: format!("已分析日志文件：{}。这是一个示例分析结果。", log_path),
        issues: vec![],
        recommendations: vec![
            "建议定期清理旧日志文件".to_string(),
            "监控服务器内存使用情况".to_string(),
            "检查插件兼容性".to_string(),
        ],
    })
}

/// 获取 AI 配置
#[command]
pub async fn get_ai_config() -> Result<AIConfig, String> {
    let settings = global::settings_manager().get();
    Ok(settings.ai_config.clone())
}

/// 保存 AI 配置
#[command]
pub async fn save_ai_config(config: AIConfig) -> Result<(), String> {
    let mut settings = global::settings_manager().get();
    settings.ai_config = config;
    global::settings_manager().update(settings)
        .map_err(|e| format!("保存 AI 配置失败：{}", e))
}

/// 测试 AI 连接
#[command]
pub async fn test_ai_connection() -> Result<bool, String> {
    let settings = global::settings_manager().get();

    // 检查 AI 功能是否启用
    if !settings.ai_config.enabled {
        return Err("AI 功能未启用，请在设置中启用".to_string());
    }

    // 创建 AI 服务实例并测试连接
    let ai_service = ai_service::AIService::new(settings.ai_config.clone());
    match ai_service.test_connection().await {
        Ok(success) => Ok(success),
        Err(e) => Err(e.to_string()),
    }
}

/// 获取 AI 功能状态
#[command]
pub async fn get_ai_status() -> Result<serde_json::Value, String> {
    let settings = global::settings_manager().get();

    let status = serde_json::json!({
        "enabled": settings.ai_config.enabled,
        "mode": match settings.ai_config.mode {
            crate::models::ai::AIMode::API => "api",
            crate::models::ai::AIMode::Local => "local",
        },
        "api_provider": match settings.ai_config.api_provider {
            crate::models::ai::AIProvider::OpenAI => "openai",
            crate::models::ai::AIProvider::Anthropic => "anthropic",
            crate::models::ai::AIProvider::Custom => "custom",
        },
        "model": settings.ai_config.model,
        "features": {
            "command_generation": settings.ai_config.command_generation_enabled,
            "log_analysis": settings.ai_config.log_analysis_enabled,
            "performance_analysis": settings.ai_config.performance_analysis_enabled,
        }
    });

    Ok(status)
}