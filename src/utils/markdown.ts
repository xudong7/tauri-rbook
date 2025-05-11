import MarkdownIt from 'markdown-it';
import markdownItAnchor from 'markdown-it-anchor';

// 创建markdown-it的实例 - 简化配置，减少插件依赖
const md = new MarkdownIt({
  html: true,
  linkify: true,
  typographer: true,
  breaks: true,
  // 内置语法高亮
  highlight: function (str: string, lang: string) {
    if (lang && lang !== '') {
      try {
        return `<pre class="language-${lang}"><code class="language-${lang}">${str}</code></pre>`;
      } catch (e) {}
    }
    return `<pre><code>${str}</code></pre>`;
  }
});

// 配置锚点插件
md.use(markdownItAnchor, {
  permalink: true,
  permalinkBefore: true,
  permalinkSymbol: '§',
  level: [1, 2, 3, 4, 5, 6],
});

/**
 * 渲染Markdown内容为HTML
 * @param content Markdown格式的内容
 * @param isDark 是否使用暗色主题
 * @returns 渲染后的HTML字符串
 */
export function renderMarkdown(content: string, isDark = false): string {
  // 渲染Markdown内容
  const rendered = md.render(content);
  
  // 如果是暗色模式，在渲染后的HTML中添加特定的类
  if (isDark) {
    // 这里我们通过动态添加类名的方式让highlight.js应用暗色样式
    setTimeout(() => {
      document.querySelectorAll('pre code').forEach((block) => {
        // 添加dark类名以便CSS选择器匹配
        block.parentElement?.classList.add('dark-theme');
        block.classList.add('dark-theme');
      });
    }, 0);
  }

  return rendered;
}

/**
 * 从HTML内容中提取目录信息
 * @param html 渲染后的HTML内容
 * @returns 目录项数组
 */
export function extractToc(html: string): { level: number; text: string; id: string }[] {
  const tempDiv = document.createElement('div');
  tempDiv.innerHTML = html;
  
  const headings = Array.from(tempDiv.querySelectorAll('h1, h2, h3, h4, h5, h6'));
  
  return headings.map(heading => {
    const level = parseInt(heading.tagName.substring(1));
    const text = heading.textContent?.replace(/§/g, '') || '';
    const id = heading.id || text.toLowerCase().replace(/[^\w]+/g, '-');
    
    return {
      level,
      text,
      id
    };
  });
}
