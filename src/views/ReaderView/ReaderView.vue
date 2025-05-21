<template>
  <div class="reader-container">
    <div v-if="loading" class="loading-container">
      <el-icon class="loading-icon"><Loading /></el-icon>
      <p>Loading book...</p>
    </div>
    <div v-else-if="error" class="error-container">
      <el-icon class="error-icon"><CircleClose /></el-icon>
      <p>{{ error }}</p>
      <el-button @click="backToMenu">Back to Library</el-button>
    </div>
    <div v-else class="epub-container">
      <div class="controls">
        <el-button @click="backToMenu" class="back-button">
          <el-icon><Back /></el-icon> Back
        </el-button>
        <div class="navigation-controls">
          <el-button @click="prevPage" :disabled="!canPrevPage">
            <el-icon><ArrowLeft /></el-icon>
          </el-button>
          <span class="page-info">{{ currentPage }} / {{ totalPages }}</span>
          <el-button @click="nextPage" :disabled="!canNextPage">
            <el-icon><ArrowRight /></el-icon>
          </el-button>
        </div>
      </div>
      <div id="epub-viewer" ref="epubViewerRef"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, computed } from "vue";
import { useRouter } from "vue-router";
import { readFile } from "@tauri-apps/plugin-fs";
import ePub from "epubjs";
import {
  Loading,
  CircleClose,
  Back,
  ArrowLeft,
  ArrowRight,
} from "@element-plus/icons-vue";

// Props and refs
const props = defineProps<{
  initialFilePath?: string;
}>();

const router = useRouter();
const epubViewerRef = ref<HTMLElement | null>(null);
const loading = ref(true);
const error = ref<string | null>(null);
const book = ref<any>(null);
const rendition = ref<any>(null);
const currentPage = ref(1);
const totalPages = ref(0);
const canPrevPage = computed(() => currentPage.value > 1);
const canNextPage = computed(() => currentPage.value < totalPages.value);

// Functions
const backToMenu = () => {
  router.push("/");
};

const loadEpub = async (filePath: string) => {
  try {
    console.log("Loading EPUB from path:", filePath);
    loading.value = true;
    error.value = null;

    // Read the epub file
    console.log("Reading file content...");
    try {
      // 尝试直接使用路径读取文件
      const fileContent = await readFile(filePath);
      console.log("File content read successfully, size:", fileContent.byteLength);
      
      // Create a new book
      console.log("Creating book object...");
      const arrayBuffer = new Uint8Array(fileContent).buffer;
      book.value = ePub(arrayBuffer);
      console.log("Book object created");
      
      // Wait for the book to be open
      console.log("Waiting for book to be ready...");
      await book.value.ready;
      console.log("Book is ready");
    } catch (readError) {
      console.error("Error reading file:", readError);
      error.value = `无法读取文件: ${readError}`;
      loading.value = false;
      return;
    }

    // Get the total number of pages (sections)
    const spine = book.value.spine;
    totalPages.value = spine.length;

    // Create a rendition of the book
    rendition.value = book.value.renderTo("epub-viewer", {
      width: "100%",
      height: "100%",
      spread: "none",
    });

    // Display the book
    await rendition.value.display();

    // Add event listeners for keyboard navigation
    window.addEventListener("keydown", handleKeyboardNavigation);

    // Add event listeners for rendition
    rendition.value.on("rendered", (section: any) => {
      currentPage.value = spine.indexOf(section.href) + 1;
    });

    loading.value = false;
  } catch (err) {
    console.error("Error loading EPUB:", err);
    error.value = `Failed to load book: ${err}`;
    loading.value = false;
  }
};

const prevPage = () => {
  if (rendition.value && canPrevPage.value) {
    rendition.value.prev();
  }
};

const nextPage = () => {
  if (rendition.value && canNextPage.value) {
    rendition.value.next();
  }
};

const handleKeyboardNavigation = (e: KeyboardEvent) => {
  if (e.key === "ArrowLeft") {
    prevPage();
  } else if (e.key === "ArrowRight") {
    nextPage();
  }
};

// Lifecycle hooks
onMounted(async () => {
  const filePath = props.initialFilePath;
  console.log("ReaderView mounted with filePath:", filePath);
  
  if (filePath) {
    await loadEpub(filePath);
  } else {
    error.value = "No file path provided";
    loading.value = false;
  }
});

onBeforeUnmount(() => {
  // Clean up event listeners
  window.removeEventListener("keydown", handleKeyboardNavigation);

  // Destroy the book and rendition
  if (book.value) {
    book.value.destroy();
  }
});
</script>

<style scoped>
.reader-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100%;
  overflow: hidden;
  background-color: #f5f5f5;
}

.loading-container,
.error-container {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 100%;
}

.loading-icon,
.error-icon {
  font-size: 48px;
  margin-bottom: 16px;
}

.epub-container {
  display: flex;
  flex-direction: column;
  flex: 1;
  overflow: hidden;
}

.controls {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background-color: #fff;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.navigation-controls {
  display: flex;
  align-items: center;
  gap: 16px;
}

.page-info {
  min-width: 80px;
  text-align: center;
}

#epub-viewer {
  flex: 1;
  overflow: hidden;
  background-color: #fff;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
  margin: 16px;
  border-radius: 4px;
}
</style>
