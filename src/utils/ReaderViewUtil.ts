// views/ReaderView/ReaderView.vue的工具函数
import { saveReaderStyle, getReaderStyle } from "../api/style";
import { ReaderStyle } from "../api/types";

// 默认样式配置
let defaultStyle: ReaderStyle = {
  font_family: "Noto Serif",
  font_size: 18,
  line_height: 1.4,
};

// 默认图片缩放比例
let imageRatio = 0.9;

// 当前样式对象
let currentStyle: ReaderStyle = { ...defaultStyle };

// 在应用启动时从本地加载样式
export const loadStyleFromLocal = async (): Promise<ReaderStyle> => {
  try {
    const style = await getReaderStyle();
    currentStyle = style;
    return style;
  } catch (error) {
    console.error("Error loading style from local:", error);
    return defaultStyle;
  }
};

// 保存样式到本地
export const saveStyleToLocal = async (
  fontFamily: string = currentStyle.font_family,
  fontSize: number = currentStyle.font_size,
  lineHeight: number = currentStyle.line_height
): Promise<void> => {
  try {
    // 更新当前样式
    currentStyle = {
      font_family: fontFamily,
      font_size: fontSize,
      line_height: lineHeight,
    };

    // 保存到本地
    await saveReaderStyle(fontFamily, fontSize, lineHeight);
  } catch (error) {
    console.error("Error saving style to local:", error);
  }
};

// 根据窗口大小生成样式
export const generateStyle = (
  fontFamily = "Noto Serif",
  fontSize = 18,
  lineHeight = 1.4
) => {
  // if (window.innerWidth <= 960) {
  //   fontSize = Math.min(16, fontSize);
  //   lineHeight = 1.2;
  // }

  const globalStyle = `<style>
  html { overflow: hidden!important; margin: 20px; padding: 0; }
  body {
    font-family: '${fontFamily}', 'Times New Roman', serif!important;
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
  // 如果提供了新的参数并与当前值不同，保存到本地
  if (
    fontFamily !== currentStyle.font_family ||
    fontSize !== currentStyle.font_size ||
    lineHeight !== currentStyle.line_height
  ) {
    // 更新当前样式，但不立即保存（saveStyleToLocal 函数会在合适的时机调用）
    currentStyle = {
      font_family: fontFamily,
      font_size: fontSize,
      line_height: lineHeight,
    };
  }

  return globalStyle;
};

// 处理图片大小并返回处理后的HTML
export const resizeImgAndReturnInnerHTML = (
  paragraph: string,
  pageWidth: number,
  pageHeight: number,
  ratio?: number,
) => {
  // 创建临时容器来获取图片并处理图片尺寸
  const tempImgContainer = document.createElement("div");
  tempImgContainer.innerHTML = paragraph; // 设置最大尺寸限制
  let scaleRatio = ratio || imageRatio; // 默认比例为0.9
  if (pageWidth <= 480) scaleRatio = 0.8; // 小屏幕时使用更小的比例
  const maxWidth = pageWidth * scaleRatio;
  const maxHeight = pageHeight * scaleRatio; // 为页面留出一些空间

  // 处理img标签图片大小
  const imgElements = tempImgContainer.querySelectorAll("img");
  if (imgElements.length > 0) {
    for (const img of imgElements) {
      // 设置图片样式，确保其不超出页面
      img.style.maxWidth = `${maxWidth}px`;
      img.style.maxHeight = `${maxHeight}px`;
      img.style.width = "auto"; // 保持纵横比
      img.style.height = "auto"; // 保持纵横比
      img.style.display = "block";
      img.style.margin = "1em auto"; // 居中显示
    }
  }

  // 处理SVG中的image标签
  const svgElements = tempImgContainer.querySelectorAll("svg");
  if (svgElements.length > 0) {
    for (const svg of svgElements) {
      // 获取SVG原始尺寸
      const svgWidth = parseFloat(svg.getAttribute("width") || "0");
      const svgHeight = parseFloat(svg.getAttribute("height") || "0");

      if (svgWidth > 0 && svgHeight > 0) {
        // 计算缩放比例
        const scale = Math.min(
          maxWidth / svgWidth,
          maxHeight / svgHeight,
          1 // 不放大，只缩小
        );

        // 设置新尺寸
        const newWidth = Math.floor(svgWidth * scale);
        const newHeight = Math.floor(svgHeight * scale);

        // 应用新尺寸
        svg.setAttribute("width", newWidth.toString());
        svg.setAttribute("height", newHeight.toString());
        svg.style.display = "block";
        svg.style.margin = "1em auto"; // 居中显示
      }

      // 处理SVG内部的image标签
      const imageElements = svg.querySelectorAll("image");
      for (const image of imageElements) {
        // 获取image标签的原始尺寸
        const imageWidth = parseFloat(image.getAttribute("width") || "0");
        const imageHeight = parseFloat(image.getAttribute("height") || "0");

        if (imageWidth > 0 && imageHeight > 0) {
          // 应用与SVG相同的缩放
          const scale = Math.min(
            maxWidth / imageWidth,
            maxHeight / imageHeight,
            1 // 不放大，只缩小
          );

          // 设置新尺寸
          const newWidth = Math.floor(imageWidth * scale);
          const newHeight = Math.floor(imageHeight * scale);

          // 应用新尺寸
          image.setAttribute("width", newWidth.toString());
          image.setAttribute("height", newHeight.toString());
        }
      }
    }
  }

  // 返回处理后的HTML内容
  return tempImgContainer.innerHTML;
};

// 拆分包含多个图像或多个span的段落或标题
export const splitParagraphWithImages = (
  paragraph: string,
  style: string,
  pageContainer?: HTMLDivElement,
  pageHeight?: number,
  currentPageContent?: string
): string[] => {
  // 如果段落不包含图像，且不需要处理span，则直接返回
  if (
    !paragraph.includes("<img") &&
    !paragraph.includes("<svg") &&
    !paragraph.includes("<image") &&
    (!pageContainer || !pageHeight || currentPageContent === undefined)
  ) {
    return [paragraph];
  }

  // 创建临时DOM元素来解析段落内容
  const tempPara = document.createElement("div");
  tempPara.innerHTML = paragraph;
  // 确定元素类型 (p, h1-h6)
  const firstChild = tempPara.firstChild;
  const nodeName = firstChild?.nodeName || "";
  const isPossiblyHeading = /^H[1-6]$/i.test(nodeName);
  // 使用原始标签类型或默认为p
  const tagName = isPossiblyHeading ? nodeName.toLowerCase() : "p";

  // 如果需要处理有多个span的段落
  if (pageContainer && pageHeight && currentPageContent !== undefined) {
    // 获取段落内所有的子节点（包括span, text, img等）
    const childNodes = Array.from(tempPara.firstChild?.childNodes || []);
    // 如果节点数量太少，不需要拆分
    if (childNodes.length <= 1) {
      return [paragraph];
    }

    // 处理段落中的span和其他元素
    const result: string[] = [];
    let currentElement = document.createElement(tagName);
    let currentPageContentCopy = currentPageContent;

    // 处理每个子节点
    for (let i = 0; i < childNodes.length; i++) {
      const childNode = childNodes[i];

      // 如果是图像节点，单独处理
      if (
        childNode.nodeName === "IMG" ||
        childNode.nodeName === "SVG" ||
        (childNode.nodeName === "IMAGE" &&
          childNode.parentNode?.nodeName !== "SVG")
      ) {
        // 如果当前元素中已有内容，先检查添加后是否会溢出
        if (currentElement.hasChildNodes()) {
          // 测试当前内容加上已有内容是否会溢出
          const testElement = document.createElement("div");
          testElement.innerHTML =
            style + currentPageContentCopy + currentElement.outerHTML;
          pageContainer.innerHTML = testElement.innerHTML;
          const testHeight = pageContainer.clientHeight;

          // 如果会溢出，先保存当前内容
          if (testHeight > pageHeight) {
            result.push(currentElement.outerHTML);
            currentPageContentCopy = "";
            currentElement = document.createElement(tagName);
          } else {
            result.push(currentElement.outerHTML);
            currentElement = document.createElement(tagName);
            currentPageContentCopy += result[result.length - 1];
          }
        }

        // 创建一个只包含图像的标签
        const imgContainer = document.createElement(tagName);
        imgContainer.appendChild(childNode.cloneNode(true));
        result.push(imgContainer.outerHTML);
        currentPageContentCopy = ""; // 图片后一律分页
      } else {
        // 添加当前节点到当前元素
        const tempElement = document.createElement(tagName);
        const clone = childNode.cloneNode(true);
        tempElement.appendChild(clone);

        // 测试添加此节点后是否会导致页面溢出
        const testElement = document.createElement("div");
        testElement.innerHTML =
          style +
          currentPageContentCopy +
          currentElement.outerHTML +
          tempElement.outerHTML;
        pageContainer.innerHTML = testElement.innerHTML;
        const testHeight = pageContainer.clientHeight;

        // 测试该节点单独在一个新页面是否会溢出
        const testNewPageElement = document.createElement("div");
        testNewPageElement.innerHTML = style + tempElement.outerHTML;
        pageContainer.innerHTML = testNewPageElement.innerHTML;
        const testNewPageHeight = pageContainer.clientHeight;

        // 如果添加到当前页会溢出
        if (testHeight > pageHeight) {
          // 先保存当前内容
          if (currentElement.hasChildNodes()) {
            result.push(currentElement.outerHTML);
          }

          // 如果节点本身也很大，即使单独放在一页也会溢出
          if (testNewPageHeight > pageHeight) {
            // 需要分割span内文字
            const splitResult = splitLargeSpanText(
              clone,
              tagName,
              pageContainer,
              pageHeight,
              "",
              style
            );
            const elements = splitResult.elements;

            // 将分割后的元素添加到结果中
            for (let j = 0; j < elements.length; j++) {
              result.push(elements[j].outerHTML);
            }

            // 重置当前元素
            currentElement = document.createElement(tagName);
            currentPageContentCopy = "";
          } else {
            // 将节点添加到新页面
            currentElement = document.createElement(tagName);
            currentElement.appendChild(clone);
            currentPageContentCopy = "";
          }
        } else {
          // 如果不会溢出，正常添加
          currentElement.appendChild(clone);

          // 继续检查是否需要分页
          if (i < childNodes.length - 1) {
            // 不是最后一个节点才需要检查
            const testNextElement = document.createElement("div");
            testNextElement.innerHTML =
              style + currentPageContentCopy + currentElement.outerHTML;
            pageContainer.innerHTML = testNextElement.innerHTML;
            const testNextHeight = pageContainer.clientHeight;

            // 如果会溢出，保存当前内容并开始新元素
            if (testNextHeight > pageHeight) {
              result.push(currentElement.outerHTML);
              currentPageContentCopy = "";
              currentElement = document.createElement(tagName);
            }
          }
        }
      }
    }

    // 如果当前元素中还有内容，保存它
    if (currentElement.hasChildNodes()) {
      result.push(currentElement.outerHTML);
    }

    // 清空测试容器
    pageContainer.innerHTML = "";

    return result.length > 0 ? result : [paragraph];
  }

  // 处理图像分离的情况（原有逻辑）
  const result: string[] = [];
  let currentElement = document.createElement(tagName);
  let hasContent = false;

  // 处理段落中的所有子节点
  for (const childNode of Array.from(tempPara.firstChild?.childNodes || [])) {
    // 如果是图像节点
    if (
      childNode.nodeName === "IMG" ||
      childNode.nodeName === "SVG" ||
      (childNode.nodeName === "IMAGE" &&
        childNode.parentNode?.nodeName !== "SVG")
    ) {
      // 如果当前元素中已有内容，先保存它
      if (hasContent) {
        result.push(currentElement.outerHTML);
        currentElement = document.createElement(tagName);
        hasContent = false;
      }

      // 创建一个只包含图像的标签，使用原始标签类型
      const imgContainer = document.createElement(tagName);
      imgContainer.appendChild(childNode.cloneNode(true));
      result.push(imgContainer.outerHTML);
    } else {
      // 将非图像节点添加到当前元素
      currentElement.appendChild(childNode.cloneNode(true));
      hasContent = true;
    }
  }

  // 如果当前元素中还有内容，保存它
  if (hasContent) {
    result.push(currentElement.outerHTML);
  }

  return result.length > 0 ? result : [paragraph];
};

// 拆分大型span中的文本
const splitLargeSpanText = (
  spanNode: Node,
  tagName: string,
  pageContainer: HTMLDivElement,
  pageHeight: number,
  currentPageContent: string,
  style: string
): { elements: HTMLElement[]; firstPageContent: string } => {
  // 只处理SPAN节点和文本节点
  if (
    spanNode.nodeName !== "SPAN" &&
    spanNode.nodeName !== "#text" &&
    spanNode.nodeName !== "TEXT"
  ) {
    const wrapper = document.createElement(tagName);
    wrapper.appendChild(spanNode.cloneNode(true));
    return { elements: [wrapper], firstPageContent: currentPageContent };
  }

  // 获取节点的文本内容
  let text = spanNode.textContent || "";

  // 如果文本很短，不需要分割
  if (text.length < 100) {
    const wrapper = document.createElement(tagName);
    wrapper.appendChild(spanNode.cloneNode(true));
    return { elements: [wrapper], firstPageContent: currentPageContent };
  }

  // 用于存储分割后的元素
  const elements: HTMLElement[] = [];
  let remainingText = text;
  let span: HTMLElement;
  let testElement: HTMLElement = document.createElement("div");
  let segmentLength = 50; // 初始分段长度
  let updatedPageContent = currentPageContent;
  let isFirstSegment = true;
  let currentTestLength = 0;

  // 循环拆分文本，直到所有文本都被处理
  while (remainingText.length > 0) {
    // 创建一个新的段落和span元素
    const newElement = document.createElement(tagName);

    if (spanNode.nodeName === "#text" || spanNode.nodeName === "TEXT") {
      // 如果是文本节点，直接添加文本
      let testLength = Math.min(segmentLength, remainingText.length);

      // 尝试增加文本长度，直到页面溢出
      while (testLength < remainingText.length) {
        const testText = remainingText.substring(0, testLength);
        newElement.textContent = testText;

        testElement.innerHTML =
          style + updatedPageContent + newElement.outerHTML;
        pageContainer.innerHTML = testElement.innerHTML;

        if (pageContainer.clientHeight > pageHeight) {
          // 如果溢出，回退到上一个长度
          if (testLength > segmentLength) {
            testLength =
              testLength -
              Math.max(20, Math.floor((testLength - segmentLength) / 2));
          } else {
            // 即使最小分段也溢出，则取最小值
            testLength = Math.max(10, testLength - 10);
          }
          break;
        }

        // 增加测试长度
        testLength = Math.min(testLength + 50, remainingText.length);

        // 如果已经是全部文本，结束循环
        if (testLength >= remainingText.length) {
          break;
        }
      }
      // 保存当前测试长度
      currentTestLength = testLength;

      // 确保在单词边界处切割（避免截断单词）
      let finalLength = testLength;
      if (testLength < remainingText.length) {
        // 向前查找最近的空格
        const lastSpaceIndex = remainingText
          .substring(0, testLength)
          .lastIndexOf(" ");
        if (lastSpaceIndex > 0) {
          finalLength = lastSpaceIndex + 1; // +1 to include the space
        }
      }

      // 取出当前可以放下的文本
      const currentSegment = remainingText.substring(0, finalLength);
      remainingText = remainingText.substring(finalLength);

      newElement.textContent = currentSegment;
    } else {
      // 如果是span节点，创建一个新的span
      span = document.createElement("span");

      // 获取原始span的所有属性并复制
      if (spanNode instanceof Element) {
        Array.from(spanNode.attributes).forEach((attr) => {
          span.setAttribute(attr.name, attr.value);
        });
      }

      let testLength = Math.min(segmentLength, remainingText.length);

      // 尝试增加文本长度，直到页面溢出
      while (testLength < remainingText.length) {
        const testText = remainingText.substring(0, testLength);
        span.textContent = testText;
        newElement.innerHTML = ""; // 清空之前的内容
        newElement.appendChild(span.cloneNode(true));

        testElement.innerHTML =
          style + updatedPageContent + newElement.outerHTML;
        pageContainer.innerHTML = testElement.innerHTML;

        if (pageContainer.clientHeight > pageHeight) {
          // 如果溢出，回退到上一个长度
          if (testLength > segmentLength) {
            testLength =
              testLength -
              Math.max(20, Math.floor((testLength - segmentLength) / 2));
          } else {
            // 即使最小分段也溢出，则取最小值
            testLength = Math.max(10, testLength - 10);
          }
          break;
        }

        // 增加测试长度
        testLength = Math.min(testLength + 50, remainingText.length);

        // 如果已经是全部文本，结束循环
        if (testLength >= remainingText.length) {
          break;
        }
      }
      // 保存当前测试长度
      currentTestLength = testLength;

      // 确保在单词边界处切割（避免截断单词）
      let finalLength = testLength;
      if (testLength < remainingText.length) {
        // 向前查找最近的空格
        const lastSpaceIndex = remainingText
          .substring(0, testLength)
          .lastIndexOf(" ");
        if (lastSpaceIndex > 0) {
          finalLength = lastSpaceIndex + 1; // +1 to include the space
        }
      }

      // 取出当前可以放下的文本
      const currentSegment = remainingText.substring(0, finalLength);
      remainingText = remainingText.substring(finalLength);

      // 重新创建span以确保干净
      span = document.createElement("span");
      if (spanNode instanceof Element) {
        Array.from(spanNode.attributes).forEach((attr) => {
          span.setAttribute(attr.name, attr.value);
        });
      }
      span.textContent = currentSegment;
      newElement.innerHTML = ""; // 确保元素是空的
      newElement.appendChild(span);
    }

    elements.push(newElement);

    // 更新当前页面内容，只有第一段需要保留原来页面的内容
    if (isFirstSegment) {
      updatedPageContent += newElement.outerHTML;
      isFirstSegment = false;
    } else {
      updatedPageContent = "";
    }

    // 动态调整分段长度
    segmentLength = currentTestLength;
  }

  return { elements, firstPageContent: updatedPageContent };
};
