# Image Mosaic Tool

一个用 Rust 编写的图片指定区域打码工具，用于对图片的敏感区域进行马赛克处理。

## 参考来源

本文源码来源于微信公众号文章：[程序辕的极客日常：Rust开发批量图片打码小工具，解放财务岗手工繁琐](https://mp.weixin.qq.com/s/SoXejKMvjYAcXqEHY-cGQQ)

## 功能特性

- **指定区域打码**：支持对图片的多个指定区域进行马赛克打码
- **灵活坐标系统**：支持绝对坐标和相对坐标（从右下角开始计算）
- **可配置参数**：马赛克块大小和模糊强度可调
- **批量处理**：支持处理目录中的所有图片
- **并行处理**：可选的并行处理模式，提升处理速度
- **多种格式支持**：支持 JPG、JPEG、PNG、BMP、GIF 等常见图片格式
- **进度显示**：实时显示处理进度

## 使用方法

### 路径配置

```toml
[paths]
input_dir = "./input"          # 输入图片目录
output_dir = "./output"        # 输出图片目录
supported_formats = ["jpg", "jpeg", "png", "bmp", "gif"]  # 支持的图片格式
```

### 打码参数

```toml
[mosaic]
block_size = 20     # 马赛克块大小（像素）
blur_strength = 8   # 模糊强度（1-10）
```

### 打码区域

```toml
[[regions]]
name = "区域1"      # 区域名称
x = 100             # 左上角x坐标
y = 100             # 左上角y坐标
width = 200         # 宽度
height = 150        # 高度

[[regions]]
name = "区域2"
x = -300            # 负数：从右边界往左300像素
y = -200            # 负数：从下边界往上200像素
width = 250
height = 180
```

**坐标说明：**
- 正数：从图片左上角 (0,0) 开始计算
- 负数：从图片右下角开始计算

## 示例

1. 将图片放入 `input` 目录
2. 编辑 `config.toml` 配置打码区域
3. 运行工具：`cargo run`
4. 处理后的图片将保存在 `output` 目录

## 依赖

- `image`: 图片处理库
- `clap`: 命令行参数解析
- `serde` & `toml`: 配置解析
- `anyhow`: 错误处理
- `indicatif`: 进度条显示
- `rayon`: 并行处理

## 许可证

MIT License
