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
  Close,
} from "@element-plus/icons-vue";

// MenuView传来的epub文件路径
const props = defineProps<{
  initialFilePath?: string;
}>();

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

// Book metadata
const bookMetadata = ref<any>({
  title: "",
  creator: "",
  publisher: "",
});

// Book options
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

const loadEpub = async (filePath: string) => {
  try {
    loading.value = true;
    error.value = null;

    // Load the EPUB file and display it
    const fileContent = await invoke<number[]>(
      "read_epub_file_content_command",
      {
        filePath,
      }
    );
    const arrayBuffer = new Uint8Array(fileContent).buffer;
    book.value = ePub(arrayBuffer);
    await book.value.ready;
    await nextTick();
    loadBookMetadata();

    const spine = book.value.spine;
    if (!spine) {
      throw new Error("No spine found in the EPUB file");
    }
    totalPages.value = spine.length;

    // Check if the EPUB viewer element exists
    if (!checkIfEPUBViewerExists()) {
      return;
    }

    // Create a rendition of the book
    rendition.value = book.value.renderTo(epubViewerRef.value, GLOBAL_OPTIONS); // Display the book
    await rendition.value.display();

    await updateTotalPages(); // Update the total pages after rendering

    loading.value = false;
  } catch (err) {
    console.error("Error loading EPUB:", err);
    error.value = `Failed to load book: ${err}`;
    loading.value = false;
  }
};

// Check if the EPUB viewer element exists
const checkIfEPUBViewerExists = () => {
  if (!epubViewerRef.value) {
    console.error("EPUB viewer element not found");
    error.value = "Failed to initialize EPUB viewer";
    loading.value = false;
    return false;
  }
  return true;
};

// Update the total pages based on the current book
const updateTotalPages = async () => {
  if (book.value) {
    const pageList = await book.value.locations.generate();
    totalPages.value = pageList.length;
  }
};

// 翻页方法
const prevPage = () => {
  rendition.value.prev();
};

const nextPage = () => {
  rendition.value.next();
};

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

// 监听滚轮事件，翻页
const onWheel = (e: WheelEvent) => {
  if (!wheelPagingEnabled.value) return;
  if (e.deltaY > 0) nextPage();
  else if (e.deltaY < 0) prevPage();
};

// 返回菜单
const backToMenu = () => {
  router.push("/");
};

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
      </div>

      <div class="book-info" v-if="bookMetadata.title">
        <div class="book-title">{{ bookMetadata.title }}</div>
        <!-- <div class="book-author">{{ bookMetadata.creator }}</div> -->
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
  </div>
</template>

<style scoped src="./ReaderView.css" />
