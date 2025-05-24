<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { Close, Check } from "@element-plus/icons-vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import type { ReaderStyle } from "../../types/model";
import { themeManager, type Theme } from "../../utils/themeManager";

// 字体设置
const fontFamily = ref("");
const fontSize = ref(0);
const lineHeight = ref(0);
const theme = ref<Theme>("light");

// 可选字体列表
const fontOptions = [
  { label: "Noto Serif", value: "Noto Serif" },
  { label: "Microsoft YaHei", value: "Microsoft YaHei" },
  { label: "Source Han Sans CN", value: "Source Han Sans CN" },
  { label: "Noto Sans SC", value: "Noto Sans SC" },
  { label: "Times New Roman", value: "Times New Roman" },
  { label: "Arial", value: "Arial" },
  { label: "Georgia", value: "Georgia" },
  { label: "仿宋", value: "FangSong" },
  { label: "楷体", value: "KaiTi" },
];

// 主题选项
const themeOptions = themeManager.getThemeOptions();

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
      theme: theme.value,
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
      if (style.theme) {
        theme.value = style.theme as Theme;
        themeManager.setTheme(style.theme as Theme);
      }
      console.log("已加载阅读设置:", style);
    }
  } catch (error) {
    console.error("加载阅读设置失败:", error);
  }
};

// 监听主题变化并实时应用
watch(theme, async (newTheme) => {
  // 立即应用主题到当前设置窗口
  themeManager.setTheme(newTheme);

  // 同时保存主题设置，确保主窗口也能同步更新
  try {
    // 获取当前的阅读器样式设置
    const currentStyle = await invoke<ReaderStyle>("get_reader_style_command");

    // 保存更新后的样式（包含新主题）
    await invoke("save_reader_style_command", {
      fontFamily: currentStyle.font_family || fontFamily.value,
      fontSize: currentStyle.font_size || fontSize.value,
      lineHeight: currentStyle.line_height || lineHeight.value,
      theme: newTheme,
    });

    console.log(`主题已切换到 ${newTheme} 并立即保存`);
  } catch (error) {
    console.error("实时保存主题设置失败:", error);
  }
});

// 组件加载时获取保存的阅读设置
onMounted(async () => {
  // 初始化主题
  theme.value = themeManager.getCurrentTheme();
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
      <!-- 主题设置 -->
      <div class="settings-section">
        <h3>主题设置</h3>

        <!-- 主题选择 -->
        <div class="setting-item">
          <span class="setting-label">颜色模式</span>
          <el-select v-model="theme" class="setting-control">
            <el-option
              v-for="option in themeOptions"
              :key="option.value"
              :label="option.label"
              :value="option.value"
            />
          </el-select>
        </div>

        <!-- 主题预览 -->
        <div class="setting-item preview-item">
          <span class="setting-label">主题预览</span>
          <div class="theme-preview">
            <div class="theme-sample" :class="`theme-${theme}`">
              <div class="sample-header">标题栏</div>
              <div class="sample-content">
                <p>这是一段示例文本，展示当前主题的效果。</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 字体设置 -->
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
        <!-- 字体样式预览 -->
        <div class="setting-item preview-item">
          <span class="setting-label">样式预览</span>
          <div
            class="font-preview"
            :style="{
              fontFamily: fontFamily,
              fontSize: `${fontSize}px`,
              lineHeight: lineHeight,
            }"
          >
            <p>千里之行，始于足下。不积跬步，无以至千里。</p>
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
