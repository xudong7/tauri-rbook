<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { Close, Check } from "@element-plus/icons-vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import {
  saveStyleToLocal,
  loadStyleFromLocal,
  generateStyle,
} from "../../utils/ReaderViewUtil";
import { ElMessage } from "element-plus";

const closeWindow = async () => {
  // 获取当前窗口并关闭它
  const currentWindow = getCurrentWindow();
  await currentWindow.close();
};

// 阅读设置相关变量
const fontFamily = ref("");
const fontSize = ref();
const lineHeight = ref();

// 可选字体和字号
const fontFamilyOptions = [
  { label: "Noto Serif", value: "Noto Serif" },
  { label: "宋体", value: "SimSun" },
  { label: "仿宋", value: "FangSong" },
  { label: "楷体", value: "KaiTi" },
  { label: "微软雅黑", value: "Microsoft YaHei" },
  { label: "Times New Roman", value: "Times New Roman" },
  { label: "Arial", value: "Arial" },
];
const fontSizeOptions = [14, 16, 18, 20, 22, 24, 28, 32];

// 预览文本
const previewText = ref(`
我祈祷拥有一颗透明的心灵，和会流泪的眼睛。
给我再去相信的勇气，越过谎言去拥抱你。

每当我找不到存在的意义，每当我迷失在黑夜里。
夜空中最亮的星，请指引我靠近你。
`);

// 样式预览
const previewStyle = ref("");

// 更新样式预览
const updatePreview = () => {
  previewStyle.value = generateStyle(
    fontFamily.value,
    fontSize.value,
    lineHeight.value
  );
};

// 保存样式设置
const saveSettings = async () => {
  // 保存字体、字号和行高
  await saveStyleToLocal(fontFamily.value, fontSize.value, lineHeight.value);

  ElMessage({
    message: "设置已保存",
    type: "success",
  });
};

// 加载已保存的样式设置
const loadSettings = async () => {
  try {
    const style = await loadStyleFromLocal();
    fontFamily.value = style.font_family;
    fontSize.value = style.font_size;
    lineHeight.value = style.line_height;
    updatePreview();
  } catch (error) {
    console.error("Error loading style settings:", error);
  }
};

// 组件加载时获取保存的阅读设置
onMounted(() => {
  loadSettings();
});

// 监听设置变化更新预览
watch([fontFamily, fontSize, lineHeight], () => {
  updatePreview();
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
        <!-- 字体选择 -->
        <div class="setting-item">
          <span class="setting-label">字体</span>
          <div class="setting-control">
            <el-select
              v-model="fontFamily"
              placeholder="选择字体"
              class="full-width"
              popper-class="font-select-dropdown"
            >
              <el-option
                v-for="option in fontFamilyOptions"
                :key="option.value"
                :label="option.label"
                :value="option.value"
                :style="{ fontFamily: option.value }"
              >
                {{ option.label }}
              </el-option>
            </el-select>
          </div>
        </div>
        <!-- 字号选择 -->
        <div class="setting-item">
          <span class="setting-label">字号</span>
          <div class="setting-control">
            <el-radio-group v-model="fontSize" class="font-size-group">
              <el-radio-button
                v-for="size in fontSizeOptions"
                :key="size"
                :label="size"
              >
                {{ size }}
              </el-radio-button>
            </el-radio-group>
          </div>
        </div>

        <!-- 行高选择 -->
        <div class="setting-item">
          <span class="setting-label">行高</span>
          <div class="setting-control">
            <el-slider
              v-model="lineHeight"
              :min="1.0"
              :max="2.0"
              :step="0.1"
              :marks="{ 1: '1.0', 1.5: '1.5', 2: '2.0' }"
              class="full-width"
            />
          </div>
        </div>
      </div>

      <!-- 预览区域 -->
      <div class="settings-section preview-section">
        <h3>预览效果</h3>
        <div class="preview-container">
          <div
            class="preview-content"
            :style="{
              fontFamily: fontFamily,
              fontSize: `${fontSize}px`,
              lineHeight: lineHeight,
            }"
          >
            <p
              v-for="(line, index) in previewText.trim().split('\n\n')"
              :key="index"
              class="preview-paragraph"
            >
              {{ line }}
            </p>
          </div>
        </div>
      </div>

      <!-- 保存按钮 -->
      <div class="settings-actions">
        <el-button type="primary" @click="saveSettings" class="save-button">
          <el-icon class="save-icon"><Check /></el-icon>
          保存
        </el-button>
      </div>
    </div>
  </div>
</template>

<style scoped src="./SettingsView.css" />
