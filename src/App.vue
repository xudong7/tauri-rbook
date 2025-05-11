<script setup lang="ts">
import BookSearch from './components/BookSearch.vue';
</script>

<template>
  <div class="app-container">
    <header>
      <div class="header-content">
        <div class="logo">
          <img src="/tauri.svg" alt="Logo" class="logo-icon" />
          <h1>RBook</h1>
        </div>
        <div class="header-actions">
          <a href="https://github.com/yourusername/rbook" target="_blank" class="github-link">
            <span class="github-icon">
              <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24">
                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
              </svg>
            </span>
            GitHub
          </a>
        </div>
      </div>
    </header>
    <main>
      <BookSearch />
    </main>
    <footer>
      <div class="footer-content">
        <p>© 2025 RBook - 电子书籍搜索引擎</p>
        <div class="footer-links">
          <a href="#">关于</a>
          <a href="#">隐私政策</a>
          <a href="#">帮助</a>
        </div>
      </div>
    </footer>
  </div>
</template>

<style>
* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

:root {
  --primary-color: #3b82f6;
  --primary-dark: #1e3a8a;
  --text-color: #1f2937;
  --text-light: #6b7280;
  --background-color: #f9fafb;
  --card-bg: #ffffff;
  --header-bg: #1e3a8a;
  --footer-bg: #1f2937;
  --border-color: #e5e7eb;
}

body {
  font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
  line-height: 1.6;
  background-color: var(--background-color);
  color: var(--text-color);
}

.app-container {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

header {
  background-color: var(--header-bg);
  color: #fff;
  padding: 16px 0;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.header-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 24px;
}

.logo {
  display: flex;
  align-items: center;
}

.logo-icon {
  height: 32px;
  margin-right: 12px;
}

.logo h1 {
  font-size: 1.8rem;
  font-weight: 700;
  letter-spacing: -0.5px;
}

.header-actions {
  display: flex;
  align-items: center;
}

.github-link {
  display: flex;
  align-items: center;
  color: #fff;
  text-decoration: none;
  font-size: 0.95rem;
  opacity: 0.9;
  transition: opacity 0.2s;
}

.github-link:hover {
  opacity: 1;
}

.github-icon {
  margin-right: 8px;
  display: flex;
}

.github-icon svg {
  fill: currentColor;
  width: 20px;
  height: 20px;
}

main {
  flex: 1;
  padding: 24px;
}

footer {
  background-color: var(--footer-bg);
  color: rgba(255, 255, 255, 0.8);
  padding: 24px 0;
  margin-top: 40px;
}

.footer-content {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 24px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.footer-links a {
  color: rgba(255, 255, 255, 0.7);
  text-decoration: none;
  margin-left: 20px;
  font-size: 0.9rem;
  transition: color 0.2s;
}

.footer-links a:hover {
  color: #fff;
}
</style>
