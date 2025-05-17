<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from "vue";
import { Window } from "@tauri-apps/api/window";
import "./ReaderView.css";
import { getEpubHtmlWithImages, HtmlWithImages } from "../api";
import {
  ArrowLeft,
  ArrowRight,
  Minus,
  FullScreen,
  Close,
} from "@element-plus/icons-vue";

// Props and emits
const props = defineProps<{
  initialFilePath?: string;
}>();

const emit = defineEmits<{
  back: [];
}>();

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

// Function to load a book from a specified path
const loadBookFromPath = async (path: string) => {
  try {
    loading.value = true;
    filePath.value = path;

    // 调用后端API获取HTML和图片
    htmlWithImages.value = await getEpubHtmlWithImages(path);

    // 跳转到第一页
    currentPage.value = 0;

    // 处理HTML内容和图片
    processHtmlContent();

    loading.value = false;
  } catch (error) {
    console.error("Error loading book:", error);
    loading.value = false;
  }
};

// Watch for initialFilePath changes to load book
watch(
  () => props.initialFilePath,
  (newPath) => {
    if (newPath) {
      loadBookFromPath(newPath);
    }
  },
  { immediate: true }
);

// Go back to menu
const goBackToMenu = () => {
  emit("back");
};

const noScrollStyle = `<style>
  html, body { overflow: hidden!important; margin: 20px; padding: 0; }
  body {
    font-family: 'Noto Serif', 'Times New Roman', serif!important;
    font-size: 18px!important;
    line-height: 1.4!important;
    color: #333!important;
    padding: 0!important;
    box-sizing: border-box!important;
  }  
  p {
    margin: 0.2em 0 0.2em 0!important;
    text-indent: 1em!important;
  }
  p:has(svg) {
    padding: 40px!important;
  }
  p:has(img) {
    padding: 10px!important;
  }
  p:has(a) {
    margin: 0!important;
    line-height: 1.2!important;
  }
  p:has(br) {
    line-height: 1!important;
  }
  h1, h2, h3, h4, h5 {
    margin: 0.2em 0 0.2em!important;
    font-weight: bold!important;
  }  
  img {
    width: auto!important;
    height: auto!important;
    max-width: 85%!important;
    max-height: 60%!important;
    object-fit: contain;
    display: block;
    margin: 0.5em auto 0.5em auto!important;
  }
  svg {
    width: auto!important;
    height: auto!important;
    max-width: 85%!important;
    max-height: 60%!important;
    object-fit: contain;
    display: block;
    margin: 0.5em auto 0 auto!important;
  }
  image {
    width: auto!important;
    height: auto!important;
    max-width: 85%!important;
    max-height: 60%!important;
    object-fit: contain;
    display: block;
    margin: 0.5em auto 0 auto!important;
  }  
  a {
    color: inherit!important;
    text-decoration: none!important;
    pointer-events: none!important;
    cursor: default!important;
    line-height: 1!important;
    padding: 0!important;
    display: inline-block!important;
  }
</style>`;

const PAGE_PADDING = 90; // px

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
  // 减少有效页面高度，确保内容不会被遮挡
  const pageHeight = window.innerHeight - PAGE_PADDING * 2;
  const pageWidth = window.innerWidth / 2;
  let currentPageContent = "";
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
    const paragraphs = element.outerHTML.match(
      /<p[\s\S]*?<\/p>|<img[\s\S]*?(?:>|<\/img>)/g
    ) || [element.outerHTML];
    for (const paragraph of paragraphs) {
      // 检查是否是图片元素
      const isImage = paragraph.includes("<img");

      // 对图片做特殊处理
      if (isImage && currentPageContent.trim() !== "") {
        // 先测量当前内容高度
        pageContainer.innerHTML = currentPageContent;
        const currentHeight = pageContainer.clientHeight;

        // 测量当前图片高度
        pageContainer.innerHTML = paragraph;
        const imageHeight = pageContainer.clientHeight;

        // 再测量当前内容加上图片的高度
        pageContainer.innerHTML = currentPageContent + paragraph;
        const totalHeight = pageContainer.clientHeight;

        // 如果图片高度占据页面高度的50%以上，或者添加图片后超出页面高度，则开始新页面
        const imageHeightRatio = imageHeight / pageHeight;
        if (
          (imageHeightRatio > 0.5 && currentHeight > pageHeight * 0.5) ||
          totalHeight > pageHeight * 0.8
        ) {
          allPages.value.push(noScrollStyle + currentPageContent);
          currentPageContent = "";
        } else {
          // 恢复只有当前内容的状态，后面会正常添加图片
          pageContainer.innerHTML = currentPageContent;
        }
      } // 添加内容并检查高度
      pageContainer.innerHTML = currentPageContent + paragraph;
      if (pageContainer.clientHeight > pageHeight) {
        if (currentPageContent) {
          allPages.value.push(noScrollStyle + currentPageContent);
        }
        currentPageContent = paragraph;
        pageContainer.innerHTML = currentPageContent; // 如果单个段落或图片本身就超出一页，强制分页
        if (pageContainer.clientHeight > pageHeight) {
          // 对于超大图片，调整其大小限制以确保完整显示
          if (isImage) {
            // 不使用溢出隐藏，而是缩放图片到适合的大小
            const wrappedImage = `<div style="display:flex; align-items:center; justify-content:center;">
              <img style="max-width:100%; max-height:100%; object-fit:contain;" 
              src=${
                paragraph.match(/src=["']([^"']*)["']/)?.[1] || ""
              } alt="image" />
            </div>`;
            allPages.value.push(noScrollStyle + wrappedImage);
          } else {
            allPages.value.push(noScrollStyle + currentPageContent);
          }
          currentPageContent = "";
          pageContainer.innerHTML = "";
        }
      } else {
        currentPageContent += paragraph;
      }
    }
    pageContainer.innerHTML = "";
    return true;
  };

  for (const element of elements) {
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
        <button class="icon-button" @click="goBackToMenu" title="返回书架">
          <el-icon :size="20"><ArrowLeft /></el-icon>
        </button>
      </div>
      <div class="page-indicator-inline" v-if="currentContent">
        {{ currentPage + 1 }} - {{ Math.min(currentPage + 2, totalPages) }} /
        {{ totalPages }}
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
      <div class="instruction">正在准备电子书内容...</div>
    </div>
    <!-- HTML Viewer - 书籍双页式布局 -->
    <div v-if="currentContent" class="html-view-container">
      <div class="resize-hint" v-if="resizeTimeout !== null">
        <div class="hint-text">重新布局内容中...</div>
      </div>

      <div class="two-column-layout">
        <button
          class="page-button-side prev-button-side"
          @click="goToPreviousPage"
          :disabled="currentPage <= 0"
          title="上一页"
        >
          <el-icon :size="16">
            <ArrowLeft />
          </el-icon>
        </button>

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
        <button
          class="page-button-side next-button-side"
          @click="goToNextPage"
          :disabled="currentPage + 2 >= totalPages"
          title="下一页"
        >
          <el-icon :size="16">
            <ArrowRight />
          </el-icon>
        </button>
      </div>
    </div>
  </div>
</template>
