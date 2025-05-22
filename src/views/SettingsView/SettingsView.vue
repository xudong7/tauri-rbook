<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { Close, Check } from "@element-plus/icons-vue";
import { getCurrentWindow } from "@tauri-apps/api/window";

const closeWindow = async () => {
  // 获取当前窗口并关闭它
  const currentWindow = getCurrentWindow();
  await currentWindow.close();
};

// 组件加载时获取保存的阅读设置
onMounted(() => {});
</script>

<template>
  <div class="settings-container">
    <!-- 自定义标题栏 -->
    <div class="settings-header">
      <div class="title">阅读设置</div>
      <div class="window-controls">
        <button
          class="window-control-button close-button"
          @click="closeWindow"
          title="关闭"
        >
          <el-icon :size="16"><Close /></el-icon>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped src="./SettingsView.css" />
