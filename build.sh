#!/bin/bash

# 自定义构建脚本，支持 --app 参数

# 解析参数
APP_NAME=""
BUILD_TYPE="release"

while [[ $# -gt 0 ]]; do
    case $1 in
        --app)
            APP_NAME="$2"
            shift 2
            ;;
        --debug)
            BUILD_TYPE="debug"
            shift
            ;;
        --release)
            BUILD_TYPE="release"
            shift
            ;;
        *)
            echo "未知参数: $1"
            echo "用法: $0 --app <app_name> [--debug|--release]"
            exit 1
            ;;
    esac
done

if [[ -z "$APP_NAME" ]]; then
    echo "错误: 必须指定 --app 参数"
    echo "用法: $0 --app <app_name> [--debug|--release]"
    exit 1
fi

echo "正在构建应用: $APP_NAME (模式: $BUILD_TYPE)"

# 执行实际的 cargo 命令
if [[ "$BUILD_TYPE" == "release" ]]; then
    cargo build --release --bin "$APP_NAME"
else
    cargo build --bin "$APP_NAME"
fi

echo "构建完成！" 