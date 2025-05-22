<script setup lang="ts">
import { ref, onMounted } from "vue";
import { Close, Check } from "@element-plus/icons-vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import type { ReaderStyle } from "../../types/model";

// 字体设置
const fontFamily = ref("");
const fontSize = ref(0);
const lineHeight = ref(0);

// 可选字体列表
const fontOptions = [
  { label: "Noto Serif", value: "Noto Serif" },
  { label: "Microsoft YaHei", value: "Microsoft YaHei" },
  { label: "Source Han Sans CN", value: "Source Han Sans CN" },
  { label: "Noto Sans SC", value: "Noto Sans SC" },
  { label: "Times New Roman", value: "Times New Roman" },
  { label: "Arial", value: "Arial" },
  { label: "Georgia", value: "Georgia" },
];

// 保存设置并关闭窗口
const saveAndClose = async () => {
  try {
    await saveReaderStyle();
    await closeWindow();
  } catch (error) {
    console.error("保存设置失败:", error);
  }
};

// 关闭窗口不保存
const closeWindow = async () => {
  // 获取当前窗口并关闭它
  const currentWindow = getCurrentWindow();
  await currentWindow.close();
};

// 保存阅读器样式
const saveReaderStyle = async () => {
  try {
    await invoke("save_reader_style_command", {
      fontFamily: fontFamily.value,
      fontSize: fontSize.value,
      lineHeight: lineHeight.value,
    });
    console.log("阅读设置已保存");
  } catch (error) {
    console.error("保存阅读设置失败:", error);
    throw error;
  }
};

// 加载保存的阅读器样式
const loadReaderStyle = async () => {
  try {
    const style = await invoke<ReaderStyle>("get_reader_style_command");
    if (style) {
      fontFamily.value = style.font_family;
      fontSize.value = style.font_size;
      lineHeight.value = style.line_height;
      console.log("已加载阅读设置:", style);
    }
  } catch (error) {
    console.error("加载阅读设置失败:", error);
  }
};

// 组件加载时获取保存的阅读设置
onMounted(async () => {
  await loadReaderStyle();
});
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

    <!-- 设置内容 -->
    <div class="settings-content">
      <div class="settings-section">
        <h3>字体设置</h3>

        <!-- 字体选择 -->
        <div class="setting-item">
          <span class="setting-label">字体类型</span>
          <el-select v-model="fontFamily" class="setting-control">
            <el-option
              v-for="option in fontOptions"
              :key="option.value"
              :label="option.label"
              :value="option.value"
            />
          </el-select>
        </div>

        <!-- 字体大小 -->
        <div class="setting-item">
          <span class="setting-label">字体大小</span>
          <div class="slider-with-value">
            <el-slider
              v-model="fontSize"
              :min="12"
              :max="36"
              :step="1"
              class="setting-slider"
            />
            <span class="slider-value">{{ fontSize }}px</span>
          </div>
        </div>

        <!-- 行高 -->
        <div class="setting-item">
          <span class="setting-label">行高</span>
          <div class="slider-with-value">
            <el-slider
              v-model="lineHeight"
              :min="1.0"
              :max="2.5"
              :step="0.1"
              class="setting-slider"
            />
            <span class="slider-value">{{ lineHeight }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- 底部操作栏 -->
    <div class="settings-footer">
      <el-button type="primary" @click="saveAndClose">
        <el-icon><Check /></el-icon>
        保存设置
      </el-button>
    </div>
  </div>
</template>

<style scoped src="./SettingsView.css" />
