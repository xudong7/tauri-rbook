use epub::doc::EpubDoc;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn read_epub_cover(epub_path: &str) -> Result<(), String> {
    let mut doc = EpubDoc::new(epub_path).map_err(|e| e.to_string())?;
    let cover_data = doc.get_cover().unwrap();
    let (image_data, _mime_type) = cover_data;
    let f = fs::File::create("cover.png");
    assert!(f.is_ok());
    let mut f = f.unwrap();
    let write_result = f.write_all(&image_data);
    Ok(write_result.map_err(|e| e.to_string())?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const EPUB_PATH: &str = "[三河ごーすと].义妹生活.11.[日翻].epub";
    // 打开 EPUB 文件
    let doc = EpubDoc::new(EPUB_PATH)?;

    let title = doc.mdata("title").unwrap();
    let creator =  doc.mdata("creator").unwrap();
    println!("Title: {}", title);
    println!("Creator: {}", creator);

    // 创建输出目录
    let output_dir = "output";
    fs::create_dir_all(output_dir)?;

    let mut doc_clone = EpubDoc::new(EPUB_PATH)?;
    // 遍历所有资源
    for path in doc.resources.keys() {
        // 构建完整输出路径
        let output_path = Path::new(output_dir).join(path);
        
        // 创建父目录
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // 获取资源内容
        if let Some(content) = doc_clone.get_resource(path) {
            // 写入文件（二进制安全）
            let mut file = File::create(&output_path)?;
            let (content, mimetype) = content;
            file.write_all(&content)?;
            println!("写入文件: {:?} - {}", output_path, mimetype);
        }
    }

    read_epub_cover(EPUB_PATH)?;
    println!("封面图片已保存为 cover.png");

    Ok(())
}