# Jito Shredstream Proxy

ShredStream provides the lowest latency to shreds from leaders on Solana. 

See more at https://docs.jito.wtf/lowlatencytxnfeed/

## 项目简介

这是一个用于连接 Jito Shredstream 代理服务器的 Rust 应用程序，可以实时接收和处理 Solana 区块链的 entries 数据。

## 功能特性

- 🔗 **实时连接**: 连接到 Jito Shredstream 代理服务器
- 📊 **数据处理**: 实时处理 Solana entries 和交易数据
- 🚀 **高性能**: 使用异步处理和并发控制
- ⚙️ **可配置**: 支持命令行参数自定义服务器地址
- 🇨🇳 **中文界面**: 完整的中文用户界面和错误提示

## 编译

### 环境要求

- Rust 1.70+ 
- Cargo

### 本地编译

#### macOS (Apple Silicon)
```bash
# 编译 macOS ARM64 版本
cargo build --release --example deshred
```

#### Linux x86_64
```bash
# 在 Linux x86_64 系统上编译
cargo build --release --example deshred
```

### 交叉编译 (Linux x86_64)

由于架构差异，在 Apple Silicon Mac 上编译会生成 ARM64 架构的二进制文件。要在 CentOS 9 等 x86_64 Linux 系统上运行，可以使用以下方法：

#### 方法 1: 使用 Docker 交叉编译脚本
```bash
# 运行交叉编译脚本
chmod +x build-linux.sh
./build-linux.sh
```

#### 方法 2: 手动 Docker 编译
```bash
# 使用 Docker 进行交叉编译
docker run --rm -v "$(pwd):/app" -w /app rust:1.84-slim bash -c "
    apt-get update && apt-get install -y build-essential pkg-config libssl-dev
    cargo build --release --example deshred
    cp target/release/examples/deshred deshred-linux-x86_64
    chmod +x deshred-linux-x86_64
"
```

### 编译输出

编译完成后，二进制文件将生成在：
```
target/release/examples/deshred          # macOS ARM64 版本
deshred-linux-x86_64                     # Linux 版本 (如果使用交叉编译)
```

## 使用方法

### 基本用法

```bash
# 使用默认服务器地址 (172.245.211.10:8002)
./target/release/examples/deshred

# 指定自定义服务器地址
./target/release/examples/deshred --host x.x.x.x:8001

# 查看帮助信息
./target/release/examples/deshred --help
```

### 架构兼容性说明

⚠️ **重要提示**: 由于架构差异，在不同平台上编译的二进制文件可能无法在其他平台上运行：

- **Apple Silicon Mac**: 编译生成 ARM64 架构二进制文件
- **Intel Mac**: 编译生成 x86_64 架构二进制文件  
- **Linux x86_64**: 编译生成 x86_64 架构二进制文件

**解决方案**:
1. 在目标平台上直接编译
2. 使用 Docker 容器运行 (推荐)
3. 使用交叉编译 (需要额外配置)

### Docker 运行方式 (推荐)

```bash
# 创建 Docker 镜像
docker build -f Dockerfile.simple -t shredstream .

# 运行容器
docker run --rm shredstream --host 172.245.211.10:8002
```

### 命令行参数

| 参数 | 说明 | 默认值            | 示例 |
|------|------|----------------|------|
| `--host` | 服务器地址 | `x.x.x.x:8001` | `--host x.x.x.x:8001` |
| `-h, --help` | 显示帮助信息 | -              | `--help` |

### 运行示例

```bash
# 连接到默认服务器
$ ./target/release/examples/deshred
正在连接到服务器 http://x.x.x.x:8001...
成功连接到服务器
正在订阅 entries 流...
成功订阅 entries 流
开始接收数据...
slot 123456789, entries: 5, transactions: 25
Slot: 123456789, Entry: 0, Tx: 0, Hash: 5J7X...
Slot: 123456789, Entry: 0, Tx: 1, Hash: 3K8Y...
...
```

## 输出说明

程序运行时会输出以下信息：

1. **连接状态**: 显示服务器连接进度
2. **订阅状态**: 显示 entries 流订阅状态
3. **数据统计**: 每个 slot 的 entries 数量和交易数量
4. **交易详情**: 每个交易的 slot、entry、交易索引和哈希值

## 错误处理

程序包含完整的错误处理机制：

- **连接失败**: 显示详细的连接错误信息
- **订阅失败**: 显示订阅错误信息
- **反序列化错误**: 跳过损坏的数据并继续处理
- **网络错误**: 自动重连和错误恢复

## 性能优化

- **并发控制**: 使用信号量限制并发任务数量（默认10个）
- **异步处理**: 所有网络操作都是异步的
- **内存优化**: 使用流式处理，避免内存溢出

## 开发

### 项目结构

```
shredstream-proxy/
├── app/                    # 应用程序代码
│   ├── Cargo.toml         # 应用依赖配置
│   └── deshred.rs         # 主程序代码
├── jito_protos/           # Jito 协议定义
├── proxy/                 # 代理服务器代码
└── target/                # 编译输出目录
```

### 开发环境设置

```bash
# 安装 Rust (如果还没有安装)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 更新 Rust
rustup update

# 检查版本
rustc --version
cargo --version
```

### 调试模式编译

```bash
# 调试版本编译
cargo build --example deshred

# 运行调试版本
cargo run --example deshred -- --host x.x.x.x:8001
```

## 故障排除

### 常见问题

1. **连接被拒绝**
   - 检查服务器地址是否正确
   - 确认服务器是否正在运行
   - 检查网络连接

2. **编译错误**
   - 确保 Rust 版本 >= 1.70
   - 运行 `cargo clean` 清理缓存
   - 检查依赖是否正确安装

3. **权限问题**
   - 确保二进制文件有执行权限：`chmod +x target/release/examples/deshred`

## 许可证

本项目遵循相应的开源许可证。

## 免责声明

Use this at your own risk.

## 相关链接

- [Jito 官方文档](https://docs.jito.wtf/lowlatencytxnfeed/)
- [Solana 官方文档](https://docs.solana.com/)
- [Rust 官方文档](https://doc.rust-lang.org/)
