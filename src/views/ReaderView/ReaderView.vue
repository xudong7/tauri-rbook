<template>
  <div class="reader-container">
    <!-- Always render the EPUB viewer container, but hide it when loading or error -->
    <div class="epub-container" :class="{ hidden: loading || error }">
      <div class="controls">
        <el-button @click="backToMenu" class="back-button">
          <el-icon><Back /></el-icon> Back
        </el-button>
        <div class="book-info" v-if="bookMetadata.title">
          <div class="book-title">{{ bookMetadata.title }}</div>
          <div class="book-author">{{ bookMetadata.creator }}</div>
        </div>
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

    <!-- Overlay the loading or error containers on top -->
    <div v-if="loading" class="loading-container overlay">
      <el-icon class="loading-icon"><Loading /></el-icon>
      <p>Loading book...</p>
    </div>
    <div v-if="error" class="error-container overlay">
      <el-icon class="error-icon"><CircleClose /></el-icon>
      <p>{{ error }}</p>
      <el-button @click="backToMenu">Back to Library</el-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, computed, nextTick } from "vue";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
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

// Book metadata
const bookMetadata = ref<any>({
  title: "",
  creator: "",
  publisher: "",
});

// Functions
const backToMenu = () => {
  router.push("/");
};

const loadEpub = async (filePath: string) => {
  try {
    loading.value = true;
    error.value = null;
    console.log("Loading EPUB from path:", filePath);

    // Ensure the viewer element is available in the DOM
    await new Promise((resolve) => setTimeout(resolve, 10));

    // Use the custom Tauri command to read the EPUB file
    const fileContent = await invoke<number[]>(
      "read_epub_file_content_command",
      {
        filePath,
      }
    );
    console.log("EPUB file loaded successfully, size:", fileContent.length);

    // Create a new book
    const arrayBuffer = new Uint8Array(fileContent).buffer;
    book.value = ePub(arrayBuffer); // Wait for the book to be open
    await book.value.ready;
    console.log("EPUB book ready"); // Ensure DOM is fully rendered before proceeding
    await nextTick();

    // Get the total number of pages (sections)
    const spine = book.value.spine;
    totalPages.value = spine.length;
    console.log("Total pages:", totalPages.value);

    // Give DOM time to render fully - use multiple ticks and delays
    for (let i = 0; i < 3; i++) {
      await nextTick();
      await new Promise((resolve) => setTimeout(resolve, 50));
    }

    // Make sure the epubViewerRef is available
    if (!epubViewerRef.value) {
      console.error("EPUB viewer element not found");
      error.value = "Failed to initialize EPUB viewer";
      loading.value = false;
      return;
    } // Create a rendition of the book
    rendition.value = book.value.renderTo(epubViewerRef.value, {
      width: "100%",
      height: "100%",
      spread: "none",
      flow: "paginated",
      manager: "default",
    });

    // Display the book
    await rendition.value.display();
    console.log("Book displayed successfully");

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

// Function to load book metadata
const loadBookMetadata = async () => {
  if (book.value) {
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
  }
};

// Lifecycle hooks
onMounted(async () => {
  // Wait for DOM to be fully rendered
  await nextTick();

  const filePath = props.initialFilePath;
  if (filePath) {
    await loadEpub(filePath);
    await loadBookMetadata(); // Load metadata after the book is opened
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

<style scoped src="./ReaderView.css" />
