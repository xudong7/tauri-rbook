<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import "./ReaderView.css";

interface EpubFile {
  content: number[];
  mime: string;
  path: string;
}

interface EpubMetadata {
  title: string | null;
  author: string | null;
  description: string | null;
  language: string | null;
  publisher: string | null;
  toc: string[];
  spine: string[];
  cover_id: string | null;
}

interface EpubBook {
  metadata: EpubMetadata;
  resources: Record<string, EpubFile>;
  current_page: number;
}

const book = ref<EpubBook | null>(null);
const currentContent = ref<string>("");
const currentPage = ref<number>(0);
const totalPages = ref<number>(0);
const loading = ref<boolean>(false);
const filePath = ref<string>("");
const showToc = ref<boolean>(false);
const showDebug = ref<boolean>(false);
const missingResources = ref<string[]>([]);

const coverImage = computed(() => {
  if (!book.value || !book.value.metadata.cover_id) return null;

  const coverId = book.value.metadata.cover_id;
  const coverResource = book.value.resources[coverId];

  if (coverResource) {
    const blob = new Blob([Uint8Array.from(coverResource.content)], {
      type: coverResource.mime,
    });
    return URL.createObjectURL(blob);
  }

  return null;
});

const binaryToBase64 = (content: number[]): string => {
  const uint8Array = new Uint8Array(content);
  let binaryString = "";
  uint8Array.forEach((byte) => {
    binaryString += String.fromCharCode(byte);
  });
  return btoa(binaryString);
};

const getResourceUrl = (path: string): string => {
  if (!book.value) return "";

  // Try exact match first
  if (book.value.resources[path]) {
    const resource = book.value.resources[path];
    return `data:${resource.mime};base64,${binaryToBase64(resource.content)}`;
  }

  // If not found, try case-insensitive matching
  // Some EPUB files might have case differences between references
  const lowerPath = path.toLowerCase();
  const resourceKey = Object.keys(book.value.resources).find(
    (key) => key.toLowerCase() === lowerPath
  );

  if (resourceKey) {
    const resource = book.value.resources[resourceKey];
    return `data:${resource.mime};base64,${binaryToBase64(resource.content)}`;
  }

  // If still not found, try to match the filename part only
  const filename = path.split("/").pop() || "";
  if (filename) {
    const filenameWithoutParams = filename.split("?")[0]; // Remove any URL parameters

    for (const key of Object.keys(book.value.resources)) {
      // Try exact filename match at the end of the path
      if (
        key.endsWith(`/${filenameWithoutParams}`) ||
        key === filenameWithoutParams
      ) {
        const resource = book.value.resources[key];
        return `data:${resource.mime};base64,${binaryToBase64(
          resource.content
        )}`;
      }

      // Try case-insensitive filename match
      if (
        key.toLowerCase().endsWith(`/${filenameWithoutParams.toLowerCase()}`) ||
        key.toLowerCase() === filenameWithoutParams.toLowerCase()
      ) {
        const resource = book.value.resources[key];
        return `data:${resource.mime};base64,${binaryToBase64(
          resource.content
        )}`;
      }
    }
  }
  // Try to match by pattern (e.g., match "3847280816788470217_illus02.jpg" with "img_images_illus02.jpg")
  if (filename) {
    // Extract the identifier pattern from filenames like "3847280816788470217_illus02.jpg" -> "illus02"
    const idMatch = filename.match(/[_-]([a-zA-Z]+\d+)\.[a-zA-Z]+$/);
    if (idMatch && idMatch[1]) {
      const idPattern = idMatch[1]; // e.g., "illus02"

      const matchingFiles = Object.keys(book.value.resources).filter((key) => {
        const keyFilename = key.split("/").pop() || "";
        return keyFilename.includes(idPattern);
      });

      if (matchingFiles.length > 0) {
        const resource = book.value.resources[matchingFiles[0]];
        return `data:${resource.mime};base64,${binaryToBase64(
          resource.content
        )}`;
      }
    }
  }

  // As a last resort, try to find any image with a similar name
  if (filename) {
    const similarFiles = Object.keys(book.value.resources).filter((key) => {
      const keyFilename = key.split("/").pop() || "";
      return keyFilename.includes(filename) || filename.includes(keyFilename);
    });

    if (similarFiles.length > 0) {
      const resource = book.value.resources[similarFiles[0]];
      return `data:${resource.mime};base64,${binaryToBase64(resource.content)}`;
    }
  }

  // Console log for debugging
  console.warn(`Could not find resource: ${path}`);

  return "";
};

const processHtml = (html: string): string => {
  // Replace relative URLs with data URLs
  const parser = new DOMParser();
  const doc = parser.parseFromString(html, "text/html");

  // Helper function to resolve potentially relative paths
  const resolveRelativePath = (
    basePath: string,
    relativePath: string
  ): string => {
    // If it's already an absolute path or data URL
    if (
      relativePath.startsWith("/") ||
      relativePath.startsWith("data:") ||
      relativePath.startsWith("http")
    ) {
      return relativePath;
    }

    // If the path doesn't contain a slash, it might be a direct reference to a resource
    if (!relativePath.includes("/")) {
      return relativePath;
    }

    // Handle parent directory references (../)
    const baseDir = basePath.split("/").slice(0, -1).join("/");

    if (relativePath.startsWith("../")) {
      let resultPath = baseDir;
      let relPath = relativePath;

      while (relPath.startsWith("../")) {
        resultPath = resultPath.split("/").slice(0, -1).join("/");
        relPath = relPath.substring(3);
      }

      return `${resultPath}/${relPath}`;
    }

    // Handle current directory references (./)
    if (relativePath.startsWith("./")) {
      return `${baseDir}/${relativePath.substring(2)}`;
    }

    // Simple relative path
    return `${baseDir}/${relativePath}`;
  };

  // Get the current spine item path to use as base for relative paths
  const currentSpinePath = book.value?.metadata.spine[currentPage.value] || "";
  // Clear missing resources list
  missingResources.value = [];
  // Process images
  doc.querySelectorAll("img").forEach((img) => {
    const src = img.getAttribute("src");
    if (src && !src.startsWith("data:") && !src.startsWith("http")) {
      // Resolve the path correctly
      const resolvedPath = resolveRelativePath(currentSpinePath, src);
      const dataUrl = getResourceUrl(resolvedPath);

      if (dataUrl) {
        img.setAttribute("src", dataUrl);

        // Make sure the image is displayed properly
        if (!img.hasAttribute("style")) {
          // Only apply default styles if no specific styles are set
          if (!img.closest("figure")) {
            // Don't override figure styles
            img.setAttribute("style", "max-width: 100%; height: auto;");
          }
        }

        // Ensure parent figure has proper alignment if it specifies alignment classes
        const parentFigure = img.closest("figure");
        if (parentFigure) {
          if (parentFigure.classList.contains("figright")) {
            parentFigure.style.float = "right";
            parentFigure.style.marginLeft = "20px";
            parentFigure.style.marginBottom = "10px";
          } else if (parentFigure.classList.contains("figleft")) {
            parentFigure.style.float = "left";
            parentFigure.style.marginRight = "20px";
            parentFigure.style.marginBottom = "10px";
          }
        }
      } else {
        console.warn(
          `Could not find resource for image: ${resolvedPath} (original: ${src})`
        );
        missingResources.value.push(
          `Image: ${resolvedPath} (original: ${src})`
        );
      }
    }
  });
  // Process stylesheets
  doc.querySelectorAll('link[rel="stylesheet"]').forEach((link) => {
    const href = link.getAttribute("href");
    if (href && !href.startsWith("data:") && !href.startsWith("http")) {
      // Resolve the path correctly
      const resolvedPath = resolveRelativePath(currentSpinePath, href);
      const dataUrl = getResourceUrl(resolvedPath);

      if (dataUrl) {
        link.setAttribute("href", dataUrl);
      } else {
        console.warn(
          `Could not find resource for stylesheet: ${resolvedPath} (original: ${href})`
        );
        missingResources.value.push(
          `Stylesheet: ${resolvedPath} (original: ${href})`
        );
      }
    }
  });

  // Also process background images in inline styles
  doc.querySelectorAll("[style*='background']").forEach((element) => {
    const style = element.getAttribute("style");
    if (style && style.includes("url(")) {
      const urlMatch = style.match(/url\(['"]?([^'")]+)['"]?\)/);
      if (
        urlMatch &&
        urlMatch[1] &&
        !urlMatch[1].startsWith("data:") &&
        !urlMatch[1].startsWith("http")
      ) {
        const resolvedPath = resolveRelativePath(currentSpinePath, urlMatch[1]);
        const dataUrl = getResourceUrl(resolvedPath);

        if (dataUrl) {
          const newStyle = style.replace(urlMatch[0], `url("${dataUrl}")`);
          element.setAttribute("style", newStyle);
        }
      }
    }
  });

  return doc.documentElement.outerHTML;
};

const openFile = async () => {
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

    // Read the EPUB file
    book.value = (await invoke("read_epub_file", {
      path: filePath.value,
    })) as EpubBook;
    totalPages.value = book.value.metadata.spine.length;

    // Navigate to the first page
    await navigateToPage(0);

    loading.value = false;
  } catch (error) {
    console.error("Error opening file:", error);
    loading.value = false;
  }
};

const navigateToPage = async (pageIndex: number) => {
  if (!filePath.value || pageIndex < 0 || pageIndex >= totalPages.value) return;

  try {
    loading.value = true;

    // Get the page content
    const content = (await invoke("get_epub_page", {
      path: filePath.value,
      pageIndex,
    })) as string;

    // Process the HTML to replace resources with data URLs
    currentContent.value = processHtml(content);
    currentPage.value = pageIndex;

    loading.value = false;
  } catch (error) {
    console.error("Error navigating to page:", error);
    loading.value = false;
  }
};

const nextPage = () => {
  if (currentPage.value < totalPages.value - 1) {
    navigateToPage(currentPage.value + 1);
  }
};

const prevPage = () => {
  if (currentPage.value > 0) {
    navigateToPage(currentPage.value - 1);
  }
};

const goToTocItem = (index: number) => {
  if (!book.value) return;

  const tocItem = book.value.metadata.toc[index];
  const spineIndex = book.value.metadata.spine.findIndex(
    (path) => path === tocItem
  );

  if (spineIndex !== -1) {
    navigateToPage(spineIndex);
  }

  showToc.value = false;
};

// Handle keyboard navigation
const handleKeyDown = (event: KeyboardEvent) => {
  if (event.key === "ArrowLeft") {
    prevPage();
  } else if (event.key === "ArrowRight") {
    nextPage();
  }
};

onMounted(() => {
  window.addEventListener("keydown", handleKeyDown);
});
</script>

<template>
  <div class="reader-container">
    <!-- Toolbar -->
    <div class="reader-toolbar">
      <button @click="openFile" :disabled="loading">
        {{ loading ? "Loading..." : "Open EPUB" }}
      </button>
      <span v-if="book">{{ currentPage + 1 }} / {{ totalPages }}</span>
      <button @click="showToc = !showToc" :disabled="!book">
        Table of Contents
      </button>
      <button @click="showDebug = !showDebug" :disabled="!book">Debug</button>
    </div>

    <!-- Book Cover or Placeholder -->
    <div v-if="!book" class="placeholder">
      <div class="instruction">Please open an EPUB file to start reading</div>
    </div>

    <!-- TOC Sidebar -->
    <div v-if="showToc && book" class="toc-sidebar">
      <div class="toc-header">
        <h3>Table of Contents</h3>
        <button @click="showToc = false" class="close-btn">×</button>
      </div>
      <ul class="toc-list">
        <li
          v-for="(item, index) in book.metadata.toc"
          :key="index"
          @click="goToTocItem(index)"
        >
          {{ item.replace(/\.x?html$/, "").replace(/[-_]/g, " ") }}
        </li>
      </ul>
    </div>
    <!-- Content Viewer with optional cover display -->
    <div v-if="book" class="content-container">
      <!-- Navigation Arrows -->
      <div
        v-if="currentPage > 0"
        class="nav-arrow nav-arrow-left"
        @click="prevPage"
        title="Previous Page"
      >
        ‹
      </div>
      <div
        v-if="currentPage < totalPages - 1"
        class="nav-arrow nav-arrow-right"
        @click="nextPage"
        title="Next Page"
      >
        ›
      </div>

      <div v-if="currentPage === 0 && coverImage" class="cover-container">
        <img :src="coverImage" alt="Book Cover" class="book-cover" />
      </div>
      <!-- 只在非第一页或第一页没有封面图的情况下显示内容 -->
      <div
        v-if="currentPage > 0 || !coverImage"
        class="content-viewer"
        v-html="currentContent"
      ></div>
      <!-- 第一页且有封面图的情况下，不显示内容部分 -->
      <div
        v-else-if="currentPage === 0 && coverImage"
        class="empty-content"
      ></div>
    </div>

    <!-- Debug Panel -->
    <div v-if="showDebug && book" class="debug-panel">
      <div class="debug-header">
        <h3>Debug Information</h3>
        <button @click="showDebug = false" class="close-btn">×</button>
      </div>
      <div class="debug-content">
        <h4>Current Page</h4>
        <p>Index: {{ currentPage }} / {{ totalPages - 1 }}</p>
        <p>Path: {{ book.metadata.spine[currentPage] }}</p>

        <h4>Missing Resources ({{ missingResources.length }})</h4>
        <ul v-if="missingResources.length > 0" class="debug-list">
          <li v-for="(resource, index) in missingResources" :key="index">
            {{ resource }}
          </li>
        </ul>
        <p v-else>No missing resources</p>

        <h4>Available Resources</h4>
        <ul class="debug-list resources-list">
          <li v-for="(_, key) in book.resources" :key="key">
            {{ key }}
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>
