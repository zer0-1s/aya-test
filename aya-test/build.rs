use anyhow::{Context as _, anyhow};
use aya_build::Toolchain;
use cargo_metadata::MetadataCommand;
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    // 优先使用环境变量 PROJECT_NAME，否则提示用户输入
    let project_name = match std::env::var("PROJECT_NAME") {
        Ok(val) => val,
        Err(_) => {
            print!("please input project_name: ");
            io::stdout().flush()?; // 确保提示立即输出
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let name = input.trim().to_string();
            if name.is_empty() {
                return Err(anyhow!("project_name is empty"));
            }
            name
        }
    };

    let target_pkg = format!("{}-ebpf", project_name);

    // 读取 Cargo 元数据
    let metadata = MetadataCommand::new()
        .no_deps()
        .exec()
        .context("MetadataCommand::exec")?;

    // 查找对应 ebpf 子包
    let ebpf_package = metadata.packages
        .into_iter()
        .find(|pkg| pkg.name == target_pkg)
        .ok_or_else(|| anyhow!("package not found:{}", target_pkg))?;

    // 获取 ebpf 包路径
    let root_dir = ebpf_package
        .manifest_path
        .parent()
        .ok_or_else(|| anyhow!("no parent for manifest_path"))?
        .as_str();

    let ebpf_pkg = aya_build::Package {
        name: &ebpf_package.name,
        root_dir,
    };

    aya_build::build_ebpf([ebpf_pkg], Toolchain::default())
}
