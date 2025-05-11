<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { openMarkdownFile } from "../api/markdown.ts";
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
import { renderMarkdown, extractToc } from "../utils/markdown";

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
      markdown.value = result.content;
      // 使用markdown-it渲染Markdown
      htmlContent.value = renderMarkdown(
        result.content,
        theme.value === "dark"
      );
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
  theme.value = theme.value === "light" ? "dark" : "light";
  // 切换主题后重新渲染Markdown以应用新的代码高亮样式
  if (markdown.value) {
    htmlContent.value = renderMarkdown(markdown.value, theme.value === "dark");
  }
};

const toggleToc = () => {
  showToc.value = !showToc.value;
};

onMounted(() => {
  // Welcome message
  const welcomeMarkdown =
    '# 欢迎使用 RBook\n\n这是一个基于 Tauri 和 Vue 开发的 Markdown 阅读器。\n\n## 功能特点\n\n- 支持 Markdown 格式文件阅读\n- 目录导航功能\n- 支持代码高亮\n- 支持明暗主题切换\n- 字体大小调整\n\n## 使用方法\n\n点击左上角的"打开文件"按钮，选择一个 Markdown 文件开始阅读。\n\n```javascript\n// 代码示例\nfunction hello() {\n  console.log("Hello, Markdown!");\n}\n```\n\n## 任务列表示例\n\n- [x] 基础阅读功能\n- [x] 代码高亮\n- [x] 目录导航\n- [ ] 书签功能\n- [ ] 搜索功能';

  markdown.value = welcomeMarkdown;
  htmlContent.value = renderMarkdown(welcomeMarkdown, theme.value === "dark");
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

<style scoped>
.reader-container {
  height: 100vh;
  width: 100%;
  background-color: var(--el-bg-color);
  color: var(--el-text-color-primary);
}

.dark-theme {
  --el-bg-color: #1e1e1e;
  --el-text-color-primary: #ffffff;
  --el-color-primary: #409eff;
  --el-fill-color-blank: #141414;
  --el-mask-color: rgba(0, 0, 0, 0.8);
  --el-border-color: #434343;
  --el-text-color-regular: #d0d0d0;
}

.el-header {
  padding: 10px 20px;
  background-color: var(--el-bg-color-overlay);
  border-bottom: 1px solid var(--el-border-color-light);
  height: auto !important;
}

.controls-col {
  text-align: center;
}

.filename-container {
  display: flex;
  justify-content: flex-end;
}

.main-container {
  height: calc(100vh - 70px);
}

.toc-aside {
  border-right: 1px solid var(--el-border-color-light);
  background-color: var(--el-fill-color-blank);
}

.toc {
  padding: 16px 0;
}

.toc h3 {
  padding: 0 16px 16px;
  margin: 0;
  color: var(--el-text-color-primary);
  font-size: 16px;
  display: flex;
  align-items: center;
  gap: 8px;
}

.toc-menu {
  border-right: none;
}

.toc-menu .el-menu-item {
  height: auto;
  line-height: 1.4;
  padding-top: 8px;
  padding-bottom: 8px;
  min-width: 0;
  text-overflow: ellipsis;
  overflow: hidden;
  white-space: nowrap;
}

.content-main {
  padding: 0;
  background-color: var(--el-bg-color);
}

.content {
  padding: 30px;
  max-width: 900px;
  margin: 0 auto;
}
</style>

<style>
/* 全局样式，用于markdown-it生成的内容 */
.dark-theme pre {
  filter: invert(0) !important;
}

.markdown-content {
  line-height: 1.6;
}

.markdown-content h1,
.markdown-content h2,
.markdown-content h3,
.markdown-content h4,
.markdown-content h5,
.markdown-content h6 {
  margin-top: 24px;
  margin-bottom: 16px;
  font-weight: 600;
  line-height: 1.25;
  color: var(--el-text-color-primary);
}

.markdown-content h1 .markdown-it-permalink,
.markdown-content h2 .markdown-it-permalink,
.markdown-content h3 .markdown-it-permalink,
.markdown-content h4 .markdown-it-permalink,
.markdown-content h5 .markdown-it-permalink,
.markdown-content h6 .markdown-it-permalink {
  opacity: 0;
  transition: opacity 0.2s;
  font-size: 0.8em;
  vertical-align: middle;
  text-decoration: none;
  color: var(--el-text-color-secondary);
  margin-right: 0.5em;
}

.markdown-content h1:hover .markdown-it-permalink,
.markdown-content h2:hover .markdown-it-permalink,
.markdown-content h3:hover .markdown-it-permalink,
.markdown-content h4:hover .markdown-it-permalink,
.markdown-content h5:hover .markdown-it-permalink,
.markdown-content h6:hover .markdown-it-permalink {
  opacity: 1;
}

.markdown-content h1 {
  font-size: 2em;
  border-bottom: 1px solid var(--el-border-color-light);
  padding-bottom: 0.3em;
}

.markdown-content h2 {
  font-size: 1.5em;
  border-bottom: 1px solid var(--el-border-color-light);
  padding-bottom: 0.3em;
}

.markdown-content p {
  margin-top: 0;
  margin-bottom: 16px;
}

.markdown-content a {
  color: var(--el-color-primary);
  text-decoration: none;
}

.markdown-content a:hover {
  text-decoration: underline;
}

.markdown-content code {
  padding: 0.2em 0.4em;
  margin: 0;
  font-size: 85%;
  background-color: var(--el-fill-color-light);
  border-radius: 3px;
  font-family: "SFMono-Regular", Consolas, "Liberation Mono", Menlo, monospace;
}

.markdown-content pre {
  padding: 16px;
  overflow: auto;
  font-size: 85%;
  line-height: 1.45;
  background-color: var(--el-fill-color-light);
  border-radius: 3px;
  margin-bottom: 16px;
}

.markdown-content pre code {
  background-color: transparent;
  padding: 0;
}

.dark-theme .markdown-content pre {
  background-color: var(--el-fill-color-dark);
}

/* 代码高亮样式 - 自定义实现，不依赖highlight.js样式 */
/* 浅色主题 */
.language-js .keyword,
.language-javascript .keyword,
.language-ts .keyword,
.language-typescript .keyword,
.language-java .keyword,
.language-python .keyword,
.language-c .keyword,
.language-cpp .keyword,
.language-csharp .keyword {
  color: #d73a49;
}

.language-js .string,
.language-javascript .string,
.language-ts .string,
.language-typescript .string,
.language-java .string,
.language-python .string,
.language-c .string,
.language-cpp .string,
.language-csharp .string {
  color: #032f62;
}

.language-js .comment,
.language-javascript .comment,
.language-ts .comment,
.language-typescript .comment,
.language-java .comment,
.language-python .comment,
.language-c .comment,
.language-cpp .comment,
.language-csharp .comment {
  color: #6a737d;
  font-style: italic;
}

.language-js .function,
.language-javascript .function,
.language-ts .function,
.language-typescript .function,
.language-java .function,
.language-python .function,
.language-c .function,
.language-cpp .function,
.language-csharp .function {
  color: #6f42c1;
}

.language-js .number,
.language-javascript .number,
.language-ts .number,
.language-typescript .number,
.language-java .number,
.language-python .number,
.language-c .number,
.language-cpp .number,
.language-csharp .number {
  color: #005cc5;
}

/* 暗色主题 */
.dark-theme .language-js .keyword,
.dark-theme .language-javascript .keyword,
.dark-theme .language-ts .keyword,
.dark-theme .language-typescript .keyword,
.dark-theme .language-java .keyword,
.dark-theme .language-python .keyword,
.dark-theme .language-c .keyword,
.dark-theme .language-cpp .keyword,
.dark-theme .language-csharp .keyword {
  color: #c678dd;
}

.dark-theme .language-js .string,
.dark-theme .language-javascript .string,
.dark-theme .language-ts .string,
.dark-theme .language-typescript .string,
.dark-theme .language-java .string,
.dark-theme .language-python .string,
.dark-theme .language-c .string,
.dark-theme .language-cpp .string,
.dark-theme .language-csharp .string {
  color: #98c379;
}

.dark-theme .language-js .comment,
.dark-theme .language-javascript .comment,
.dark-theme .language-ts .comment,
.dark-theme .language-typescript .comment,
.dark-theme .language-java .comment,
.dark-theme .language-python .comment,
.dark-theme .language-c .comment,
.dark-theme .language-cpp .comment,
.dark-theme .language-csharp .comment {
  color: #7f848e;
  font-style: italic;
}

.dark-theme .language-js .function,
.dark-theme .language-javascript .function,
.dark-theme .language-ts .function,
.dark-theme .language-typescript .function,
.dark-theme .language-java .function,
.dark-theme .language-python .function,
.dark-theme .language-c .function,
.dark-theme .language-cpp .function,
.dark-theme .language-csharp .function {
  color: #61afef;
}

.dark-theme .language-js .number,
.dark-theme .language-javascript .number,
.dark-theme .language-ts .number,
.dark-theme .language-typescript .number,
.dark-theme .language-java .number,
.dark-theme .language-python .number,
.dark-theme .language-c .number,
.dark-theme .language-cpp .number,
.dark-theme .language-csharp .number {
  color: #d19a66;
}

.markdown-content blockquote {
  padding: 0 1em;
  color: var(--el-text-color-secondary);
  border-left: 0.25em solid var(--el-border-color);
  margin: 0 0 16px 0;
}

.markdown-content ul,
.markdown-content ol {
  padding-left: 2em;
  margin-top: 0;
  margin-bottom: 16px;
}

.markdown-content img {
  max-width: 100%;
  box-sizing: content-box;
}

.markdown-content table {
  display: block;
  width: 100%;
  overflow: auto;
  border-spacing: 0;
  border-collapse: collapse;
  margin-bottom: 16px;
}

.markdown-content table th,
.markdown-content table td {
  padding: 6px 13px;
  border: 1px solid var(--el-border-color);
}

.markdown-content table tr {
  background-color: var(--el-bg-color);
  border-top: 1px solid var(--el-border-color-light);
}

.markdown-content table tr:nth-child(2n) {
  background-color: var(--el-fill-color-light);
}

/* 任务列表样式 */
.markdown-content .task-list-item {
  list-style-type: none;
  margin-left: -20px;
}

.markdown-content .task-list-item input {
  margin-right: 6px;
}

/* Mermaid图表样式 */
.markdown-content .mermaid {
  text-align: center;
  margin: 16px 0;
}

/* Emoji样式 */
.markdown-content .emoji {
  height: 1.2em;
  vertical-align: middle;
}

.empty-state {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
  color: var(--el-text-color-secondary);
  font-style: italic;
}

/* 代码高亮适配暗色主题 */
.markdown-content pre {
  padding: 16px;
  overflow: auto;
  font-size: 85%;
  line-height: 1.45;
  background-color: #f6f8fa;
  border-radius: 3px;
  margin-bottom: 16px;
}

.markdown-content code {
  font-family: SFMono-Regular, Consolas, Liberation Mono, Menlo, monospace;
  color: #24292e;
  background-color: rgba(27, 31, 35, 0.05);
  padding: 0.2em 0.4em;
  border-radius: 3px;
}

.markdown-content pre code {
  display: block;
  overflow-x: auto;
  background: transparent;
  padding: 0;
  color: #24292e;
}

/* 暗色主题代码高亮样式 */
.dark-theme .markdown-content pre {
  background-color: #282c34;
}

.dark-theme .markdown-content code {
  background-color: rgba(255, 255, 255, 0.1);
  color: #ddd;
}

.dark-theme .markdown-content pre code {
  color: #abb2bf;
}

/* 语法高亮 - 浅色主题 */
.markdown-content .hljs-keyword {
  color: #d73a49;
}

.markdown-content .hljs-string {
  color: #032f62;
}

.markdown-content .hljs-comment {
  color: #6a737d;
  font-style: italic;
}

.markdown-content .hljs-function {
  color: #6f42c1;
}

.markdown-content .hljs-number {
  color: #005cc5;
}

.markdown-content .hljs-built_in {
  color: #e36209;
}

/* 语法高亮 - 暗色主题 */
.dark-theme .markdown-content .hljs-keyword {
  color: #c678dd;
}

.dark-theme .markdown-content .hljs-string {
  color: #98c379;
}

.dark-theme .markdown-content .hljs-comment {
  color: #5c6370;
}

.dark-theme .markdown-content .hljs-function {
  color: #61afef;
}

.dark-theme .markdown-content .hljs-number {
  color: #d19a66;
}

.dark-theme .markdown-content .hljs-built_in {
  color: #e6c07b;
}
</style>
