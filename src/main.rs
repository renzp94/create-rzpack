use crate::utils::tools::run_command;
use clap::Parser;
use remove_dir_all::*;
use tokio;
use tokio::fs;
use utils::prompts::Prompts;
use utils::{args, file, log, pkg, prompts, render};
mod utils;

#[tokio::main]
async fn main() {
    let pkg = pkg::get_pkg();
    let welcome = format!("{} V{}", pkg.name.to_uppercase(), pkg.version);
    println!("\n{}", log::blue(welcome.as_str()));

    let args = args::Args::parse();
    let options = prompts::get_prompts(args);
    create(options).await;
}

async fn create(options: Prompts) {
    let root = options.root.as_path();
    if options.overwrite {
        log::info(format!("正在清除{:?}目录", root));
        match remove_dir_all(root) {
            Ok(_) => {}
            Err(e) => log::error(format!("清除目录出错: {}", e)),
        };
    }

    log::info(format!("正在创建{:?}目录", root));
    if !file::file_exists(root) {
        let _ = fs::create_dir(root).await;
    }

    // 渲染package.json
    render::package::create(&options).await;
    // 模板目录名
    let mut template_dirs = vec!["base", options.template.as_str()];
    if !options.js_lint.is_empty() {
        template_dirs.push(options.js_lint.as_str());
    }

    if options.style_lint {
        template_dirs.push("stylelint");
    }
    // 渲染模板
    render::template::copy(template_dirs, &options.root).unwrap();
    render::config::rzpack_config(&options);
    render::config::gitignore(&options.root);
    if options.commit_lint {
        render::config::commit_lint_config(&options.root);
    }

    if options.rs {
        render::config::nodemon(&options.root);
    }
    render::config::readme(&options);
    log::info("正在初始化git仓库...".to_string());
    match run_command("git", &["init"]) {
        Ok(_) => {}
        Err(_) => {
            log::error("初始化git仓库失败".to_string());
        }
    };

    let project_name = log::bold(options.project_name.as_str());
    println!(
        "✨  项目{}创建成功!!! 🚀🚀🚀\n\n\t👉 cd {}\n\t👉 npm install\n\t👉 npm run dev\n",
        project_name, project_name,
    );
}
