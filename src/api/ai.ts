// AI统一接口与实现骨架

export enum AIMode {
  API = 'api',
  Local = 'local',
}

export interface AIRequest {
  prompt: string;
  context?: string[];
  user?: string;
  // 可扩展更多参数
}

export interface AIResponse {
  text: string;
  // 可扩展更多返回内容
}

export interface AIService {
  mode: AIMode;
  sendMessage(request: AIRequest): Promise<AIResponse>;
}

// API模式实现骨架
export class APIAIService implements AIService {
  mode = AIMode.API;
  private apiUrl: string;
  private apiKey?: string;

  constructor(apiUrl: string, apiKey?: string) {
    this.apiUrl = apiUrl;
    this.apiKey = apiKey;
  }

  async sendMessage(request: AIRequest): Promise<AIResponse> {
    // 这里只做结构示例，具体实现需根据API文档调整
    const res = await fetch(this.apiUrl, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...(this.apiKey ? { 'Authorization': `Bearer ${this.apiKey}` } : {}),
      },
      body: JSON.stringify({ prompt: request.prompt, context: request.context }),
    });
    const data = await res.json();
    return { text: data.text || data.choices?.[0]?.text || '' };
  }
}

// 本地模式实现骨架
export class LocalAIService implements AIService {
  mode = AIMode.Local;

  async sendMessage(request: AIRequest): Promise<AIResponse> {
    // 这里应调用本地AI模型服务（如通过tauri invoke、wasm、socket等）
    // 示例返回
    return { text: '（本地AI模型暂未实现）' };
  }
}

// 工厂方法，便于切换AI模式
export function createAIService(mode: AIMode, options?: { apiUrl?: string; apiKey?: string }): AIService {
  if (mode === AIMode.API && options?.apiUrl) {
    return new APIAIService(options.apiUrl, options.apiKey);
  }
  return new LocalAIService();
}
