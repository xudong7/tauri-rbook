<script setup lang="ts">
import { ref } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { Window } from "@tauri-apps/api/window";
import "./ReaderView.css";
import { getEpubHtmlWithImages, HtmlWithImages } from "../api";
import { Document, Minus, FullScreen, Close } from "@element-plus/icons-vue";

const currentContent = ref<string>("");
const loading = ref<boolean>(false);
const filePath = ref<string>("");
const htmlWithImages = ref<HtmlWithImages | null>(null);
const appWindow = Window.getCurrent();

// 打开并转换EPUB为HTML
const openAndConvertEpub = async () => {
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

    // 调用后端API获取HTML和图片
    htmlWithImages.value = await getEpubHtmlWithImages(filePath.value);

    // 处理HTML内容和图片
    processHtmlContent();

    loading.value = false;
  } catch (error) {
    console.error("Error converting EPUB to HTML:", error);
    loading.value = false;
  }
};

// 处理HTML内容和图片
const processHtmlContent = () => {
  if (!htmlWithImages.value) return;

  let html = htmlWithImages.value.html_content;
  const images = htmlWithImages.value.images;

  // 为每个图片创建data URL并替换HTML中的图片路径
  images.forEach((img) => {
    const dataUrl = `data:${img.mime_type};base64,${img.content}`;
    // 替换HTML中的图片路径为data URL
    html = html.replace(
      new RegExp(`src=['"]${img.path}['"]`, "g"),
      `src="${dataUrl}"`
    );
  });

  currentContent.value = html;
};

// Add window control functions
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
    <!-- Toolbar -->
    <div class="reader-toolbar">
      <div class="left-controls">
        <button
          class="icon-button"
          @click="openAndConvertEpub"
          :disabled="loading"
          title="Open EPUB as HTML"
        >
          <el-icon :size="20" v-if="!loading"><Document /></el-icon>
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

    <!-- Placeholder -->
    <div v-if="!currentContent" class="placeholder">
      <div class="instruction">Please open an EPUB file to view as HTML</div>
    </div>

    <!-- HTML Viewer -->
    <div v-if="currentContent" class="html-view-container">
      <div class="html-content-viewer">
        <iframe :srcdoc="currentContent" class="html-iframe"></iframe>
      </div>
    </div>
  </div>
</template>
