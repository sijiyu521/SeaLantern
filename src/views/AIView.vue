<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useAIStore } from '../stores/aiStore';
import SLCard from '../components/common/SLCard.vue';
import SLButton from '../components/common/SLButton.vue';
import SLInput from '../components/common/SLInput.vue';
import SLSpinner from '../components/common/SLSpinner.vue';
import SLBadge from '../components/common/SLBadge.vue';
import { i18n } from '../locales';

const aiStore = useAIStore();

const messageInput = ref('');
const isSending = ref(false);

// 计算属性
const isAIEnabled = computed(() => aiStore.isEnabled);
const canSendMessage = computed(() =>
  messageInput.value.trim().length > 0 &&
  !aiStore.isProcessing &&
  !isSending.value &&
  isAIEnabled.value
);

const statusText = computed(() => {
  if (!isAIEnabled.value) return 'AI 功能已禁用';
  if (aiStore.isProcessing) return 'AI 正在思考...';
  if (aiStore.error) return `错误: ${aiStore.error}`;
  return 'AI 就绪';
});

const statusColor = computed(() => {
  if (!isAIEnabled.value) return 'warning';
  if (aiStore.isProcessing) return 'info';
  if (aiStore.error) return 'error';
  return 'success';
});

// 生命周期
onMounted(() => {
  aiStore.initialize();
});

// 方法
async function handleSendMessage() {
  if (!canSendMessage.value) return;

  isSending.value = true;
  try {
    await aiStore.sendMessage(messageInput.value.trim());
    messageInput.value = '';
    scrollToBottom();
  } catch (error) {
    console.error('发送消息失败:', error);
  } finally {
    isSending.value = false;
  }
}

function handleKeyPress(event: KeyboardEvent) {
  if (event.key === 'Enter' && !event.shiftKey) {
    event.preventDefault();
    handleSendMessage();
  }
}

function scrollToBottom() {
  // 在下一个 tick 滚动到底部
  setTimeout(() => {
    const container = document.querySelector('.chat-messages');
    if (container) {
      container.scrollTop = container.scrollHeight;
    }
  }, 100);
}

function clearChat() {
  aiStore.clearMessages();
}

function openSettings() {
  // TODO: 跳转到设置页面的 AI 配置部分
  console.log('打开 AI 设置');
}

function formatTime(date: Date): string {
  return date.toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit'
  });
}

function formatMessageContent(content: string): string {
  // 简单的 Markdown 转换
  return content
    .replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>')
    .replace(/\*(.*?)\*/g, '<em>$1</em>')
    .replace(/`(.*?)`/g, '<code>$1</code>')
    .replace(/\n/g, '<br>');
}
</script>

<template>
  <div class="ai-view">
    <!-- 状态栏 -->
    <div class="status-bar">
      <SLBadge :type="statusColor" size="sm">
        {{ statusText }}
      </SLBadge>
      <div class="status-actions">
        <SLButton
          variant="ghost"
          size="sm"
          @click="clearChat"
          :disabled="!aiStore.hasMessages"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M3 6h18M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2" />
          </svg>
          清空对话
        </SLButton>
        <SLButton
          variant="ghost"
          size="sm"
          @click="openSettings"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 15a3 3 0 100-6 3 3 0 000 6z" />
            <path d="M19.4 15a1.65 1.65 0 00.33 1.82l.06.06a2 2 0 010 2.83 2 2 0 01-2.83 0l-.06-.06a1.65 1.65 0 00-1.82-.33 1.65 1.65 0 00-1 1.51V21a2 2 0 01-2 2 2 2 0 01-2-2v-.09A1.65 1.65 0 009 19.4a1.65 1.65 0 00-1.82.33l-.06.06a2 2 0 01-2.83 0 2 2 0 010-2.83l.06-.06a1.65 1.65 0 00.33-1.82 1.65 1.65 0 00-1.51-1H3a2 2 0 01-2-2 2 2 0 012-2h.09A1.65 1.65 0 004.6 9a1.65 1.65 0 00-.33-1.82l-.06-.06a2 2 0 010-2.83 2 2 0 012.83 0l.06.06a1.65 1.65 0 001.82.33H9a1.65 1.65 0 001-1.51V3a2 2 0 012-2 2 2 0 012 2v.09a1.65 1.65 0 001 1.51 1.65 1.65 0 001.82-.33l.06-.06a2 2 0 012.83 0 2 2 0 010 2.83l-.06.06a1.65 1.65 0 00-.33 1.82V9a1.65 1.65 0 001.51 1H21a2 2 0 012 2 2 2 0 01-2 2h-.09a1.65 1.65 0 00-1.51 1z" />
          </svg>
          AI 设置
        </SLButton>
      </div>
    </div>

    <!-- 聊天容器 -->
    <SLCard class="chat-container" :padding="false">
      <!-- 消息区域 -->
      <div class="chat-messages">
        <div v-if="!aiStore.hasMessages" class="empty-state">
          <div class="empty-icon">
            <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M13 10V3L4 14h7v7l9-11h-7z" />
            </svg>
          </div>
          <h3>AI 助手</h3>
          <p v-if="isAIEnabled">
            开始与 AI 助手对话，获取 Minecraft 服务器管理建议。
          </p>
          <p v-else class="disabled-text">
            AI 功能已禁用，请在设置中启用。
          </p>
          <div class="example-prompts" v-if="isAIEnabled">
            <p>试试问：</p>
            <div class="prompt-buttons">
              <SLButton
                variant="outline"
                size="sm"
                @click="messageInput = '如何优化服务器性能？'"
              >
                如何优化服务器性能？
              </SLButton>
              <SLButton
                variant="outline"
                size="sm"
                @click="messageInput = '生成一个传送点的命令'"
              >
                生成一个传送点的命令
              </SLButton>
              <SLButton
                variant="outline"
                size="sm"
                @click="messageInput = '分析服务器日志的常见错误'"
              >
                分析服务器日志的常见错误
              </SLButton>
            </div>
          </div>
        </div>

        <!-- 消息列表 -->
        <div v-else class="messages-list">
          <div
            v-for="message in aiStore.messages"
            :key="message.id"
            :class="['message', message.role]"
          >
            <div class="message-header">
              <span class="message-role">
                {{ message.role === 'user' ? '你' : 'AI 助手' }}
              </span>
              <span class="message-time">
                {{ formatTime(message.timestamp) }}
              </span>
              <span
                v-if="message.tokens_used && message.tokens_used > 0"
                class="message-tokens"
              >
                {{ message.tokens_used }} tokens
              </span>
            </div>
            <div
              class="message-content"
              v-html="formatMessageContent(message.content)"
            />
          </div>

          <!-- 思考指示器 -->
          <div v-if="aiStore.isProcessing" class="thinking-indicator">
            <SLSpinner size="sm" />
            <span>AI 正在思考...</span>
          </div>
        </div>
      </div>

      <!-- 输入区域 -->
      <div class="chat-input-area">
        <div class="input-wrapper">
          <SLInput
            v-model="messageInput"
            type="textarea"
            placeholder="输入你的问题..."
            :rows="3"
            :disabled="!isAIEnabled || aiStore.isProcessing"
            @keydown="handleKeyPress"
            class="message-input"
          />
          <div class="input-actions">
            <span class="char-count">
              {{ messageInput.length }} / 1000
            </span>
            <SLButton
              @click="handleSendMessage"
              :disabled="!canSendMessage"
              :loading="isSending"
              class="send-button"
            >
              <template v-if="!isSending">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M22 2L11 13M22 2l-7 20-4-9-9-4 20-7z" />
                </svg>
                发送
              </template>
            </SLButton>
          </div>
        </div>
      </div>
    </SLCard>

    <!-- 功能面板 -->
    <div class="function-panel" v-if="isAIEnabled">
      <SLCard title="快捷功能" class="function-card">
        <div class="function-buttons">
          <SLButton
            variant="outline"
            @click="messageInput = '生成一个传送点的命令'"
            :disabled="!aiStore.canGenerateCommands"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M17 8l4 4m0 0l-4 4m4-4H3" />
            </svg>
            生成命令
          </SLButton>
          <SLButton
            variant="outline"
            @click="messageInput = '分析服务器日志'"
            :disabled="!aiStore.canAnalyzeLogs"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
            分析日志
          </SLButton>
          <SLButton
            variant="outline"
            @click="messageInput = '服务器性能优化建议'"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M13 10V3L4 14h7v7l9-11h-7z" />
            </svg>
            性能建议
          </SLButton>
        </div>
      </SLCard>

      <!-- 最近生成的内容 -->
      <SLCard title="最近生成" class="recent-card" v-if="aiStore.lastCommand || aiStore.lastAnalysis">
        <div v-if="aiStore.lastCommand" class="recent-item">
          <h4>生成的命令</h4>
          <code class="command-text">{{ aiStore.lastCommand.command }}</code>
          <p class="command-explanation">{{ aiStore.lastCommand.explanation }}</p>
          <SLButton
            variant="ghost"
            size="sm"
            @click="aiStore.clearLastCommand"
          >
            清除
          </SLButton>
        </div>
        <div v-if="aiStore.lastAnalysis" class="recent-item">
          <h4>日志分析</h4>
          <p>{{ aiStore.lastAnalysis.summary }}</p>
          <SLButton
            variant="ghost"
            size="sm"
            @click="aiStore.clearLastAnalysis"
          >
            清除
          </SLButton>
        </div>
      </SLCard>
    </div>
  </div>
</template>

<style scoped>
.ai-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  gap: var(--sl-space-md);
  padding: var(--sl-space-md);
}

.status-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--sl-space-sm) var(--sl-space-md);
  background: var(--sl-bg-secondary);
  border-radius: var(--sl-radius-md);
  border: 1px solid var(--sl-border-light);
}

.status-actions {
  display: flex;
  gap: var(--sl-space-sm);
}

.chat-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.chat-messages {
  flex: 1;
  overflow-y: auto;
  padding: var(--sl-space-md);
  min-height: 300px;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  text-align: center;
  color: var(--sl-text-secondary);
}

.empty-icon {
  margin-bottom: var(--sl-space-md);
  color: var(--sl-primary);
}

.empty-state h3 {
  margin-bottom: var(--sl-space-sm);
  color: var(--sl-text-primary);
}

.disabled-text {
  color: var(--sl-text-disabled);
}

.example-prompts {
  margin-top: var(--sl-space-lg);
  max-width: 500px;
}

.prompt-buttons {
  display: flex;
  flex-wrap: wrap;
  gap: var(--sl-space-sm);
  margin-top: var(--sl-space-sm);
  justify-content: center;
}

.messages-list {
  display: flex;
  flex-direction: column;
  gap: var(--sl-space-md);
}

.message {
  padding: var(--sl-space-md);
  border-radius: var(--sl-radius-lg);
  max-width: 80%;
}

.message.user {
  align-self: flex-end;
  background: var(--sl-primary-bg);
  border: 1px solid var(--sl-primary-border);
}

.message.assistant {
  align-self: flex-start;
  background: var(--sl-bg-secondary);
  border: 1px solid var(--sl-border-light);
}

.message-header {
  display: flex;
  align-items: center;
  gap: var(--sl-space-sm);
  margin-bottom: var(--sl-space-xs);
  font-size: 0.875rem;
  color: var(--sl-text-secondary);
}

.message-role {
  font-weight: 600;
}

.message-time {
  opacity: 0.7;
}

.message-tokens {
  margin-left: auto;
  font-size: 0.75rem;
  opacity: 0.6;
}

.message-content {
  line-height: 1.6;
  white-space: pre-wrap;
}

.message-content :deep(strong) {
  font-weight: 600;
}

.message-content :deep(em) {
  font-style: italic;
}

.message-content :deep(code) {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  background: var(--sl-bg-tertiary);
  padding: 2px 6px;
  border-radius: var(--sl-radius-sm);
  font-size: 0.875em;
}

.thinking-indicator {
  display: flex;
  align-items: center;
  gap: var(--sl-space-sm);
  padding: var(--sl-space-md);
  color: var(--sl-text-secondary);
  font-size: 0.875rem;
}

.chat-input-area {
  border-top: 1px solid var(--sl-border-light);
  padding: var(--sl-space-md);
}

.input-wrapper {
  display: flex;
  flex-direction: column;
  gap: var(--sl-space-sm);
}

.input-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.char-count {
  font-size: 0.75rem;
  color: var(--sl-text-tertiary);
}

.send-button {
  min-width: 80px;
}

.function-panel {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--sl-space-md);
}

.function-card,
.recent-card {
  height: 100%;
}

.function-buttons {
  display: flex;
  flex-direction: column;
  gap: var(--sl-space-sm);
}

.recent-item {
  margin-bottom: var(--sl-space-md);
}

.recent-item:last-child {
  margin-bottom: 0;
}

.recent-item h4 {
  margin-bottom: var(--sl-space-xs);
  font-size: 0.875rem;
  color: var(--sl-text-secondary);
}

.command-text {
  display: block;
  padding: var(--sl-space-sm);
  background: var(--sl-bg-tertiary);
  border-radius: var(--sl-radius-sm);
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 0.875rem;
  margin-bottom: var(--sl-space-xs);
  overflow-x: auto;
}

.command-explanation {
  font-size: 0.875rem;
  color: var(--sl-text-secondary);
  margin-bottom: var(--sl-space-sm);
}

@media (max-width: 768px) {
  .function-panel {
    grid-template-columns: 1fr;
  }

  .message {
    max-width: 90%;
  }
}
</style>