use crate::utils::{file, log, pkg};

use super::args::Args;
use dialoguer::{Confirm, Input, Select};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Prompts {
    pub project_name: String,
    pub overwrite: bool,
    pub package_name: String,
    pub template: String,
    pub jts_loader: String,
    pub css_scoped: bool,
    pub js_lint: String,
    pub style_lint: bool,
    pub commit_lint: bool,
    pub rs: bool,
    pub root: PathBuf,
}

#[derive(Debug, Clone)]
struct SelectOption {
    value: &'static str,
    name: String,
}

pub fn get_prompts(args: Args) -> Prompts {
    let project_name = get_project_name(args.project_name);
    let project_name = project_name.as_str();
    // 是否需要覆盖
    let overwrite = overwrite_dir(project_name, args.force);
    let template = get_template_value(args.template);
    let package_name = get_package_name(project_name);
    let jts_loader = get_jts_loader_value();
    let js_lint = get_js_lint_value();
    let css_scoped = get_confirm_value("是否使用Css Scoped?");
    let style_lint = get_confirm_value("是否使用styleLint?");
    let commit_lint = get_confirm_value("是否使用CommitLint?");
    let rs = get_confirm_value("是否开启配置文件更改自动重启?");

    let root = file::resolve_path(file::get_current_dir().as_path(), Path::new(project_name));

    Prompts {
        project_name: project_name.to_string(),
        overwrite,
        root,
        package_name: package_name.to_string(),
        template: template.to_string(),
        jts_loader: jts_loader.to_string(),
        css_scoped,
        js_lint: js_lint.to_string(),
        style_lint,
        commit_lint,
        rs,
    }
}

fn get_project_name(project_name: String) -> String {
    let project_name: String = Input::new()
        .with_prompt(log::yellow("项目名称"))
        .allow_empty(false)
        .with_initial_text(project_name)
        .interact_text()
        .unwrap();

    project_name.trim().to_string()
}

fn overwrite_dir(project_name: &str, force: Option<bool>) -> bool {
    // 是否需要覆盖
    let can_overwrite = file::can_overwrite_directory(project_name);
    let dir = if project_name == "." {
        "当前目录".to_owned()
    } else {
        format!("{}{}", "目标目录", project_name)
    };
    let overwrite = can_overwrite == true || force == Some(true);
    if overwrite {
        if !get_confirm_value(
            log::red(format!("{}不为空，是否删除{}并继续?", dir, dir).as_str()).as_str(),
        ) {
            panic!("操作取消")
        }
    }

    overwrite
}

fn get_package_name(project_name: &str) -> String {
    if pkg::is_valid_package_name(project_name) {
        project_name.trim().to_string()
    } else {
        let name: String = Input::new()
            .with_prompt(log::yellow("Package name"))
            .allow_empty(false)
            .validate_with(|input: &String| -> Result<(), &str> {
                if pkg::is_valid_package_name(input.as_str()) {
                    Ok(())
                } else {
                    Err("Package name错误")
                }
            })
            .interact_text()
            .unwrap();

        name.trim().to_string()
    }
}

fn get_template_value(template: Option<String>) -> &'static str {
    let templates = vec![
        SelectOption {
            value: "react_ts",
            name: log::cyan("ts - ts模板"),
        },
        SelectOption {
            value: "antd",
            name: log::yellow("antd - antd模板"),
        },
        SelectOption {
            value: "admin",
            name: log::blue("admin - 基础后台管理平台(侧边菜单版)的模版"),
        },
        SelectOption {
            value: "admin_header_menu",
            name: log::blue("admin_header_menu - 基础后台管理平台(顶部菜单版)的模版"),
        },
    ];
    let template = match template {
        Some(value) => match templates.iter().position(|tmp| tmp.value == value) {
            Some(pos) => pos as i8,
            None => -1,
        },
        None => -1,
    };

    if template == -1 {
        // 选择模板
        get_select_value(templates, "模板")
    } else {
        templates[template as usize].clone().value
    }
}

fn get_jts_loader_value() -> &'static str {
    let jts_loaders = vec![
        SelectOption {
            value: "babel",
            name: log::cyan("Babel"),
        },
        SelectOption {
            value: "esbuild",
            name: log::yellow("Esbuild"),
        },
        SelectOption {
            value: "swc",
            name: log::blue("Swc"),
        },
    ];
    get_select_value(jts_loaders, "Js/Ts文件的loader")
}

fn get_js_lint_value() -> &'static str {
    let jts_loaders = vec![
        SelectOption {
            value: "eslint",
            name: log::cyan("Eslint"),
        },
        SelectOption {
            value: "rome",
            name: log::yellow("Rome(实验性)"),
        },
        SelectOption {
            value: "",
            name: log::blue("无"),
        },
    ];
    get_select_value(jts_loaders, "js格式化工具")
}

fn get_select_value(options: Vec<SelectOption>, prompt: &str) -> &str {
    let names: Vec<String> = options.iter().map(|tmp| tmp.name.clone()).collect();
    let jts = Select::new()
        .with_prompt(log::yellow(prompt))
        .default(0)
        .items(&names)
        .interact()
        .unwrap();

    options[jts].clone().value
}

fn get_confirm_value(prompt: &str) -> bool {
    Confirm::new()
        .with_prompt(log::yellow(prompt))
        .interact()
        .unwrap()
}
