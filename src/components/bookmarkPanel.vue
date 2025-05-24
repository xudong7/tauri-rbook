<script setup lang="ts">
import { ElMessage, ElMessageBox } from "element-plus";
import {
  Close,
  Edit,
  ArrowRight,
  Star,
  StarFilled,
} from "@element-plus/icons-vue";
import { invoke } from "@tauri-apps/api/core";
import type { BookMark, Mark } from "../types/model";

const props = defineProps<{
  bookmarks: BookMark;
  showBookmarks: boolean;
  currentPage: number;
  rendition: any;
  book: any;
  currentBookPath: string;
}>();

const emit = defineEmits<{
  (e: "toggleBookmarks"): void;
  (e: "bookmarkUpdated"): void;
}>();

/**
 * 切换书签面板显示
 */
// const toggleBookmarks = () => {
//   emit("toggleBookmarks");
// };

/**
 * 检查当前页是否已有书签
 */
const hasBookmarkOnCurrentPage = (): boolean => {
  return props.bookmarks.list.some((mark) => mark.page === props.currentPage);
};

/**
 * 添加或删除书签
 * @param action 0:添加, 1:删除
 * @param bookmarkPage 指定的书签页码，如果不提供则使用当前页
 * @param bookmarkCfi 指定的CFI，如果不提供则使用当前位置的CFI
 * @param content 书签备注内容
 */
const updateBookmark = async (
  action: number = 0,
  bookmarkPage?: number,
  bookmarkCfi?: string,
  content?: string
) => {
  if (!props.rendition || !props.currentBookPath) return;

  try {
    // 获取页码，优先使用参数提供的页码，否则使用当前页
    const page = bookmarkPage !== undefined ? bookmarkPage : props.currentPage;

    // 获取CFI (Content Fragment Identifier)，用于精确定位
    let cfi = bookmarkCfi;
    if (!cfi && action === 0) {
      // 如果没有指定CFI且是添加书签，获取当前位置的CFI
      const location = props.rendition.currentLocation();
      if (location && location.start) {
        cfi = location.start.cfi;
        console.log(`获取当前位置CFI: ${cfi}`);
      }
    }

    // 书签内容，默认为空字符串
    const bookmarkContent = content !== undefined ? content : "";

    // 记录操作的页码信息
    console.log(
      `操作书签: action=${action}, page=${page}, cfi=${cfi}, content=${bookmarkContent}`
    );

    // 调用后端保存书签
    await invoke<string>("save_bookmark_command", {
      bookPath: props.currentBookPath,
      page,
      content: bookmarkContent,
      width: window.innerWidth,
      height: window.innerHeight,
      cfi,
      action,
    });

    // 发布书签更新事件
    emit("bookmarkUpdated");

    // 显示成功消息
    ElMessage({
      type: "success",
      message: action === 1 ? "书签已删除" : "书签已添加",
      duration: 2000,
    });
  } catch (error) {
    console.error("操作书签失败:", error);
    ElMessage({
      type: "error",
      message: "操作书签失败",
      duration: 2000,
    });
  }
};

/**
 * 跳转到书签页
 */
const navigateToBookmark = (mark: Mark) => {
  if (!props.rendition || !props.book) return;

  try {
    console.log("跳转到书签页:", mark);

    // 优先使用CFI进行精确导航
    if (mark.cfi) {
      console.log(`使用CFI精确跳转: ${mark.cfi}`);
      props.rendition
        .display(mark.cfi)
        .then(() => {
          console.log("成功使用CFI跳转到精确位置");
          // 关闭书签面板
          emit("toggleBookmarks");
        })
        .catch((err: Error) => {
          console.error("使用CFI跳转失败，尝试使用页码跳转:", err);
          // 如果CFI导航失败，回退到基于页面索引的导航
          navigateByPageIndex(mark.page);
        });
    } else {
      // 如果没有CFI，使用页码导航
      console.log("无CFI信息，使用页码导航");
      navigateByPageIndex(mark.page);
    }
  } catch (error) {
    console.error("跳转到书签失败:", error);
    ElMessage({
      type: "error",
      message: "跳转到书签失败",
      duration: 2000,
    });
  }
};

/**
 * 使用页码索引进行导航（作为后备方案）
 */
const navigateByPageIndex = (page: number) => {
  if (!props.rendition || !props.book) return;

  // 页面索引处理 (确保页面索引在合理范围内)
  const pageIndex = Math.min(
    Math.max(0, page),
    props.book.spine.items.length - 1
  );
  console.log(
    "调整后的页面索引:",
    pageIndex,
    "总章节数:",
    props.book.spine.items.length
  );

  // 跳转到指定页码对应的章节
  const spineItem = props.book.spine.items[pageIndex];
  if (spineItem) {
    console.log("找到对应章节:", spineItem);
    props.rendition
      .display(spineItem.href)
      .then(() => {
        console.log("成功跳转到书签章节");
        // 关闭书签面板
        emit("toggleBookmarks");
      })
      .catch((err: Error) => {
        console.error("章节跳转错误:", err);
        ElMessage({
          type: "error",
          message: "章节跳转失败",
          duration: 2000,
        });
      });
  } else {
    console.error("未找到对应页码的章节:", pageIndex);
    ElMessage({
      type: "error",
      message: "未找到对应页码的章节",
      duration: 2000,
    });
  }
};

/**
 * 编辑书签备注
 */
const editBookmarkContent = (mark: Mark) => {
  ElMessageBox.prompt("请输入书签备注:", "编辑书签", {
    confirmButtonText: "确定",
    cancelButtonText: "取消",
    inputValue: mark.content || "",
    type: "warning",
    inputType: "textarea",
    inputPlaceholder: "输入备注内容...",
    showCancelButton: true,
    distinguishCancelAndClose: true,
  })
    .then(({ value }) => {
      // 保存更新后的书签内容
      updateBookmark(0, mark.page, mark.cfi, value);
    })
    .catch(() => {
      // 用户取消编辑
      ElMessage({
        type: "info",
        message: "取消编辑",
        duration: 2000,
      });
    });
};

/**
 * 切换当前页的书签状态
 */
const toggleCurrentPageBookmark = async () => {
  if (hasBookmarkOnCurrentPage()) {
    await updateBookmark(1); // 删除书签
  } else {
    // 添加书签时，弹出输入框询问备注内容
    ElMessageBox.prompt("请输入书签备注:", "添加书签", {
      confirmButtonText: "确定",
      cancelButtonText: "取消",
      inputType: "textarea",
      inputPlaceholder: "输入备注内容...",
      type: "info",
      showCancelButton: true,
      distinguishCancelAndClose: true,
    })
      .then(({ value }) => {
        // 添加带备注的书签
        updateBookmark(0, undefined, undefined, value);
      })
      .catch(() => {
        // 用户取消添加
        ElMessage({
          type: "info",
          message: "取消添加书签",
          duration: 2000,
        });
      });
  }
};
</script>

<template>
  <!-- 书签面板 -->
  <Transition name="slide-fade">
    <div class="bookmark-panel" v-if="showBookmarks">
      <!--
        <div class="bookmark-header">
          <span class="bookmark-title">书签</span>
          <button class="close-bookmark" @click="toggleBookmarks">
            <el-icon :size="20"><Close /></el-icon>
          </button>
        </div>
      -->
      <div class="bookmark-content">
        <div v-if="bookmarks.list.length === 0" class="no-bookmarks">
          暂无书签，点击下方按钮添加书签
        </div>
        <div v-else>
          <div
            v-for="(mark, index) in bookmarks.list"
            :key="index"
            class="bookmark-item"
          >
            <div class="bookmark-info">
              <span class="bookmark-page">第 {{ mark.page + 1 }} 页</span>
              <div class="bookmark-content-text">
                {{ mark.content || "暂无备注" }}
              </div>
            </div>
            <div class="bookmark-actions">
              <button
                class="edit-bookmark"
                @click="editBookmarkContent(mark)"
                title="编辑备注"
              >
                <el-icon :size="16"><Edit /></el-icon>
              </button>
              <button
                class="goto-bookmark"
                @click="navigateToBookmark(mark)"
                title="跳转到书签"
              >
                <el-icon :size="16"><ArrowRight /></el-icon>
              </button>
              <button
                class="remove-bookmark"
                @click="updateBookmark(1, mark.page)"
                title="删除书签"
              >
                <el-icon :size="16"><Close /></el-icon>
              </button>
            </div>
          </div>
        </div>
      </div>
      <div class="bookmark-footer">
        <button
          class="bookmark-action-button"
          @click="toggleCurrentPageBookmark"
        >
          <el-icon v-if="hasBookmarkOnCurrentPage()" :size="16">
            <StarFilled />
          </el-icon>
          <el-icon v-else :size="16">
            <Star />
          </el-icon>
          {{
            hasBookmarkOnCurrentPage() ? "取消当前页书签" : "将当前页加入书签"
          }}
        </button>
      </div>
    </div>
  </Transition>
</template>

<style scoped>
/* 书签面板样式 */
.bookmark-panel {
  position: fixed;
  top: 48px;
  left: 0;
  width: 300px;
  height: calc(100vh - 48px);
  background-color: var(--app-background);
  box-shadow: 2px 0 10px rgba(0, 0, 0, 0.1);
  z-index: 1000;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  transition: background-color 0.3s ease;
}

/* Slide-fade transition */
.slide-fade-enter-active,
.slide-fade-leave-active {
  transition: all 0.3s ease-out;
}

.slide-fade-enter-from,
.slide-fade-leave-to {
  transform: translateX(-100%);
  opacity: 0;
}

.bookmark-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  border-bottom: 1px solid var(--app-border);
  transition: border-color 0.3s ease;
}

.bookmark-title {
  font-size: 18px;
  font-weight: 500;
  color: var(--app-text-color);
  transition: color 0.3s ease;
}

.close-bookmark {
  background: none;
  border: none;
  cursor: pointer;
  color: var(--app-text-secondary);
  padding: 4px;
  border-radius: 4px;
  transition: all 0.3s ease;
}

.close-bookmark:hover {
  background-color: var(--app-surface);
  color: var(--app-accent);
}

.bookmark-content {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
  scrollbar-width: thin;
  scrollbar-color: var(--app-scrollbar-thumb) var(--app-scrollbar-track);
}

.bookmark-content::-webkit-scrollbar {
  width: 6px;
}

.bookmark-content::-webkit-scrollbar-track {
  background: var(--app-scrollbar-track);
  border-radius: 10px;
}

.bookmark-content::-webkit-scrollbar-thumb {
  background: var(--app-scrollbar-thumb);
  border-radius: 10px;
}

.bookmark-content::-webkit-scrollbar-thumb:hover {
  background: var(--app-scrollbar-thumb-hover);
}

.bookmark-item {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding: 12px 0;
  border-bottom: 1px solid var(--app-border);
  transition: border-color 0.3s ease;
}

.bookmark-info {
  flex: 1;
  margin-right: 10px;
}

.bookmark-page {
  font-size: 14px;
  font-weight: 500;
  color: var(--app-text-color);
  margin-bottom: 4px;
  transition: color 0.3s ease;
}

.bookmark-content-text {
  font-size: 13px;
  color: var(--app-text-secondary);
  margin-top: 4px;
  word-break: break-word;
  line-height: 1.4;
  max-height: 80px;
  overflow-y: auto;
  padding-left: 2px;
  scrollbar-width: thin;
  scrollbar-color: var(--app-scrollbar-thumb) var(--app-scrollbar-track);
  transition: color 0.3s ease;
}

.bookmark-content-text::-webkit-scrollbar {
  width: 4px;
}

.bookmark-content-text::-webkit-scrollbar-track {
  background: var(--app-scrollbar-track);
}

.bookmark-content-text::-webkit-scrollbar-thumb {
  background: var(--app-scrollbar-thumb);
  border-radius: 4px;
}

.bookmark-actions {
  display: flex;
  gap: 8px;
  align-self: flex-start;
  margin-top: 2px;
}

.goto-bookmark,
.remove-bookmark,
.edit-bookmark {
  background: none;
  border: none;
  cursor: pointer;
  color: var(--app-text-secondary);
  padding: 4px;
  border-radius: 4px;
  transition: all 0.3s ease;
}

.goto-bookmark:hover {
  background-color: var(--app-surface);
  color: var(--app-accent);
}

.edit-bookmark:hover {
  background-color: var(--app-surface);
  color: var(--app-success);
}

.remove-bookmark:hover {
  background-color: var(--app-surface);
  color: var(--app-danger);
}

.no-bookmarks {
  text-align: center;
  color: var(--app-text-secondary);
  padding: 20px 0;
  transition: color 0.3s ease;
}

.bookmark-footer {
  padding: 16px;
  border-top: 1px solid var(--app-border);
  transition: border-color 0.3s ease;
}

.bookmark-action-button {
  width: 100%;
  padding: 8px 16px;
  background-color: var(--app-accent-light);
  color: var(--app-accent);
  border: 1px solid var(--app-accent-border);
  border-radius: 4px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  transition: all 0.3s ease;
}

.bookmark-action-button:hover {
  background-color: var(--app-accent);
  color: var(--app-accent-text);
  border-color: var(--app-accent);
}
</style>
