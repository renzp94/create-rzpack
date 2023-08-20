use serde::{Deserialize, Serialize};
use serde_json::{self, json, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use crate::utils::{
    json::{json_insert, json_merge},
    prompts::Prompts,
    tools,
};

#[derive(Serialize, Deserialize)]
struct Packages {
    name: String,
    version: String,
    scripts: HashMap<&'static str, &'static str>,
    browserslist: [&'static str; 4],
    simple_git_hooks: HashMap<&'static str, &'static str>,
    license: &'static str,
    lint_staged: Option<HashMap<String, Vec<String>>>,
    config: HashMap<&'static str, HashMap<&'static str, &'static str>>,
    dependencies: HashMap<String, String>,
    dev_dependencies: HashMap<String, String>,
}

pub async fn create(options: &Prompts) {
    let scripts = get_scripts(options.rs, options.commit_lint);
    let simple_git_hooks = get_simple_git_hooks(options.commit_lint);
    let lint_staged = get_lint_staged_scripts(&options.js_lint, options.style_lint);
    let commit_config = json!( {
      "commitizen": {
        "path":"node_modules/cz-customizable",
      },
      "cz-customizable":{
        "config": "cz.config.js",
      },
    });
    let dependencies = get_dependencies(&options.template);

    let dev_dependencies = get_dev_dependencies(
        &options.template,
        options.commit_lint,
        &options.js_lint,
        options.style_lint,
    );

    let mut pkgs = json!({
        "name": options.package_name,
        "version": "0.0.1",
        "scripts":scripts,
        "browserslist": [">0.2%", "not dead", "not IE 11", "not op_mini all"],
        "simple-git-hooks":simple_git_hooks,
        "lint-staged":lint_staged,
        "license": "MIT",
        "dependencies": dependencies,
        "devDependencies":dev_dependencies,
    });

    if options.commit_lint {
        json_insert(&mut pkgs, "config", commit_config);
    }

    let json_string = serde_json::to_string_pretty(&pkgs).expect("Serialization failed");
    let path = options.root.join("package.json");

    let mut file = File::create(path).expect("File creation failed");
    file.write_all(json_string.as_bytes())
        .expect("Write failed");
}

fn get_scripts(rs: bool, commit_lint: bool) -> Value {
    let mut scripts = json!( {
     "dev":"rzpack",
     "build": "rzpack build",
     "build:time": "rzpack build --bundle-time",
     "build:size": "rzpack build --bundle-size",
     "preview": "rzpack preview",
     "prepare": "npx simple-git-hooks",
    });

    if rs {
        json_insert(&mut scripts, "dev:rs", json!("nodemon"));
    }

    if commit_lint {
        json_insert(&mut scripts, "cz", json!("git-cz"));
        json_insert(&mut scripts, "release", json!("standard-version"));
    }

    return scripts;
}

fn get_simple_git_hooks(commit_lint: bool) -> Value {
    let mut hooks = json!({
      "pre-commit": "npx lint-staged",
    });

    if commit_lint {
        json_insert(
            &mut hooks,
            "commit-msg",
            json!("npx --no -- commitlint --edit $1"),
        );
    }

    return hooks;
}

fn get_lint_staged_scripts(js_lint: &String, style_lint: bool) -> Option<Value> {
    let lint_staged_scripts: Option<Value> = if !js_lint.is_empty() {
        Some(json!({
          "src/**/*.{js,jsx,ts,tsx}":
            if js_lint == "rome" { vec!["rome check".to_string(), "rome format --write".to_string()]} else {vec!["eslint --fix".to_string(), "prettier --write".to_string()]},
        }))
    } else {
        None
    };

    if style_lint {
        match lint_staged_scripts {
            Some(mut value) => {
                json_insert(
                    &mut value,
                    "src/**/*.{less,css}",
                    json!(vec!["stylelint --fix".to_string()]),
                );
                Some(value)
            }
            None => Some(json!({"src/**/*.{less,css}":vec!["stylelint --fix".to_string()]})),
        }
    } else {
        None
    }
}

fn get_dependencies(template: &String) -> Value {
    let is_admin_template = ["admin", "admin_header_menu"].contains(&template.as_str());
    let mut dependencies = json!({
      "react":"^18.2.0",
      "react-dom":"^18.2.0",
      "dayjs": "^1.11.9",
    });

    if template != "react_ts" {
        json_insert(&mut dependencies, "@ant-design/icons", json!("^5.2.4"));
        json_insert(&mut dependencies, "antd", json!("^5.8.1"));
    }

    if is_admin_template {
        json_insert(&mut dependencies, "@renzp/storage", json!("^0.0.1"));
        json_insert(&mut dependencies, "axios", json!("^1.4.0"));
        json_insert(&mut dependencies, "lodash-es", json!("^4.17.21"));
        json_insert(&mut dependencies, "nprogress", json!("^0.2.0"));
        json_insert(&mut dependencies, "react-router-dom", json!("^6.14.2"));
        json_insert(&mut dependencies, "zustand", json!("^4.4.0"));
    }

    return dependencies;
}

fn get_dev_dependencies(
    template: &String,
    commit_lint: bool,
    js_lint: &String,
    style_lint: bool,
) -> Value {
    let rzpack_version = match tools::run_command("npm", &["view", "rzpack", "version"]) {
        Ok(output) => String::from_utf8_lossy(&output.stdout)
            .replacen("\n", "", 1)
            .to_string(),
        Err(_) => String::from("0.1.13"),
    };
    let rzpack_version = format!("^{}", rzpack_version);

    let is_admin_template = ["admin", "admin_header_menu"].contains(&template.as_str());
    let js_lint_packages = get_js_lint_packages(js_lint.as_str());

    let mut dev_dependencies = json!({
      "@types/react": "^18.0.25",
      "@types/react-dom":"^18.0.9",
      "rzpack":rzpack_version,
      "typescript": "5.1.6",
      "nodemon":"^3.0.1",
      "simple-git-hooks": "^2.9.0",
      "lint-staged": "^13.2.3",
    });

    dev_dependencies = match js_lint_packages {
        Some(value) => json_merge(dev_dependencies, value),
        None => dev_dependencies,
    };

    if is_admin_template {
        json_insert(&mut dev_dependencies, "@types/lodash-es", json!("^4.17.8"));
        json_insert(&mut dev_dependencies, "@types/nprogress", json!("^0.2.0"));
    }

    if commit_lint {
        json_insert(&mut dev_dependencies, "@commitlint/cli", json!("^17.6.7"));
        json_insert(
            &mut dev_dependencies,
            "@commitlint/config-conventional",
            json!("^17.6.7"),
        );
        json_insert(
            &mut dev_dependencies,
            "commitlint-config-cz",
            json!("^0.13.3"),
        );
        json_insert(&mut dev_dependencies, "cz-customizable", json!("^7.0.0"));
        json_insert(&mut dev_dependencies, "commitizen", json!("^4.3.0"));
        json_insert(&mut dev_dependencies, "standard-version", json!("^9.5.0"));
    }

    if style_lint {
        dev_dependencies = json_merge(
            dev_dependencies,
            json!({
              "stylelint": "^14.16.1".to_string(),
              "stylelint-config-property-sort-order-smacss":"^9.1.0",
              "stylelint-config-standard":"^29.0.0",
              "stylelint-order": "^5.0.0",
              "postcss-less":"^6.0.0",
            }),
        );
    }

    return dev_dependencies;
}

fn get_js_lint_packages(js_lint: &str) -> Option<Value> {
    let rzpack_lint_version =
        match tools::run_command("npm", &["view", "eslint-config-rzpack", "version"]) {
            Ok(output) => String::from_utf8_lossy(&output.stdout)
                .replacen("\n", "", 1)
                .to_string(),
            Err(_) => String::from("0.0.1"),
        };

    let rzpack_lint_version = format!("^{}", rzpack_lint_version);

    let eslint_packages = json!( {
      "eslint":"^8.46.0",
      "prettier":"^2.8.8",
      "eslint-config-rzpack":rzpack_lint_version,
    });

    let rome_packages = json!({
      "rome":"^12.1.3",
    });

    if !js_lint.is_empty() {
        if js_lint == "rome" {
            Some(rome_packages)
        } else {
            Some(eslint_packages)
        }
    } else {
        None
    }
}
