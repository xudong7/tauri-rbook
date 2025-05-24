<div align="center">
  <img src="screenshots/icon.png" alt="RBook Logo" width="240" height="240">
  
# RBook - 现代化EPUB电子书阅读器

  <p>
    <strong>高性能 · 跨平台 · 现代化</strong>
  </p>
  
  <p>
    <a href="#项目截图">截图</a> •
    <a href="#项目特点">特点</a> •
    <a href="#技术栈">技术栈</a> •
    <a href="#核心功能与技术实现">核心功能</a> •
    <a href="#如何运行">安装运行</a>
  </p>
  
  <p>
    <a href="README_zh.md">中文</a> |
    <a href="README.md">English</a>
  </p>
</div>

RBook是一款现代化的EPUB电子书阅读器，基于Tauri框架构建，结合了Vue3的前端优势和Rust的高性能后端，提供了流畅、高效、跨平台的阅读体验。

## 项目截图

<div align="center">
  <div style="justify-content: space-between;">
    <img src="/screenshots/image-1.png" alt="RBook书库界面" width="560px" />
    <img src="/screenshots/image-2.png" alt="RBook阅读界面" width="560px" />
  </div>
  <p style="color: #666; font-size: 12px;">
    RBook书库界面 | RBook阅读器界面
  </p>
</div>

## 项目特点

- 📚 本地EPUB文件管理与渲染
- 🔖 智能书签管理与跳转
- 📋 完整目录导航功能
- 🔄 最近阅读排序功能
- 🎨 现代化UI界面与平滑过渡动画
- 🚀 高性能EPUB渲染引擎
- 💻 跨平台支持（Windows, macOS, Linux）

## 技术栈

### 前端

- **Vue 3**: 现代化的响应式框架
- **TypeScript**: 提供类型安全
- **Element Plus**: UI组件库
- **EPubJS**: EPUB渲染引擎
- **Vue Router**: 页面路由管理
- **Vite**: 快速的构建工具

### 后端

- **Rust**: 高性能系统编程语言
- **Tauri**: 构建轻量级、安全的桌面应用框架
- **Serde**: 高效的序列化/反序列化库
- **Tokio**: 异步运行时

## 核心功能与技术实现

### 1. 本地EPUB文件处理

通过Rust后端直接解析EPUB文件，无需依赖外部API，保证了加载速度和离线可用性。相比`br-beta`分支使用外部API转换HTML的方式，大幅提升了性能和稳定性。

### 2. 智能排序系统

<div>
  <img align="right" src="screenshots/sort.gif" alt="排序演示" width="560" style="margin-left: 20px; border-radius: 6px; box-shadow: 0 4px 8px rgba(0,0,0,0.1);">
  
  <ul>
    <li><b>时间排序</b>: 追踪并记录每本书的最后阅读时间</li>
    <li><b>字母排序</b>: 支持按文件名进行字母排序</li>
    <li><b>一键切换</b>: 通过工具栏按钮轻松切换排序方式</li>
  </ul>
  
  <p><b>实现关键点：</b></p>
  <ul>
    <li>Rust后端存储时间戳信息</li>
    <li>Tauri命令API进行前后端通信</li>
    <li>Vue计算属性实现动态排序</li>
  </ul>
</div>

### 3. 书签与目录系统

- **书签功能**: 支持添加、编辑、删除和跳转
- **目录导航**: 支持多级目录结构
- **平滑动画**: 使用Vue的`<Transition>`组件实现平滑的面板切换效果

技术亮点：

- 使用Vue的Transition组件处理动画
- CSS过渡效果增强用户体验
- 智能CFI定位技术，确保精确跳转

### 4. 高级导航策略

针对EPUB文件中可能出现的导航挑战，实现了多层次导航策略：

1. 直接导航尝试
2. 基于文件名匹配
3. 路径修正策略
4. 锚点处理机制
5. 智能回退方案

### 5. 用户界面优化

- 现代化设计语言
- 响应式布局适应不同屏幕尺寸
- 平滑过渡动画增强用户体验
- 自定义滚动条样式

## 性能提升

相比早期版本，本项目实现了以下性能提升：

<table>
  <tr>
    <td><b>🚀 解析速度</b></td>
    <td>EPUB解析速度提升300%+</td>
  </tr>
  <tr>
    <td><b>🔌 离线可用</b></td>
    <td>无需外部API，保证离线可用性</td>
  </tr>
  <tr>
    <td><b>💾 存储优化</b></td>
    <td>减少本地存储占用</td>
  </tr>
  <tr>
    <td><b>⚡ 资源效率</b></td>
    <td>优化内存使用，降低资源消耗</td>
  </tr>
</table>

## 如何运行

### 分支说明

- **稳定版本**: `br-gamma` 分支 - 推荐生产环境使用
- **开发版本**: `dev` 分支 - 包含最新功能和实验性更改

### 环境要求

1. Node.js & pnpm
2. Rust环境
3. Tauri CLI

### 安装与运行

```bash
# 安装依赖
pnpm install

# 开发模式运行
pnpm run tauri dev

# 构建生产版本
pnpm run tauri build
```

## 功能完成情况

### 前端功能

- [x] 排版改进
- [x] header美化
- [x] 添加控制按钮
- [x] 多文件上传
- [x] 主页书库页面
- [x] 目录页跳转功能
- [x] 设置窗口
- [x] 主页按阅读时间/首字母排序
- [x] 书签样式美化与动画
- [x] 目录面板动画效果

### 后端功能

- [x] 加载默认文件夹下的epub文件
- [x] 保存epub文件
- [x] 多个文件同时上传
- [x] 标记位置(书签)
- [x] 保存最后阅读时间
- [ ] 集成在线电子书搜索功能 [网站1](https://digilibraries.com/) 和 [网站2](https://www.gutenberg.org/) 的epub文件

## 开发者

<div align="center">

[![Contributors](http://contrib.nn.ci/api?repo=xudong7/tauri-rbook)](https://github.com/xudong7/tauri-rbook/graphs/contributors)  

  <p>本项目由中山大学软件工程学院大二学生开发，致力于提供一个高效、现代化的EPUB阅读体验。</p>
</div>

## 许可证

<div align="center">
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT" style="max-width: 100%;">
  </a>
  <p>Copyright © 2025 RBook</p>
</div>
