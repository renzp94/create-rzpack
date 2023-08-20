use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// 创建项目模板名称
    #[arg(short, long, value_name = "String", default_value_t = String::from("rzpack-app"))]
    pub project_name: String,
    /// 模板类型
    #[arg(value_enum, short, long)]
    pub template: Option<String>,
    /// 是否覆盖目录
    #[arg(short, long, value_name = None)]
    pub force: Option<bool>,
}
