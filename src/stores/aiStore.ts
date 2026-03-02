import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { aiApi } from '../api/ai';
import type { AIRequest, AIResponse, AIConfig, GeneratedCommand, LogAnalysis, AIStatus } from '../api/ai';

// 聊天消息类型
export interface ChatMessage {
  id: string;
  role: 'user' | 'assistant';
  content: string;
  timestamp: Date;
  tokens_used?: number;
  processing_time_ms?: number;
}

export const useAIStore = defineStore('ai', () => {
  // 状态
  const messages = ref<ChatMessage[]>([]);
  const isProcessing = ref(false);
  const config = ref<AIConfig | null>(null);
  const status = ref<AIStatus | null>(null);
  const error = ref<string | null>(null);
  const lastCommand = ref<GeneratedCommand | null>(null);
  const lastAnalysis = ref<LogAnalysis | null>(null);

  // 计算属性
  const conversationHistory = computed(() => {
    return messages.value.map(msg => `${msg.role}: ${msg.content}`).join('\n');
  });

  const hasMessages = computed(() => messages.value.length > 0);

  const isEnabled = computed(() => config.value?.enabled ?? false);

  const canGenerateCommands = computed(() =>
    isEnabled.value && (config.value?.command_generation_enabled ?? false)
  );

  const canAnalyzeLogs = computed(() =>
    isEnabled.value && (config.value?.log_analysis_enabled ?? false)
  );

  // 动作
  async function sendMessage(content: string, context?: string[]) {
    isProcessing.value = true;
    error.value = null;

    try {
      // 检查 AI 是否启用
      if (!isEnabled.value) {
        throw new Error('AI 功能未启用，请在设置中启用');
      }

      // 添加用户消息
      const userMessage: ChatMessage = {
        id: generateId(),
        role: 'user',
        content,
        timestamp: new Date(),
      };
      messages.value.push(userMessage);

      // 构建请求
      const request: AIRequest = {
        prompt: content,
        context: context || [conversationHistory.value],
        user: 'user', // TODO: 获取实际用户标识
      };

      // 发送请求
      const response = await aiApi.sendMessage(request);

      // 添加 AI 回复
      const assistantMessage: ChatMessage = {
        id: generateId(),
        role: 'assistant',
        content: response.text,
        timestamp: new Date(),
        tokens_used: response.tokens_used,
        processing_time_ms: response.processing_time_ms,
      };
      messages.value.push(assistantMessage);

      return response;
    } catch (err) {
      error.value = err instanceof Error ? err.message : '未知错误';
      throw err;
    } finally {
      isProcessing.value = false;
    }
  }

  async function generateCommand(description: string, version?: string) {
    error.value = null;

    try {
      // 检查命令生成功能是否启用
      if (!canGenerateCommands.value) {
        throw new Error('命令生成功能未启用，请在 AI 设置中启用');
      }

      const command = await aiApi.generateCommand(description, version);
      lastCommand.value = command;
      return command;
    } catch (err) {
      error.value = err instanceof Error ? err.message : '未知错误';
      throw err;
    }
  }

  async function analyzeLogs(logPath: string, serverId?: string) {
    error.value = null;

    try {
      // 检查日志分析功能是否启用
      if (!canAnalyzeLogs.value) {
        throw new Error('日志分析功能未启用，请在 AI 设置中启用');
      }

      const analysis = await aiApi.analyzeLogs(logPath, serverId);
      lastAnalysis.value = analysis;
      return analysis;
    } catch (err) {
      error.value = err instanceof Error ? err.message : '未知错误';
      throw err;
    }
  }

  async function loadConfig() {
    try {
      config.value = await aiApi.getConfig();
      error.value = null;
    } catch (err) {
      error.value = err instanceof Error ? err.message : '未知错误';
      throw err;
    }
  }

  async function saveConfig(newConfig: AIConfig) {
    try {
      await aiApi.saveConfig(newConfig);
      config.value = newConfig;
      error.value = null;
    } catch (err) {
      error.value = err instanceof Error ? err.message : '未知错误';
      throw err;
    }
  }

  async function testConnection() {
    error.value = null;

    try {
      const success = await aiApi.testConnection();
      if (!success) {
        throw new Error('连接测试失败');
      }
      return success;
    } catch (err) {
      error.value = err instanceof Error ? err.message : '未知错误';
      throw err;
    }
  }

  async function loadStatus() {
    try {
      status.value = await aiApi.getStatus();
      error.value = null;
    } catch (err) {
      error.value = err instanceof Error ? err.message : '未知错误';
      throw err;
    }
  }

  function clearMessages() {
    messages.value = [];
    error.value = null;
  }

  function clearError() {
    error.value = null;
  }

  function clearLastCommand() {
    lastCommand.value = null;
  }

  function clearLastAnalysis() {
    lastAnalysis.value = null;
  }

  // 工具函数
  function generateId(): string {
    return Date.now().toString(36) + Math.random().toString(36).substring(2);
  }

  // 初始化
  async function initialize() {
    try {
      await loadConfig();
      await loadStatus();
    } catch (err) {
      console.error('初始化 AI Store 失败:', err);
    }
  }

  return {
    // 状态
    messages,
    isProcessing,
    config,
    status,
    error,
    lastCommand,
    lastAnalysis,

    // 计算属性
    conversationHistory,
    hasMessages,
    isEnabled,
    canGenerateCommands,
    canAnalyzeLogs,

    // 动作
    sendMessage,
    generateCommand,
    analyzeLogs,
    loadConfig,
    saveConfig,
    testConnection,
    loadStatus,
    clearMessages,
    clearError,
    clearLastCommand,
    clearLastAnalysis,
    initialize,
  };
});