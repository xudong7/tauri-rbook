<script setup lang="ts">
import "./assets/global.css";
</script>

<template>
  <el-config-provider>
    <div class="app-container">
      <router-view></router-view>
    </div>
  </el-config-provider>
</template>
