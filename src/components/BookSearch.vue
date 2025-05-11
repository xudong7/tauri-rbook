<template>
  <div class="book-search">
    <h1>书籍搜索</h1>
    
    <!-- 搜索表单 -->
    <div class="search-form">
      <input
        v-model="keyword"
        type="text"
        placeholder="输入关键词搜索书籍..."
        @keyup.enter="handleSearch"
      />
      <button @click="handleSearch" :disabled="isLoading">
        <span v-if="isLoading">
          <span class="loading-indicator"></span>
          搜索中...
        </span>
        <span v-else>搜索</span>
      </button>
    </div>

    <!-- 错误信息 -->
    <div v-if="error" class="error-message">
      <i class="icon-error"></i>
      {{ error }}
    </div>

    <!-- 加载中提示 -->
    <div v-if="isLoading" class="loading">
      <div class="loading-spinner"></div>
      <p>正在搜索书籍，请稍候...</p>
    </div>

    <!-- 搜索结果 -->
    <div v-if="!isLoading && bookList && bookList.books.length > 0" class="search-results">
      <h2>搜索结果 <span class="result-count">共 {{ bookList.books.length }} 项</span></h2>
      
      <div class="book-list">
        <div v-for="(book, index) in bookList.books" :key="index" class="book-card">
          <!-- 书籍封面 -->
          <div class="book-cover">
            <img
              v-if="book.cover_url"
              :src="book.cover_url"
              :alt="book.title"
              @error="$event.target.src = 'https://via.placeholder.com/150x200?text=No+Cover'"
            />
            <div v-else class="no-cover">
              <span class="no-cover-text">暂无封面</span>
            </div>
          </div>
          
          <!-- 书籍信息 -->
          <div class="book-info">
            <h3 class="book-title">
              <a :href="book.book_url" target="_blank" rel="noopener noreferrer">
                {{ book.title }}
              </a>
            </h3>
            
            <div class="book-meta">
              <div v-if="book.author" class="book-author">
                <span class="meta-label">作者:</span> {{ book.author }}
              </div>
              <div v-if="book.year" class="book-year">
                <span class="meta-label">年份:</span> {{ book.year }}
              </div>
              <div v-if="book.publisher" class="book-publisher">
                <span class="meta-label">出版商:</span> {{ book.publisher }}
              </div>
            </div>
            
            <div v-if="book.description" class="book-description">
              {{ book.description }}
            </div>
            
            <div class="book-actions">
              <a :href="book.book_url" target="_blank" rel="noopener noreferrer" class="view-button">
                查看详情
              </a>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 无搜索结果提示 -->
    <div v-else-if="!isLoading && searched" class="no-results">
      <div class="no-results-icon">🔍</div>
      <h3>未找到相关书籍</h3>
      <p>请尝试其他关键词，或者检查是否有拼写错误</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue';
import { searchBooks } from '../api/books';
import type { BookList } from '../types/model';

// 响应式状态
const keyword = ref('');
const isLoading = ref(false);
const error = ref('');
const bookList = ref<BookList | null>(null);
const searched = ref(false);

// 搜索书籍方法
async function handleSearch() {
  if (!keyword.value.trim()) {
    error.value = '请输入搜索关键词';
    return;
  }

  try {
    error.value = '';
    isLoading.value = true;
    searched.value = true;
    bookList.value = await searchBooks(keyword.value);
    console.log('搜索结果:', bookList.value);
  } catch (err: any) {
    error.value = `搜索出错: ${err.message || err}`;
    bookList.value = null;
  } finally {
    isLoading.value = false;
  }
}
</script>

<style scoped>
.book-search {
  max-width: 1200px;
  margin: 0 auto;
  padding: 24px;
  color: #333;
}

h1 {
  font-size: 2.2rem;
  margin-bottom: 24px;
  color: #1e3a8a;
  text-align: center;
}

/* 搜索表单 */
.search-form {
  display: flex;
  margin-bottom: 32px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
  border-radius: 8px;
  overflow: hidden;
}

input {
  flex: 1;
  padding: 16px 20px;
  font-size: 1rem;
  border: 2px solid #e5e7eb;
  border-right: none;
  border-radius: 8px 0 0 8px;
  outline: none;
  transition: all 0.3s;
}

input:focus {
  border-color: #3b82f6;
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.3);
}

button {
  padding: 0 30px;
  background-color: #3b82f6;
  color: white;
  font-weight: 600;
  border: none;
  border-radius: 0 8px 8px 0;
  cursor: pointer;
  font-size: 1rem;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
  min-width: 120px;
}

button:hover {
  background-color: #2563eb;
}

button:active {
  transform: translateY(1px);
}

button:disabled {
  background-color: #9ca3af;
  cursor: not-allowed;
}

.loading-indicator {
  display: inline-block;
  width: 16px;
  height: 16px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-radius: 50%;
  border-top-color: #fff;
  animation: spin 1s ease-in-out infinite;
  margin-right: 8px;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

/* 错误信息 */
.error-message {
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 16px 0;
  padding: 12px;
  color: #b91c1c;
  background-color: #fee2e2;
  border-radius: 8px;
  border-left: 4px solid #ef4444;
  font-size: 0.95rem;
}

.icon-error {
  width: 20px;
  height: 20px;
  display: inline-block;
  margin-right: 8px;
  position: relative;
}

.icon-error:before {
  content: "!";
  font-weight: bold;
  color: #b91c1c;
}

/* 加载中状态 */
.loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  margin: 48px 0;
  color: #6b7280;
}

.loading-spinner {
  width: 50px;
  height: 50px;
  border: 4px solid rgba(59, 130, 246, 0.2);
  border-radius: 50%;
  border-top-color: #3b82f6;
  animation: spin 1s linear infinite;
  margin-bottom: 16px;
}

.loading p {
  font-size: 1rem;
}

/* 搜索结果 */
.search-results h2 {
  font-size: 1.5rem;
  margin-bottom: 24px;
  color: #1f2937;
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 2px solid #e5e7eb;
  padding-bottom: 10px;
}

.result-count {
  font-size: 1rem;
  color: #6b7280;
  font-weight: normal;
}

.book-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 24px;
}

.book-card {
  border: 1px solid #e5e7eb;
  border-radius: 12px;
  overflow: hidden;
  transition: all 0.3s ease;
  background-color: white;
  display: flex;
  flex-direction: column;
  height: 100%;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
}

.book-card:hover {
  transform: translateY(-5px);
  box-shadow: 0 10px 20px rgba(0, 0, 0, 0.1);
  border-color: #bfdbfe;
}

.book-cover {
  height: 220px;
  overflow: hidden;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: #f3f4f6;
  position: relative;
}

.book-cover img {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
  transition: transform 0.3s ease;
}

.book-cover:hover img {
  transform: scale(1.05);
}

.no-cover {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 100%;
  background: linear-gradient(135deg, #dbeafe, #eff6ff);
  color: #3b82f6;
}

.no-cover-text {
  font-style: italic;
  opacity: 0.8;
  border: 2px dashed #93c5fd;
  padding: 10px 15px;
  border-radius: 6px;
}

.book-info {
  padding: 20px;
  flex: 1;
  display: flex;
  flex-direction: column;
  background-color: #fff;
}

.book-title {
  margin-top: 0;
  margin-bottom: 16px;
  font-size: 1.2rem;
  line-height: 1.4;
}

.book-title a {
  color: #1e3a8a;
  text-decoration: none;
  transition: color 0.2s;
}

.book-title a:hover {
  color: #3b82f6;
}

.book-meta {
  margin-bottom: 16px;
}

.book-author,
.book-year,
.book-publisher {
  margin-bottom: 8px;
  font-size: 0.9rem;
  color: #4b5563;
}

.meta-label {
  font-weight: 600;
  color: #6b7280;
}

.book-description {
  margin-top: 12px;
  font-size: 0.9rem;
  color: #4b5563;
  overflow: hidden;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  line-height: 1.6;
  margin-bottom: 16px;
  flex: 1;
}

.book-actions {
  margin-top: auto;
  display: flex;
}

.view-button {
  display: inline-block;
  padding: 8px 16px;
  background-color: #dbeafe;
  color: #1e40af;
  text-decoration: none;
  border-radius: 6px;
  font-weight: 500;
  font-size: 0.9rem;
  transition: all 0.2s;
  text-align: center;
}

.view-button:hover {
  background-color: #bfdbfe;
  transform: translateY(-2px);
}

/* 无搜索结果 */
.no-results {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  margin: 60px 0;
  color: #6b7280;
  text-align: center;
}

.no-results-icon {
  font-size: 3rem;
  margin-bottom: 16px;
  opacity: 0.6;
}

.no-results h3 {
  font-size: 1.3rem;
  color: #374151;
  margin-bottom: 8px;
}

.no-results p {
  color: #6b7280;
  font-size: 1rem;
}

/* 响应式布局 */
@media (max-width: 768px) {
  .book-list {
    grid-template-columns: 1fr;
  }
  
  .search-form {
    flex-direction: column;
    box-shadow: none;
  }
  
  input {
    border-radius: 8px;
    border-right: 2px solid #e5e7eb;
    margin-bottom: 10px;
  }
  
  button {
    border-radius: 8px;
    width: 100%;
  }
}
</style>
