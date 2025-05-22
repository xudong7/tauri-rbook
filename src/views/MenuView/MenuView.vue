<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from "vue";
import { useRouter } from "vue-router";
import { open } from "@tauri-apps/plugin-dialog";
import { Window } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import { readFile } from "@tauri-apps/plugin-fs";

const router = useRouter();
import {
  Upload,
  Minus,
  FullScreen,
  Close,
  ArrowLeft,
  ArrowRight,
} from "@element-plus/icons-vue";

interface MenuItem {
  cover: string; // path to cover image
  path: string; // file path to the .epub file
}

const books = ref<MenuItem[]>([]);
const loading = ref<boolean>(false);
const appWindow = Window.getCurrent();

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
  () => Math.ceil(books.value.length / itemsPerPage.value) || 1
);

// 计算当前页面应该显示的书籍
const paginatedBooks = computed(() => {
  const startIndex = (currentPage.value - 1) * itemsPerPage.value;
  const endIndex = startIndex + itemsPerPage.value;
  return books.value.slice(startIndex, endIndex);
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
    const maxPage = Math.ceil(books.value.length / newValue);
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

const goToPage = (page: number) => {
  if (page >= 1 && page <= totalPages.value) {
    currentPage.value = page;
  }
};

onMounted(() => {
  loadLocalBooks();

  // 添加窗口大小变化监听
  window.addEventListener("resize", handleWindowResize);

  // 初始化窗口尺寸
  windowWidth.value = window.innerWidth;
  windowHeight.value = window.innerHeight;
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
  let binary = '';
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
    const bookResults = await invoke<{ cover: string; path: string }[]>(
      "load_all_local_epub_files_command"
    );

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
        });
      } catch (err) {
        // If there's an error reading the cover, use a placeholder
        console.error(`Failed to load cover for ${book.path}:`, err);
        processedBooks.push({
          cover: "", // Empty string or you could use a default cover base64
          path: book.path,
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
const openBook = (filePath: string) => {
  // Use router to navigate to reader with file path
  router.push({
    path: "/reader",
    query: { filePath },
  });
};

// Window control functions
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

        <div class="pagination-footer" v-if="books.length > 0">
          <div class="pagination-controls">
            <button
              class="pagination-button"
              @click="goToPreviousPage"
              :disabled="currentPage === 1"
              title="上一页"
            >
              <el-icon><ArrowLeft /></el-icon>
            </button>

            <!-- 页码选择器 -->
            <div class="page-selector">
              <span class="current-page">
                {{ currentPage }} / {{ totalPages }}
              </span>
              <div class="quick-page-nav" v-if="totalPages > 3">
                <button
                  v-for="page in Math.min(5, totalPages)"
                  :key="page"
                  class="page-number-button"
                  :class="{ active: currentPage === page }"
                  @click="goToPage(page)"
                >
                  {{ page }}
                </button>
                <span v-if="totalPages > 5">...</span>
              </div>
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
