use regex::Regex;
use std::fs;
use toml::Value;

pub struct Package {
    pub name: String,
    pub version: String,
}

pub fn get_pkg() -> Package {
    // 读取 Cargo.toml 文件内容
    let toml_str = fs::read_to_string("Cargo.toml").expect("Failed to read Cargo.toml file");

    // 解析 TOML 格式内容
    let parsed_toml: Value =
        toml::from_str(&toml_str).expect("Failed to parse Cargo.toml content as TOML");

    // 提取 name 和 version 字段的值
    let name = parsed_toml["package"]["name"]
        .as_str()
        .expect("Missing or invalid version field")
        .to_string();
    let version = parsed_toml["package"]["version"]
        .as_str()
        .expect("Missing or invalid version field")
        .to_string();

    Package { name, version }
}

pub fn is_valid_package_name(name: &str) -> bool {
    let pattern = Regex::new(r"^(?:@[a-z0-9-*~][a-z0-9-*._~]*/)?[a-z0-9-~][a-z0-9-._~]*$").unwrap();

    pattern.is_match(name)
}
