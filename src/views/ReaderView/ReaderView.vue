<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from "vue";
import { useRouter } from "vue-router";
import { Window } from "@tauri-apps/api/window";
import { getEpubHtmlWithImages, HtmlWithImages } from "../../api";
import {
  ArrowLeft,
  ArrowRight,
  Minus,
  FullScreen,
  Close,
} from "@element-plus/icons-vue";

const router = useRouter();

// Props
const props = defineProps<{
  initialFilePath?: string;
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
  router.push("/");
};

const noScrollStyle = `<style>
  html { overflow: hidden!important; margin: 20px; padding: 0; }
  body {
    font-family: 'Noto Serif', 'Times New Roman', serif!important;
    font-size: 18px!important;
    line-height: 1.4!important;
    color: #333!important;
    box-sizing: border-box!important;
  }  
  p {
    margin: 1em!important;
    text-indent: 1em!important;
  }
</style>`;

const PAGE_PADDING = 20; // px

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
  // 存储所有页面的内容
  const tempDiv = document.createElement("div");
  tempDiv.innerHTML = html;
  const elements = Array.from(tempDiv.children);

  // 得到页面宽度和高度
  const pageHeight = window.innerHeight * 0.9;
  const pageWidth = window.innerWidth / 2;

  // 初始化当前页面的内容
  let currentPageContent = "";
  allPages.value = [];

  const pageContainer = document.createElement("div");
  pageContainer.style.width = `${pageWidth}px`;
  pageContainer.style.padding = `${PAGE_PADDING}px`;
  pageContainer.style.overflow = "hidden";
  pageContainer.style.position = "relative";
  pageContainer.style.boxSizing = "border-box";
  document.body.appendChild(pageContainer);

  const processElement = async (element: Element) => {
    const paragraphs = element.outerHTML.match(/<p[\s\S]*?<\/p>/g) || [
      element.outerHTML,
    ];
    for (const paragraph of paragraphs) {
      // 检查是否包含'img'标签
      const isImage = paragraph.includes("<img");
      // 检查是否包含'svg'标签
      const isSvg = paragraph.includes("<svg");
      // 检查是否包含'image'标签
      const isImageTag = paragraph.includes("<image");
      // 包含'img'或'svg'或'image'的段落
      if (
        (isImage || isSvg || isImageTag) &&
        currentPageContent.trim() !== ""
      ) {
        // 创建临时容器来获取图片并处理图片尺寸
        const tempImgContainer = document.createElement("div");
        tempImgContainer.innerHTML = paragraph; // 设置最大尺寸限制
        const maxWidth = pageWidth * 0.9;
        const maxHeight = pageHeight * 0.9; // 为页面留出一些空间

        // 处理img标签图片大小
        const imgElements = tempImgContainer.querySelectorAll("img");
        if (imgElements.length > 0) {
          for (const img of imgElements) {
            // 设置图片样式，确保其不超出页面
            // img.style.maxWidth = `${maxWidth}px`;
            img.style.maxHeight = `${maxHeight}px`;
            img.style.width = "auto"; // 保持纵横比
            img.style.height = "auto"; // 保持纵横比
            img.style.display = "block";
            img.style.margin = "1em auto"; // 居中显示
          }
        }

        // 处理SVG中的image标签
        const svgElements = tempImgContainer.querySelectorAll("svg");
        if (svgElements.length > 0) {
          for (const svg of svgElements) {
            // 获取SVG原始尺寸
            const svgWidth = parseFloat(svg.getAttribute("width") || "0");
            const svgHeight = parseFloat(svg.getAttribute("height") || "0");

            if (svgWidth > 0 && svgHeight > 0) {
              // 计算缩放比例
              const scale = Math.min(
                maxWidth / svgWidth,
                maxHeight / svgHeight,
                1 // 不放大，只缩小
              );

              // 设置新尺寸
              const newWidth = Math.floor(svgWidth * scale);
              const newHeight = Math.floor(svgHeight * scale);

              // 应用新尺寸
              svg.setAttribute("width", newWidth.toString());
              svg.setAttribute("height", newHeight.toString());
              svg.style.display = "block";
              svg.style.margin = "1em auto"; // 居中显示
            }

            // 处理SVG内部的image标签
            const imageElements = svg.querySelectorAll("image");
            for (const image of imageElements) {
              // 获取image标签的原始尺寸
              const imageWidth = parseFloat(image.getAttribute("width") || "0");
              const imageHeight = parseFloat(
                image.getAttribute("height") || "0"
              );

              if (imageWidth > 0 && imageHeight > 0) {
                // 应用与SVG相同的缩放
                const scale = Math.min(
                  maxWidth / imageWidth,
                  maxHeight / imageHeight,
                  1 // 不放大，只缩小
                );

                // 设置新尺寸
                const newWidth = Math.floor(imageWidth * scale);
                const newHeight = Math.floor(imageHeight * scale);

                // 应用新尺寸
                image.setAttribute("width", newWidth.toString());
                image.setAttribute("height", newHeight.toString());
              }
            }
          }
        }
        // 替换原始段落为处理过尺寸的段落
        const processedParagraph = tempImgContainer.innerHTML;

        // 如果当前页面为空，直接添加
        if (currentPageContent === "") {
          currentPageContent += processedParagraph;
          continue;
        }

        // 计算加了noScrollStyle样式后的页面的高度
        pageContainer.innerHTML =
          noScrollStyle + currentPageContent + processedParagraph;
        const currentHeight = pageContainer.clientHeight;

        // 如果当前高度超过页面高度，强制分页后再添加图片
        if (currentHeight > pageHeight) {
          allPages.value.push(noScrollStyle + currentPageContent);
          currentPageContent = processedParagraph; // 将图片放到新页
        } else {
          // 当前高度未超过页面高度，继续添加
          currentPageContent += processedParagraph;
        }
      } else {
        // 非图片段落
        // 计算加了noScrollStyle样式后的页面的高度
        pageContainer.innerHTML =
          noScrollStyle + currentPageContent + paragraph;
        const currentHeight = pageContainer.clientHeight;
        // 如果当前高度超过页面高度，强制分页
        if (currentHeight > pageHeight) {
          allPages.value.push(noScrollStyle + currentPageContent);
          currentPageContent = "";
        }
        // 如果当前高度未超过页面高度，继续添加
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
  document.body.removeChild(pageContainer);
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

<style scoped src="./ReaderView.css" />
