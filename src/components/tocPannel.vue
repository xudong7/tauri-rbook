<script setup lang="ts">
import { Close } from "@element-plus/icons-vue";
import type { TocItem } from "../types/model";

const props = defineProps<{
  tableOfContents: TocItem[];
  showToc: boolean;
  book: any;
  rendition: any;
}>();

const emit = defineEmits<{
  (e: "toggleToc"): void;
}>();

/**
 * 切换目录显示
 */
const toggleToc = () => {
  emit("toggleToc");
};

/**
 * 准备href用于导航
 */
const prepareHref = (href: string): string => {
  let cleanHref = href;

  // 如果href以#开头，需要特殊处理
  if (cleanHref.startsWith("#")) {
    const currentUrl =
      props.rendition.location && props.rendition.location.start
        ? props.rendition.location.start.href
        : "";
    const baseUrl = currentUrl.split("#")[0];
    cleanHref = baseUrl + cleanHref;
  }

  return cleanHref;
};

/**
 * 使用spine项导航
 */
const navigateUsingSpineItem = (spineItem: any) => {
  console.log("Found spine item:", spineItem);
  props.rendition.display(spineItem.href).catch((error: Error) => {
    console.error("Failed to navigate with spine item href:", error);
  });
};

/**
 * 使用href导航，失败时尝试备选策略
 */
const navigateUsingHref = (href: string) => {
  console.log("No spine item found directly. Trying alternative approaches...");

  props.rendition.display(href).catch((error: Error) => {
    console.error("Failed to navigate with provided href:", error);
    tryFallbackNavigationStrategies(href);
  });
};

/**
 * 尝试备选导航策略
 */
const tryFallbackNavigationStrategies = (href: string) => {
  const fileName = href.split("/").pop() || "";
  let matchedItem = findMatchingSpineItem(href, fileName);

  if (matchedItem) {
    console.log(
      "Found matching chapter using fallback strategies:",
      matchedItem.href
    );
    props.rendition.display(matchedItem.href);
  } else {
    // 最后的尝试：使用第一个spine项目
    useFirstChapterAsFallback();
  }
};

/**
 * 查找匹配的spine项
 */
const findMatchingSpineItem = (href: string, fileName: string) => {
  // 策略1: 根据文件名匹配
  let matchedItem = props.book.spine.items.find((item: any) =>
    item.href.includes(fileName)
  );

  // 策略2: 检查href是否包含在spine项的href中
  if (!matchedItem) {
    matchedItem = props.book.spine.items.find((item: any) =>
      href.includes(item.href)
    );
  }

  // 策略3: 检查是否是相对路径问题，尝试添加基本路径
  if (!matchedItem && !href.startsWith("/")) {
    const possibleFullPath = "/" + href;
    matchedItem = props.book.spine.items.find(
      (item: any) =>
        item.href === possibleFullPath || item.href.includes(fileName)
    );
  }

  // 策略4: 如果是一个锚点引用，尝试在当前章节内导航
  if (!matchedItem && href.includes("#")) {
    const currentSpinePosition =
      props.book.rendition.currentLocation().start.index;
    if (
      currentSpinePosition >= 0 &&
      currentSpinePosition < props.book.spine.items.length
    ) {
      matchedItem = props.book.spine.items[currentSpinePosition];
    }
  }

  return matchedItem;
};

/**
 * 使用第一章作为备选导航选项
 */
const useFirstChapterAsFallback = () => {
  console.error(
    "No matching chapter found. Attempting to use the first chapter as fallback."
  );
  if (props.book.spine.items.length > 0) {
    props.rendition.display(props.book.spine.items[0].href);
  }
};

/**
 * 跳转到指定章节
 */
const navigateToChapter = (href: string) => {
  if (!props.rendition) return;

  try {
    const cleanHref = prepareHref(href);
    console.log("Navigating to chapter:", cleanHref);

    // 尝试使用多种策略导航到章节
    const spineItem = props.book.spine.get(cleanHref);

    if (spineItem) {
      // 直接找到了spine项
      navigateUsingSpineItem(spineItem);
    } else {
      // 尝试直接使用提供的href
      navigateUsingHref(cleanHref);
    }

    // 关闭目录面板
    emit("toggleToc");
  } catch (error: unknown) {
    console.error(
      "Error navigating to chapter:",
      error instanceof Error ? error.message : error
    );
  }
};
</script>

<template>
  <!-- 目录面板 -->
  <div class="toc-panel" v-if="showToc">
    <div class="toc-header">
      <span class="toc-title">目录</span>
      <button class="close-toc" @click="toggleToc">
        <el-icon :size="20"><Close /></el-icon>
      </button>
    </div>
    <div class="toc-content">
      <div
        v-for="(item, index) in tableOfContents"
        :key="index"
        class="toc-item"
        :style="{ paddingLeft: `${item.level ? item.level * 12 : 0}px` }"
      >
        <a
          href="#"
          @click.prevent="navigateToChapter(item.href)"
          class="toc-link"
          :title="item.href"
        >
          {{ item.label }}
        </a>
        <!-- 处理嵌套目录 -->
        <div
          v-if="item.subitems && item.subitems.length > 0"
          class="toc-subitems"
        >
          <div
            v-for="(subitem, subIndex) in item.subitems"
            :key="`${index}-${subIndex}`"
            class="toc-item"
            :style="{
              paddingLeft: `${subitem.level ? subitem.level * 12 : 12}px`,
            }"
          >
            <a
              href="#"
              @click.prevent="navigateToChapter(subitem.href)"
              class="toc-link"
              :title="subitem.href"
            >
              {{ subitem.label }}
            </a>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* TOC Panel Styles */
.toc-panel {
  position: fixed;
  top: 48px;
  left: 0;
  width: 300px;
  height: calc(100vh - 48px);
  background-color: #ffffff;
  box-shadow: 2px 0 10px rgba(0, 0, 0, 0.1);
  z-index: 1000;
  display: flex;
  flex-direction: column;
  animation: slide-in 0.3s ease-in-out;
  overflow: hidden;
}

@keyframes slide-in {
  from {
    transform: translateX(-100%);
  }
  to {
    transform: translateX(0);
  }
}

.toc-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  border-bottom: 1px solid #f0f0f0;
}

.toc-title {
  font-size: 18px;
  font-weight: 500;
  color: #333;
}

.close-toc {
  background: none;
  border: none;
  cursor: pointer;
  color: #909399;
  padding: 4px;
  border-radius: 4px;
}

.close-toc:hover {
  background-color: #f5f7fa;
  color: #409eff;
}

.toc-content {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
}

.toc-item {
  margin-bottom: 12px;
}

.toc-subitems {
  margin-top: 8px;
  margin-bottom: 8px;
}

.toc-link {
  color: #606266;
  text-decoration: none;
  font-size: 14px;
  display: block;
  padding: 8px 12px;
  border-radius: 4px;
  transition: all 0.2s;
}

.toc-link:hover {
  background-color: #ecf5ff;
  color: #409eff;
}
</style>
