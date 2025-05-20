<script setup lang="ts">
import { Close } from "@element-plus/icons-vue";
import { getCurrentWindow } from "@tauri-apps/api/window";

const closeWindow = async () => {
  // 获取当前窗口并关闭它
  const currentWindow = getCurrentWindow();
  await currentWindow.close();
};

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
          title="Close"
        >
          <el-icon :size="16"><Close /></el-icon>
        </button>
      </div>
    </div>

    <!-- 设置内容 -->
    <div class="settings-content">
      <h2>阅读器设置</h2>

      <!-- 这里添加各种设置选项 -->
      <div class="settings-section">
        <h3>外观</h3>
        <!-- 设置项内容 -->
      </div>

      <div class="settings-section">
        <h3>阅读选项</h3>
        <!-- 设置项内容 -->
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background-color: #f5f5f5;
}

.settings-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 16px;
  background-color: #2c3e50;
  color: white;
  -webkit-app-region: drag; /* 让标题栏可拖动 */
}

.title {
  font-size: 18px;
  font-weight: 500;
}

.window-controls {
  display: flex;
  -webkit-app-region: no-drag; /* 控制按钮不可拖动 */
}

.window-control-button {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: transparent;
  border: none;
  border-radius: 4px;
  color: white;
  cursor: pointer;
  margin-left: 4px;
  transition: background-color 0.2s;
}

.window-control-button:hover {
  background-color: rgba(255, 255, 255, 0.2);
}

.close-button:hover {
  background-color: #e81123;
}

.settings-content {
  flex: 1;
  padding: 24px;
  overflow-y: auto;
}

.settings-section {
  background-color: white;
  padding: 16px;
  border-radius: 8px;
  margin-bottom: 16px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

h2 {
  margin-top: 0;
  margin-bottom: 24px;
  color: #2c3e50;
}

h3 {
  margin-top: 0;
  margin-bottom: 16px;
  color: #2c3e50;
}
</style>
