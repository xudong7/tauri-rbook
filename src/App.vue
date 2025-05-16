<script setup lang="ts">
import { ref, onMounted } from "vue";
import ReaderView from "./components/ReaderView.vue";
import MenuView from "./components/MenuView.vue";
import "./assets/global.css";

// Simple routing system
const currentView = ref<'menu' | 'reader'>('menu');
const selectedFilePath = ref<string>('');

// Listen for the open-book event from MenuView
onMounted(() => {
  window.addEventListener('open-book', ((event: CustomEvent) => {
    selectedFilePath.value = event.detail.filePath;
    currentView.value = 'reader';
  }) as EventListener);
});

// Function to go back to menu
const goBackToMenu = () => {
  currentView.value = 'menu';
};
</script>

<template>
  <el-config-provider>
    <div class="app-container">
      <MenuView v-if="currentView === 'menu'" />
      <ReaderView 
        v-else 
        :initialFilePath="selectedFilePath" 
        @back="goBackToMenu" 
      />
    </div>
  </el-config-provider>
</template>
