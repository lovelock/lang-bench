# 使用官方的 Golang 镜像作为基础镜像
FROM golang:1.23-alpine AS builder

# 设置工作目录
WORKDIR /app

# 将 Go 模块文件复制到工作目录
COPY go.mod go.sum ./

# 下载依赖
RUN go mod download

# 将项目代码复制到工作目录
COPY *.go .

# 编译 Go 程序
RUN CGO_ENABLED=0 GOOS=linux go build -o bench .

# 使用轻量级的 Alpine 镜像作为运行时镜像
FROM alpine:latest

# 设置工作目录
WORKDIR /app

# 从 builder 阶段复制编译好的二进制文件
COPY --from=builder /app/bench .

# 暴露端口（如果需要）
EXPOSE 8080

# 运行程序
CMD ["./bench"]
