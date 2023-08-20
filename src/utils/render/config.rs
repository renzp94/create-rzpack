use crate::utils::{json::json_insert, prompts::Prompts};
use serde_json::{self, json};
use std::fs;
use std::path::Path;

pub fn rzpack_config(options: &Prompts) {
    let is_ts_template = options.template == "react_ts";
    let has_jst_loader = !options.jts_loader.is_empty();
    let mut assets = json!({});

    if options.css_scoped {
        json_insert(&mut assets, "cssScoped", json!(true));
    }

    if has_jst_loader {
        json_insert(&mut assets, "jsxTools", json!("JSX_TOOLS.{}"));
    }

    let mut config = json!({
      "html": {
        "title": options.project_name
      },
    });

    if options.css_scoped || has_jst_loader {
        json_insert(&mut config, "assets", assets);
    }

    if !is_ts_template {
        json_insert(
            &mut config,
            "antdTheme",
            json!({
              "file": "./src/theme/index.ts",
            }),
        );
        json_insert(
            &mut config,
            "lessVars",
            json!({
              "file":  "./src/theme/globalVars.ts",
            }),
        );
    }

    let config_string = serde_json::to_string_pretty(&config).expect("Serialization failed");
    let mut content = String::from("import { defineConfig")
        + if has_jst_loader { ", JSX_TOOLS" } else { "" }
        + "} from 'rzpack'\n\n"
        + "export default defineConfig(\n"
        + config_string.as_str()
        + ")\n";

    if has_jst_loader {
        content = content
            .replace("\"JSX_TOOLS", "JSX_TOOLS")
            .replace("{}\"", options.jts_loader.to_uppercase().as_str());
    }

    let path = options.root.join("rzpack.config.ts");
    fs::write(path, content).unwrap();
}

pub fn gitignore(root: &Path) {
    let path = root.join(".gitignore");
    fs::write(path, "node_modules\nbin\n*.log\n.vscode\n.DS_Store\ndist").unwrap();
}

pub fn commit_lint_config(root: &Path) {
    let commmit_lint_path = root.join("commitlint.config.js");
    fs::write(
        commmit_lint_path,
        "module.exports = {\n  extends: ['@commitlint/config-conventional', 'cz'],\n}\n",
    )
    .unwrap();

    let cz_lint_path = root.join("cz.config.js");
    let cz_config = String::from("module.exports = ")
        + serde_json::to_string_pretty(&json!({
            "messages": {
              "type": "请选择要提交的更改类型: ",
              "subject": "请输入此次更改内容的简短描述:",
              "body": "请输入此次更改内容的详细描述[可选]:",
              "confirmCommit": "是否提交本次内容?",
            },
            "skipQuestions": ["breaking", "scope", "footer"],
            "subjectLimit": 100,
            "types": [
              { "value": "feat", "name": "feat: 新功能" },
              { "value": "fix", "name": "fix: Bug修复" },
              { "value": "docs", "name": "docs: 文档更改" },
              { "value": "style", "name": "style: 不影响代码含义的更改(空白、格式、缺少分号等)" },
              { "value": "refactor", "name": "refactor: 代码重构" },
              { "value": "perf", "name": "perf: 性能优化" },
              { "value": "test", "name": "test: 测试更改" },
              {
                "value": "build",
                "name": "build: 影响构建系统或外部依赖关系的更改(示例范围: gulp、Brocoli、npm)",
              },
              { "value": "ci", "name": "ci: CI配置文件和脚本更改" },
              { "value": "chore", "name": "chore: 其他" },
              { "value": "revert", "name": "revert: 代码回退" },
            ],
        }))
        .expect("Serialization failed")
        .as_str();
    fs::write(cz_lint_path, cz_config).unwrap();
}

pub fn nodemon(root: &Path) {
    let path = root.join("nodemon.json");
    let config = json!({
      "watch": ["rzpack.config.ts"],
      "exec": "npm run dev",
    });

    fs::write(
        path,
        serde_json::to_string_pretty(&config)
            .expect("Serialization failed")
            .as_str(),
    )
    .unwrap();
}

pub fn readme(options: &Prompts) {
    let mut plugin_info = String::from("\n");
    let eslint_plugin = "- `ESLint`\n- `Prettier - Code formatter`\n";
    let rome_plugin = "- `Rome`\n";

    let js_lint_plugin = if options.js_lint.is_empty() {
        ""
    } else {
        if options.js_lint == "rome" {
            rome_plugin
        } else {
            eslint_plugin
        }
    };

    let style_lint_plugin = if options.style_lint {
        "- `Stylelint`\n"
    } else {
        ""
    };
    let css_module_plugin = "- `CSS Modules`\n";

    let vscode_setting_title = String::from("## 配置 Vscode\n\n")
        + "在`Vscode`配置文件`settings.json`中添加如下配置\n\n"
        + "```json\n"
        + "\"editor.formatOnSave\": true,\n"
        + "\"editor.codeActionsOnSave\": {\n"
        + "    \"source.fixAll\": true\n"
        + "}\n"
        + "```\n\n";

    let prettier_settings = String::from("\"prettier.jsxSingleQuote\": true,\n")
        + "\"prettier.requireConfig\": true,\n"
        + "\"prettier.semi\": false,\n"
        + "\"prettier.singleQuote\": true,\n"
        + "\"prettier.arrowParens\": \"avoid\",\n";

    let eslint_settings = String::from("### 配置 Eslint+Prettier\n\n")
        + "在`Vscode`配置文件`settings.json`中添加如下配置\n\n"
        + "```json\n"
        + prettier_settings.as_str()
        + "```\n\n";

    let rome_settings = String::from("### 配置 Rome\n\n")
        + "在根目录下创建`.vscode/settings.json`\n"
        + "```json\n"
        + serde_json::to_string_pretty(&json!({
          "editor.defaultFormatter": "rome.rome",
          "[javascript]": {
            "editor.defaultFormatter": "rome.rome"
          },
          "[typescriptreact]": {
            "editor.defaultFormatter": "rome.rome"
          },
          "[typescript]": {
            "editor.defaultFormatter": "rome.rome"
          },
          "editor.codeActionsOnSave":{
            "source.organizeImports.rome": true
          }
        }))
        .expect("Serialization failed")
        .as_str()
        + "```\n\n";

    plugin_info = plugin_info
        + "## Vscode 插件\n\n"
        + js_lint_plugin
        + style_lint_plugin
        + css_module_plugin
        + "\n"
        + vscode_setting_title.as_str()
        + if options.js_lint == "rome" {
            rome_settings.as_str()
        } else {
            eslint_settings.as_str()
        }
        + "\n";

    let content = String::from("")
        + "# "
        + options.project_name.as_str()
        + "\n\n"
        + "> create-rzpack创建的React项目\n\n"
        + "## 开发\n\n"
        + "```bash\n"
        + "npm run dev\n"
        + "```\n"
        + "## 打包\n\n"
        + "```bash\n"
        + "npm run build\n"
        + "```\n"
        + plugin_info.as_str();

    let path = options.root.join("README.md");
    fs::write(path, content).unwrap();
}
