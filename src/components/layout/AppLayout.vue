<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import AppSidebar from "./AppSidebar.vue";
import AppHeader from "./AppHeader.vue";
import { useUiStore } from "../../stores/uiStore";
import { settingsApi } from "../../api/settings";
import { convertFileSrc } from "@tauri-apps/api/core";

const ui = useUiStore();
const backgroundImage = ref("");
const backgroundOpacity = ref(0.3);
const backgroundBlur = ref(0);
const backgroundBrightness = ref(1.0);
const backgroundSize = ref("cover");

onMounted(async () => {
  await loadBackgroundSettings();

  // 监听设置更新事件
  window.addEventListener('settings-updated', loadBackgroundSettings);
});

async function loadBackgroundSettings() {
  try {
    const settings = await settingsApi.get();
    if (settings.background_image) {
      backgroundImage.value = convertFileSrc(settings.background_image);
    } else {
      backgroundImage.value = "";
    }
    backgroundOpacity.value = settings.background_opacity;
    backgroundBlur.value = settings.background_blur;
    backgroundBrightness.value = settings.background_brightness;
    backgroundSize.value = settings.background_size;
    console.log("Background settings loaded:", {
      image: backgroundImage.value,
      opacity: backgroundOpacity.value,
      blur: backgroundBlur.value,
      brightness: backgroundBrightness.value,
      size: backgroundSize.value
    });
  } catch (e) {
    console.error("Failed to load background settings:", e);
  }
}

const backgroundStyle = computed(() => {
  if (!backgroundImage.value) return {};
  return {
    backgroundImage: `url(${backgroundImage.value})`,
    backgroundSize: backgroundSize.value,
    backgroundPosition: "center",
    backgroundRepeat: "no-repeat",
    opacity: backgroundOpacity.value,
    filter: `blur(${backgroundBlur.value}px) brightness(${backgroundBrightness.value})`,
  };
});
</script>

<template>
  <div class="app-layout">
    <div class="app-background" :style="backgroundStyle"></div>
    <AppSidebar />
    <div class="app-main" :class="{ 'sidebar-collapsed': ui.sidebarCollapsed }">
      <AppHeader />
      <main class="app-content">
        <router-view v-slot="{ Component }">
          <transition name="page-fade" mode="out-in">
            <keep-alive :max="5">
              <component :is="Component" />
            </keep-alive>
          </transition>
        </router-view>
      </main>
    </div>
  </div>
</template>

<style scoped>
.app-layout {
  position: relative;
  display: flex;
  width: 100vw;
  height: 100vh;
  background-color: var(--sl-bg);
  overflow: hidden;
}

.app-background {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 0;
  pointer-events: none;
  transition: all 0.3s ease;
}

.app-main {
  position: relative;
  z-index: 1;
  flex: 1;
  display: flex;
  flex-direction: column;
  margin-left: var(--sl-sidebar-width);
  transition: margin-left var(--sl-transition-normal);
  min-width: 0;
}

.app-main.sidebar-collapsed {
  margin-left: var(--sl-sidebar-collapsed-width);
}

.app-content {
  flex: 1;
  padding: var(--sl-space-lg);
  overflow-y: auto;
  overflow-x: hidden;
}

.page-fade-enter-active,
.page-fade-leave-active {
  transition: opacity 0.12s ease, transform 0.12s ease;
}

.page-fade-enter-from {
  opacity: 0;
  transform: translateY(4px);
}

.page-fade-leave-to {
  opacity: 0;
  transform: translateY(-2px);
}
</style>
