<script setup lang="ts">
import { Window } from "@tauri-apps/api/window";
import { Minus, FullScreen, Close } from "@element-plus/icons-vue";
const props = defineProps<{
  appWindow?: Window;
}>();

/**
 * 最小化窗口
 */
const minimizeWindow = async () => {
  await props.appWindow?.minimize();
};

/**
 * 最大化/还原窗口
 */
const maximizeWindow = async () => {
  if (await props.appWindow?.isMaximized()) {
    await props.appWindow?.unmaximize();
  } else {
    await props.appWindow?.maximize();
  }
};

/**
 * 关闭窗口
 */
const closeWindow = async () => {
  await props.appWindow?.close();
};
</script>

<template>
  <div class="window-controls">
    <button class="icon-button" @click="minimizeWindow" title="Minimize">
      <el-icon :size="20"><Minus /></el-icon>
    </button>
    <button class="icon-button" @click="maximizeWindow" title="Maximize">
      <el-icon :size="20"><FullScreen /></el-icon>
    </button>
    <button class="icon-button close-button" @click="closeWindow" title="Close">
      <el-icon :size="20"><Close /></el-icon>
    </button>
  </div>
</template>

<style scoped>
.icon-button {
  background: none;
  border: none;
  cursor: pointer;
  padding: 8px;
  border-radius: 4px;
  color: var(--app-text-secondary);
  transition: all 0.3s ease;
}

.icon-button:hover {
  background-color: var(--app-surface);
  color: var(--app-accent);
}

.window-controls {
  display: flex;
  gap: 8px;
  -webkit-app-region: no-drag;
}

.close-button:hover {
  background-color: var(--app-danger);
  color: white;
}
</style>