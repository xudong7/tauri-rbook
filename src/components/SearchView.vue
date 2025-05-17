<script setup lang="ts">
import { Window } from "@tauri-apps/api/window";
import {
  Search,
  Minus,
  ArrowLeft,
  FullScreen,
  Close,
} from "@element-plus/icons-vue";
import "./SearchView.css";

const appWindow = Window.getCurrent();
const emit = defineEmits<{
  back: [];
}>();

// Window control functions
const minimizeWindow = async () => {
  await appWindow.minimize();
};

const maximizeWindow = async () => {
  if (await appWindow.isMaximized()) {
    await appWindow.unmaximize();
  } else {
    await appWindow.maximize();
  }
};

const closeWindow = async () => {
  await appWindow.close();
};

const goBackToMenu = () => {
  emit("back");
};
</script>

<template>
  <div class="menu-container">
    <!-- Toolbar -->
    <div class="menu-toolbar">
      <div class="left-controls">
        <button class="icon-button" @click="goBackToMenu" title="返回菜单">
          <el-icon :size="20"><ArrowLeft /></el-icon>
        </button>
        <button class="icon-button" title="搜索电子书">
          <el-icon :size="20"><Search /></el-icon>
        </button>
      </div>
      <div class="window-controls">
        <button
          class="window-control-button"
          @click="minimizeWindow"
          title="Minimize"
        >
          <el-icon :size="16"><Minus /></el-icon>
        </button>
        <button
          class="window-control-button"
          @click="maximizeWindow"
          title="Maximize"
        >
          <el-icon :size="16"><FullScreen /></el-icon>
        </button>
        <button
          class="window-control-button close-button"
          @click="closeWindow"
          title="Close"
        >
          <el-icon :size="16"><Close /></el-icon>
        </button>
      </div>
    </div>

    <!-- Book Grid -->
    <div class="book-grid">
      <!-- TODO: Implement book grid layout -->
    </div>
  </div>
</template>
