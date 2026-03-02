use serde::{Deserialize, Serialize};

/// AI 配置模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIMode {
    /// API 模式 - 调用外部 AI 服务
    API,
    /// 本地模式 - 使用本地 AI 模型
    Local,
}

impl Default for AIMode {
    fn default() -> Self {
        AIMode::API
    }
}

/// AI 提供商类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIProvider {
    /// OpenAI API
    OpenAI,
    /// Anthropic Claude API
    Anthropic,
    /// 自定义 API 端点
    Custom,
}

impl Default for AIProvider {
    fn default() -> Self {
        AIProvider::OpenAI
    }
}

/// AI 配置结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    /// 是否启用 AI 功能
    #[serde(default = "default_ai_enabled")]
    pub enabled: bool,

    /// AI 模式
    #[serde(default)]
    pub mode: AIMode,

    /// API 提供商
    #[serde(default)]
    pub api_provider: AIProvider,

    /// API 端点 URL (仅用于自定义提供商)
    #[serde(default)]
    pub api_url: String,

    /// API 密钥 (加密存储)
    #[serde(default)]
    pub api_key: String,

    /// 模型名称
    #[serde(default = "default_model")]
    pub model: String,

    /// 温度参数 (0.0-2.0)
    #[serde(default = "default_temperature")]
    pub temperature: f32,

    /// 最大 token 数
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,

    /// 系统提示词
    #[serde(default = "default_system_prompt")]
    pub system_prompt: String,

    /// 是否启用命令生成功能
    #[serde(default = "default_command_generation_enabled")]
    pub command_generation_enabled: bool,

    /// 是否启用日志分析功能
    #[serde(default = "default_log_analysis_enabled")]
    pub log_analysis_enabled: bool,

    /// 是否启用性能分析功能
    #[serde(default = "default_performance_analysis_enabled")]
    pub performance_analysis_enabled: bool,
}

/// AI 请求结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRequest {
    /// 用户提示
    pub prompt: String,

    /// 上下文信息
    #[serde(default)]
    pub context: Vec<String>,

    /// 用户标识
    #[serde(default)]
    pub user: String,
}

/// AI 响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    /// 响应文本
    pub text: String,

    /// 使用的 token 数
    #[serde(default)]
    pub tokens_used: u32,

    /// 处理时间 (毫秒)
    #[serde(default)]
    pub processing_time_ms: u64,
}

/// 生成的 Minecraft 命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedCommand {
    /// 命令文本
    pub command: String,

    /// 命令解释
    pub explanation: String,

    /// 警告信息 (如果有)
    #[serde(default)]
    pub warning: Option<String>,
}

/// 日志分析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAnalysis {
    /// 分析摘要
    pub summary: String,

    /// 发现的问题
    pub issues: Vec<LogIssue>,

    /// 建议
    pub recommendations: Vec<String>,
}

/// 日志问题
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogIssue {
    /// 问题类型
    pub issue_type: LogIssueType,

    /// 问题描述
    pub message: String,

    /// 出现次数
    pub count: u32,

    /// 建议解决方案
    pub suggestions: Vec<String>,
}

/// 日志问题类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogIssueType {
    /// 错误
    Error,
    /// 警告
    Warning,
    /// 信息
    Info,
}

// 默认值函数

fn default_ai_enabled() -> bool {
    false
}

fn default_model() -> String {
    "gpt-3.5-turbo".to_string()
}

fn default_temperature() -> f32 {
    0.7
}

fn default_max_tokens() -> u32 {
    1000
}

fn default_system_prompt() -> String {
    "你是一个 Minecraft 服务器管理助手，专门帮助用户管理 Minecraft 服务器。请提供专业、准确的建议。".to_string()
}

fn default_command_generation_enabled() -> bool {
    true
}

fn default_log_analysis_enabled() -> bool {
    true
}

fn default_performance_analysis_enabled() -> bool {
    true
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            enabled: default_ai_enabled(),
            mode: AIMode::default(),
            api_provider: AIProvider::default(),
            api_url: String::new(),
            api_key: String::new(),
            model: default_model(),
            temperature: default_temperature(),
            max_tokens: default_max_tokens(),
            system_prompt: default_system_prompt(),
            command_generation_enabled: default_command_generation_enabled(),
            log_analysis_enabled: default_log_analysis_enabled(),
            performance_analysis_enabled: default_performance_analysis_enabled(),
        }
    }
}