// AI统一接口与实现骨架
import { tauriInvoke } from "./tauri";

export enum AIMode {
  API = 'api',
  Local = 'local',
}

export enum AIProvider {
  OpenAI = 'openai',
  Anthropic = 'anthropic',
  Custom = 'custom',
}

export interface AIRequest {
  prompt: string;
  context?: string[];
  user?: string;
}

export interface AIResponse {
  text: string;
  tokens_used: number;
  processing_time_ms: number;
}

export interface AIConfig {
  enabled: boolean;
  mode: AIMode;
  api_provider: AIProvider;
  api_url: string;
  api_key: string;
  model: string;
  temperature: number;
  max_tokens: number;
  system_prompt: string;
  command_generation_enabled: boolean;
  log_analysis_enabled: boolean;
  performance_analysis_enabled: boolean;
}

export interface GeneratedCommand {
  command: string;
  explanation: string;
  warning?: string;
}

export interface LogAnalysis {
  summary: string;
  issues: LogIssue[];
  recommendations: string[];
}

export interface LogIssue {
  issue_type: LogIssueType;
  message: string;
  count: number;
  suggestions: string[];
}

export enum LogIssueType {
  Error = 'error',
  Warning = 'warning',
  Info = 'info',
}

export interface AIStatus {
  enabled: boolean;
  mode: string;
  api_provider: string;
  model: string;
  features: {
    command_generation: boolean;
    log_analysis: boolean;
    performance_analysis: boolean;
  };
}

// AI API 封装
export const aiApi = {
  // 发送消息到 AI 服务
  async sendMessage(request: AIRequest): Promise<AIResponse> {
    return tauriInvoke<AIResponse>('send_ai_message', {
      prompt: request.prompt,
      context: request.context,
      user: request.user,
    });
  },

  // 生成 Minecraft 命令
  async generateCommand(description: string, version?: string): Promise<GeneratedCommand> {
    return tauriInvoke<GeneratedCommand>('generate_minecraft_command', {
      description,
      version,
    });
  },

  // 分析服务器日志
  async analyzeLogs(logPath: string, serverId?: string): Promise<LogAnalysis> {
    return tauriInvoke<LogAnalysis>('analyze_server_logs', {
      log_path: logPath,
      server_id: serverId,
    });
  },

  // 获取 AI 配置
  async getConfig(): Promise<AIConfig> {
    return tauriInvoke<AIConfig>('get_ai_config');
  },

  // 保存 AI 配置
  async saveConfig(config: AIConfig): Promise<void> {
    return tauriInvoke<void>('save_ai_config', { config });
  },

  // 测试 AI 连接
  async testConnection(): Promise<boolean> {
    return tauriInvoke<boolean>('test_ai_connection');
  },

  // 获取 AI 状态
  async getStatus(): Promise<AIStatus> {
    return tauriInvoke<AIStatus>('get_ai_status');
  },
};

// 兼容旧的接口实现
export interface AIService {
  mode: AIMode;
  sendMessage(request: AIRequest): Promise<AIResponse>;
}

export class APIAIService implements AIService {
  mode = AIMode.API;
  private apiUrl: string;
  private apiKey?: string;

  constructor(apiUrl: string, apiKey?: string) {
    this.apiUrl = apiUrl;
    this.apiKey = apiKey;
  }

  async sendMessage(request: AIRequest): Promise<AIResponse> {
    // 使用新的 API 封装
    return aiApi.sendMessage(request);
  }
}

export class LocalAIService implements AIService {
  mode = AIMode.Local;

  async sendMessage(request: AIRequest): Promise<AIResponse> {
    // 使用新的 API 封装
    return aiApi.sendMessage(request);
  }
}

export function createAIService(mode: AIMode, options?: { apiUrl?: string; apiKey?: string }): AIService {
  if (mode === AIMode.API && options?.apiUrl) {
    return new APIAIService(options.apiUrl, options.apiKey);
  }
  return new LocalAIService();
}
