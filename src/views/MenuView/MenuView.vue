<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";
import { getAllLocalFiles } from "../../api";
import { open } from "@tauri-apps/plugin-dialog";
import { Window } from "@tauri-apps/api/window";

const router = useRouter();
import {
  Upload,
  Search,
  Minus,
  FullScreen,
  Close,
} from "@element-plus/icons-vue";

interface MenuItem {
  cover: string; // base64 encoded cover image
  file_path: string; // file path to the EPUB
}

const books = ref<MenuItem[]>([]);
const loading = ref<boolean>(false);
const filePath = ref<string>("");
const appWindow = Window.getCurrent();

onMounted(async () => {
  await loadLocalBooks();
});

// Load local books using the Rust function
const loadLocalBooks = async () => {
  try {
    loading.value = true;
    books.value = await getAllLocalFiles();
    loading.value = false;
  } catch (error) {
    console.error("Failed to load local books:", error);
    loading.value = false;
  }
};

// Upload new EPUB file
const uploadEpub = async () => {
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

    openBook(filePath.value);

    loading.value = false;
  } catch (error) {
    console.error("Error uploading EPUB:", error);
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

// Open search view
const openSearch = () => {
  router.push("/search");
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
    <!-- Toolbar -->
    <div class="menu-toolbar">
      <div class="left-controls">
        <button
          class="icon-button"
          @click="uploadEpub"
          :disabled="loading"
          title="上传电子书"
        >
          <el-icon :size="20" v-if="!loading"><Upload /></el-icon>
          <span v-else class="loading-spinner"></span>
        </button>
        <button class="icon-button" @click="openSearch" title="搜索电子书">
          <el-icon :size="20"><Search /></el-icon>
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

    <!-- Book Grid -->
    <div class="book-grid">
      <div v-if="loading" class="loading-container">
        <div class="loading-spinner large"></div>
        <div>正在加载书籍...</div>
      </div>
      <div v-else-if="books.length === 0" class="empty-state">
        <p>当前没有书籍</p>
        <div class="empty-state-buttons">
          <button @click="uploadEpub" class="upload-button">
            <el-icon :size="24"><Upload /></el-icon>
            上传电子书
          </button>
          <button @click="openSearch" class="upload-button">
            <el-icon :size="24"><Search /></el-icon>
            搜索电子书
          </button>
        </div>
      </div>

      <div v-else class="book-list">
        <div
          v-for="(book, index) in books"
          :key="index"
          class="book-item"
          @click="openBook(book.file_path)"
        >
          <div class="book-cover">
            <img
              :src="`data:image/jpeg;base64,${book.cover}`"
              alt="Book cover"
            />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped src="./MenuView.css" />
