import MarkdownIt from "markdown-it";
import markdownItAnchor from "markdown-it-anchor";

// 创建markdown-it的实例 - 简化配置，减少插件依赖
const md = new MarkdownIt({
  html: true,
  linkify: true,
  typographer: true,
  breaks: true,
  // 内置语法高亮，通过添加语言类名来启用CSS选择器样式
  highlight: function (str: string, lang: string) {
    // 转义HTML特殊字符，避免安全问题
    const escapeHTML = (text: string): string => {
      return text
        .replace(/&/g, "&amp;")
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;")
        .replace(/"/g, "&quot;")
        .replace(/'/g, "&#039;");
    };

    // 添加特定语言类，用于CSS选择器
    const code = escapeHTML(str);
    const validLang = lang && lang !== "" ? lang : "plaintext";

    // 生成具有语言类名的代码块
    return `<pre class="language-${validLang}"><code class="language-${validLang}">${code}</code></pre>`;
  },
});

// 配置锚点插件
md.use(markdownItAnchor, {
  permalink: true,
  permalinkBefore: true,
  permalinkSymbol: "§",
  level: [1, 2, 3, 4, 5, 6],
});

/**
 * 渲染Markdown内容为HTML
 * @param content Markdown格式的内容
 * @returns 渲染后的HTML字符串
 */
export function renderMarkdown(content: string): string {
  // 渲染Markdown内容
  return md.render(content);
}

/**
 * 从HTML内容中提取目录信息
 * @param html 渲染后的HTML内容
 * @returns 目录项数组
 */
export function extractToc(
  html: string
): { level: number; text: string; id: string }[] {
  const tempDiv = document.createElement("div");
  tempDiv.innerHTML = html;

  const headings = Array.from(
    tempDiv.querySelectorAll("h1, h2, h3, h4, h5, h6")
  );

  return headings.map((heading) => {
    const level = parseInt(heading.tagName.substring(1));
    const text = heading.textContent?.replace(/§/g, "") || "";
    const id = heading.id || text.toLowerCase().replace(/[^\w]+/g, "-");

    return {
      level,
      text,
      id,
    };
  });
}
