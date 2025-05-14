use crate::model::ApiResponse;
use reqwest::multipart::{Form, Part};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use zip::ZipArchive;

const API_URL: &str =
    "https://api.products.fileformat.app/zh/word-processing/conversion/api/convert?outputType=HTML";

// 上传EPUB文件到API
pub async fn upload_epub_to_fileformat_api(
    epub_path: &str,
) -> Result<ApiResponse, Box<dyn std::error::Error>> {
    // 读取EPUB文件
    let file_content = fs::read(epub_path)?;
    let file_name = Path::new(epub_path)
        .file_name()
        .ok_or("Invalid file path")?
        .to_str()
        .ok_or("Invalid file name")?;

    // 创建multipart表单
    let part = Part::bytes(file_content)
        .file_name(file_name.to_string())
        .mime_str("application/epub")?;

    let form = Form::new().part("1", part);

    // 发送请求
    let client = reqwest::Client::new();
    let response = client.post(API_URL).multipart(form).send().await?;

    // 处理响应
    let result = response.json::<ApiResponse>().await?;
    println!("API Response: {:#?}", result);

    Ok(result)
}

// 解压zip文件
pub async fn download_converted_file(
    id: &str,
    output_path: &str,
    extract_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let download_url = format!(
        "https://products.fileformat.app/zh/word-processing/conversion/Download?id={}",
        id
    );

    println!("Downloading converted file from: {}", download_url);

    // 发送GET请求下载文件
    let client = reqwest::Client::new();
    let response = client.get(&download_url).send().await?;

    if !response.status().is_success() {
        return Err(format!("Failed to download file: HTTP status {}", response.status()).into());
    }

    // 确保输出目录存在
    fs::create_dir_all(Path::new(output_path).parent().unwrap_or(Path::new(".")))?;

    // 保存下载的zip文件
    let content = response.bytes().await?;
    let mut file = File::create(output_path)?;
    file.write_all(&content)?;

    // 确保解压目录存在
    fs::create_dir_all(extract_dir)?;

    // 解压缩文件到指定目录
    let file = File::open(output_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = Path::new(extract_dir).join(file.name());

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    println!("File extracted to: {}", extract_dir);

    Ok(())
}
