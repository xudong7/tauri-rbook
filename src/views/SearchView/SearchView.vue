<script setup lang="ts">
import { Window } from "@tauri-apps/api/window";
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";
import { ElMessage, ElLoading } from "element-plus";

const router = useRouter();
import {
  Search,
  Minus,
  ArrowLeft,
  FullScreen,
  Close,
  Download,
} from "@element-plus/icons-vue";
import {
  BookSearchResult,
  searchOnlineBooks,
  downloadOnlineBook,
} from "../../api";

// 默认封面图片
const DEFAULT_COVER_IMAGE = "https://via.placeholder.com/150x200?text=No+Cover";

const appWindow = Window.getCurrent();

// 搜索相关状态
const keyword = ref<string>("");
const currentPage = ref<number>(1);
const totalPages = ref<number>(1); // 总页数，实际可能需要后端提供
const loading = ref<boolean>(false);
const books = ref<BookSearchResult[]>([]);
const downloadingBooks = ref<Set<string>>(new Set());

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

const goBackToMenu = () => {
  router.push("/");
};

// 处理图片加载错误
const handleImageError = (e: Event) => {
  const target = e.target as HTMLImageElement;
  target.src = DEFAULT_COVER_IMAGE;
};

// 搜索书籍
const searchBooks = async () => {
  if (!keyword.value.trim()) {
    ElMessage({
      message: "请输入搜索关键词",
      type: "warning",
    });
    return;
  }

  loading.value = true;
  try {
    books.value = await searchOnlineBooks(keyword.value, currentPage.value);
    // 估算总页数 - 每页10个结果，总共假设100个结果
    totalPages.value = Math.max(
      1,
      Math.ceil(books.value.length > 0 ? 100 / 10 : 0)
    );
  } catch (error) {
    console.error("搜索书籍失败:", error);
    ElMessage({
      message: `搜索失败: ${error}`,
      type: "error",
    });
    books.value = [];
  } finally {
    loading.value = false;
  }
};

// 处理页码变化
const handlePageChange = (page: number) => {
  currentPage.value = page;
  searchBooks();
};

// 下载书籍
const downloadBook = async (book: BookSearchResult) => {
  // 使用书籍details_url作为唯一标识
  const bookId = book.details_url;

  if (downloadingBooks.value.has(bookId)) {
    ElMessage({
      message: "该书籍正在下载中...",
      type: "info",
    });
    return;
  }

  downloadingBooks.value.add(bookId);
  const loadingInstance = ElLoading.service({
    lock: true,
    text: `正在下载《${book.title}》...`,
    background: "rgba(0, 0, 0, 0.7)",
  });
  try {
    const filePath = await downloadOnlineBook(book);
    ElMessage({
      message: `《${book.title}》下载成功！`,
      type: "success",
    });
    console.log("下载的文件路径:", filePath);

    // After successful download, navigate back to home page
    router.push("/");
  } catch (error) {
    console.error("下载书籍失败:", error);
    ElMessage({
      message: `下载失败: ${error}`,
      type: "error",
    });
  } finally {
    downloadingBooks.value.delete(bookId);
    loadingInstance.close();
  }
};

// 处理回车键搜索
const handleKeyDown = (event: KeyboardEvent) => {
  if (event.key === "Enter") {
    searchBooks();
  }
};

// 在组件挂载时监听键盘事件
onMounted(() => {
  window.addEventListener("keydown", handleKeyDown);
});
</script>

<template>
  <div class="search-container">
    <!-- Toolbar -->
    <div class="search-toolbar">
      <div class="left-controls">
        <button class="icon-button" @click="goBackToMenu" title="返回菜单">
          <el-icon :size="20"><ArrowLeft /></el-icon>
        </button>
        <div class="search-input-container">
          <el-input
            v-model="keyword"
            placeholder="输入关键词搜索书籍"
            clearable
            class="search-input"
            :prefix-icon="Search"
          />
          <el-button
            class="search-button"
            type="primary"
            circle
            @click="searchBooks"
            :loading="loading"
          >
            <el-icon v-if="!loading"><Search /></el-icon>
          </el-button>
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

    <!-- Book Grid -->
    <div class="book-grid">
      <div v-if="loading" class="loading-container">
        <el-skeleton :rows="6" animated />
      </div>
      <div v-else-if="books.length === 0" class="empty-state">
        <el-empty description="暂无搜索结果，请尝试搜索其他关键词" />
      </div>
      <div v-else class="books-container">
        <el-card
          v-for="book in books"
          :key="book.details_url"
          class="book-card"
          shadow="hover"
        >
          <div class="book-content">
            <div class="book-cover" @click="downloadBook(book)">
              <img
                :src="book.cover_image"
                :alt="book.title"
                class="cover-image"
                @error="handleImageError"
              />
            </div>
            <div class="book-info">
              <h3 class="book-title">{{ book.title }}</h3>
              <p class="book-author">{{ book.author }}</p>
              <div class="book-actions">
                <el-button
                  type="primary"
                  :icon="Download"
                  size="small"
                  circle
                  @click="downloadBook(book)"
                  :loading="downloadingBooks.has(book.details_url)"
                  :disabled="downloadingBooks.has(book.details_url)"
                ></el-button>
              </div>
            </div>
          </div>
        </el-card>
      </div>

      <!-- Pagination -->
      <div class="pagination" v-if="books.length > 0">
        <el-pagination
          v-model:current-page="currentPage"
          :page-size="10"
          layout="prev, pager, next"
          :total="totalPages * 10"
          @current-change="handlePageChange"
        />
      </div>
    </div>
  </div>
</template>

<style scoped src="./SearchView.css" />
