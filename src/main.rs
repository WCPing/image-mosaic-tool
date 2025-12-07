mod config;
mod processor;

use anyhow::Result;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 配置文件路径
    #[arg(short, long, default_value = "config.toml")]
    config: String,

    /// 是否并行处理
    #[arg(short, long, default_value_t = false)]
    parallel: bool,

    /// 显示详细信息
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // 加载配置
    println!("加载配置文件:{}", args.config);
    let config = config::Config::from_file(&args.config)?;
    config.validate()?;

    // 创建输出目录
    std::fs::create_dir_all(&config.paths.output_dir)?;

    // 创建处理器
    let processor = Arc::new(processor::ImageProcessor::new(config));

    // 获取所有图片文件
    let files = processor.get_image_files()?;
    if files.is_empty() {
        println!("未找到需要处理的图片文件");
        return Ok(());
    }

    println!("找到{}个图片文件", files.len());

    // 创建进度条
    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )?
            .progress_chars("#>-"),
    );

    // 处理图片
    if args.parallel {
        println!("开始并行处理");
        files.par_iter().for_each(|file| {
            if let Err(e) = processor.process_imgae(file) {
                eprintln!("处理文件 {:?} 失败:{}", file, e);
            }
            pb.inc(1);
        })
    } else {
        println!("开始顺序处理");
        for file in &files {
            if let Err(e) = processor.process_imgae(file) {
                eprintln!("处理文件 {:?} 失败:{}", file, e);
            }
            pb.inc(1);
        }
    }

    pb.finish_with_message("处理完成");
    println!(
        "\n所有图片处理完成！输出目录: {:?}",
        processor.config.paths.output_dir
    );

    Ok(())
}
