use rust_embed::RustEmbed;
use std::fs;
use std::path::Path;

use crate::utils::log;

#[derive(RustEmbed)]
#[folder = "template/"]
struct Asset;

pub fn copy(template: Vec<&str>, dest: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for file in Asset::iter() {
        let filename = file.trim();
        // 获取文件名的前缀作为子目录名
        let prefix = Path::new(filename).components().next().unwrap();
        let subdir = prefix.as_os_str().to_str().unwrap_or("");

        // 如果文件位于需要的子目录下，将其写入目标文件夹
        if template.contains(&subdir) {
            let relative_path = filename.strip_prefix(subdir).unwrap().replacen("/", "", 1);
            let file_path = dest.join(relative_path);

            // 创建目录（如果不存在）
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent)?;
            }

            let content = Asset::get(filename).unwrap();
            // 写入嵌入的文件内容
            fs::write(&file_path, content.data)?;
            log::info(format!("创建文件成功：{:?}", file_path));
        }
    }

    Ok(())
}
