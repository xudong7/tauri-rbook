<script setup lang="ts">
import { ref, onMounted } from "vue";
import ReaderView from "./components/ReaderView.vue";
import MenuView from "./components/MenuView.vue";
import SearchView from "./components/SearchView.vue";
import "./assets/global.css";

// Simple routing system
const currentView = ref<"menu" | "reader" | "search">("menu");
const selectedFilePath = ref<string>("");

// Listen for events
onMounted(() => {
  // Open book event
  window.addEventListener("open-book", ((event: CustomEvent) => {
    selectedFilePath.value = event.detail.filePath;
    currentView.value = "reader";
  }) as EventListener);

  // Open search event
  window.addEventListener("open-search", (() => {
    currentView.value = "search";
  }) as EventListener);

  // Back to menu event (used after download completion)
  window.addEventListener("back-to-menu", (() => {
    currentView.value = "menu";
  }) as EventListener);
});

// Function to go back to menu
const goBackToMenu = () => {
  currentView.value = "menu";
};
</script>

<template>
  <el-config-provider>
    <div class="app-container">
      <MenuView v-if="currentView === 'menu'" />
      <ReaderView
        v-if="currentView === 'reader'"
        :initialFilePath="selectedFilePath"
        @back="goBackToMenu"
      />
      <SearchView v-if="currentView === 'search'" @back="goBackToMenu" />
    </div>
  </el-config-provider>
</template>
