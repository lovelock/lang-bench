# 使用官方 Rust 镜像作为基础镜像
FROM rust:latest

# 为了后续操作，更新apt-get并安装必要工具
RUN apt-get update && apt-get install -y \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# 创建一个工作目录
WORKDIR /usr/src/myapp

COPY config.toml /usr/local/cargo/

# 挂载点，您可以在运行容器时指定
VOLUME ["/usr/src/myapp"]

# 设置默认命令为进入交互模式
CMD ["cargo", "run", "--release"]
# CMD ["bash"]
