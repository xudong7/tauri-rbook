
// 根据窗口大小生成样式
export const generateStyle = () => {
  let fontSize = 18;
  let lineHeight = 1.4;
  if (window.innerWidth <= 960) {
    fontSize = 16;
    lineHeight = 1.2;
  }

  return `<style>
  html { overflow: hidden!important; margin: 20px; padding: 0; }
  body {
    font-family: 'Noto Serif', 'Times New Roman', serif!important;
    font-size: ${fontSize}px!important;
    line-height: ${lineHeight}!important;
    color: #333!important;
    box-sizing: border-box!important;
  }
  p {
    margin: 1em!important;
    text-indent: 1em!important;
  }
  p.pre {
    margin: 0!important;
  }
  a {
    pointer-events: none!important;
    text-decoration: none!important;
  }
</style>`;
};
