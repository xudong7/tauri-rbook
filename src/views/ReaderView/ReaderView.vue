<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, nextTick } from "vue";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import { Window } from "@tauri-apps/api/window";
import ePub from "epubjs";
import {
  ArrowLeft,
  ArrowRight,
  Minus,
  Setting,
  FullScreen,
  Expand,
  Close,
} from "@element-plus/icons-vue";

// 定义类型
interface BookMetadata {
  title: string;
  creator: string;
  publisher: string;
}

interface TocItem {
  label: string;
  href: string;
  level: number;
  subitems?: TocItem[];
}

interface SpineItem {
  href: string;
}

// MenuView传来的epub文件路径
const props = defineProps<{
  initialFilePath?: string;
}>();

//------------------------------------------------
// 状态变量
//------------------------------------------------
const router = useRouter();
const appWindow = Window.getCurrent();
const wheelPagingEnabled = ref<boolean>(true); // 是否启用鼠标滚轮翻页
const epubViewerRef = ref<HTMLElement | null>(null);
const loading = ref(true);
const error = ref<string | null>(null);
const book = ref<any>(null);
const rendition = ref<any>(null);
const currentPage = ref(1);
const totalPages = ref(0);
const showToc = ref(false); // 控制是否显示目录
const tableOfContents = ref<TocItem[]>([]); // 存储书籍目录

// 书籍元数据
const bookMetadata = ref<BookMetadata>({
  title: "",
  creator: "",
  publisher: "",
});

// 电子书阅读器配置选项
const GLOBAL_OPTIONS = {
  width: "100%",
  height: "100%",
  view: "iframe",
  spread: "true",
  minSpreadWidth: 960,
  resizeOnOrientationChange: true,
  snap: true,
  flow: "paginated",
  manager: "continuous",
  allowScriptedContent: true,
};

//------------------------------------------------
// 电子书加载相关函数
//------------------------------------------------

/**
 * 加载EPUB文件
 */
const loadEpub = async (filePath: string) => {
  try {
    loading.value = true;
    error.value = null;

    // 从Tauri后端读取EPUB文件内容
    const fileContent = await readEpubFileContent(filePath);

    // 初始化电子书
    await initializeBook(fileContent);

    // 设置渲染器和事件监听
    if (!setupRendition()) return;

    loading.value = false;
  } catch (err) {
    handleError("Error loading EPUB:", err);
  }
};

/**
 * 从Tauri后端读取EPUB文件内容
 */
const readEpubFileContent = async (filePath: string): Promise<ArrayBuffer> => {
  const fileContent = await invoke<number[]>("read_epub_file_content_command", {
    filePath,
  });
  return new Uint8Array(fileContent).buffer;
};

/**
 * 初始化电子书对象和加载相关数据
 */
const initializeBook = async (arrayBuffer: ArrayBuffer) => {
  book.value = ePub(arrayBuffer);
  await book.value.ready;
  await nextTick();

  // 加载元数据和目录
  await loadBookMetadata();
  await loadTableOfContents();

  // 检查和设置页数
  const spine = book.value.spine;
  if (!spine) {
    throw new Error("No spine found in the EPUB file");
  }
  totalPages.value = spine.length;
};

/**
 * 设置电子书渲染器和事件监听
 */
const setupRendition = (): boolean => {
  // 检查EPUB视图元素是否存在
  if (!checkIfEPUBViewerExists()) {
    return false;
  }

  // 创建渲染器并显示
  rendition.value = book.value.renderTo(epubViewerRef.value, GLOBAL_OPTIONS);
  rendition.value.display();

  // 添加翻页事件监听
  setupEventListeners();

  // 更新总页数
  updateTotalPages();

  return true;
};

/**
 * 设置电子书事件监听器
 */
const setupEventListeners = () => {
  rendition.value.on("relocated", (location: any) => {
    // 更新当前页面信息
    if (location && location.start) {
      currentPage.value = location.start.displayed.page;
    }
  });
};

/**
 * 检查EPUB视图元素是否存在
 */
const checkIfEPUBViewerExists = (): boolean => {
  if (!epubViewerRef.value) {
    console.error("EPUB viewer element not found");
    error.value = "Failed to initialize EPUB viewer";
    loading.value = false;
    return false;
  }
  return true;
};

/**
 * 处理加载错误
 */
const handleError = (message: string, err: any) => {
  console.error(message, err);
  error.value = `Failed to load book: ${err}`;
  loading.value = false;
};

//------------------------------------------------
// 元数据和目录相关函数
//------------------------------------------------

/**
 * 加载书籍元数据
 */
const loadBookMetadata = async () => {
  if (!book.value) return;

  try {
    const metadata = await book.value.loaded.metadata;
    bookMetadata.value = {
      title: metadata.title || "Unknown Title",
      creator: metadata.creator || "Unknown Author",
      publisher: metadata.publisher || "Unknown Publisher",
    };
    console.log("Book metadata:", bookMetadata.value);
  } catch (err) {
    console.error("Failed to load book metadata:", err);
  }
};

/**
 * 加载书籍目录
 */
const loadTableOfContents = async () => {
  if (!book.value) return;

  try {
    const toc = await book.value.loaded.navigation;
    if (toc && toc.toc) {
      // 处理TOC，添加嵌套层级信息并确保href是正确的
      tableOfContents.value = toc.toc.map((item: any) => {
        return processTocItem(item, 0);
      });

      // 记录调试信息
      logTocDebugInfo();
    } else {
      console.warn("No table of contents found in the book");
      tableOfContents.value = [];
    }
  } catch (err) {
    console.error("Failed to load table of contents:", err);
    tableOfContents.value = [];
  }
};

/**
 * 记录目录调试信息
 */
const logTocDebugInfo = () => {
  console.log("Table of contents:", tableOfContents.value);

  if (book.value && book.value.spine) {
    console.log(
      "Spine items:",
      book.value.spine.items.map((item: any) => item.href)
    );
  }
};

/**
 * 处理目录项，添加层级信息
 */
const processTocItem = (item: any, level: number): TocItem => {
  // 创建处理后的项目对象
  const processedItem: TocItem = {
    ...item,
    level,
    // 尝试规范化href
    href: normalizeHref(item.href),
  };

  // 递归处理子项目
  if (item.subitems && item.subitems.length > 0) {
    processedItem.subitems = item.subitems.map((subitem: any) => {
      return processTocItem(subitem, level + 1);
    });
  }

  return processedItem;
};

/**
 * 规范化href以匹配spine项
 */
const normalizeHref = (href: string): string => {
  if (!href) return href;

  // 记录原始href和规范化后的href
  console.log(`Original href: ${href}, Normalized: ${href}`);

  return href;
};

//------------------------------------------------
// 导航和翻页相关函数
//------------------------------------------------

/**
 * 跳转到指定章节
 */
const navigateToChapter = (href: string) => {
  if (!rendition.value) return;

  try {
    const cleanHref = prepareHref(href);
    console.log("Navigating to chapter:", cleanHref);

    // 尝试导航到章节
    navigateUsingMultipleStrategies(cleanHref);

    // 关闭目录面板并更新当前页面信息
    showToc.value = false;
    updateCurrentPage();
  } catch (error: unknown) {
    console.error(
      "Error navigating to chapter:",
      error instanceof Error ? error.message : error
    );
  }
};

/**
 * 准备href用于导航
 */
const prepareHref = (href: string): string => {
  let cleanHref = href;

  // 如果href以#开头，需要特殊处理
  if (cleanHref.startsWith("#")) {
    const currentUrl =
      rendition.value.location && rendition.value.location.start
        ? rendition.value.location.start.href
        : "";
    const baseUrl = currentUrl.split("#")[0];
    cleanHref = baseUrl + cleanHref;
  }

  return cleanHref;
};

/**
 * 使用多种策略尝试导航到章节
 */
const navigateUsingMultipleStrategies = (href: string) => {
  // 使用book的spine查找章节
  const spineItem = book.value.spine.get(href);

  if (spineItem) {
    // 直接找到了spine项
    navigateUsingSpineItem(spineItem);
  } else {
    // 尝试直接使用提供的href
    navigateUsingHref(href);
  }
};

/**
 * 使用spine项导航
 */
const navigateUsingSpineItem = (spineItem: any) => {
  console.log("Found spine item:", spineItem);
  rendition.value.display(spineItem.href).catch((error: Error) => {
    console.error("Failed to navigate with spine item href:", error);
  });
};

/**
 * 使用href导航，失败时尝试备选策略
 */
const navigateUsingHref = (href: string) => {
  console.log("No spine item found directly. Trying alternative approaches...");

  rendition.value.display(href).catch((error: Error) => {
    console.error("Failed to navigate with provided href:", error);
    tryFallbackNavigationStrategies(href);
  });
};

/**
 * 尝试备选导航策略
 */
const tryFallbackNavigationStrategies = (href: string) => {
  const fileName = href.split("/").pop() || "";
  let matchedItem = findMatchingSpineItem(href, fileName);

  if (matchedItem) {
    console.log(
      "Found matching chapter using fallback strategies:",
      matchedItem.href
    );
    rendition.value.display(matchedItem.href);
  } else {
    // 最后的尝试：使用第一个spine项目
    useFirstChapterAsFallback();
  }
};

/**
 * 查找匹配的spine项
 */
const findMatchingSpineItem = (href: string, fileName: string) => {
  // 策略1: 根据文件名匹配
  let matchedItem = book.value.spine.items.find((item: any) =>
    item.href.includes(fileName)
  );

  // 策略2: 检查href是否包含在spine项的href中
  if (!matchedItem) {
    matchedItem = book.value.spine.items.find((item: any) =>
      href.includes(item.href)
    );
  }

  // 策略3: 检查是否是相对路径问题，尝试添加基本路径
  if (!matchedItem && !href.startsWith("/")) {
    const possibleFullPath = "/" + href;
    matchedItem = book.value.spine.items.find(
      (item: any) =>
        item.href === possibleFullPath || item.href.includes(fileName)
    );
  }

  // 策略4: 如果是一个锚点引用，尝试在当前章节内导航
  if (!matchedItem && href.includes("#")) {
    const currentSpinePosition =
      book.value.rendition.currentLocation().start.index;
    if (
      currentSpinePosition >= 0 &&
      currentSpinePosition < book.value.spine.items.length
    ) {
      matchedItem = book.value.spine.items[currentSpinePosition];
    }
  }

  return matchedItem;
};

/**
 * 使用第一章作为备选导航选项
 */
const useFirstChapterAsFallback = () => {
  console.error(
    "No matching chapter found. Attempting to use the first chapter as fallback."
  );
  if (book.value.spine.items.length > 0) {
    rendition.value.display(book.value.spine.items[0].href);
  }
};

/**
 * 上一页
 */
const prevPage = () => {
  if (rendition.value) {
    rendition.value.prev();
  }
};

/**
 * 下一页
 */
const nextPage = () => {
  if (rendition.value) {
    rendition.value.next();
  }
};

/**
 * 更新当前页面信息
 */
const updateCurrentPage = () => {
  if (!rendition.value || !book.value) return;

  const currentLocation = rendition.value.currentLocation();
  if (currentLocation) {
    const { displayed } = currentLocation;
    if (displayed && displayed.page) {
      currentPage.value = displayed.page;
    }
  }
};

/**
 * 更新总页数
 */
const updateTotalPages = async () => {
  if (book.value) {
    const pageList = await book.value.locations.generate();
    totalPages.value = pageList.length;
  }
};

//------------------------------------------------
// UI 交互相关函数
//------------------------------------------------

/**
 * 切换目录显示
 */
const toggleToc = () => {
  showToc.value = !showToc.value;
};

/**
 * 处理鼠标滚轮事件
 */
const onWheel = (e: WheelEvent) => {
  if (!wheelPagingEnabled.value) return;
  if (e.deltaY > 0) nextPage();
  else if (e.deltaY < 0) prevPage();
};

/**
 * 返回菜单
 */
const backToMenu = () => {
  router.push("/");
};

/**
 * 最小化窗口
 */
const minimizeWindow = async () => {
  await appWindow.minimize();
};

/**
 * 最大化/还原窗口
 */
const maximizeWindow = async () => {
  if (await appWindow.isMaximized()) {
    await appWindow.unmaximize();
  } else {
    await appWindow.maximize();
  }
};

/**
 * 关闭窗口
 */
const closeWindow = async () => {
  await appWindow.close();
};

//------------------------------------------------
// 生命周期钩子
//------------------------------------------------

onMounted(async () => {
  const filePath = props.initialFilePath;
  if (filePath) {
    await loadEpub(filePath);
  } else {
    error.value = "No file path provided";
    loading.value = false;
  }
});

onBeforeUnmount(() => {
  if (book.value) {
    book.value.destroy();
  }
});
</script>

<template>
  <div class="reader-container">
    <div class="reader-toolbar">
      <div class="left-controls">
        <button @click="backToMenu" class="icon-button">
          <el-icon :size="16"><ArrowLeft /></el-icon>
        </button>
        <button class="icon-button">
          <el-icon :size="16">
            <Setting />
          </el-icon>
        </button>
        <button class="icon-button" @click="toggleToc">
          <el-icon :size="16">
            <Expand />
          </el-icon>
        </button>
      </div>
      <div class="book-info" v-if="bookMetadata.title">
        <div class="book-title">{{ bookMetadata.title }}</div>
        <!-- <div class="book-page-info">{{ currentPage }} / {{ totalPages }}</div> -->
      </div>

      <div class="window-controls">
        <button class="icon-button" @click="minimizeWindow" title="Minimize">
          <el-icon :size="16"><Minus /></el-icon>
        </button>
        <button class="icon-button" @click="maximizeWindow" title="Maximize">
          <el-icon :size="16"><FullScreen /></el-icon>
        </button>
        <button
          class="icon-button close-button"
          @click="closeWindow"
          title="Close"
        >
          <el-icon :size="16"><Close /></el-icon>
        </button>
      </div>
    </div>

    <div class="epub-container">
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
        @click="prevPage"
        :disabled="currentPage <= 0"
        title="上一页"
      >
        <el-icon :size="16">
          <ArrowLeft />
        </el-icon>
      </button>
      <div id="epub-viewer" ref="epubViewerRef"></div>
      <button
        class="page-button-side next-button-side"
        @click="nextPage"
        :disabled="currentPage + 1 >= totalPages"
        title="下一页"
      >
        <el-icon :size="16">
          <ArrowRight />
        </el-icon>
      </button>
    </div>

    <!-- 目录面板 -->
    <div class="toc-panel" v-if="showToc">
      <div class="toc-header">
        <span class="toc-title">目录</span>
        <button class="close-toc" @click="toggleToc">
          <el-icon :size="16"><Close /></el-icon>
        </button>
      </div>
      <div class="toc-content">
        <div
          v-for="(item, index) in tableOfContents"
          :key="index"
          class="toc-item"
          :style="{ paddingLeft: `${item.level ? item.level * 12 : 0}px` }"
        >
          <a
            href="#"
            @click.prevent="navigateToChapter(item.href)"
            class="toc-link"
            :title="item.href"
          >
            {{ item.label }}
          </a>
          <!-- 处理嵌套目录 -->
          <div
            v-if="item.subitems && item.subitems.length > 0"
            class="toc-subitems"
          >
            <div
              v-for="(subitem, subIndex) in item.subitems"
              :key="`${index}-${subIndex}`"
              class="toc-item"
              :style="{
                paddingLeft: `${subitem.level ? subitem.level * 12 : 12}px`,
              }"
            >
              <a
                href="#"
                @click.prevent="navigateToChapter(subitem.href)"
                class="toc-link"
                :title="subitem.href"
              >
                {{ subitem.label }}
              </a>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped src="./ReaderView.css" />
