<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from "vue";
import { useRouter } from "vue-router";
import { open } from "@tauri-apps/plugin-dialog";
import { Window } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import { readFile } from "@tauri-apps/plugin-fs";
import { createSettingsWindow } from "../../utils/settingsWindow"; // Adjust the import path as necessary
import WindowControl from "../../components/windowControl.vue";
import { themeManager, type Theme } from "../../utils/themeManager";
const router = useRouter();
import {
  Upload,
  Setting,
  ArrowLeft,
  ArrowRight,
  Sort,
  Sunny,
  Moon,
  Coffee,
  Search,
  Close,
} from "@element-plus/icons-vue";
import type { MenuItem, ReaderStyle } from "../../types/model";

const books = ref<MenuItem[]>([]);
const loading = ref<boolean>(false);
const appWindow = Window.getCurrent();

// 主题相关
const currentTheme = ref<Theme>(themeManager.getCurrentTheme());

// 窗口尺寸相关变量
const windowWidth = ref<number>(window.innerWidth);
const windowHeight = ref<number>(window.innerHeight);
const resizeTimeout = ref<number | null>(null);

const booksPerRow = 6; // 每行固定6本书
const rowsPerPage = 3; // 每页固定3行
const gridGap = 10; // 间距（与CSS一致）
const padding = 10; // 内边距（与CSS一致）

// 每页显示的书籍数量
const itemsPerPage = computed(() => booksPerRow * rowsPerPage);

// 分页相关的变量
const currentPage = ref<number>(1);
const totalPages = computed(
  () => Math.ceil(sortedBooks.value.length / itemsPerPage.value) || 1
);

const sortByDate = ref<boolean>(true); // true: 按时间排序，false: 按名称排序

// 搜索功能相关变量
const searchQuery = ref<string>("");
const showSearchInput = ref<boolean>(false);

// 模糊搜索匹配函数
const fuzzyMatch = (text: string, query: string): boolean => {
  const textLower = text.toLowerCase();
  const queryLower = query.toLowerCase();

  let textIndex = 0;
  let queryIndex = 0;

  // 遍历查询字符串的每个字符
  while (queryIndex < queryLower.length && textIndex < textLower.length) {
    // 如果当前字符匹配，则移动到查询字符串的下一个字符
    if (textLower[textIndex] === queryLower[queryIndex]) {
      queryIndex++;
    }
    textIndex++;
  }

  // 如果所有查询字符都被匹配，则认为匹配成功
  return queryIndex === queryLower.length;
};

// 按最近打开时间排序和搜索过滤后的书籍列表
const sortedBooks = computed(() => {
  let filteredBooks = books.value;

  // 先进行搜索过滤
  if (searchQuery.value.trim()) {
    const query = searchQuery.value.trim();
    filteredBooks = books.value.filter((book) => {
      const fileName = book.path.split("/").pop() || "";
      const fileNameWithoutExt = fileName.replace(/\.[^/.]+$/, ""); // 移除文件扩展名
      return fuzzyMatch(fileNameWithoutExt, query);
    });
  }

  // 再进行排序
  if (sortByDate.value) {
    // 按最近打开时间排序
    return [...filteredBooks].sort((a, b) => {
      const timeA = a.last_opened || 0;
      const timeB = b.last_opened || 0;
      return timeB - timeA;
    });
  } else {
    // 按文件名排序
    return [...filteredBooks].sort((a, b) => {
      const nameA = a.path.split("/").pop() || "";
      const nameB = b.path.split("/").pop() || "";
      return nameA.localeCompare(nameB, undefined, { numeric: true });
    });
  }
});

// 切换排序方式
const toggleSortMethod = () => {
  sortByDate.value = !sortByDate.value;
};

// 搜索相关函数
const toggleSearchInput = () => {
  showSearchInput.value = !showSearchInput.value;
  if (!showSearchInput.value) {
    searchQuery.value = "";
  }
};

const clearSearch = () => {
  searchQuery.value = "";
};

// 监听搜索查询变化，重置到第一页
watch(searchQuery, () => {
  currentPage.value = 1;
});

// 计算当前页面应该显示的书籍
const paginatedBooks = computed(() => {
  const startIndex = (currentPage.value - 1) * itemsPerPage.value;
  const endIndex = startIndex + itemsPerPage.value;
  return sortedBooks.value.slice(startIndex, endIndex);
});

// 计算书籍元素的样式
const bookItemStyle = computed(() => {
  return {
    // 允许书籍项在网格中占据分配的空间
    width: "100%",
    height: "100%",
    display: "flex",
    flexDirection: "column" as const, // TypeScript needs explicit type for CSS properties
    alignItems: "center",
  };
});

// 点击设置按钮时，打开设置窗口
const openSettingWindow = async () => {
  try {
    createSettingsWindow();
  } catch (error) {
    console.error("打开设置窗口失败:", error);
  }
};

// 主题切换函数
const toggleTheme = async () => {
  const nextTheme = themeManager.toggleToNextTheme();

  // 更新响应式变量
  currentTheme.value = nextTheme;

  // 立即保存主题更改到后端
  try {
    // 获取当前的阅读器样式设置
    const currentStyle = await invoke<ReaderStyle>("get_reader_style_command");

    // 保存更新后的样式（包含新主题）
    await invoke("save_reader_style_command", {
      fontFamily: currentStyle.font_family || "Microsoft YaHei",
      fontSize: currentStyle.font_size || 16,
      lineHeight: currentStyle.line_height || 1.6,
      theme: nextTheme,
    });

    console.log(`主题已切换到 ${nextTheme} 并保存`);
  } catch (error) {
    console.error("保存主题设置失败:", error);
  }
};

// 获取主题提示文本
const getThemeTooltip = () => {
  return themeManager.getThemeConfig(currentTheme.value).tooltip;
};

// 获取当前主题图标
const getCurrentThemeIcon = () => {
  return themeManager.getThemeConfig(currentTheme.value).icon;
};

// 监听窗口大小变化
const handleWindowResize = () => {
  // 防抖处理
  if (resizeTimeout.value !== null) {
    clearTimeout(resizeTimeout.value);
  }

  // 立即更新窗口尺寸以获得更流畅的体验
  windowWidth.value = window.innerWidth;
  windowHeight.value = window.innerHeight;

  // 使用防抖处理进一步优化调整和分页处理
  resizeTimeout.value = window.setTimeout(() => {
    // 再次确认窗口尺寸是最新的
    windowWidth.value = window.innerWidth;
    windowHeight.value = window.innerHeight;

    // 如果当前页超出新的总页数，调整到最后一页
    if (currentPage.value > totalPages.value && totalPages.value > 0) {
      currentPage.value = totalPages.value;
    }

    resizeTimeout.value = null;
  }, 200); // 减少延迟以获得更快的响应
};

// 当itemsPerPage变化时，如果当前页面没有内容，则回到前一页
watch(itemsPerPage, (newValue, oldValue) => {
  if (newValue !== oldValue && currentPage.value > 1) {
    const maxPage = Math.ceil(sortedBooks.value.length / newValue);
    if (currentPage.value > maxPage) {
      currentPage.value = maxPage;
    }
  }
});

// 页面导航函数
const goToPreviousPage = () => {
  if (currentPage.value > 1) {
    currentPage.value--;
  }
};

const goToNextPage = () => {
  if (currentPage.value < totalPages.value) {
    currentPage.value++;
  }
};

onMounted(() => {
  loadLocalBooks();

  // 添加窗口大小变化监听
  window.addEventListener("resize", handleWindowResize);

  // 初始化窗口尺寸
  windowWidth.value = window.innerWidth;
  windowHeight.value = window.innerHeight;

  // 初始化主题并监听变化
  currentTheme.value = themeManager.getCurrentTheme();
  // 添加主题变化监听器（用于跨窗口同步）
  const handleThemeChange = () => {
    const newTheme = themeManager.getCurrentTheme();
    if (newTheme !== currentTheme.value) {
      currentTheme.value = newTheme;
      console.log("MenuView主题已同步:", newTheme);
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
});

// 监听窗口尺寸变化，动态调整布局
watch(
  [windowWidth, windowHeight],
  () => {
    // 当窗口尺寸变化时，重新计算布局
    if (currentPage.value > totalPages.value && totalPages.value > 0) {
      currentPage.value = totalPages.value;
    }
  },
  { immediate: true }
);

// 组件卸载时移除事件监听
onUnmounted(() => {
  window.removeEventListener("resize", handleWindowResize);
  if (resizeTimeout.value !== null) {
    clearTimeout(resizeTimeout.value);
  }
});

// Convert ArrayBuffer to base64 string
function arrayBufferToBase64(buffer: ArrayBuffer): string {
  const bytes = new Uint8Array(buffer);
  let binary = "";
  for (let i = 0; i < bytes.byteLength; i++) {
    binary += String.fromCharCode(bytes[i]);
  }
  return btoa(binary);
}

// Load local books using the Rust function
const loadLocalBooks = async () => {
  try {
    loading.value = true;
    // Use the load_all_local_epub_files_command from the Rust backend
    const bookResults = await invoke<
      { cover: string; path: string; last_opened?: number }[]
    >("load_all_local_epub_files_command");

    const processedBooks = [];
    for (const book of bookResults) {
      try {
        // Read the cover image file and convert it to base64
        const coverData = await readFile(book.cover);
        // Use a safe method to convert binary data to base64
        const base64Cover = arrayBufferToBase64(coverData);

        processedBooks.push({
          cover: base64Cover,
          path: book.path,
          last_opened: book.last_opened,
        });
      } catch (err) {
        // If there's an error reading the cover, use a placeholder
        console.error(`Failed to load cover for ${book.path}:`, err);
        processedBooks.push({
          cover: "", // Empty string or you could use a default cover base64
          path: book.path,
          last_opened: book.last_opened,
        });
      }
    }

    books.value = processedBooks;
    // 重置为第一页
    currentPage.value = 1;
    loading.value = false;
  } catch (error) {
    console.error("Failed to load local books:", error);
    loading.value = false;
  }
};

// Upload new EPUB files (supports multiple files)
const uploadEpub = async () => {
  try {
    loading.value = true;

    // Open file dialog with multiple selection enabled
    const selected = await open({
      multiple: true,
      filters: [
        {
          name: "EPUB",
          extensions: ["epub"],
        },
      ],
    });

    if (!selected) {
      loading.value = false;
      return;
    }

    // Handle multiple files or single file selection
    const filePaths = Array.isArray(selected) ? selected : [selected];

    // Process each selected file
    for (const filePath of filePaths) {
      await invoke("save_file_and_return_local_path_command", {
        originPath: filePath,
      });
    }

    // Reload books after upload
    await loadLocalBooks();

    loading.value = false;
  } catch (error) {
    console.error("Error uploading EPUB files:", error);
    loading.value = false;
  }
};

// Open a book in ReaderView
const openBook = async (filePath: string) => {
  // Update last opened time
  await invoke("update_last_opened_command", { filePath });

  // Use router to navigate to reader with file path
  router.push({
    path: "/reader",
    query: { filePath },
  });
};
</script>

<template>
  <div class="menu-container">
    <!-- 重新布局指示器 -->
    <div class="resize-indicator" v-if="resizeTimeout !== null">
      <div class="loading-spinner large"></div>
      <div>正在调整布局...</div>
    </div>
    <!-- Toolbar -->
    <div class="menu-toolbar">
      <div class="left-controls">
        <button
          class="icon-button"
          @click="uploadEpub"
          :disabled="loading"
          title="上传电子书 (支持多选)"
        >
          <el-icon :size="20" v-if="!loading"><Upload /></el-icon>
          <span v-else class="loading-spinner"></span>
        </button>
        <button class="icon-button" @click="openSettingWindow" title="设置">
          <el-icon :size="20">
            <Setting />
          </el-icon>
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
        <button
          class="icon-button"
          @click="toggleSortMethod"
          :title="sortByDate ? '当前：按时间排序' : '当前：按名称排序'"
        >
          <el-icon :size="20">
            <Sort />
          </el-icon>
        </button>
        <button
          class="icon-button"
          @click="toggleSearchInput"
          :title="showSearchInput ? '关闭搜索' : '搜索书籍'"
        >
          <el-icon :size="20">
            <Search />
          </el-icon>
        </button>
      </div>

      <!-- 搜索输入框 -->
      <div class="search-container" v-if="showSearchInput">
        <div class="search-input-wrapper">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="搜索书籍..."
            class="search-input"
            @keyup.escape="toggleSearchInput"
          />
          <button
            v-if="searchQuery"
            @click="clearSearch"
            class="clear-search-button"
            title="清除搜索"
          >
            <el-icon :size="16">
              <Close />
            </el-icon>
          </button>
        </div>
      </div>

      <WindowControl :appWindow="appWindow" />
    </div>

    <div class="main-container">
      <div v-if="loading" class="loading-container">
        <div class="loading-spinner large"></div>
        <div>正在加载书籍...</div>
      </div>
      <div v-else-if="books.length === 0" class="empty-state">
        <p>当前没有书籍</p>
        <div class="empty-state-buttons">
          <button @click="uploadEpub" class="upload-button">
            <el-icon :size="24"><Upload /></el-icon>
            上传电子书 (支持多选)
          </button>
        </div>
      </div>

      <div
        v-else-if="sortedBooks.length === 0 && searchQuery.trim()"
        class="empty-state"
      >
        <p>没有找到匹配的书籍</p>
        <p style="font-size: 14px; margin-top: 8px">
          搜索: "{{ searchQuery }}"
        </p>
        <div class="empty-state-buttons">
          <button @click="clearSearch" class="upload-button">
            <el-icon :size="20"><Close /></el-icon>
            清除搜索
          </button>
        </div>
      </div>

      <div v-else class="book-content-area">
        <div
          class="book-grid"
          :style="{
            'grid-template-columns': `repeat(${booksPerRow}, 1fr)`,
            'grid-template-rows': `repeat(${rowsPerPage}, minmax(auto, 1fr))`,
            gap: `${gridGap}px`,
            padding: `${padding}px`,
          }"
        >
          <div
            v-for="(book, index) in paginatedBooks"
            :key="index"
            class="book-item"
            :style="bookItemStyle"
            @click="openBook(book.path)"
          >
            <div class="book-cover">
              <img
                :src="`data:image/jpeg;base64,${book.cover}`"
                alt="Book cover"
                loading="lazy"
              />
            </div>
          </div>
        </div>
        <div
          class="pagination-footer"
          v-if="sortedBooks.length > 0 && totalPages > 1"
        >
          <div class="pagination-controls">
            <button
              class="pagination-button"
              @click="goToPreviousPage"
              :disabled="currentPage === 1"
              title="上一页"
            >
              <el-icon><ArrowLeft /></el-icon>
            </button>

            <!-- 简化的页码显示 -->
            <div class="page-selector">
              <span class="current-page">
                {{ currentPage }} / {{ totalPages }}
              </span>
            </div>

            <button
              class="pagination-button"
              @click="goToNextPage"
              :disabled="currentPage === totalPages"
              title="下一页"
            >
              <el-icon><ArrowRight /></el-icon>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped src="./MenuView.css" />
