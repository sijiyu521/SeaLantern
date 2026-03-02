<script setup lang="ts">
import { ref, onMounted, watch, computed } from 'vue';
import { useAIStore } from '../../stores/aiStore';
import SLCard from '../common/SLCard.vue';
import SLButton from '../common/SLButton.vue';
import SLInput from '../common/SLInput.vue';
import SLSwitch from '../common/SLSwitch.vue';
import SLSelect from '../common/SLSelect.vue';
import SLSpinner from '../common/SLSpinner.vue';
import SLBadge from '../common/SLBadge.vue';
import type { AIConfig, AIMode, AIProvider } from '../../api/ai';

const aiStore = useAIStore();

// 本地配置副本
const localConfig = ref<AIConfig | null>(null);
const loading = ref(false);
const saving = ref(false);
const testing = ref(false);
const error = ref<string | null>(null);
const success = ref<string | null>(null);
const hasChanges = ref(false);

// 选项
const modeOptions = [
  { label: 'API 模式 (调用外部 AI 服务)', value: 'api' as AIMode },
  { label: '本地模式 (使用本地 AI 模型)', value: 'local' as AIMode },
];

const providerOptions = [
  { label: 'OpenAI', value: 'openai' as AIProvider },
  { label: 'Anthropic (Claude)', value: 'anthropic' as AIProvider },
  { label: '自定义 API', value: 'custom' as AIProvider },
];

const modelOptions = computed(() => {
  if (!localConfig.value) return [];

  const baseModels = [
    { label: 'GPT-3.5 Turbo', value: 'gpt-3.5-turbo' },
    { label: 'GPT-4', value: 'gpt-4' },
    { label: 'GPT-4 Turbo', value: 'gpt-4-turbo-preview' },
    { label: 'Claude 3 Haiku', value: 'claude-3-haiku-20240307' },
    { label: 'Claude 3 Sonnet', value: 'claude-3-sonnet-20240229' },
    { label: 'Claude 3 Opus', value: 'claude-3-opus-20240229' },
  ];

  // 根据提供商过滤模型
  if (localConfig.value.api_provider === 'openai') {
    return baseModels.filter(m => m.value.startsWith('gpt-'));
  } else if (localConfig.value.api_provider === 'anthropic') {
    return baseModels.filter(m => m.value.startsWith('claude-'));
  }

  return baseModels;
});

// 计算属性
const isAPIEnabled = computed(() => localConfig.value?.mode === 'api');
const isCustomProvider = computed(() => localConfig.value?.api_provider === 'custom');
const showAPIKeyField = computed(() => isAPIEnabled.value && localConfig.value?.api_provider !== 'custom');

// 生命周期
onMounted(async () => {
  await loadConfig();
});

// 方法
async function loadConfig() {
  loading.value = true;
  error.value = null;

  try {
    await aiStore.loadConfig();
    localConfig.value = aiStore.config ? { ...aiStore.config } : null;
    hasChanges.value = false;
  } catch (err) {
    error.value = err instanceof Error ? err.message : '加载配置失败';
  } finally {
    loading.value = false;
  }
}

async function saveConfig() {
  if (!localConfig.value) return;

  saving.value = true;
  error.value = null;
  success.value = null;

  try {
    await aiStore.saveConfig(localConfig.value);
    success.value = '配置保存成功';
    hasChanges.value = false;

    // 3秒后清除成功消息
    setTimeout(() => {
      success.value = null;
    }, 3000);
  } catch (err) {
    error.value = err instanceof Error ? err.message : '保存配置失败';
  } finally {
    saving.value = false;
  }
}

async function testConnection() {
  if (!localConfig.value?.enabled) {
    error.value = '请先启用 AI 功能';
    return;
  }

  testing.value = true;
  error.value = null;
  success.value = null;

  try {
    const connected = await aiStore.testConnection();
    if (connected) {
      success.value = '连接测试成功！AI 服务可用。';
    } else {
      error.value = '连接测试失败，请检查配置。';
    }

    // 3秒后清除消息
    setTimeout(() => {
      success.value = null;
      error.value = null;
    }, 3000);
  } catch (err) {
    error.value = err instanceof Error ? err.message : '连接测试失败';
  } finally {
    testing.value = false;
  }
}

function resetToDefaults() {
  if (!localConfig.value) return;

  // 重置为默认值
  localConfig.value = {
    enabled: false,
    mode: 'api',
    api_provider: 'openai',
    api_url: '',
    api_key: '',
    model: 'gpt-3.5-turbo',
    temperature: 0.7,
    max_tokens: 1000,
    system_prompt: '你是一个 Minecraft 服务器管理助手，专门帮助用户管理 Minecraft 服务器。请提供专业、准确的建议。',
    command_generation_enabled: true,
    log_analysis_enabled: true,
    performance_analysis_enabled: true,
  };

  hasChanges.value = true;
  error.value = null;
  success.value = null;
}

function discardChanges() {
  loadConfig();
}

// 监听配置变化
watch(() => localConfig.value, (newConfig, oldConfig) => {
  if (newConfig && oldConfig) {
    hasChanges.value = JSON.stringify(newConfig) !== JSON.stringify(oldConfig);
  }
}, { deep: true });

// 模式切换时重置相关字段
watch(() => localConfig.value?.mode, (newMode) => {
  if (newMode === 'local' && localConfig.value) {
    // 切换到本地模式时清空 API 相关字段
    localConfig.value.api_provider = 'openai';
    localConfig.value.api_url = '';
    localConfig.value.api_key = '';
  }
});

// 提供商切换时重置模型
watch(() => localConfig.value?.api_provider, (newProvider) => {
  if (newProvider && localConfig.value) {
    // 根据提供商设置默认模型
    if (newProvider === 'openai') {
      localConfig.value.model = 'gpt-3.5-turbo';
    } else if (newProvider === 'anthropic') {
      localConfig.value.model = 'claude-3-haiku-20240307';
    }
  }
});
</script>

<template>
  <div class="ai-settings">
    <!-- 状态指示器 -->
    <div class="status-indicator" v-if="loading || saving || testing">
      <SLSpinner size="sm" />
      <span v-if="loading">加载配置中...</span>
      <span v-if="saving">保存配置中...</span>
      <span v-if="testing">测试连接中...</span>
    </div>

    <!-- 错误和成功消息 -->
    <div v-if="error" class="message error">
      <SLBadge type="error" size="sm">错误</SLBadge>
      <span>{{ error }}</span>
    </div>

    <div v-if="success" class="message success">
      <SLBadge type="success" size="sm">成功</SLBadge>
      <span>{{ success }}</span>
    </div>

    <SLCard title="AI 功能配置" class="settings-card">
      <!-- 启用开关 -->
      <div class="setting-section">
        <SLSwitch
          v-model="localConfig.enabled"
          label="启用 AI 功能"
          :disabled="loading"
        />
        <p class="setting-description">
          启用后可以在 AI 助手页面使用 AI 功能，包括聊天、命令生成和日志分析。
        </p>
      </div>

      <div v-if="localConfig?.enabled" class="enabled-settings">
        <!-- 模式选择 -->
        <div class="setting-section">
          <SLSelect
            v-model="localConfig.mode"
            :options="modeOptions"
            label="AI 模式"
            :disabled="loading"
          />
          <p class="setting-description" v-if="localConfig.mode === 'api'">
            使用外部 AI 服务 API，需要配置 API 密钥。
          </p>
          <p class="setting-description" v-else>
            使用本地 AI 模型，无需网络连接但需要本地计算资源。
          </p>
        </div>

        <!-- API 配置 -->
        <div v-if="isAPIEnabled" class="api-settings">
          <div class="setting-section">
            <SLSelect
              v-model="localConfig.api_provider"
              :options="providerOptions"
              label="API 提供商"
              :disabled="loading"
            />
          </div>

          <!-- 自定义 API URL -->
          <div v-if="isCustomProvider" class="setting-section">
            <SLInput
              v-model="localConfig.api_url"
              label="API 端点 URL"
              placeholder="https://api.example.com/v1/chat/completions"
              :disabled="loading"
            />
            <p class="setting-description">
              输入自定义 API 端点的完整 URL，需要支持 OpenAI 兼容的 API 格式。
            </p>
          </div>

          <!-- API 密钥 -->
          <div v-if="showAPIKeyField" class="setting-section">
            <SLInput
              v-model="localConfig.api_key"
              type="password"
              label="API 密钥"
              placeholder="输入 API 密钥"
              :disabled="loading"
            />
            <p class="setting-description">
              {{ localConfig.api_provider === 'openai' ? 'OpenAI' : 'Anthropic' }} API 密钥，请妥善保管。
            </p>
          </div>

          <!-- 模型选择 -->
          <div class="setting-section">
            <SLSelect
              v-model="localConfig.model"
              :options="modelOptions"
              label="模型"
              :disabled="loading || modelOptions.length === 0"
            />
            <p class="setting-description">
              选择要使用的 AI 模型。不同模型在性能、成本和能力上有所差异。
            </p>
          </div>
        </div>

        <!-- 高级参数 -->
        <div class="setting-section">
          <h3 class="section-title">高级参数</h3>

          <div class="param-row">
            <div class="param-item">
              <label>温度 (Temperature)</label>
              <SLInput
                v-model="localConfig.temperature"
                type="number"
                min="0"
                max="2"
                step="0.1"
                :disabled="loading"
              />
              <p class="param-description">
                控制输出的随机性。值越高输出越随机，值越低输出越确定。
              </p>
            </div>

            <div class="param-item">
              <label>最大 Token 数</label>
              <SLInput
                v-model="localConfig.max_tokens"
                type="number"
                min="1"
                max="4000"
                :disabled="loading"
              />
              <p class="param-description">
                限制 AI 响应的最大长度。1个 token 约等于 0.75 个英文单词。
              </p>
            </div>
          </div>

          <div class="setting-section">
            <SLInput
              v-model="localConfig.system_prompt"
              type="textarea"
              label="系统提示词"
              placeholder="你是一个 Minecraft 服务器管理助手..."
              :rows="3"
              :disabled="loading"
            />
            <p class="setting-description">
              定义 AI 助手的角色和行为。这会影响 AI 的回复风格和内容。
            </p>
          </div>
        </div>

        <!-- 功能开关 -->
        <div class="setting-section">
          <h3 class="section-title">功能开关</h3>

          <div class="feature-switches">
            <SLSwitch
              v-model="localConfig.command_generation_enabled"
              label="命令生成功能"
              :disabled="loading"
            />
            <p class="feature-description">
              允许 AI 根据描述生成 Minecraft 命令。
            </p>

            <SLSwitch
              v-model="localConfig.log_analysis_enabled"
              label="日志分析功能"
              :disabled="loading"
            />
            <p class="feature-description">
              允许 AI 分析服务器日志并提供建议。
            </p>

            <SLSwitch
              v-model="localConfig.performance_analysis_enabled"
              label="性能分析功能"
              :disabled="loading"
            />
            <p class="feature-description">
              允许 AI 分析服务器性能并提供优化建议。
            </p>
          </div>
        </div>
      </div>

      <!-- 操作按钮 -->
      <div class="action-buttons">
        <SLButton
          @click="testConnection"
          :disabled="!localConfig?.enabled || testing || loading"
          :loading="testing"
          variant="outline"
        >
          测试连接
        </SLButton>

        <div class="right-buttons">
          <SLButton
            @click="discardChanges"
            :disabled="!hasChanges || loading"
            variant="ghost"
          >
            放弃更改
          </SLButton>

          <SLButton
            @click="resetToDefaults"
            :disabled="loading"
            variant="ghost"
          >
            恢复默认
          </SLButton>

          <SLButton
            @click="saveConfig"
            :disabled="!hasChanges || loading || saving"
            :loading="saving"
          >
            保存配置
          </SLButton>
        </div>
      </div>
    </SLCard>

    <!-- 使用说明 -->
    <SLCard title="使用说明" class="instructions-card">
      <div class="instructions">
        <h4>API 模式配置指南</h4>
        <ol>
          <li>获取 API 密钥：
            <ul>
              <li><strong>OpenAI</strong>: 访问 <a href="https://platform.openai.com/api-keys" target="_blank">OpenAI Platform</a> 创建 API 密钥</li>
              <li><strong>Anthropic</strong>: 访问 <a href="https://console.anthropic.com/" target="_blank">Anthropic Console</a> 创建 API 密钥</li>
            </ul>
          </li>
          <li>选择合适的模型：
            <ul>
              <li><strong>GPT-3.5 Turbo</strong>: 性价比高，适合一般用途</li>
              <li><strong>GPT-4</strong>: 能力更强，但成本更高</li>
              <li><strong>Claude 系列</strong>: 适合长文本和复杂推理</li>
            </ul>
          </li>
          <li>调整参数：
            <ul>
              <li><strong>温度</strong>: 建议 0.7-0.9 获得平衡的响应</li>
              <li><strong>最大 Token</strong>: 根据需求调整，一般 500-1000 足够</li>
            </ul>
          </li>
          <li>点击"测试连接"验证配置是否正确</li>
        </ol>

        <h4>注意事项</h4>
        <ul>
          <li>API 密钥会安全存储，不会明文显示</li>
          <li>使用 API 可能会产生费用，请关注使用量</li>
          <li>本地模式目前为占位实现，未来会支持本地 LLM</li>
          <li>系统提示词可以定制 AI 助手的专业领域</li>
        </ul>
      </div>
    </SLCard>
  </div>
</template>

<style scoped>
.ai-settings {
  display: flex;
  flex-direction: column;
  gap: var(--sl-space-lg);
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: var(--sl-space-sm);
  padding: var(--sl-space-sm) var(--sl-space-md);
  background: var(--sl-bg-secondary);
  border-radius: var(--sl-radius-md);
  border: 1px solid var(--sl-border-light);
  font-size: 0.875rem;
  color: var(--sl-text-secondary);
}

.message {
  display: flex;
  align-items: center;
  gap: var(--sl-space-sm);
  padding: var(--sl-space-sm) var(--sl-space-md);
  border-radius: var(--sl-radius-md);
  font-size: 0.875rem;
}

.message.error {
  background: var(--sl-error-bg);
  border: 1px solid var(--sl-error-border);
  color: var(--sl-error);
}

.message.success {
  background: var(--sl-success-bg);
  border: 1px solid var(--sl-success-border);
  color: var(--sl-success);
}

.settings-card,
.instructions-card {
  width: 100%;
}

.setting-section {
  margin-bottom: var(--sl-space-lg);
}

.setting-section:last-child {
  margin-bottom: 0;
}

.setting-description {
  margin-top: var(--sl-space-xs);
  font-size: 0.875rem;
  color: var(--sl-text-secondary);
  line-height: 1.5;
}

.enabled-settings {
  margin-top: var(--sl-space-lg);
  padding-top: var(--sl-space-lg);
  border-top: 1px solid var(--sl-border-light);
}

.api-settings {
  margin-top: var(--sl-space-md);
}

.section-title {
  margin-bottom: var(--sl-space-md);
  font-size: 1rem;
  font-weight: 600;
  color: var(--sl-text-primary);
}

.param-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--sl-space-md);
  margin-bottom: var(--sl-space-md);
}

@media (max-width: 768px) {
  .param-row {
    grid-template-columns: 1fr;
  }
}

.param-item {
  display: flex;
  flex-direction: column;
  gap: var(--sl-space-xs);
}

.param-item label {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--sl-text-primary);
}

.param-description {
  font-size: 0.75rem;
  color: var(--sl-text-tertiary);
  line-height: 1.4;
}

.feature-switches {
  display: flex;
  flex-direction: column;
  gap: var(--sl-space-md);
}

.feature-description {
  margin-left: 28px; /* 对齐开关 */
  margin-top: var(--sl-space-xs);
  font-size: 0.875rem;
  color: var(--sl-text-secondary);
  line-height: 1.5;
}

.action-buttons {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: var(--sl-space-xl);
  padding-top: var(--sl-space-lg);
  border-top: 1px solid var(--sl-border-light);
}

.right-buttons {
  display: flex;
  gap: var(--sl-space-sm);
}

.instructions {
  font-size: 0.875rem;
  line-height: 1.6;
}

.instructions h4 {
  margin-top: var(--sl-space-md);
  margin-bottom: var(--sl-space-sm);
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--sl-text-primary);
}

.instructions h4:first-child {
  margin-top: 0;
}

.instructions ol,
.instructions ul {
  margin-left: var(--sl-space-md);
  margin-bottom: var(--sl-space-md);
}

.instructions li {
  margin-bottom: var(--sl-space-xs);
}

.instructions ul ul {
  margin-top: var(--sl-space-xs);
  margin-bottom: var(--sl-space-xs);
}

.instructions a {
  color: var(--sl-primary);
  text-decoration: none;
}

.instructions a:hover {
  text-decoration: underline;
}
</style>