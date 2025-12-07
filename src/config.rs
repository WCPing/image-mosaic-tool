use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub paths: PathConfig,
    pub mosaic: MosaicConfig,
    pub regions: Vec<Region>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PathConfig {
    pub input_dir: PathBuf,
    pub output_dir: PathBuf,
    pub supported_formats: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MosaicConfig {
    pub block_size: u32,
    pub blur_strength: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Region {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<()> {
        // 判断目录是否存在
        if !self.paths.input_dir.exists() {
            anyhow::bail!("输入目录不存在:{:?}", self.paths.input_dir);
        }
        // 检测马赛克块大小
        if self.mosaic.block_size == 0 {
            anyhow::bail!("马赛克块大小不能为0");
        }
        // 检测模糊强度
        if self.mosaic.blur_strength == 0 || self.mosaic.blur_strength > 10 {
            anyhow::bail!("模糊强度必须在1-10之间");
        }
        // 检测区域坐标
        for region in &self.regions {
            if region.width ==0 || region.height == 0 {
                anyhow::bail!("区域{}的宽度或高度不能为0", region.name);
            }
        }
        Ok(())
    }
}

impl Region {
    /// 将配置的坐标转换为实际图片坐标
    /// 负数坐标从图片右下角开始计算
    pub fn to_absolute_coords(&self, img_width: u32, img_height: u32) -> (u32, u32, u32, u32) {
        let x = if self.x >= 0 {
            self.x as u32
        } else {
            // x小于0，说明往右偏移
            ((img_width as i64) + (self.x as i64)).max(0) as u32
        };

        let y = if self.y >= 0 {
            self.y as u32
        } else {
            ((img_height as i64) + (self.y as i64)).max(0) as u32
        };

        // 确保不超出图片边界
        let x = x.min(img_width.saturating_sub(1)); // 确保x坐标不超过图像宽度-1
        let y = y.min(img_height.saturating_sub(1));
        let width = self.width.min(img_width - x); // 确保宽度不会超出图像右边界
        let height = self.height.min(img_height - y);

        (x, y, width, height)
    }
}
