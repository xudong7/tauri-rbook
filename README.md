<div align="center">
  <img src="screenshots/icon.png" alt="RBook Logo" width="240" height="240">
  
# RBook - Modern EPUB eBook Reader

  <p>
    <strong>High Performance · Cross-Platform · Modern Design</strong>
  </p>
  
  <p>
    <a href="#screenshots">Screenshots</a> •
    <a href="#features">Features</a> •
    <a href="#tech-stack">Tech Stack</a> •
    <a href="#core-functionality">Core Functionality</a> •
    <a href="#getting-started">Getting Started</a>
  </p>
  
  <p>
    <a href="README_zh.md">中文</a> |
    <a href="README.md">English</a>
  </p>
</div>

RBook is a modern EPUB eBook reader built with the Tauri framework, combining the frontend advantages of Vue3 with the high-performance backend of Rust to provide a smooth, efficient, and cross-platform reading experience.

## Screenshots

<div align="center">
  <div style="justify-content: space-between;">
    <img src="/screenshots/image-1.png" alt="RBook Library Interface" width="560px" />
    <img src="/screenshots/image-2.png" alt="RBook Reader Interface" width="560px" />
  </div>
  <p style="color: #666; font-size: 12px;">
    RBook Library Interface | RBook Reader Interface
  </p>
</div>

## Features

- 📚 Local EPUB file management and rendering
- 🔖 Smart bookmark management with navigation
- 📋 Complete table of contents navigation
- 🔄 Sort by recently opened or filename
- 🎨 Modern UI with smooth transition animations
- 🚀 High-performance EPUB rendering engine
- 💻 Cross-platform support (Windows, macOS, Linux)

## Tech Stack

### Frontend

- **Vue 3**: Modern reactive framework
- **TypeScript**: Type-safe programming
- **Element Plus**: UI component library
- **EPubJS**: EPUB rendering engine
- **Vue Router**: Page routing management
- **Vite**: Fast build tool

### Backend

- **Rust**: High-performance systems programming language
- **Tauri**: Lightweight, secure desktop application framework
- **Serde**: Efficient serialization/deserialization library
- **Tokio**: Asynchronous runtime

## Core Functionality

### 1. Local EPUB File Processing

Direct EPUB file parsing through the Rust backend without relying on external APIs, ensuring loading speed and offline availability. Compared to the `br-beta` branch that uses external APIs to convert to HTML, this approach significantly improves performance and stability.

### 2. Smart Sorting System

<div>
  <img align="right" src="screenshots/sort.gif" alt="Sorting Demo" width="560" style="margin-left: 20px; border-radius: 6px; box-shadow: 0 4px 8px rgba(0,0,0,0.1);">
  
  <ul>
    <li><b>Time-based Sorting</b>: Tracks and records the last read time for each book</li>
    <li><b>Alphabetical Sorting</b>: Sorts by filename</li>
    <li><b>One-click Toggle</b>: Easily switch between sorting methods via toolbar button</li>
  </ul>
  
  <p><b>Implementation Highlights:</b></p>
  <ul>
    <li>Rust backend stores timestamp information</li>
    <li>Tauri command API for frontend-backend communication</li>
    <li>Vue computed properties for dynamic sorting</li>
  </ul>
</div>

### 3. Bookmarks and Table of Contents System

- **Bookmark Functionality**: Add, edit, delete, and navigate to bookmarks
- **TOC Navigation**: Support for multi-level table of contents
- **Smooth Animations**: Using Vue's `<Transition>` component for smooth panel transitions

Technical Highlights:

- Vue Transition components for animation handling
- CSS transition effects to enhance user experience
- Smart CFI (Content Fragment Identifier) positioning for precise navigation

### 4. Advanced Navigation Strategies

To address navigation challenges in EPUB files, a multi-layered navigation strategy was implemented:

1. Direct navigation attempt
2. Filename-based matching
3. Path correction strategy
4. Anchor handling mechanism
5. Smart fallback approach

### 5. User Interface Optimization

- Modern design language
- Responsive layout that adapts to different screen sizes
- Smooth transition animations for enhanced user experience
- Custom scrollbar styling

## Performance Improvements

Compared to earlier versions, this project achieves the following performance improvements:

<table>
  <tr>
    <td><b>🚀 Parsing Speed</b></td>
    <td>EPUB parsing speed increased by 300%+</td>
  </tr>
  <tr>
    <td><b>🔌 Offline Availability</b></td>
    <td>No external API dependency ensures offline usability</td>
  </tr>
  <tr>
    <td><b>💾 Storage Optimization</b></td>
    <td>Reduced local storage usage</td>
  </tr>
  <tr>
    <td><b>⚡ Resource Efficiency</b></td>
    <td>Optimized memory usage with lower resource consumption</td>
  </tr>
</table>

## Getting Started

### Branch Information

- **Stable Version**: `br-gamma` branch - Recommended for production use
- **Development Version**: `dev` branch - Latest features and experimental changes

### Requirements

1. Node.js & pnpm
2. Rust environment
3. Tauri CLI

### Installation and Running

```bash
# Install dependencies
pnpm install

# Run in development mode
pnpm run tauri dev

# Build production version
pnpm run tauri build
```

## Feature Completion Status

### Frontend Features

- [x] Layout improvements
- [x] Header beautification
- [x] Added control buttons
- [x] Multi-file upload
- [x] Home library page
- [x] TOC page navigation function
- [x] Settings window
- [x] Home page sorting by read time/alphabet
- [x] Bookmark style beautification and animation
- [x] TOC panel animation effects

### Backend Features

- [x] Load EPUB files from default folder
- [x] Save EPUB files
- [x] Upload multiple files simultaneously
- [x] Position marking (bookmarks)
- [x] Save last read time
- [ ] Integrate online eBook search functionality from [Site 1](https://digilibraries.com/) and [Site 2](https://www.gutenberg.org/)

## Developers

<div align="center">
  <table>
    <tr>
      <td align="center">
        <a href="https://github.com/xudong7">
          <img src="https://avatars.githubusercontent.com/u/144703941?v=4&size=64" width="100px;" alt="xudong7"/>
          <br />
          <sub><b>xudong7</b></sub>
        </a>
        <br />
      </td>
        <td align="center">
            <a href="https://github.com/zhongjf25">
            <img src="https://avatars.githubusercontent.com/u/96942496?s=64&v=4" width="100px;" alt="zhongjf25"/>
            <br />
            <sub><b>zhongjf25</b></sub>
            </a>
            <br />
        </td>
        <td align="center">
            <a href="https://github.com/woshouhujiaran">
            <img src="https://avatars.githubusercontent.com/u/182353547?s=64&v=4" width="100px;" alt="woshouhujiaran"/>
            <br />
            <sub><b>woshouhujiaran</b></sub>
            </a>
        </td>
        <td align="center">
            <a href="https://github.com/main-j">
            <img src="https://avatars.githubusercontent.com/u/182583279?s=64&v=4" width="100px;" alt="main-j"/>
            <br />
            <sub><b>main-j</b></sub>
            </a>
        </td>
    </tr>
  </table>

  <p>This project was developed by sophomore students from the School of Software Engineering at Sun Yat-sen University, dedicated to providing an efficient, modern EPUB reading experience.</p>
</div>

## License

<div align="center">
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT" style="max-width: 100%;">
  </a>
  <p>Copyright © 2025 RBook</p>
</div>
