<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, nextTick } from "vue";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import { Window } from "@tauri-apps/api/window";
import ePub from "epubjs";
import WindowControl from "../../components/windowControl.vue";
import TocPannel from "../../components/tocPannel.vue";
import BookmarkPanel from "../../components/bookmarkPanel.vue";
import type {
  ReaderStyle,
  BookMetadata,
  TocItem,
  BookMark,
} from "../../types/model";
import { themeManager, type Theme } from "../../utils/themeManager";
import { applyBookContentTheme, getBookContentTheme } from "../../utils/bookContentThemes";
import {
  ArrowLeft,
  ArrowRight,
  Expand,
  Star,
  Sunny,
  Moon,
  Coffee,
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

// 阅读器样式设置
const readerStyle = ref<ReaderStyle>({
  font_family: "Noto Serif",
  font_size: 18,
  line_height: 1.4,
  theme: "light",
});

// 主题相关
const currentTheme = ref<Theme>(themeManager.getCurrentTheme());

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
 * 切换书签面板显示
 */
const toggleBookmarks = () => {
  showBookmarks.value = !showBookmarks.value;
  if (showToc.value && showBookmarks.value) {
    showToc.value = false; // 如果目录是打开的，则关闭目录
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
  if (showBookmarks.value && showToc.value) {
    showBookmarks.value = false; // 如果书签面板是打开的，则关闭书签面板
  }
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

// 主题切换函数
const toggleTheme = async () => {
  const nextTheme = themeManager.toggleToNextTheme();

  // 更新响应式变量
  currentTheme.value = nextTheme;

  // 更新阅读器样式中的主题
  readerStyle.value.theme = nextTheme;

  // 立即保存主题更改到后端
  try {
    await invoke("save_reader_style_command", {
      fontFamily: readerStyle.value.font_family,
      fontSize: readerStyle.value.font_size,
      lineHeight: readerStyle.value.line_height,
      theme: nextTheme,
    });

    console.log(`主题已切换到 ${nextTheme} 并保存`);
  } catch (error) {
    console.error("保存主题设置失败:", error);
  }
  // 重新应用样式到电子书内容
  applyBookContentTheme(rendition.value, nextTheme);
};

// 获取主题提示文本
const getThemeTooltip = () => {
  return themeManager.getThemeConfig(currentTheme.value).tooltip;
};

// 获取当前主题图标
const getCurrentThemeIcon = () => {
  return themeManager.getThemeConfig(currentTheme.value).icon;
};
//------------------------------------------------
// 生命周期钩子
//------------------------------------------------

onMounted(async () => {
  // 初始化主题
  currentTheme.value = themeManager.getCurrentTheme();
  // 添加主题变化监听器（用于跨窗口同步）
  const handleThemeChange = () => {
    const newTheme = themeManager.getCurrentTheme();
    if (newTheme !== currentTheme.value) {
      currentTheme.value = newTheme;
      readerStyle.value.theme = newTheme;

      // 重新应用书籍内容主题
      if (rendition.value) {
        applyBookContentTheme(rendition.value, newTheme);
      }

      console.log("ReaderView主题已同步:", newTheme);
    }
  };

  // 监听localStorage变化来同步主题（跨窗口）
  window.addEventListener("storage", (e) => {
    if (e.key === "app-theme") {
      handleThemeChange();
    }
  });

  // 监听自定义主题变化事件（同窗口内）
  window.addEventListener("themeChanged", () => {
    handleThemeChange();
  });

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

      // 更新当前主题
      if (style.theme) {
        currentTheme.value = style.theme as Theme;
        themeManager.setTheme(style.theme as Theme);
      }

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

  // 获取当前主题的内容颜色样式
  const contentTheme = getBookContentTheme(currentTheme.value);
  
  // 合并字体样式和主题颜色样式
  const mergedStyle = {
    body: {
      "font-family": `"${readerStyle.value.font_family}", sans-serif !important`,
      "font-size": `${readerStyle.value.font_size}px !important`,
      "line-height": `${readerStyle.value.line_height} !important`,
      ...contentTheme.body,
    },
    p: contentTheme.p,
    h1: contentTheme.h1,
    h2: contentTheme.h2,
    h3: contentTheme.h3,
    h4: contentTheme.h4,
    h5: contentTheme.h5,
    h6: contentTheme.h6,
    "*": contentTheme["*"],
  };

  // 注册并应用合并后的主题
  rendition.value.themes.register("merged-theme", mergedStyle);
  rendition.value.themes.select("merged-theme");

  console.log("应用合并后的阅读样式:", mergedStyle);
};
</script>

<template>
  <div class="reader-container">
    <div class="reader-toolbar">
      <div class="left-controls">
        <button @click="backToMenu" class="icon-button">
          <el-icon :size="20"><ArrowLeft /></el-icon>
        </button>
        <button
          class="icon-button"
          @click="toggleTheme"
          :title="getThemeTooltip()"
        >
          <el-icon :size="20">
            <Sunny v-if="getCurrentThemeIcon() === 'Sunny'" />
            <Moon v-else-if="getCurrentThemeIcon() === 'Moon'" />
            <Coffee v-else />
          </el-icon>
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

      <WindowControl :appWindow="appWindow" />
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
    <TocPannel
      :tableOfContents="tableOfContents"
      :showToc="showToc"
      :book="book"
      :rendition="rendition"
      @toggleToc="toggleToc"
    />

    <!-- 书签面板 -->
    <BookmarkPanel
      :bookmarks="bookmarks"
      :showBookmarks="showBookmarks"
      :currentPage="currentPage"
      :rendition="rendition"
      :book="book"
      :currentBookPath="currentBookPath"
      @toggleBookmarks="toggleBookmarks"
      @bookmarkUpdated="loadBookmarks(currentBookPath)"
    />
  </div>
</template>

<style scoped src="./ReaderView.css" />
