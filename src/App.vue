<script setup lang="ts">
import ReaderView from "./components/ReaderView.vue";
import "./assets/global.css";
</script>

<template>
  <el-config-provider>
    <div class="app-container">
      <ReaderView />
    </div>
  </el-config-provider>
</template>
