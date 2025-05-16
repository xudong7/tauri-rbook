<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { Window } from "@tauri-apps/api/window";
import { Upload, Minus, FullScreen, Close } from "@element-plus/icons-vue";
import "./MenuView.css";

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
    books.value = await invoke<MenuItem[]>("get_all_local_files_command");
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
  // Emit event to App.vue to switch to ReaderView with this book
  window.dispatchEvent(new CustomEvent("open-book", { detail: { filePath } }));
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
        <button @click="uploadEpub" class="upload-button">
          <el-icon :size="24"><Upload /></el-icon>
          上传电子书
        </button>
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
