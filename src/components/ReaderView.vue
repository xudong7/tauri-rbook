<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { Window } from "@tauri-apps/api/window";
import "./ReaderView.css";
import { getEpubHtmlWithImages, HtmlWithImages } from "../api";
import { Document, Minus, FullScreen, Close } from "@element-plus/icons-vue";

const currentContent = ref<string>("");
const leftColumnContent = ref<string>("");
const rightColumnContent = ref<string>("");
const loading = ref<boolean>(false);
const filePath = ref<string>("");
const htmlWithImages = ref<HtmlWithImages | null>(null);
const appWindow = Window.getCurrent();
const lastWindowSize = ref<{ width: number; height: number }>({
  width: 0,
  height: 0,
});
const resizeTimeout = ref<number | null>(null);

// 监听窗口大小变化，以重新布局页面内容
const handleWindowResize = () => {
  // 使用防抖，避免频繁重新计算
  if (resizeTimeout.value !== null) {
    clearTimeout(resizeTimeout.value);
  }

  resizeTimeout.value = window.setTimeout(() => {
    const currentWidth = window.innerWidth;
    const currentHeight = window.innerHeight;

    // 如果窗口大小变化超过一定阈值，则重新计算页面布局
    if (
      Math.abs(currentWidth - lastWindowSize.value.width) > 50 ||
      Math.abs(currentHeight - lastWindowSize.value.height) > 50
    ) {
      lastWindowSize.value = { width: currentWidth, height: currentHeight };

      // 如果当前有内容，则重新分割页面
      if (htmlWithImages.value) {
        processHtmlContent();
      }
    }

    resizeTimeout.value = null;
  }, 300);
};

// 组件挂载和卸载时添加/移除窗口大小变化监听
onMounted(() => {
  lastWindowSize.value = {
    width: window.innerWidth,
    height: window.innerHeight,
  };
  window.addEventListener("resize", handleWindowResize);
});

onUnmounted(() => {
  window.removeEventListener("resize", handleWindowResize);
  if (resizeTimeout.value !== null) {
    clearTimeout(resizeTimeout.value);
  }
});

// 打开并转换EPUB为HTML
const openAndConvertEpub = async () => {
  try {
    loading.value = true;

    // Open file dialog
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "EPUB",
          extensions: ["epub"],
        },
      ],
    });

    if (!selected || Array.isArray(selected)) {
      loading.value = false;
      return;
    }

    filePath.value = selected;

    // 调用后端API获取HTML和图片
    htmlWithImages.value = await getEpubHtmlWithImages(filePath.value);

    // 处理HTML内容和图片
    processHtmlContent();

    loading.value = false;
  } catch (error) {
    console.error("Error converting EPUB to HTML:", error);
    loading.value = false;
  }
};

// 处理HTML内容和图片
const processHtmlContent = () => {
  if (!htmlWithImages.value) return;

  let html = htmlWithImages.value.html_content;
  const images = htmlWithImages.value.images;

  // 为每个图片创建data URL并替换HTML中的图片路径
  images.forEach((img) => {
    const dataUrl = `data:${img.mime_type};base64,${img.content}`;
    // 替换HTML中的图片路径为data URL
    html = html.replace(
      new RegExp(`src=['"]${img.path}['"]`, "g"),
      `src="${dataUrl}"`
    );
  });

  currentContent.value = html;

  // 分割内容到左右栏
  splitContentForTwoColumns(html);
};

// 页面状态变量
const currentPage = ref<number>(0);
const totalPages = ref<number>(0);
const allPages = ref<string[]>([]);
// 添加基础样式和双页布局所需的CSS
const styleForBookPages = `    <style>
      body {
        margin: 0;
        padding: 15px 20px;
        font-family: Arial, sans-serif;
        font-size: 16px;
        line-height: 1.6;
        overflow: hidden; /* 防止内容溢出 */
        height: 100%; /* 高度填满 iframe */
        box-sizing: border-box;
        background-color: #fff9ee; /* 添加暖色背景 */
      }
      img {
        max-width: 100%;
        height: auto;
        display: block;
        margin: 10px auto;
        border: 1px solid #ddd;
        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        object-fit: contain; /* 确保图片保持比例 */
        max-height: 75vh; /* 限制图片高度为视口高度的75% */
      }
      h1, h2, h3, h4, h5, h6 {
        margin-top: 0.8em;
        margin-bottom: 0.4em;
        color: #5a4b3e; /* 标题使用棕色 */
        font-family: 'Times New Roman', serif;
      }
      h1 { font-size: 1.6em; }
      h2 { font-size: 1.4em; }
      h3 { font-size: 1.2em; }
      h4, h5, h6 { font-size: 1em; }
      p {
        margin-top: 0;
        margin-bottom: 0.8em;
        text-align: justify;
        text-indent: 2em; /* 段落首行缩进 */
      }
      .page-content {
        height: calc(100vh - 40px); /* 减去边距 */
        overflow: hidden; /* 不允许滚动，确保内容适合页面 */
        padding: 0;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
      }
      /* 图片适应页面大小的特殊处理 */
      .full-page-image {
        max-height: calc(100vh - 80px);
        max-width: 100%;
        margin: 0 auto;
        display: flex;
        align-items: center;
        justify-content: center;
      }
      .full-page-image img {
        max-height: 100%;
        max-width: 100%;
        object-fit: contain;
      }
      /* 响应式字体大小 */
      @media (max-height: 600px) {
        body {
          font-size: 14px;
        }
        h1 { font-size: 1.4em; }
        h2 { font-size: 1.3em; }
        h3 { font-size: 1.1em; }
      }
    </style>
  `;

// 将HTML文档分割成页面大小的块，根据内容动态适配页面
const extractPages = (htmlContent: string) => {
  // 提取原HTML的head部分
  let headContent = "";
  const headMatch = htmlContent.match(/<head[^>]*>([\s\S]*?)<\/head>/i);
  if (headMatch) {
    headContent = headMatch[1];
  }

  // 提取body中的内容
  let bodyContent = "";
  const bodyMatch = htmlContent.match(/<body[^>]*>([\s\S]*?)<\/body>/i);
  if (bodyMatch) {
    bodyContent = bodyMatch[1];
  } else {
    // 如果没有找到body标签，使用整个HTML
    bodyContent = htmlContent;
  }

  // 使用正则表达式查找段落、标题、图片和其他块级HTML元素
  const elements = [];
  let match;
  const regex =
    /<(p|h[1-6]|div|section|article|aside|blockquote|figure|table)[^>]*>([\s\S]*?)<\/\1>|<(img|hr)[^>]*\/?>/gi;

  while ((match = regex.exec(bodyContent)) !== null) {
    // 检查是否是图片元素
    if (match[0].startsWith("<img") || (match[3] && match[3] === "img")) {
      // 为图片添加特殊标记，以便后续处理
      elements.push({
        content: match[0],
        type: "image",
        estimatedHeight: 400, // 图片估计高度，可根据实际情况调整
      });
    } else {
      // 其他元素
      elements.push({
        content: match[0],
        type: "text",
        // 估算元素的高度，根据内容长度和类型
        estimatedHeight: calculateEstimatedHeight(match[0]),
      });
    }
  }

  // 如果没有找到元素，尝试使用简单的文本分割
  if (elements.length === 0) {
    // 尝试分割文本段落
    const textBlocks = bodyContent.split(/\n\s*\n/);
    if (textBlocks.length > 0) {
      elements.push(
        ...textBlocks.map((block) => ({
          content: `<p>${block.trim()}</p>`,
          type: "text",
          estimatedHeight: 50 + block.length / 20, // 简单估算段落高度
        }))
      );
    } else {
      // 如果仍然没有内容，直接作为一个段落
      elements.push({
        content: `<p>${bodyContent}</p>`,
        type: "text",
        estimatedHeight: 50 + bodyContent.length / 20,
      });
    }
  }

  // 计算视窗高度，作为每页内容的高度限制
  const viewportHeight = window.innerHeight - 80; // 减去页面上下边距和控制区域

  // 生成页面
  const pages = [];
  let currentPageElements = [];
  let currentPageHeight = 0;

  // 处理图片元素 - 如果图片较大，单独分配一页
  for (let i = 0; i < elements.length; i++) {
    const element = elements[i];

    // 如果是图片元素且估计较大，则单独分配一页
    if (
      element.type === "image" &&
      element.estimatedHeight > viewportHeight * 0.5
    ) {
      // 如果当前页已经有内容，先完成当前页
      if (currentPageElements.length > 0) {
        pages.push(createPageHtml(currentPageElements, headContent));
        currentPageElements = [];
        currentPageHeight = 0;
      }

      // 创建只包含图片的页面
      pages.push(
        createPageHtml(
          [
            {
              content: element.content,
              type: "image",
              estimatedHeight: element.estimatedHeight,
            },
          ],
          headContent,
          true
        ) // true表示这是一个图片页
      );
    } else {
      // 检查添加此元素后是否会超过页面高度
      if (
        currentPageHeight + element.estimatedHeight > viewportHeight &&
        currentPageElements.length > 0
      ) {
        // 当前页已满，创建新页
        pages.push(createPageHtml(currentPageElements, headContent));
        currentPageElements = [];
        currentPageHeight = 0;
      }

      // 添加元素到当前页
      currentPageElements.push(element);
      currentPageHeight += element.estimatedHeight;
    }
  }

  // 将剩余元素添加到最后一页
  if (currentPageElements.length > 0) {
    pages.push(createPageHtml(currentPageElements, headContent));
  }

  // 确保至少有一页
  if (pages.length === 0) {
    pages.push(
      createPageHtml(
        [
          {
            content: `<p>内容为空</p>`,
            type: "text",
            estimatedHeight: 50,
          },
        ],
        headContent
      )
    );
  }

  return pages;
};

// 帮助函数：估算元素的显示高度
const calculateEstimatedHeight = (elementHtml: string): number => {
  // 根据元素类型和内容长度估算高度
  if (elementHtml.startsWith("<h1")) return 70;
  if (elementHtml.startsWith("<h2")) return 60;
  if (elementHtml.startsWith("<h3")) return 55;
  if (
    elementHtml.startsWith("<h4") ||
    elementHtml.startsWith("<h5") ||
    elementHtml.startsWith("<h6")
  )
    return 50;
  if (elementHtml.startsWith("<p")) {
    // 根据段落文本长度估算高度
    const textLength = elementHtml.length;
    return 40 + textLength / 20; // 假设每20个字符增加1行
  }
  if (
    elementHtml.startsWith("<div") ||
    elementHtml.startsWith("<section") ||
    elementHtml.startsWith("<article")
  ) {
    return 100 + elementHtml.length / 15;
  }
  if (elementHtml.startsWith("<blockquote"))
    return 80 + elementHtml.length / 25;
  if (elementHtml.startsWith("<table")) return 150 + elementHtml.length / 10;

  // 默认高度
  return 50 + elementHtml.length / 30;
};

// 帮助函数：创建一个页面的HTML
const createPageHtml = (
  elements: Array<{
    content: string;
    type: string;
    estimatedHeight: number;
  }>,
  headContent: string,
  isImagePage: boolean = false
) => {
  // 如果是图片页，添加特殊样式使图片居中且适合页面
  const imageStyles = isImagePage
    ? `
          <style>
            .image-container {
              display: flex;
              justify-content: center;
              align-items: center;
              height: 100%;
              padding: 10px;
              box-sizing: border-box;
              overflow: hidden;
            }
            .image-container img {
              max-width: 90%;
              max-height: 90%;
              object-fit: contain;
              margin: 0 auto;
              display: block;
              box-shadow: none;
              border: none;
            }
          </style>
        `
    : "";

  const pageContent = isImagePage
    ? `<div class="image-container">${elements
        .map((e) => e.content)
        .join("")}</div>`
    : elements.map((e) => e.content).join("");

  return `
        <!DOCTYPE html>
        <html>
          <head>
            ${headContent || ""}
            ${styleForBookPages}
            ${imageStyles}
          </head>
          <body>
            <div class="page-content">
              ${pageContent}
            </div>
          </body>
        </html>
      `;
};

// 分割HTML内容到左右两栏，模拟书籍页面
const splitContentForTwoColumns = (html: string) => {
  try {
    // 生成所有页面
    allPages.value = extractPages(html);
    totalPages.value = allPages.value.length;
    currentPage.value = 0;

    // 初始显示第一页和第二页（如果有的话）
    updateVisiblePages();
  } catch (error) {
    console.error("Error creating pages:", error);

    // 如果遇到错误，确保至少显示一些内容
    const simplePageTemplate = `
      <!DOCTYPE html>
      <html>
        <head>
          ${styleForBookPages}
        </head>
        <body>
          <div class="page-content">
            PAGE_CONTENT
          </div>
        </body>
      </html>
    `;

    // 简单地将HTML内容分为两半
    const cleanedHtml = html
      .replace(/<head>[\s\S]*?<\/head>/i, "")
      .replace(/<\/?html>/gi, "")
      .replace(/<\/?body>/gi, "");
    const halfPoint = Math.ceil(cleanedHtml.length / 2);

    leftColumnContent.value = simplePageTemplate.replace(
      "PAGE_CONTENT",
      `<h2>页面 1</h2><div>${cleanedHtml.substring(0, halfPoint)}</div>`
    );
    rightColumnContent.value = simplePageTemplate.replace(
      "PAGE_CONTENT",
      `<h2>页面 2</h2><div>${cleanedHtml.substring(halfPoint)}</div>`
    );
  }

  // 保留原始内容
  currentContent.value = html;
};

// 更新当前可见的页面
const updateVisiblePages = () => {
  // 确保页码有效
  if (currentPage.value < 0) {
    currentPage.value = 0;
  }

  if (currentPage.value >= totalPages.value - 1) {
    currentPage.value = Math.max(0, totalPages.value - 2);
  }

  // 如果是奇数页，确保当前页是偶数（左页显示偶数页，右页显示奇数页）
  if (currentPage.value % 2 !== 0) {
    currentPage.value = Math.max(0, currentPage.value - 1);
  }

  // 设置左右页面内容
  leftColumnContent.value = allPages.value[currentPage.value] || "";
  rightColumnContent.value = allPages.value[currentPage.value + 1] || "";
};

// 翻页方法
const goToNextPage = () => {
  if (currentPage.value + 2 < totalPages.value) {
    currentPage.value += 2;
    updateVisiblePages();
  }
};

const goToPreviousPage = () => {
  if (currentPage.value > 0) {
    currentPage.value -= 2;
    updateVisiblePages();
  }
};

// Add window control functions
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
</script>

<template>
  <div class="reader-container">
    <!-- Toolbar -->
    <div class="reader-toolbar">
      <div class="left-controls">
        <button
          class="icon-button"
          @click="openAndConvertEpub"
          :disabled="loading"
          title="Open EPUB as HTML"
        >
          <el-icon :size="20" v-if="!loading"><Document /></el-icon>
          <span v-else class="loading-spinner"></span>
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

    <!-- Placeholder -->
    <div v-if="!currentContent" class="placeholder">
      <div class="instruction">Please open an EPUB file to view as HTML</div>
    </div>

    <!-- HTML Viewer - 书籍双页式布局 -->
    <div v-if="currentContent" class="html-view-container">
      <div class="book-controls">
        <button
          class="page-button prev-button"
          @click="goToPreviousPage"
          :disabled="currentPage <= 0"
          title="上一页"
        >
          « 上一页
        </button>
        <div class="page-indicator">
          第 {{ currentPage + 1 }}-{{
            Math.min(currentPage + 2, totalPages)
          }}
          页，共 {{ totalPages }} 页
        </div>
        <button
          class="page-button next-button"
          @click="goToNextPage"
          :disabled="currentPage + 2 >= totalPages"
          title="下一页"
        >
          下一页 »
        </button>
      </div>
      <div class="resize-hint" v-if="resizeTimeout !== null">
        <div class="hint-text">重新布局内容中...</div>
      </div>

      <div class="two-column-layout">
        <div class="column left-column">
          <iframe
            :srcdoc="leftColumnContent"
            class="html-iframe"
            frameborder="0"
          ></iframe>
        </div>
        <div class="column right-column">
          <iframe
            :srcdoc="rightColumnContent"
            class="html-iframe"
            frameborder="0"
          ></iframe>
        </div>
      </div>
    </div>
  </div>
</template>
