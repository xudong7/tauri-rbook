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

- [ ] header美化

- [x] 添加控制按钮

- [ ] 目录页跳转功能

- [x] 多文件上传

- [x] 主页书库页面

### Rust

- [x] 加载默认文件夹下的epub文件

- [x] 保存epub文件

- [ ] 标记位置(书签) -- 返回html时可以返回一个json对象，包含书签位置

- [x] 多个文件同时上传

- [ ] 搜索功能：[网站1](https://digilibraries.com/) 和 [网站2](https://www.gutenberg.org/) 的epub文件
