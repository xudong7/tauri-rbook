<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from "vue";
import { useRouter } from "vue-router";
import { Window } from "@tauri-apps/api/window";
import {
  getEpubHtmlWithImages,
  HtmlWithImages,
  BookMark,
  Mark,
  saveBookmark,
} from "../../api";
import {
  generateStyle,
  resizeImgAndReturnInnerHTML,
  splitParagraphWithImages,
} from "../../utils/ReaderViewUtil";
import {
  ArrowLeft,
  ArrowRight,
  Minus,
  FullScreen,
  Close,
  Setting,
  Check,
  Collection,
  Operation,
} from "@element-plus/icons-vue";
import { ElDropdown, ElDropdownItem, ElDropdownMenu } from "element-plus";

const router = useRouter();
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
// 添加书签相关的状态
const hasBookmark = ref<boolean>(false);
const currentBookmark = ref<BookMark | null>(null);
//  添加设置相关的响应式变量
const wheelPagingEnabled = ref<boolean>(true); // 是否启用鼠标滚轮翻页
const dropdownRef = ref(); // 设置下拉菜单的引用


const fontFamily = ref("Noto Serif");
const fontSize = ref(18);

// 可选字体和字号
const fontFamilyOptions = [
  { label: "Noto Serif", value: "Noto Serif" },
  { label: "宋体", value: "SimSun" },
  { label: "仿宋", value: "FangSong"},
  { label: "楷体", value: "KaiTi" },
  { label: "微软雅黑", value: "Microsoft YaHei" },
  { label: "Times New Roman", value: "Times New Roman" },
  { label: "Arial", value: "Arial" },
];
const fontSizeOptions = [14, 16, 18, 20, 22, 24, 28, 32];

// 切换鼠标滚轮翻页状态
const toggleWheelPaging = (event?: Event) => {
  wheelPagingEnabled.value = !wheelPagingEnabled.value;
  if (event) event.stopPropagation();
};
// 自动关闭设置下拉菜单
const closeDropdown = () => {
  //延时0.5s关闭下拉菜单
  setTimeout(() => {
    dropdownRef.value?.handleClose();
  }, 200);
};

// Function to load a book from a specified path

// 全局样式
let GLOBAL_STYLE = generateStyle(fontFamily.value, fontSize.value);
let WINDOW_WIDTH = window.innerWidth;
let WINDOW_HEIGHT = window.innerHeight;
let PAGE_WIDTH = WINDOW_WIDTH / 2; // 页面宽度
let PAGE_HEIGHT = WINDOW_HEIGHT * 0.9; // 页面高度
const PAGE_PADDING = 20; // px


// 加载电子书
const loadBookFromPath = async (path: string) => {
  try {
    loading.value = true;
    filePath.value = path;

    // 调用后端API获取HTML和图片以及书签
    htmlWithImages.value = await getEpubHtmlWithImages(path);

    // 处理书签信息
    if (
      !htmlWithImages.value?.bookmark ||
      htmlWithImages.value?.bookmark?.list.length <= 0
    ) {
      // 如果没有书签信息，直接从第一页开始
      currentPage.value = 0;
    } else {
      currentBookmark.value = htmlWithImages.value.bookmark;
      // TODO：现在是获取最新添加的书签，后续应该默认为0，之后可以选择书签进行跳转
      const lastMark =
        currentBookmark.value.list[currentBookmark.value.list.length - 1];
      // TODO：现在是直接根据页码跳转到书签页，后续可以考虑根据书签的宽高来判断是否需要重新布局
      currentPage.value = lastMark.page;
      // 确保页码是偶数，左右页布局的需要
      if (currentPage.value % 2 !== 0) {
        currentPage.value = Math.max(0, currentPage.value - 1);
      }
    }

    // 处理HTML内容和图片
    processHtmlContent();

    loading.value = false;
  } catch (error) {
    console.error("Error loading book:", error);
    loading.value = false;
  }
};


// 监听初始文件路径的变化
watch(
  () => props.initialFilePath,
  (newPath) => {
    if (newPath) {
      loadBookFromPath(newPath);
    }
  },
  { immediate: true }
);

// 返回书架
const goBackToMenu = () => {
  router.push("/");
};

// 根据窗口大小生成全局样式
const updateGlobalStyle = () => {
  GLOBAL_STYLE = generateStyle(fontFamily.value, fontSize.value);
};

let GLOBAL_STYLE = generateStyle(fontFamily.value, fontSize.value);
const PAGE_PADDING = 20; // px

watch([fontFamily, fontSize], () => {
  updateGlobalStyle();
  if (htmlWithImages.value) processHtmlContent();
});


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
      updateGlobalStyle();

      // 如果当前有内容，则重新分割页面
      if (htmlWithImages.value) {
        processHtmlContent();
      }
    }

    resizeTimeout.value = null;
  }, 300);
};

// 监听滚轮事件，翻页
const onWheel = (e: WheelEvent) => {
  if (!wheelPagingEnabled.value) return;
  if (!currentContent.value) return;
  if (e.deltaY > 0) goToNextPage();
  else if (e.deltaY < 0) goToPreviousPage();
};

// 组件挂载和卸载时添加/移除窗口大小变化监听
onMounted(() => {
  lastWindowSize.value = {
    width: window.innerWidth,
    height: window.innerHeight,
  };
  window.addEventListener("resize", handleWindowResize);
});

// 组件卸载时清除事件监听
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

// 处理单个元素并添加到页面中
const processElement = async (
  element: Element,
  pageContainer: HTMLDivElement,
  currentPageContent: string
): Promise<{ currentPageContent: string; newPages: string[] }> => {
  // 获取所有段落和标题
  const paragraphs = element.outerHTML.match(
    /<(p|h[1-6])[\s\S]*?<\/(p|h[1-6])>/g
  ) || [element.outerHTML];

  // 处理后产生的新页面
  const newPages: string[] = [];
  let updatedPageContent = currentPageContent;

  // 处理每个段落和标题
  for (const paragraph of paragraphs) {
    // 拆分包含多个图像或多个span的段落
    const splitParagraphs = splitParagraphWithImages(
      paragraph,
      GLOBAL_STYLE,
      pageContainer,
      PAGE_HEIGHT,
      updatedPageContent
    );

    // 处理拆分后的每个段落
    for (const singleParagraph of splitParagraphs) {
      // 定义一个paragraph用于存储当前段落的内容
      let resultParagraph = singleParagraph;

      // 是否包含图片 进行图片大小处理
      if (
        (resultParagraph.includes("<img") ||
          resultParagraph.includes("<svg") ||
          resultParagraph.includes("<image")) &&
        updatedPageContent.trim() !== ""
      ) {
        // 处理图片大小
        resultParagraph = resizeImgAndReturnInnerHTML(
          singleParagraph,
          PAGE_WIDTH,
          PAGE_HEIGHT
        );
      }

      // 计算加了GLOBAL_STYLE样式后的页面的高度
      pageContainer.innerHTML =
        GLOBAL_STYLE + updatedPageContent + resultParagraph;
      const currentHeight = pageContainer.clientHeight;

      // 如果当前高度超过页面高度，强制分页
      if (currentHeight > PAGE_HEIGHT) {
        newPages.push(GLOBAL_STYLE + updatedPageContent);
        updatedPageContent = resultParagraph; // 将当前段落放到新页面
      } else {
        // 如果当前高度未超过页面高度，继续添加
        updatedPageContent += resultParagraph;
      }
    }
  }
  pageContainer.innerHTML = "";
  return { currentPageContent: updatedPageContent, newPages };
};

// 生成用于计算页面高度的容器
const generatePageContainer = () => {
  const pageContainer = document.createElement("div");
  pageContainer.style.width = `${PAGE_WIDTH}px`;
  pageContainer.style.padding = `${PAGE_PADDING}px`;
  pageContainer.style.overflow = "hidden";
  pageContainer.style.position = "relative";
  pageContainer.style.boxSizing = "border-box";
  return pageContainer;
};

// 分割内容到左右栏
const splitContentForTwoColumns = async (html: string) => {
  // 存储所有页面的内容
  const tempDiv = document.createElement("div");
  tempDiv.innerHTML = html;
  const elements = Array.from(tempDiv.children);
  // 创建一个容器用于计算页面高度
  const pageContainer = generatePageContainer();
  document.body.appendChild(pageContainer);

  // 初始化当前页面的内容
  let currentPageContent = "";
  allPages.value = [];

  for (const element of elements) {
    const result = await processElement(
      element,
      pageContainer,
      currentPageContent
    );

    // 添加新的页面到总页面集合
    allPages.value.push(...result.newPages);
    // 更新当前页面内容
    currentPageContent = result.currentPageContent;
  }

  if (currentPageContent) {
    allPages.value.push(GLOBAL_STYLE + currentPageContent);
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

  // 更新书签状态
  hasBookmark.value = !!currentBookmark.value?.list.find(
    (mark) => mark.page === currentPage.value
  );
};

// 添加或更新书签
const toggleBookmark = async () => {
  if (!htmlWithImages.value || !filePath.value) return;

  try {
    // 获取HTML文件路径和窗口尺寸
    const html_file_path =
      htmlWithImages.value?.bookmark?.book_path || filePath.value;
    const dimensions = { width: WINDOW_WIDTH, height: WINDOW_HEIGHT };

    if (hasBookmark.value) {
      await removeBookmark(html_file_path, dimensions);
    } else {
      await addBookmark(html_file_path, dimensions);
    }
  } catch (error) {
    console.error("Error toggling bookmark:", error);
  }
};

// 移除书签
const removeBookmark = async (
  bookPath: string,
  dimensions: { width: number; height: number }
) => {
  if (!currentBookmark.value) return;

  // 先在前端过滤掉当前页的书签
  currentBookmark.value.list = currentBookmark.value.list.filter(
    (mark) => mark.page !== currentPage.value
  );
  hasBookmark.value = false;

  // 使用action=1表示移除书签
  await saveBookmark(
    bookPath,
    currentPage.value,
    dimensions.width,
    dimensions.height,
    1
  );
};

// 添加书签
const addBookmark = async (
  bookPath: string,
  dimensions: { width: number; height: number }
) => {
  // 先保存到后端
  await saveBookmark(
    bookPath,
    currentPage.value,
    dimensions.width,
    dimensions.height
  );
  hasBookmark.value = true;

  // 更新本地书签状态
  if (!currentBookmark.value) {
    currentBookmark.value = {
      book_path: bookPath,
      list: [],
    };
  }

  // 创建书签对象
  const mark: Mark = {
    page: currentPage.value,
    width: dimensions.width,
    height: dimensions.height,
  };

  // 添加或更新当前页的书签
  const existingMarkIndex = currentBookmark.value.list.findIndex(
    (m) => m.page === currentPage.value
  );

  if (existingMarkIndex >= 0) {
    currentBookmark.value.list[existingMarkIndex] = mark;
  } else {
    currentBookmark.value.list.push(mark);
  }
};

const updateGlobalState = (width: number, height: number) => {
  // 更新全局变量
  WINDOW_WIDTH = width;
  WINDOW_HEIGHT = height;
  PAGE_WIDTH = WINDOW_WIDTH / 2; // 页面宽度
  PAGE_HEIGHT = WINDOW_HEIGHT * 0.9; // 页面高度
  GLOBAL_STYLE = generateStyle();
};

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
      // 更新全局变量和样式
      updateGlobalState(currentWidth, currentHeight);

      // 如果当前有内容，则重新分割页面
      if (htmlWithImages.value) {
        processHtmlContent();
      }
    }

    resizeTimeout.value = null;
  }, 300);
};

// 返回书架
const goBackToMenu = () => {
  router.push("/");
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

// 切换鼠标滚轮翻页状态
const toggleWheelPaging = (event?: Event) => {
  wheelPagingEnabled.value = !wheelPagingEnabled.value;
  if (event) event.stopPropagation();
};

// 监听滚轮事件，翻页
const onWheel = (e: WheelEvent) => {
  if (!wheelPagingEnabled.value) return;
  if (!currentContent.value) return;
  if (e.deltaY > 0) goToNextPage();
  else if (e.deltaY < 0) goToPreviousPage();
};

// 自动关闭设置下拉菜单
const closeDropdown = () => {
  //延时0.5s关闭下拉菜单
  setTimeout(() => {
    dropdownRef.value?.handleClose();
  }, 200);
};

// 窗口控制方法
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

// 监听初始文件路径的变化
watch(
  () => props.initialFilePath,
  (newPath) => {
    if (newPath) {
      loadBookFromPath(newPath);
    }
  },
  { immediate: true }
);

// 组件挂载和卸载时添加/移除窗口大小变化监听
onMounted(() => {
  lastWindowSize.value = {
    width: WINDOW_WIDTH,
    height: WINDOW_HEIGHT,
  };
  window.addEventListener("resize", handleWindowResize);
});

// 组件卸载时清除事件监听
onUnmounted(() => {
  window.removeEventListener("resize", handleWindowResize);
  if (resizeTimeout.value !== null) {
    clearTimeout(resizeTimeout.value);
  }
});
</script>

<template>
  <div class="reader-container">
    <!-- Toolbar -->
    <div class="reader-toolbar">
      <div class="left-controls">
        <button class="icon-button" @click="goBackToMenu" title="返回书架">
          <el-icon :size="20"><ArrowLeft /></el-icon>
        </button>
        <el-dropdown trigger="click" :hide-on-click="false" ref="dropdownRef">
          <button class="icon-button" title="设置">
            <el-icon :size="20"><Setting /></el-icon>
          </button>
          <template #dropdown>
            <el-dropdown-menu slot="dropdown" @mouseleave="closeDropdown">
              <el-dropdown-item @click="goBackToMenu">选项一</el-dropdown-item>
              <el-dropdown-item @click="handleWindowResize"
                >重新加载</el-dropdown-item
              >

              <el-dropdown-item
                @click="toggleWheelPaging($event)"
                :style="wheelPagingEnabled ? 'font-weight:bold;color:#409EFF' : ''">
                启用鼠标滚轮翻页
                <el-icon
                  v-if="wheelPagingEnabled"
                  :size="16"
                  style="margin-left: 8px"
                >
                  <Check />
                </el-icon>
              </el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
        <el-dropdown trigger="click" :hide-on-click="false">
          <button class="icon-button" title = "页面布局">
            <el-icon :size="20"><Operation /></el-icon>
          </button>
          <template #dropdown>
            <el-dropdown-menu slot="dropdown" style="min-width: 140px">
              <el-dropdown-item>
                <div class="dropdown-item-content">
                  <label for="font-family">字体 </label>
                  <el-select
                    v-model="fontFamily"
                    id="font-family"
                    size="small"
                    @change="updateGlobalStyle"
                  >
                    <el-option
                      v-for="option in fontFamilyOptions"
                      :key="option.value"
                      :label="option.label"
                      :value="option.value"
                    />
                  </el-select>
                </div>
              </el-dropdown-item>
              <el-dropdown-item>
                <div class="dropdown-item-content">
                  <label for="font-size">字号 </label>
                  <el-select
                    v-model="fontSize"
                    id="font-size"
                    size="small"
                    @change="updateGlobalStyle"
                  >
                    <el-option
                      v-for="size in fontSizeOptions"
                      :key="size"
                      :label="size + 'px'"
                      :value="size"
                    />
                  </el-select>
                </div>
              </el-dropdown-item>
            </el-dropdown-menu>
          </template>
          <el-dropdown-menu slot="dropdown">
            
          </el-dropdown-menu>
        </el-dropdown>

        <button
          class="icon-button"
          @click="toggleBookmark"
          :class="{ active: hasBookmark }"
          title="添加/删除书签"
        >
          <el-icon :size="20"><Collection /></el-icon>
        </button>
      </div>
      <div class="page-controls" v-if="currentContent">
        <div class="page-indicator-inline">
          Page
          {{ currentPage + 1 }} :
          {{ Math.min(currentPage + 2, totalPages) }} &nbsp;of&nbsp;
          {{ totalPages }}
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
      <div class="instruction">正在准备电子书内容...</div>
    </div>
    <!-- HTML Viewer - 书籍双页式布局 -->
    <div v-if="currentContent" class="html-view-container">
      <div class="resize-hint" v-if="resizeTimeout !== null">
        <div class="hint-text">重新布局内容中...</div>
      </div>

      <div class="two-column-layout">
        <!-- 专门用于捕获onWheel事件的绝对定位透明div -->
        <div
          style="
            position: absolute;
            inset: 0;
            z-index: 10;
            background: transparent;
          "
          @wheel="onWheel"
        ></div>

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
