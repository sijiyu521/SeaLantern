<script setup lang="ts">
import { ref, onMounted, watch, computed } from "vue";
import SLCard from "../components/common/SLCard.vue";
import SLButton from "../components/common/SLButton.vue";
import SLInput from "../components/common/SLInput.vue";
import SLSwitch from "../components/common/SLSwitch.vue";
import SLModal from "../components/common/SLModal.vue";
import SLSelect from "../components/common/SLSelect.vue";
import { settingsApi, type AppSettings } from "../api/settings";
import { systemApi } from "../api/system";
import { convertFileSrc } from "@tauri-apps/api/core";

const settings = ref<AppSettings | null>(null);
const loading = ref(true);
const saving = ref(false);
const error = ref<string | null>(null);
const success = ref<string | null>(null);
const hasChanges = ref(false);

// String versions for number inputs (avoids v-model type mismatch)
const maxMem = ref("2048");
const minMem = ref("512");
const port = ref("25565");
const fontSize = ref("13");
const logLines = ref("5000");
const bgOpacity = ref("0.3");
const bgBlur = ref("0");
const bgBrightness = ref("1.0");

const backgroundSizeOptions = [
  { label: "覆盖 (Cover)", value: "cover" },
  { label: "包含 (Contain)", value: "contain" },
  { label: "拉伸 (Fill)", value: "fill" },
  { label: "原始大小 (Auto)", value: "auto" },
];

const showImportModal = ref(false);
const importJson = ref("");
const showResetConfirm = ref(false);

const backgroundPreviewUrl = computed(() => {
  if (!settings.value?.background_image) return "";
  return convertFileSrc(settings.value.background_image);
});

onMounted(async () => {
  await loadSettings();
});

async function loadSettings() {
  loading.value = true;
  error.value = null;
  try {
    const s = await settingsApi.get();
    settings.value = s;
    maxMem.value = String(s.default_max_memory);
    minMem.value = String(s.default_min_memory);
    port.value = String(s.default_port);
    fontSize.value = String(s.console_font_size);
    logLines.value = String(s.max_log_lines);
    bgOpacity.value = String(s.background_opacity);
    bgBlur.value = String(s.background_blur);
    bgBrightness.value = String(s.background_brightness);
    hasChanges.value = false;
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
}

function markChanged() {
  hasChanges.value = true;
}

async function saveSettings() {
  if (!settings.value) return;

  // Sync string inputs back to settings object
  settings.value.default_max_memory = parseInt(maxMem.value) || 2048;
  settings.value.default_min_memory = parseInt(minMem.value) || 512;
  settings.value.default_port = parseInt(port.value) || 25565;
  settings.value.console_font_size = parseInt(fontSize.value) || 13;
  settings.value.max_log_lines = parseInt(logLines.value) || 5000;
  settings.value.background_opacity = parseFloat(bgOpacity.value) || 0.3;
  settings.value.background_blur = parseInt(bgBlur.value) || 0;
  settings.value.background_brightness = parseFloat(bgBrightness.value) || 1.0;

  saving.value = true;
  error.value = null;
  try {
    await settingsApi.save(settings.value);
    success.value = "设置已保存";
    hasChanges.value = false;
    setTimeout(() => (success.value = null), 3000);

    // 通知AppLayout重新加载背景
    window.dispatchEvent(new CustomEvent('settings-updated'));
  } catch (e) {
    error.value = String(e);
  } finally {
    saving.value = false;
  }
}

async function resetSettings() {
  try {
    const s = await settingsApi.reset();
    settings.value = s;
    maxMem.value = String(s.default_max_memory);
    minMem.value = String(s.default_min_memory);
    port.value = String(s.default_port);
    fontSize.value = String(s.console_font_size);
    logLines.value = String(s.max_log_lines);
    bgOpacity.value = String(s.background_opacity);
    bgBlur.value = String(s.background_blur);
    bgBrightness.value = String(s.background_brightness);
    showResetConfirm.value = false;
    hasChanges.value = false;
    success.value = "已恢复默认设置";
    setTimeout(() => (success.value = null), 3000);
  } catch (e) {
    error.value = String(e);
  }
}

async function exportSettings() {
  try {
    const json = await settingsApi.exportJson();
    await navigator.clipboard.writeText(json);
    success.value = "设置 JSON 已复制到剪贴板";
    setTimeout(() => (success.value = null), 3000);
  } catch (e) {
    error.value = String(e);
  }
}

async function handleImport() {
  if (!importJson.value.trim()) { error.value = "请粘贴 JSON"; return; }
  try {
    const s = await settingsApi.importJson(importJson.value);
    settings.value = s;
    maxMem.value = String(s.default_max_memory);
    minMem.value = String(s.default_min_memory);
    port.value = String(s.default_port);
    fontSize.value = String(s.console_font_size);
    logLines.value = String(s.max_log_lines);
    bgOpacity.value = String(s.background_opacity);
    bgBlur.value = String(s.background_blur);
    bgBrightness.value = String(s.background_brightness);
    showImportModal.value = false;
    importJson.value = "";
    hasChanges.value = false;
    success.value = "设置已导入";
    setTimeout(() => (success.value = null), 3000);
  } catch (e) {
    error.value = String(e);
  }
}

async function pickBackgroundImage() {
  try {
    const result = await systemApi.pickImageFile();
    console.log("Selected image:", result);
    if (result && settings.value) {
      settings.value.background_image = result;
      markChanged();
    }
  } catch (e) {
    console.error("Pick image error:", e);
    error.value = String(e);
  }
}

function clearBackgroundImage() {
  if (settings.value) {
    settings.value.background_image = "";
    markChanged();
  }
}
</script>

<template>
  <div class="settings-view animate-fade-in-up">
    <div v-if="error" class="msg-banner error-banner">
      <span>{{ error }}</span>
      <button @click="error = null">x</button>
    </div>
    <div v-if="success" class="msg-banner success-banner">
      <span>{{ success }}</span>
    </div>

    <div v-if="loading" class="loading-state">
      <div class="spinner"></div>
      <span>加载设置...</span>
    </div>

    <template v-else-if="settings">
      <!-- General -->
      <SLCard title="通用" subtitle="基本行为设置">
        <div class="settings-group">
          <div class="setting-row">
            <div class="setting-info">
              <span class="setting-label">关闭软件时停止所有服务器</span>
              <span class="setting-desc">退出 Sea Lantern 时自动向运行中的服务器发送 stop 命令，防止数据丢失</span>
            </div>
            <SLSwitch v-model="settings.close_servers_on_exit" @update:modelValue="markChanged" />
          </div>

          <div class="setting-row">
            <div class="setting-info">
              <span class="setting-label">自动同意 EULA</span>
              <span class="setting-desc">启动服务器前自动写入 eula=true，省去手动修改的步骤</span>
            </div>
            <SLSwitch v-model="settings.auto_accept_eula" @update:modelValue="markChanged" />
          </div>
        </div>
      </SLCard>

      <!-- Server Defaults -->
      <SLCard title="服务器默认值" subtitle="创建新服务器时使用的默认参数">
        <div class="settings-group">
          <div class="setting-row">
            <div class="setting-info">
              <span class="setting-label">默认最大内存 (MB)</span>
              <span class="setting-desc">建议至少 1024MB。大型模组服可能需要 4096MB 以上</span>
            </div>
            <div class="input-sm">
              <SLInput v-model="maxMem" type="number" @update:modelValue="markChanged" />
            </div>
          </div>

          <div class="setting-row">
            <div class="setting-info">
              <span class="setting-label">默认最小内存 (MB)</span>
              <span class="setting-desc">建议设为最大内存的 1/4 到 1/2</span>
            </div>
            <div class="input-sm">
              <SLInput v-model="minMem" type="number" @update:modelValue="markChanged" />
            </div>
          </div>

          <div class="setting-row">
            <div class="setting-info">
              <span class="setting-label">默认端口</span>
              <span class="setting-desc">Minecraft 默认端口为 25565。多服务器需要设置不同端口</span>
            </div>
            <div class="input-sm">
              <SLInput v-model="port" type="number" @update:modelValue="markChanged" />
            </div>
          </div>

          <div class="setting-row">
            <div class="setting-info">
              <span class="setting-label">默认 Java 路径</span>
              <span class="setting-desc">留空则每次创建服务器时自动检测最合适的 Java</span>
            </div>
            <div class="input-lg">
              <SLInput v-model="settings.default_java_path" placeholder="留空自动检测" @update:modelValue="markChanged" />
            </div>
          </div>

          <div class="setting-row full-width">
            <div class="setting-info">
              <span class="setting-label">默认 JVM 参数</span>
              <span class="setting-desc">所有服务器启动时都会附加这些参数。适合设置 GC 优化参数</span>
            </div>
            <textarea
              class="jvm-textarea"
              v-model="settings.default_jvm_args"
              placeholder="-XX:+UseG1GC -XX:+ParallelRefProcEnabled -XX:MaxGCPauseMillis=200 -XX:+UnlockExperimentalVMOptions -XX:+DisableExplicitGC"
              rows="3"
              @input="markChanged"
            ></textarea>
          </div>
        </div>
      </SLCard>

      <!-- Console -->
      <SLCard title="控制台" subtitle="控制台显示相关设置">
        <div class="settings-group">
          <div class="setting-row">
            <div class="setting-info">
              <span class="setting-label">控制台字体大小 (px)</span>
              <span class="setting-desc">控制台日志文字的大小，默认 13</span>
            </div>
            <div class="input-sm">
              <SLInput v-model="fontSize" type="number" @update:modelValue="markChanged" />
            </div>
          </div>

          <div class="setting-row">
            <div class="setting-info">
              <span class="setting-label">最大日志行数</span>
              <span class="setting-desc">单个服务器最多保留的日志行数，超出后自动清除旧日志。默认 5000</span>
            </div>
            <div class="input-sm">
              <SLInput v-model="logLines" type="number" @update:modelValue="markChanged" />
            </div>
          </div>
        </div>
      </SLCard>

      <!-- Appearance -->
      <SLCard title="外观" subtitle="自定义软件背景和视觉效果">
        <div class="settings-group">
          <div class="setting-row full-width">
            <div class="setting-info">
              <span class="setting-label">背景图片</span>
              <span class="setting-desc">上传一张图片作为软件背景，支持 PNG、JPG、WEBP 等格式</span>
            </div>
            <div class="bg-image-picker">
              <div v-if="settings.background_image" class="bg-preview">
                <img :src="backgroundPreviewUrl" alt="Background preview" />
                <div class="bg-preview-overlay">
                  <span class="bg-preview-path">{{ settings.background_image.split('\\').pop() }}</span>
                  <SLButton variant="danger" size="sm" @click="clearBackgroundImage">移除</SLButton>
                </div>
              </div>
              <SLButton v-else variant="secondary" @click="pickBackgroundImage">
                选择图片
              </SLButton>
              <SLButton v-if="settings.background_image" variant="secondary" size="sm" @click="pickBackgroundImage">
                更换图片
              </SLButton>
            </div>
          </div>

          <div class="setting-row">
            <div class="setting-info">
              <span class="setting-label">不透明度</span>
              <span class="setting-desc">调节背景图片的不透明度 (0.0 - 1.0)，数值越小越透明</span>
            </div>
            <div class="slider-control">
              <input
                type="range"
                min="0"
                max="1"
                step="0.05"
                v-model="bgOpacity"
                @input="markChanged"
                class="sl-slider"
              />
              <span class="slider-value">{{ bgOpacity }}</span>
            </div>
          </div>

          <div class="setting-row">
            <div class="setting-info">
              <span class="setting-label">模糊程度 (px)</span>
              <span class="setting-desc">为背景添加模糊效果，让前景内容更清晰</span>
            </div>
            <div class="slider-control">
              <input
                type="range"
                min="0"
                max="20"
                step="1"
                v-model="bgBlur"
                @input="markChanged"
                class="sl-slider"
              />
              <span class="slider-value">{{ bgBlur }}px</span>
            </div>
          </div>

          <div class="setting-row">
            <div class="setting-info">
              <span class="setting-label">亮度</span>
              <span class="setting-desc">调节背景图片的亮度 (0.0 - 2.0)，1.0 为原始亮度</span>
            </div>
            <div class="slider-control">
              <input
                type="range"
                min="0"
                max="2"
                step="0.1"
                v-model="bgBrightness"
                @input="markChanged"
                class="sl-slider"
              />
              <span class="slider-value">{{ bgBrightness }}</span>
            </div>
          </div>

          <div class="setting-row">
            <div class="setting-info">
              <span class="setting-label">图片填充方式</span>
              <span class="setting-desc">选择背景图片的显示方式</span>
            </div>
            <div class="input-lg">
              <SLSelect
                v-model="settings.background_size"
                :options="backgroundSizeOptions"
                @update:modelValue="markChanged"
              />
            </div>
          </div>
        </div>
      </SLCard>

      <!-- Actions -->
      <div class="settings-actions">
        <div class="actions-left">
          <SLButton variant="primary" size="lg" :loading="saving" @click="saveSettings">
            保存设置
          </SLButton>
          <SLButton variant="secondary" @click="loadSettings">放弃修改</SLButton>
          <span v-if="hasChanges" class="unsaved-hint">有未保存的更改</span>
        </div>
        <div class="actions-right">
          <SLButton variant="ghost" size="sm" @click="exportSettings">导出</SLButton>
          <SLButton variant="ghost" size="sm" @click="showImportModal = true">导入</SLButton>
          <SLButton variant="danger" size="sm" @click="showResetConfirm = true">恢复默认</SLButton>
        </div>
      </div>
    </template>

    <SLModal :visible="showImportModal" title="导入设置" @close="showImportModal = false">
      <div class="import-form">
        <p class="text-caption">粘贴之前导出的 JSON 数据</p>
        <textarea class="import-textarea" v-model="importJson" placeholder='{"close_servers_on_exit": true, ...}' rows="10"></textarea>
      </div>
      <template #footer>
        <SLButton variant="secondary" @click="showImportModal = false">取消</SLButton>
        <SLButton variant="primary" @click="handleImport">导入</SLButton>
      </template>
    </SLModal>

    <SLModal :visible="showResetConfirm" title="确认恢复默认" @close="showResetConfirm = false">
      <p class="text-body">确定要将所有设置恢复为默认值吗？此操作不可撤销。</p>
      <template #footer>
        <SLButton variant="secondary" @click="showResetConfirm = false">取消</SLButton>
        <SLButton variant="danger" @click="resetSettings">确认恢复</SLButton>
      </template>
    </SLModal>
  </div>
</template>

<style scoped>
.settings-view {
  display: flex; flex-direction: column; gap: var(--sl-space-lg);
  max-width: 860px; padding-bottom: var(--sl-space-2xl);
}

.msg-banner {
  display: flex; align-items: center; justify-content: space-between;
  padding: 10px 16px; border-radius: var(--sl-radius-md); font-size: 0.875rem;
}
.error-banner { background: rgba(239,68,68,0.1); border: 1px solid rgba(239,68,68,0.2); color: var(--sl-error); }
.success-banner { background: rgba(34,197,94,0.1); border: 1px solid rgba(34,197,94,0.2); color: var(--sl-success); }
.msg-banner button { font-weight: 600; color: inherit; }

.loading-state {
  display: flex; align-items: center; justify-content: center;
  gap: var(--sl-space-sm); padding: var(--sl-space-2xl); color: var(--sl-text-tertiary);
}
.spinner { width: 18px; height: 18px; border: 2px solid var(--sl-border); border-top-color: var(--sl-primary); border-radius: 50%; animation: sl-spin 0.8s linear infinite; }

.settings-group { display: flex; flex-direction: column; }

.setting-row {
  display: flex; align-items: center; justify-content: space-between;
  padding: var(--sl-space-md) 0; border-bottom: 1px solid var(--sl-border-light);
  gap: var(--sl-space-lg);
}
.setting-row:last-child { border-bottom: none; }
.setting-row.full-width { flex-direction: column; align-items: stretch; }

.setting-info { flex: 1; display: flex; flex-direction: column; gap: 2px; min-width: 0; }
.setting-label { font-size: 0.9375rem; font-weight: 500; color: var(--sl-text-primary); }
.setting-desc { font-size: 0.8125rem; color: var(--sl-text-tertiary); line-height: 1.4; }

.input-sm { width: 120px; flex-shrink: 0; }
.input-lg { width: 320px; flex-shrink: 0; }

.jvm-textarea, .import-textarea {
  width: 100%; margin-top: var(--sl-space-sm);
  padding: var(--sl-space-sm) var(--sl-space-md);
  font-family: var(--sl-font-mono); font-size: 0.8125rem;
  color: var(--sl-text-primary); background: var(--sl-surface);
  border: 1px solid var(--sl-border); border-radius: var(--sl-radius-md);
  resize: vertical; line-height: 1.6;
}
.jvm-textarea:focus, .import-textarea:focus {
  border-color: var(--sl-primary); box-shadow: 0 0 0 3px var(--sl-primary-bg); outline: none;
}

.settings-actions {
  display: flex; align-items: center; justify-content: space-between;
  padding: var(--sl-space-md) 0; border-top: 1px solid var(--sl-border);
}
.actions-left, .actions-right { display: flex; align-items: center; gap: var(--sl-space-sm); }

.unsaved-hint {
  font-size: 0.8125rem; color: var(--sl-warning); font-weight: 500;
  padding: 2px 10px; background: rgba(245,158,11,0.1); border-radius: var(--sl-radius-full);
}

.import-form { display: flex; flex-direction: column; gap: var(--sl-space-md); }

.bg-image-picker {
  display: flex;
  flex-direction: column;
  gap: var(--sl-space-sm);
  margin-top: var(--sl-space-sm);
}

.bg-preview {
  position: relative;
  width: 100%;
  max-width: 400px;
  height: 200px;
  border-radius: var(--sl-radius-md);
  overflow: hidden;
  border: 1px solid var(--sl-border);
}

.bg-preview img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.bg-preview-overlay {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  padding: var(--sl-space-sm) var(--sl-space-md);
  background: linear-gradient(to top, rgba(0,0,0,0.8), transparent);
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--sl-space-sm);
}

.bg-preview-path {
  font-size: 0.8125rem;
  color: white;
  font-family: var(--sl-font-mono);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.slider-control {
  display: flex;
  align-items: center;
  gap: var(--sl-space-md);
  min-width: 200px;
}

.sl-slider {
  flex: 1;
  height: 6px;
  border-radius: var(--sl-radius-full);
  background: var(--sl-border);
  outline: none;
  -webkit-appearance: none;
}

.sl-slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: var(--sl-primary);
  cursor: pointer;
  transition: all var(--sl-transition-fast);
}

.sl-slider::-webkit-slider-thumb:hover {
  transform: scale(1.2);
  box-shadow: 0 0 0 4px var(--sl-primary-bg);
}

.sl-slider::-moz-range-thumb {
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: var(--sl-primary);
  cursor: pointer;
  border: none;
  transition: all var(--sl-transition-fast);
}

.sl-slider::-moz-range-thumb:hover {
  transform: scale(1.2);
  box-shadow: 0 0 0 4px var(--sl-primary-bg);
}

.slider-value {
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--sl-text-primary);
  min-width: 50px;
  text-align: right;
}
</style>
