## br-gamma

这个分支是通过后端传递epub文件路径以及内容到前端，前端使用epubjs来渲染epub文件。
与`br-beta`分支不同的是，这个分支不依赖外部api，而是实实在在通过本地进行epub文件的渲染。
`br-beta`分支是通过后端调用外部api将epub文件转换为html文件，然后前端渲染html文件，这个方法在网络不稳定的情况下可能会导致加载失败，同时对于造成了本地文件存储的浪费。

### 提升

- 解析epub文件的速度更快
- 支持目录跳转
- 不依赖外部api

## 如何运行

首先，你需要有以下环境：

1. node & pnpm

2. rust

3. tauri

### 运行

在根目录下运行以下命令：

```bash
# 安装依赖
pnpm install
```

```bash
# 运行
pnpm run tauri dev
```

```bash
# 打包
pnpm run tauri build
```

## 需要做的

### Vue

- [x] 排版改进

- [x] header美化

- [x] 添加控制按钮

- [x] 多文件上传

- [x] 主页书库页面

- [x] 目录页跳转功能

- [x] 设置窗口

- [ ] 书签样式美化

- [ ] 主页按阅读时间/首字母排序

### Rust

- [x] 加载默认文件夹下的epub文件

- [x] 保存epub文件

- [x] 多个文件同时上传

- [ ] 标记位置(书签) -- 返回html时可以返回一个json对象，包含书签位置

- [ ] 搜索功能：[网站1](https://digilibraries.com/) 和 [网站2](https://www.gutenberg.org/) 的epub文件

## 介绍

本项目是一个基于Tauri的epub阅读器，使用Vue3作为前端框架，Rust作为后端语言。它的目标是提供一个轻量级、快速、跨平台的epub阅读体验。
本项目的主要功能包括：

- 支持多文件上传
- 支持epub文件的加载和保存
- 支持epub格式电子书的阅读

![项目截图-1](/screenshots/image-1.png)

![项目截图-2](/screenshots/image-2.png)

## 技术栈

- 前端：Vue3 + Vite + TypeScript
- 后端：Rust + Tauri
