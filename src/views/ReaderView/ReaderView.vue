<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, nextTick } from "vue";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import { Window } from "@tauri-apps/api/window";
import ePub from "epubjs";
import { createSettingsWindow } from "../../utils/settingsWindow";
import { ElMessage, ElMessageBox } from "element-plus";
import type {
  ReaderStyle,
  BookMetadata,
  TocItem,
  BookMark,
  Mark,
} from "../../types/model";
import {
  ArrowLeft,
  ArrowRight,
  Minus,
  FullScreen,
  Expand,
  Close,
  Setting,
  Star,
  StarFilled,
  Edit,
} from "@element-plus/icons-vue";

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
const showBookmarks = ref(false); // 控制是否显示书签面板
const bookmarks = ref<BookMark>({ book_path: "", list: [] }); // 存储书签列表
const currentBookPath = ref<string>(""); // 当前书籍路径
const editingBookmark = ref<Mark | null>(null); // 当前正在编辑的书签
const bookmarkContent = ref(""); // 编辑书签的内容

// 阅读器样式设置
const readerStyle = ref<ReaderStyle>({
  font_family: "Noto Serif",
  font_size: 18,
  line_height: 1.4,
});

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

    // 加载书签
    await loadBookmarks(filePath);

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

  // 应用阅读器样式
  applyReaderStyle();

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
      // 保存当前spine位置作为页面索引
      const newPage = location.start.index || 0;
      currentPage.value = newPage;

      // 记录详细的位置信息，包括CFI
      console.log("页面变更:", {
        page: currentPage.value,
        locationInfo: location.start,
        cfi: location.start.cfi,
        href: location.start.href,
        percentage: location.start.percentage,
      });
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
// 书签相关函数
//------------------------------------------------

/**
 * 加载书签
 */
const loadBookmarks = async (filePath: string) => {
  try {
    currentBookPath.value = filePath;
    const result = await invoke<BookMark>("get_bookmark_command", {
      bookPath: filePath,
    });

    if (result) {
      bookmarks.value = result;
      console.log("已加载书签:", result);
    }
  } catch (error) {
    console.error("加载书签失败:", error);
  }
};

/**
 * 添加或删除书签
 * @param action 0:添加, 1:删除
 * @param bookmarkPage 指定的书签页码，如果不提供则使用当前页
 * @param bookmarkCfi 指定的CFI，如果不提供则使用当前位置的CFI
 * @param content 书签备注内容
 */
const updateBookmark = async (
  action: number = 0,
  bookmarkPage?: number,
  bookmarkCfi?: string,
  content?: string
) => {
  if (!rendition.value || !currentBookPath.value) return;

  try {
    // 获取页码，优先使用参数提供的页码，否则使用当前页
    const page = bookmarkPage !== undefined ? bookmarkPage : currentPage.value;

    // 获取CFI (Content Fragment Identifier)，用于精确定位
    let cfi = bookmarkCfi;
    if (!cfi && action === 0) {
      // 如果没有指定CFI且是添加书签，获取当前位置的CFI
      const location = rendition.value.currentLocation();
      if (location && location.start) {
        cfi = location.start.cfi;
        console.log(`获取当前位置CFI: ${cfi}`);
      }
    }

    // 书签内容，默认为空字符串
    const bookmarkContent = content !== undefined ? content : "";

    // 记录操作的页码信息
    console.log(
      `操作书签: action=${action}, page=${page}, currentPage=${currentPage.value}, cfi=${cfi}, content=${bookmarkContent}`
    );

    // 获取窗口尺寸
    const windowSize = await appWindow.innerSize();
    const width = windowSize.width;
    const height = windowSize.height;

    // 调用后端保存书签
    await invoke<string>("save_bookmark_command", {
      bookPath: currentBookPath.value,
      page,
      content: bookmarkContent,
      width,
      height,
      cfi,
      action,
    });

    // 重新加载书签
    await loadBookmarks(currentBookPath.value);

    // 显示成功消息
    console.log(action === 1 ? "书签删除成功" : "书签添加成功");
  } catch (error) {
    console.error("操作书签失败:", error);
  }
};

/**
 * 跳转到书签页
 */
const navigateToBookmark = (mark: Mark) => {
  if (!rendition.value || !book.value) return;

  try {
    console.log("跳转到书签页:", mark);

    // 优先使用CFI进行精确导航
    if (mark.cfi) {
      console.log(`使用CFI精确跳转: ${mark.cfi}`);
      rendition.value
        .display(mark.cfi)
        .then(() => {
          console.log("成功使用CFI跳转到精确位置");
          // 关闭书签面板
          showBookmarks.value = false;
        })
        .catch((err: Error) => {
          console.error("使用CFI跳转失败，尝试使用页码跳转:", err);
          // 如果CFI导航失败，回退到基于页面索引的导航
          navigateByPageIndex(mark.page);
        });
    } else {
      // 如果没有CFI，使用页码导航
      console.log("无CFI信息，使用页码导航");
      navigateByPageIndex(mark.page);
    }
  } catch (error) {
    console.error("跳转到书签失败:", error);
  }
};

/**
 * 使用页码索引进行导航（作为后备方案）
 */
const navigateByPageIndex = (page: number) => {
  if (!rendition.value || !book.value) return;

  // 页面索引处理 (确保页面索引在合理范围内)
  const pageIndex = Math.min(
    Math.max(0, page),
    book.value.spine.items.length - 1
  );
  console.log(
    "调整后的页面索引:",
    pageIndex,
    "总章节数:",
    book.value.spine.items.length
  );

  // 跳转到指定页码对应的章节
  const spineItem = book.value.spine.items[pageIndex];
  if (spineItem) {
    console.log("找到对应章节:", spineItem);
    rendition.value
      .display(spineItem.href)
      .then(() => {
        console.log("成功跳转到书签章节");
        // 关闭书签面板
        showBookmarks.value = false;
      })
      .catch((err: Error) => {
        console.error("章节跳转错误:", err);
      });
  } else {
    console.error("未找到对应页码的章节:", pageIndex);
  }
};

/**
 * 切换书签面板显示
 */
const toggleBookmarks = () => {
  showBookmarks.value = !showBookmarks.value;
  if (showToc.value && showBookmarks.value) {
    showToc.value = false; // 如果目录是打开的，则关闭目录
  }
};

/**
 * 检查当前页是否已有书签
 */
const hasBookmarkOnCurrentPage = (): boolean => {
  return bookmarks.value.list.some((mark) => mark.page === currentPage.value);
};

/**
 * 切换当前页的书签状态
 */
const toggleCurrentPageBookmark = async () => {
  if (hasBookmarkOnCurrentPage()) {
    await updateBookmark(1); // 删除书签
    ElMessage({
      type: "success",
      message: "书签已删除",
      duration: 2000,
    });
  } else {
    // 添加书签时，弹出输入框询问备注内容
    ElMessageBox.prompt("请输入书签备注:", "添加书签", {
      confirmButtonText: "确定",
      cancelButtonText: "取消",
      inputType: "textarea",
      inputPlaceholder: "输入备注内容...",
      type: "info",
      showCancelButton: true,
      distinguishCancelAndClose: true,
    })
      .then(({ value }) => {
        // 添加带备注的书签
        updateBookmark(0, undefined, undefined, value);

        ElMessage({
          type: "success",
          message: "书签已添加",
          duration: 2000,
        });
      })
      .catch(() => {
        // 用户取消添加
        ElMessage({
          type: "info",
          message: "取消添加书签",
          duration: 2000,
        });
      });
  }
};

/**
 * 编辑书签内容
 */
const editBookmarkContent = (mark: Mark) => {
  editingBookmark.value = mark;
  bookmarkContent.value = mark.content || "";

  // 使用ElementPlus的MessageBox来编辑书签内容
  ElMessageBox.prompt("请输入书签备注:", "编辑书签", {
    confirmButtonText: "确定",
    cancelButtonText: "取消",
    inputValue: bookmarkContent.value,
    type: "warning",
    inputType: "textarea",
    inputPlaceholder: "输入备注内容...",
    showCancelButton: true,
    distinguishCancelAndClose: true,
  })
    .then(({ value }) => {
      if (editingBookmark.value) {
        // 保存更新后的书签内容
        updateBookmark(
          0, // 0表示添加或更新
          editingBookmark.value.page,
          editingBookmark.value.cfi,
          value
        );

        ElMessage({
          type: "success",
          message: "书签备注已更新",
          duration: 2000,
        });
      }
    })
    .catch((action) => {
      // 用户取消编辑
      if (action === "cancel") {
        ElMessage({
          type: "info",
          message: "取消编辑",
          duration: 2000,
        });
      }
    });
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

/**
 * 打开设置窗口
 */
// 点击设置按钮时，打开设置窗口
const openSettingWindow = async () => {
  try {
    createSettingsWindow();
  } catch (error) {
    console.error("打开设置窗口失败:", error);
  }
};
//------------------------------------------------
// 生命周期钩子
//------------------------------------------------

onMounted(async () => {
  // 加载阅读器样式设置
  await loadReaderStyle();

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

/**
 * 加载阅读器样式设置
 */
const loadReaderStyle = async () => {
  try {
    const style = await invoke<ReaderStyle>("get_reader_style_command");
    if (style) {
      readerStyle.value = style;
      console.log("已加载阅读设置:", style);

      // 如果已经初始化了渲染器，应用样式
      if (rendition.value) {
        applyReaderStyle();
      }
    }
  } catch (error) {
    console.error("加载阅读设置失败:", error);
  }
};

/**
 * 应用阅读器样式到电子书
 */
const applyReaderStyle = () => {
  if (!rendition.value) return;

  // 创建一个样式对象
  const style = {
    body: {
      "font-family": `"${readerStyle.value.font_family}", sans-serif !important`,
      "font-size": `${readerStyle.value.font_size}px !important`,
      "line-height": `${readerStyle.value.line_height} !important`,
    },
  };

  // 注册主题
  rendition.value.themes.register("user-theme", style);

  // 应用主题
  rendition.value.themes.select("user-theme");

  console.log("应用阅读样式:", style);
};
</script>

<template>
  <div class="reader-container">
    <div class="reader-toolbar">
      <div class="left-controls">
        <button @click="backToMenu" class="icon-button">
          <el-icon :size="20"><ArrowLeft /></el-icon>
        </button>
        <button class="icon-button" @click="openSettingWindow">
          <el-icon :size="20"><Setting /></el-icon>
        </button>
        <button class="icon-button" @click="toggleToc">
          <el-icon :size="20">
            <Expand />
          </el-icon>
        </button>
        <button class="icon-button" @click="toggleBookmarks">
          <el-icon :size="20">
            <Star />
          </el-icon>
        </button>
      </div>
      <div class="book-info" v-if="bookMetadata.title">
        <div class="book-title">{{ bookMetadata.title }}</div>
        <!-- <div class="book-page-info">{{ currentPage }} / {{ totalPages }}</div> -->
      </div>

      <div class="window-controls">
        <button class="icon-button" @click="minimizeWindow" title="Minimize">
          <el-icon :size="20"><Minus /></el-icon>
        </button>
        <button class="icon-button" @click="maximizeWindow" title="Maximize">
          <el-icon :size="20"><FullScreen /></el-icon>
        </button>
        <button
          class="icon-button close-button"
          @click="closeWindow"
          title="Close"
        >
          <el-icon :size="20"><Close /></el-icon>
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
        <el-icon :size="20">
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
        <el-icon :size="20">
          <ArrowRight />
        </el-icon>
      </button>
    </div>

    <!-- 目录面板 -->
    <div class="toc-panel" v-if="showToc">
      <div class="toc-header">
        <span class="toc-title">目录</span>
        <button class="close-toc" @click="toggleToc">
          <el-icon :size="20"><Close /></el-icon>
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
    <!-- 书签面板 -->
    <div class="bookmark-panel" v-if="showBookmarks">
      <div class="bookmark-header">
        <span class="bookmark-title">书签</span>
        <button class="close-bookmark" @click="toggleBookmarks">
          <el-icon :size="20"><Close /></el-icon>
        </button>
      </div>
      <div class="bookmark-content">
        <div v-if="bookmarks.list.length === 0" class="no-bookmarks">
          暂无书签，点击下方按钮添加书签
        </div>
        <div v-else>
          <div
            v-for="(mark, index) in bookmarks.list"
            :key="index"
            class="bookmark-item"
          >
            <div class="bookmark-info">
              <span class="bookmark-page">第 {{ mark.page + 1 }} 页</span>
              <div class="bookmark-content-text">
                {{ mark.content || "暂无备注" }}
              </div>
            </div>
            <div class="bookmark-actions">
              <button
                class="edit-bookmark"
                @click="editBookmarkContent(mark)"
                title="编辑备注"
              >
                <el-icon :size="16"><Edit /></el-icon>
              </button>
              <button
                class="goto-bookmark"
                @click="navigateToBookmark(mark)"
                title="跳转到书签"
              >
                <el-icon :size="16"><ArrowRight /></el-icon>
              </button>
              <button
                class="remove-bookmark"
                @click="updateBookmark(1, mark.page)"
                title="删除书签"
              >
                <el-icon :size="16"><Close /></el-icon>
              </button>
            </div>
          </div>
        </div>
      </div>
      <div class="bookmark-footer">
        <button
          class="bookmark-action-button"
          @click="toggleCurrentPageBookmark"
        >
          <el-icon v-if="hasBookmarkOnCurrentPage()" :size="16">
            <StarFilled />
          </el-icon>
          <el-icon v-else :size="16">
            <Star />
          </el-icon>
          {{
            hasBookmarkOnCurrentPage() ? "取消当前页书签" : "将当前页加入书签"
          }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped src="./ReaderView.css" />
