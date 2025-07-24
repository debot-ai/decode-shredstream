# 构建说明

## 本地编译

### macOS (Apple Silicon)
```bash
# 编译 macOS ARM64 版本
cargo build --release --example deshred
```

### Linux x86_64
```bash
# 在 Linux x86_64 系统上编译
cargo build --release --example deshred
```

## 交叉编译

### 使用 Docker 交叉编译

#### 方法 1: 使用 build-linux.sh 脚本
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

## 二进制文件说明

### 当前生成的二进制文件
- `deshred-linux-x86_64`: Linux ARM64 版本 (在 Apple Silicon Mac 上编译)
- `target/release/examples/deshred`: macOS ARM64 版本

### 在 CentOS 9 上使用
由于当前编译环境限制，生成的二进制文件是 ARM64 架构。要在 CentOS 9 x86_64 上运行，需要：

1. **在 Linux x86_64 系统上重新编译**
2. **或者使用 Docker 容器运行**

### Docker 运行方式
```bash
# 创建 Docker 镜像
docker build -f Dockerfile.simple -t shredstream .

# 运行容器
docker run --rm shredstream --host 172.245.211.10:8002
```

## 注意事项

1. 当前在 Apple Silicon Mac 上编译会生成 ARM64 架构的二进制文件
2. 要在 x86_64 Linux 系统上运行，需要在对应的架构上编译
3. 建议在目标平台上直接编译，或者使用 Docker 容器运行 