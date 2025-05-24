import type { ThemeColors } from "../types/model";

export type Theme = "light" | "dark" | "sepia";

export const themeColors: Record<Theme, ThemeColors> = {
  light: {
    background: "#ffffff",
    surface: "#f5f5f5",
    text: "#303133",
    textSecondary: "#606266",
    border: "#e0e0e0",
    accent: "#409eff",
  },
  dark: {
    background: "#1a1a1a",
    surface: "#2c2c2c",
    text: "#e5e5e5",
    textSecondary: "#b3b3b3",
    border: "#404040",
    accent: "#66b1ff",
  },
  sepia: {
    background: "#f4f1ea",
    surface: "#ebe5d6",
    text: "#4a4a3a",
    textSecondary: "#6a6a5a",
    border: "#d4c5a9",
    accent: "#8b7355",
  },
};

export class ThemeManager {
  private currentTheme: Theme = "light";

  constructor() {
    this.loadThemeFromStorage();
    this.setupStorageListener();
  }
  private setupStorageListener() {
    // 监听localStorage变化，实现跨窗口主题同步
    window.addEventListener('storage', (e) => {
      if (e.key === 'app-theme' && e.newValue) {
        if (this.isValidTheme(e.newValue)) {
          this.currentTheme = e.newValue as Theme;
          this.applyTheme(this.currentTheme);
          console.log('主题已从其他窗口同步:', this.currentTheme);
        }
      }
    });
    
    // 监听自定义主题变化事件，实现同窗口内主题同步
    window.addEventListener('themeChanged', (e: any) => {
      const { theme } = e.detail;
      if (theme !== this.currentTheme && this.isValidTheme(theme)) {
        this.currentTheme = theme;
        this.applyTheme(this.currentTheme);
        console.log('主题已从同窗口同步:', this.currentTheme);
      }
    });
  }

  private loadThemeFromStorage() {
    try {
      const saved = localStorage.getItem("app-theme");
      if (saved && this.isValidTheme(saved)) {
        this.currentTheme = saved as Theme;
      }
    } catch (error) {
      console.warn("Failed to load theme from storage:", error);
    }
  }

  private isValidTheme(theme: string): theme is Theme {
    return ["light", "dark", "sepia"].includes(theme);
  }
  public setTheme(theme: Theme) {
    this.currentTheme = theme;
    this.applyTheme(theme);
    this.saveThemeToStorage(theme);
    
    // 触发自定义事件，用于同窗口内的主题同步
    window.dispatchEvent(new CustomEvent('themeChanged', { 
      detail: { theme } 
    }));
  }

  public getCurrentTheme(): Theme {
    return this.currentTheme;
  }

  public getThemeColors(theme?: Theme): ThemeColors {
    return themeColors[theme || this.currentTheme];
  }

  private applyTheme(theme: Theme) {
    const colors = themeColors[theme];
    const root = document.documentElement;

    // 应用 CSS 变量
    root.style.setProperty("--app-background", colors.background);
    root.style.setProperty("--app-surface", colors.surface);
    root.style.setProperty("--app-text-color", colors.text);
    root.style.setProperty("--app-text-secondary", colors.textSecondary);
    root.style.setProperty("--app-border", colors.border);
    root.style.setProperty("--app-accent", colors.accent);

    // 更新 body 类名
    document.body.className = document.body.className.replace(/theme-\w+/g, "");
    document.body.classList.add(`theme-${theme}`);

    // 应用特殊样式
    this.applySpecialStyles(theme);
  }

  private applySpecialStyles(theme: Theme) {
    const existingStyle = document.getElementById("dynamic-theme-styles");
    if (existingStyle) {
      existingStyle.remove();
    }

    const style = document.createElement("style");
    style.id = "dynamic-theme-styles";

    const colors = themeColors[theme];

    style.textContent = `
      /* 动态主题样式 */
      .theme-${theme} {
        background-color: ${colors.background};
        color: ${colors.text};
      }
      
      .theme-${theme} .reader-toolbar,
      .theme-${theme} .settings-header,
      .theme-${theme} .settings-section {
        background-color: ${colors.background};
        border-color: ${colors.border};
      }
      
      .theme-${theme} .main-container,
      .theme-${theme} .reader-container {
        background-color: ${colors.surface};
      }
      
      .theme-${theme} .book-title {
        color: ${colors.text};
      }
      
      .theme-${theme} .book-page-info {
        color: ${colors.textSecondary};
      }
      
      .theme-${theme} .icon-button {
        color: ${colors.textSecondary};
      }
      
      .theme-${theme} .icon-button:hover {
        background-color: ${
          theme === "dark"
            ? "#404040"
            : theme === "sepia"
            ? "#e0d5c0"
            : "#f5f7fa"
        };
        color: ${colors.accent};
      }
      
      .theme-${theme} .toc-panel {
        background-color: ${colors.background};
        box-shadow: 2px 0 10px ${
          theme === "dark" ? "rgba(0, 0, 0, 0.3)" : "rgba(0, 0, 0, 0.1)"
        };
      }
      
      .theme-${theme} .toc-link {
        color: ${colors.textSecondary};
      }
      
      .theme-${theme} .toc-link:hover {
        background-color: ${
          theme === "dark"
            ? "#3a3a3a"
            : theme === "sepia"
            ? "#e8dcc6"
            : "#ecf5ff"
        };
        color: ${colors.accent};
      }
      
      .theme-${theme} .bookmark-panel {
        background-color: ${colors.background};
      }
      
      .theme-${theme} .bookmark-item {
        border-bottom-color: ${colors.border};
      }
      
      .theme-${theme} .bookmark-page {
        color: ${colors.text};
      }
      
      .theme-${theme} .bookmark-content-text {
        color: ${colors.textSecondary};
      }
      
      /* 特殊处理深色主题的按钮 */
      ${
        theme === "dark"
          ? `
        .theme-${theme} .page-button-side {
          background: linear-gradient(135deg, rgba(70, 70, 70, 0.95) 0%, rgba(60, 60, 60, 0.95) 100%);
          border-color: rgba(128, 128, 128, 0.3);
          color: #e5e5e5;
        }
        
        .theme-${theme} .page-button-side:hover {
          background: linear-gradient(135deg, rgba(80, 80, 80, 0.98) 0%, rgba(70, 70, 70, 0.98) 100%);
          color: ${colors.accent};
        }
      `
          : theme === "sepia"
          ? `
        .theme-${theme} .page-button-side {
          background: linear-gradient(135deg, rgba(235, 229, 214, 0.95) 0%, rgba(228, 220, 201, 0.95) 100%);
          border-color: rgba(212, 197, 169, 0.5);
          color: #4a4a3a;
        }
        
        .theme-${theme} .page-button-side:hover {
          background: linear-gradient(135deg, rgba(240, 235, 220, 0.98) 0%, rgba(235, 229, 214, 0.98) 100%);
          color: ${colors.accent};
        }
      `
          : ""
      }
    `;

    document.head.appendChild(style);
  }

  private saveThemeToStorage(theme: Theme) {
    try {
      localStorage.setItem("app-theme", theme);
    } catch (error) {
      console.warn("Failed to save theme to storage:", error);
    }
  }

  public initializeTheme() {
    this.applyTheme(this.currentTheme);
  }
}

// 导出单例实例
export const themeManager = new ThemeManager();
