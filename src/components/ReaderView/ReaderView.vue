<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { openMarkdownFile } from "../../api/markdown.ts";
import { ElMessage } from "element-plus";
import {
  Document,
  Menu as IconMenu,
  Reading,
  Moon,
  Sunny,
  Plus,
  Minus,
} from "@element-plus/icons-vue";
import { renderMarkdown, extractToc } from "../../utils/markdown.ts";
import "../../assets/markdown.css";
import "./ReaderView.css";

const markdown = ref<string>("");
const htmlContent = ref<string>("");
const isLoading = ref<boolean>(false);
const currentFile = ref<string>("");
const fontSize = ref<number>(16);
const theme = ref<string>("light");
const showToc = ref<boolean>(true);

const tableOfContents = computed(() => {
  if (!htmlContent.value) return [];
  return extractToc(htmlContent.value);
});

const openFile = async () => {
  try {
    isLoading.value = true;
    const result = await openMarkdownFile();

    if (result) {
      markdown.value = result.content; // 使用markdown-it渲染Markdown
      htmlContent.value = renderMarkdown(result.content);
      currentFile.value = result.filename;

      ElMessage({
        message: `已成功打开文件: ${result.filename}`,
        type: "success",
      });
    }

    isLoading.value = false;
  } catch (error) {
    console.error("Error opening file:", error);
    ElMessage.error("打开文件失败，请重试");
    isLoading.value = false;
  }
};

const scrollToHeading = (id: string) => {
  const element = document.getElementById(id);
  if (element) {
    element.scrollIntoView({ behavior: "smooth" });
  }
};

const increaseFontSize = () => {
  fontSize.value += 1;
};

const decreaseFontSize = () => {
  if (fontSize.value > 10) {
    fontSize.value -= 1;
  }
};

const toggleTheme = () => {
  // 切换主题
  theme.value = theme.value === "light" ? "dark" : "light";

  // 我们不需要重新渲染Markdown内容，因为我们使用CSS变量和选择器处理样式
  // 但是为了确保主题应用正确，可以添加一个小延迟来应用样式
  setTimeout(() => {
    // 重新应用代码块样式（可选）
    document.querySelectorAll(".markdown-content pre").forEach((pre) => {
      if (theme.value === "dark") {
        pre.classList.add("dark-theme");
      } else {
        pre.classList.remove("dark-theme");
      }
    });
  }, 0);
};

const toggleToc = () => {
  showToc.value = !showToc.value;
};

onMounted(() => {
  // Welcome message
  const welcomeMarkdown =
    '# 欢迎使用 RBook\n\n这是一个基于 Tauri 和 Vue 开发的 Markdown 阅读器。\n\n## 功能特点\n\n- 支持 Markdown 格式文件阅读\n- 目录导航功能\n- 支持代码高亮\n- 支持明暗主题切换\n- 字体大小调整\n\n## 使用方法\n\n点击左上角的"打开文件"按钮，选择一个 Markdown 文件开始阅读。\n\n```javascript\n// 代码示例\nfunction hello() {\n  console.log("Hello, Markdown!");\n}\n```\n\n## 任务列表示例\n\n- [x] 基础阅读功能\n- [x] 代码高亮\n- [x] 目录导航\n- [ ] 书签功能\n- [ ] 搜索功能';

  // 设置初始内容
  markdown.value = welcomeMarkdown;
  htmlContent.value = renderMarkdown(welcomeMarkdown);

  // 检查系统偏好设置
  const prefersDarkScheme = window.matchMedia(
    "(prefers-color-scheme: dark)"
  ).matches;
  if (prefersDarkScheme) {
    theme.value = "dark";
  }

  // 初始化加载完成后应用代码块样式
  setTimeout(() => {
    const codeBlocks = document.querySelectorAll(".markdown-content pre");
    if (theme.value === "dark") {
      codeBlocks.forEach((pre) => pre.classList.add("dark-theme"));
    }
  }, 100);
});
</script>

<template>
  <div class="reader-container" :class="{ 'dark-theme': theme === 'dark' }">
    <el-container>
      <el-header class="el-header">
        <el-row :gutter="20" align="middle" justify="space-between">
          <el-col :span="4">
            <el-button
              type="primary"
              @click="openFile"
              :loading="isLoading"
              :icon="Document"
            >
              打开文件
            </el-button>
          </el-col>

          <el-col :span="12" class="controls-col">
            <el-space>
              <el-tooltip content="目录">
                <el-button
                  :type="showToc ? 'success' : 'info'"
                  @click="toggleToc"
                  circle
                  :icon="IconMenu"
                />
              </el-tooltip>

              <el-tooltip content="减小字体">
                <el-button
                  type="info"
                  @click="decreaseFontSize"
                  circle
                  :icon="Minus"
                />
              </el-tooltip>

              <el-tooltip content="增大字体">
                <el-button
                  type="info"
                  @click="increaseFontSize"
                  circle
                  :icon="Plus"
                />
              </el-tooltip>

              <el-tooltip
                :content="
                  theme === 'light' ? '切换到暗色模式' : '切换到亮色模式'
                "
              >
                <el-button
                  type="warning"
                  @click="toggleTheme"
                  circle
                  :icon="theme === 'light' ? Moon : Sunny"
                />
              </el-tooltip>
            </el-space>
          </el-col>

          <el-col :span="8" v-if="currentFile">
            <div class="filename-container">
              <el-tag type="info" size="large">{{ currentFile }}</el-tag>
            </div>
          </el-col>
        </el-row>
      </el-header>

      <el-container class="main-container">
        <el-aside
          width="250px"
          v-if="showToc && tableOfContents.length > 0"
          class="toc-aside"
        >
          <div class="toc">
            <h3>
              <el-icon><Reading /></el-icon> 目录
            </h3>
            <el-scrollbar height="calc(100vh - 120px)">
              <el-menu class="toc-menu">
                <el-menu-item
                  v-for="(item, index) in tableOfContents"
                  :key="index"
                  @click="scrollToHeading(item.id)"
                  :style="{
                    paddingLeft: `${item.level * 12 + 12}px`,
                  }"
                >
                  {{ item.text }}
                </el-menu-item>
              </el-menu>
            </el-scrollbar>
          </div>
        </el-aside>

        <el-main class="content-main">
          <el-scrollbar height="calc(100vh - 70px)">
            <div class="content" :style="{ fontSize: `${fontSize}px` }">
              <div
                v-if="htmlContent"
                class="markdown-content"
                v-html="htmlContent"
              ></div>
              <div v-else class="empty-state">
                <el-empty description="暂无内容，请打开一个Markdown文件">
                  <el-button type="primary" @click="openFile"
                    >打开文件</el-button
                  >
                </el-empty>
              </div>
            </div>
          </el-scrollbar>
        </el-main>
      </el-container>
    </el-container>
  </div>
</template>
