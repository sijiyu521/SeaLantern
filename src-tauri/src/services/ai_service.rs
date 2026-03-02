use crate::models::ai::{AIConfig, AIRequest, AIResponse, AIMode, AIProvider};
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::RwLock;

/// AI 服务错误类型
#[derive(Debug)]
pub enum AIError {
    /// 配置错误
    ConfigError(String),
    /// 网络错误
    NetworkError(String),
    /// API 错误
    APIError(String),
    /// 本地模型错误
    LocalError(String),
}

impl std::fmt::Display for AIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AIError::ConfigError(msg) => write!(f, "配置错误: {}", msg),
            AIError::NetworkError(msg) => write!(f, "网络错误: {}", msg),
            AIError::APIError(msg) => write!(f, "API 错误: {}", msg),
            AIError::LocalError(msg) => write!(f, "本地模型错误: {}", msg),
        }
    }
}

/// AI 服务结构
pub struct AIService {
    /// HTTP 客户端
    client: Client,
    /// 配置
    config: Arc<RwLock<AIConfig>>,
}

impl AIService {
    /// 创建新的 AI 服务实例
    pub fn new(config: AIConfig) -> Self {
        Self {
            client: Client::new(),
            config: Arc::new(RwLock::new(config)),
        }
    }

    /// 发送消息到 AI 服务
    pub async fn send_message(&self, request: AIRequest) -> Result<AIResponse, AIError> {
        let config = self.config.read().await;

        // 检查 AI 是否启用
        if !config.enabled {
            return Err(AIError::ConfigError("AI 功能未启用".to_string()));
        }

        match config.mode {
            AIMode::API => self.send_api_message(&config, request).await,
            AIMode::Local => self.send_local_message(&config, request).await,
        }
    }

    /// 发送 API 消息
    async fn send_api_message(&self, config: &AIConfig, request: AIRequest) -> Result<AIResponse, AIError> {
        match config.api_provider {
            AIProvider::OpenAI => self.send_openai_message(config, request).await,
            AIProvider::Anthropic => self.send_anthropic_message(config, request).await,
            AIProvider::Custom => self.send_custom_message(config, request).await,
        }
    }

    /// 发送 OpenAI 消息
    async fn send_openai_message(&self, config: &AIConfig, request: AIRequest) -> Result<AIResponse, AIError> {
        // 检查 API 密钥
        if config.api_key.is_empty() {
            return Err(AIError::ConfigError("OpenAI API 密钥未设置".to_string()));
        }

        // 构建请求
        let url = "https://api.openai.com/v1/chat/completions".to_string();
        let body = serde_json::json!({
            "model": config.model,
            "messages": [
                {
                    "role": "system",
                    "content": config.system_prompt
                },
                {
                    "role": "user",
                    "content": request.prompt
                }
            ],
            "temperature": config.temperature,
            "max_tokens": config.max_tokens,
        });

        // 发送请求
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", config.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AIError::NetworkError(format!("请求失败: {}", e)))?;

        // 检查响应状态
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(AIError::APIError(format!("API 返回错误: {} - {}", status, text)));
        }

        // 解析响应
        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AIError::APIError(format!("解析响应失败: {}", e)))?;

        // 提取响应文本
        let text = response_json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let tokens_used = response_json["usage"]["total_tokens"]
            .as_u64()
            .unwrap_or(0) as u32;

        Ok(AIResponse {
            text,
            tokens_used,
            processing_time_ms: 0, // TODO: 实际计算处理时间
        })
    }

    /// 发送 Anthropic 消息
    async fn send_anthropic_message(&self, config: &AIConfig, request: AIRequest) -> Result<AIResponse, AIError> {
        // 检查 API 密钥
        if config.api_key.is_empty() {
            return Err(AIError::ConfigError("Anthropic API 密钥未设置".to_string()));
        }

        // TODO: 实现 Anthropic API 调用
        // 暂时返回模拟响应
        Ok(AIResponse {
            text: format!("Anthropic API 调用 (模型: {})：{}", config.model, request.prompt),
            tokens_used: 0,
            processing_time_ms: 0,
        })
    }

    /// 发送自定义 API 消息
    async fn send_custom_message(&self, config: &AIConfig, request: AIRequest) -> Result<AIResponse, AIError> {
        // 检查 API URL
        if config.api_url.is_empty() {
            return Err(AIError::ConfigError("自定义 API URL 未设置".to_string()));
        }

        // 构建请求
        let body = serde_json::json!({
            "prompt": request.prompt,
            "context": request.context,
            "user": request.user,
            "model": config.model,
            "temperature": config.temperature,
            "max_tokens": config.max_tokens,
        });

        // 发送请求
        let mut request_builder = self.client
            .post(&config.api_url)
            .header("Content-Type", "application/json");

        // 添加认证头（如果有 API 密钥）
        if !config.api_key.is_empty() {
            request_builder = request_builder.header("Authorization", format!("Bearer {}", config.api_key));
        }

        let response = request_builder
            .json(&body)
            .send()
            .await
            .map_err(|e| AIError::NetworkError(format!("请求失败: {}", e)))?;

        // 检查响应状态
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(AIError::APIError(format!("API 返回错误: {} - {}", status, text)));
        }

        // 解析响应
        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AIError::APIError(format!("解析响应失败: {}", e)))?;

        // 提取响应文本
        let text = response_json["text"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let tokens_used = response_json["tokens_used"]
            .as_u64()
            .unwrap_or(0) as u32;

        Ok(AIResponse {
            text,
            tokens_used,
            processing_time_ms: 0,
        })
    }

    /// 发送本地模型消息
    async fn send_local_message(&self, _config: &AIConfig, request: AIRequest) -> Result<AIResponse, AIError> {
        // TODO: 实现本地模型调用
        // 暂时返回模拟响应
        Ok(AIResponse {
            text: format!("本地模型调用 (暂未实现)：{}", request.prompt),
            tokens_used: 0,
            processing_time_ms: 0,
        })
    }

    /// 测试连接
    pub async fn test_connection(&self) -> Result<bool, AIError> {
        let config = self.config.read().await;

        if !config.enabled {
            return Err(AIError::ConfigError("AI 功能未启用".to_string()));
        }

        match config.mode {
            AIMode::API => {
                // 测试 API 连接
                match config.api_provider {
                    AIProvider::OpenAI => self.test_openai_connection(&config).await,
                    AIProvider::Anthropic => self.test_anthropic_connection(&config).await,
                    AIProvider::Custom => self.test_custom_connection(&config).await,
                }
            }
            AIMode::Local => {
                // 测试本地模型连接
                self.test_local_connection(&config).await
            }
        }
    }

    /// 测试 OpenAI 连接
    async fn test_openai_connection(&self, config: &AIConfig) -> Result<bool, AIError> {
        if config.api_key.is_empty() {
            return Err(AIError::ConfigError("OpenAI API 密钥未设置".to_string()));
        }

        // 发送一个简单的测试请求
        let response = self.client
            .get("https://api.openai.com/v1/models")
            .header("Authorization", format!("Bearer {}", config.api_key))
            .send()
            .await;

        match response {
            Ok(resp) if resp.status().is_success() => Ok(true),
            Ok(resp) => {
                let status = resp.status();
                Err(AIError::APIError(format!("OpenAI API 连接测试失败: {}", status)))
            }
            Err(e) => Err(AIError::NetworkError(format!("网络连接失败: {}", e))),
        }
    }

    /// 测试 Anthropic 连接
    async fn test_anthropic_connection(&self, _config: &AIConfig) -> Result<bool, AIError> {
        // TODO: 实现 Anthropic 连接测试
        // 暂时返回成功
        Ok(true)
    }

    /// 测试自定义 API 连接
    async fn test_custom_connection(&self, config: &AIConfig) -> Result<bool, AIError> {
        if config.api_url.is_empty() {
            return Err(AIError::ConfigError("自定义 API URL 未设置".to_string()));
        }

        // 发送一个简单的测试请求
        let response = self.client
            .get(&config.api_url)
            .send()
            .await;

        match response {
            Ok(resp) if resp.status().is_success() => Ok(true),
            Ok(resp) => {
                let status = resp.status();
                Err(AIError::APIError(format!("自定义 API 连接测试失败: {}", status)))
            }
            Err(e) => Err(AIError::NetworkError(format!("网络连接失败: {}", e))),
        }
    }

    /// 测试本地模型连接
    async fn test_local_connection(&self, _config: &AIConfig) -> Result<bool, AIError> {
        // TODO: 实现本地模型连接测试
        // 暂时返回成功
        Ok(true)
    }

    /// 更新配置
    pub async fn update_config(&self, new_config: AIConfig) {
        let mut config = self.config.write().await;
        *config = new_config;
    }

    /// 获取当前配置
    pub async fn get_config(&self) -> AIConfig {
        let config = self.config.read().await;
        config.clone()
    }
}