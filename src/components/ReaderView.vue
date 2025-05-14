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

// 添加分页相关的响应式变量
const currentPage = ref<number>(0);
const totalPages = ref<number>(0);
const allPages = ref<string[]>([]);

const noScrollStyle = `<style>html,body{overflow:hidden!important;}body::-webkit-scrollbar{display:none!important;}</style>`;

const PAGE_PADDING = 40; // px

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

    // 跳转到第一页
    currentPage.value = 0;

    // 处理HTML内容和图片
    processHtmlContent();

    loading.value = false;
  } catch (error) {
    loading.value = false;
  }
};

// 处理HTML内容和图片
const processHtmlContent = async () => {
  if (!htmlWithImages.value) return;
  let html = htmlWithImages.value.html_content;
  const images = htmlWithImages.value.images;
  images.forEach((img) => {
    const dataUrl = `data:${img.mime_type};base64,${img.content}`;
    html = html.replace(
      new RegExp(`src=['"]${img.path}['"]`, "g"),
      `src=\"${dataUrl}\"`
    );
  });
  currentContent.value = html;
  await splitContentForTwoColumns(html);
};

// 分割内容到左右栏
const splitContentForTwoColumns = async (html: string) => {
  const tempDiv = document.createElement("div");
  tempDiv.innerHTML = html;
  const elements = Array.from(tempDiv.children);
  const pageHeight = window.innerHeight * 0.85 - PAGE_PADDING * 2;
  const pageWidth = (window.innerWidth - 60) / 2 - PAGE_PADDING * 2;
  let currentPageContent = "";
  let currentHeight = 0;
  allPages.value = [];
  const measureContainer = document.createElement("div");
  measureContainer.style.width = `${pageWidth}px`;
  measureContainer.style.position = "absolute";
  measureContainer.style.visibility = "hidden";
  measureContainer.style.padding = `${PAGE_PADDING}px`;
  measureContainer.style.boxSizing = "border-box";
  document.body.appendChild(measureContainer);
  const pageContainer = document.createElement("div");
  pageContainer.style.width = `${pageWidth}px`;
  pageContainer.style.padding = `${PAGE_PADDING}px`;
  pageContainer.style.boxSizing = "border-box";
  measureContainer.appendChild(pageContainer);

  const processElement = async (element: Element) => {
    if (element.tagName === "IMG") {
      // 预加载图片，获取原始宽高
      const imgSrc = (element as HTMLImageElement).src;
      const img = new window.Image();
      img.src = imgSrc;
      await new Promise((resolve) => {
        if (img.complete) resolve(true);
        img.onload = () => resolve(true);
        img.onerror = () => resolve(true);
      });
      // 只按宽度等比例缩放图片，保证图片宽度不超过页面宽度
      const scale = Math.min(pageWidth / img.naturalWidth, pageHeight / img.naturalHeight, 1);
      const displayWidth = img.naturalWidth * scale;
      const displayHeight = img.naturalHeight * scale;
      const imgHtml = `<div style="display:flex;align-items:center;justify-content:center;width:100%;height:${pageHeight}px;padding:${PAGE_PADDING}px;box-sizing:border-box;"><img src="${imgSrc}" style="width:${displayWidth}px;height:${displayHeight}px;object-fit:contain;display:block;margin:auto;" /></div>`;
      allPages.value.push(noScrollStyle + imgHtml);
      pageContainer.innerHTML = "";
      currentPageContent = "";
      currentHeight = 0;
      return true;
    }
    // 非图片内容，按段落分割分页
    const paragraphs = element.outerHTML.match(/<p[\s\S]*?<\/p>/g) || [
      element.outerHTML,
    ];
    for (const paragraph of paragraphs) {
      pageContainer.innerHTML = currentPageContent + paragraph;
      const elementHeight = pageContainer.clientHeight;
      if (elementHeight > pageHeight) {
        if (currentPageContent) {
          allPages.value.push(noScrollStyle + currentPageContent);
        }
        currentPageContent = paragraph;
      } else {
        currentPageContent += paragraph;
      }
    }
    pageContainer.innerHTML = "";
    return true;
  };

  for (const element of elements) {
    // 依次处理每个元素，图片异步处理
    // eslint-disable-next-line no-await-in-loop
    await processElement(element);
  }

  if (currentPageContent) {
    allPages.value.push(noScrollStyle + currentPageContent);
  }
  document.body.removeChild(measureContainer);
  totalPages.value = allPages.value.length;
  updateVisiblePages();
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
        <div v-if="currentContent" class="nav-controls">
          <button
            class="page-button-inline prev-button"
            @click="goToPreviousPage"
            :disabled="currentPage <= 0"
            title="上一页"
          >
            « 上一页
          </button>
          <div class="page-indicator-inline">
            {{ currentPage + 1 }}-{{ Math.min(currentPage + 2, totalPages) }}/{{
              totalPages
            }}
          </div>
          <button
            class="page-button-inline next-button"
            @click="goToNextPage"
            :disabled="currentPage + 2 >= totalPages"
            title="下一页"
          >
            下一页 »
          </button>
        </div>
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
