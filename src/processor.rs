use anyhow::Result;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use std::path::{Path, PathBuf};

use crate::config::Config;

pub struct ImageProcessor {
    pub config: Config,
}

impl ImageProcessor {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 处理单张图片
    pub fn process_imgae(&self, input_path: &Path) -> Result<()> {
        // 加载图片
        let img = image::open(input_path)?;
        let (width, height) = img.dimensions();

        println!("处理图片:{:?}, 尺寸:{}x{}", input_path, width, height);

        // 应用所有打码区域
        let mut img = img;
        for region in &self.config.regions {
            let (x, y, w, h) = region.to_absolute_coords(width, height);
            println!(
                "  应用打码区域 '{}': 位置({}, {}), 大小({}x{})",
                region.name, x, y, w, h
            );
            img = self.apply_mosaic(img, x, y, w, h)?;
        }

        // 保存处理后的图片
        let output_path = self.get_output_path(input_path)?;
        img.save(&output_path)?;
        println!("处理完成, 保存到:{:?}", output_path);
        Ok(())
    }

    /// 对指定区域进行马赛克
    fn apply_mosaic(
        &self,
        img: DynamicImage,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Result<DynamicImage> {
        let block_size = self.config.mosaic.block_size;

        // Ensure we have an RGBA8 buffer
        let mut img_buffer = img.into_rgba8();

        // 按块处理区域
        for block_y in (y..y + height).step_by(block_size as usize) {
            for block_x in (x..x + width).step_by(block_size as usize) {
                // 计算当前块的实际大小
                let actual_block_width = block_size.min(x + width - block_x);
                let actual_block_height = block_size.min(y + height - block_y);

                // 计算块内的平均颜色
                let avg_color = self.calculate_average_color(
                    &img_buffer,
                    block_x,
                    block_y,
                    actual_block_width,
                    actual_block_height,
                );

                // 填充当前块
                for py in block_y..block_y + actual_block_height {
                    for px in block_x..block_x + actual_block_width {
                        img_buffer.put_pixel(px, py, avg_color);
                    }
                }
            }
        }

        Ok(DynamicImage::ImageRgba8(img_buffer))
    }

    /// 计算指定区域的平均颜色
    fn calculate_average_color(
        &self,
        img: &ImageBuffer<Rgba<u8>, Vec<u8>>,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Rgba<u8> {
        let mut r_sum = 0u32;
        let mut g_sum = 0u32;
        let mut b_sum = 0u32;
        let mut a_sum = 0u32;
        let mut count = 0u32;

        for py in y..y + height {
            for px in x..x + width {
                let pixel: &Rgba<u8> = img.get_pixel(px, py);
                r_sum += pixel[0] as u32;
                g_sum += pixel[1] as u32;
                b_sum += pixel[2] as u32;
                a_sum += pixel[3] as u32;
                count += 1;
            }
        }

        if count == 0 {
            return Rgba([0, 0, 0, 255]);
        }

        // 应用模糊强度
        let blur = self.config.mosaic.blur_strength as f32 / 10.0;
        let factor = 1.0 - blur * 0.3; // 降低颜色精度以增强马赛克效果

        Rgba([
            ((r_sum / count) as f32 * factor) as u8,
            ((g_sum / count) as f32 * factor) as u8,
            ((b_sum / count) as f32 * factor) as u8,
            (a_sum / count) as u8,
        ])
    }

    /// 获取输出文件路径
    fn get_output_path(&self, input_path: &Path) -> Result<PathBuf> {
        let file_name = input_path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("无法获取文件名"))?;

        // 拼接输出路径
        let output_path = self.config.paths.output_dir.join(file_name);

        // 确保输出目录存在
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        Ok(output_path)
    }

    /// 获取所有需要处理的图片文件
    pub fn get_image_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        for entry in std::fs::read_dir(&self.config.paths.input_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    // 将扩展名转换为小写
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    // 检查文件扩展名是否在支持的格式列表中
                    if self.config.paths.supported_formats.contains(&ext_str) {
                        files.push(path)
                    }
                }
            }
        }

        Ok(files)
    }
}
