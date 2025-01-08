# 使用官方的 Nginx 镜像作为基础镜像
FROM nginx:latest

# 安装 PHP-FPM
RUN apt-get update && apt-get install -y \
    php-fpm \
    php-redis \
    && rm -rf /var/lib/apt/lists/*

# 将 Nginx 的默认配置文件替换为自定义的 www.conf
COPY www.conf /etc/nginx/conf.d/default.conf
# COPY fpm.conf /etc/php/8.2/fpm/pool.d/www.conf


# 将 PHP 文件复制到 Nginx 的根目录
COPY index.php /usr/share/nginx/html/index.php

# 配置 PHP-FPM
RUN sed -i 's/;cgi.fix_pathinfo=1/cgi.fix_pathinfo=0/' /etc/php/8.2/fpm/php.ini && \
    sed -i 's/listen = \/run\/php\/php8.2-fpm.sock/listen = 127.0.0.1:9000/' /etc/php/8.2/fpm/pool.d/www.conf

# 暴露 8080 端口
EXPOSE 8080

# 启动 Nginx 和 PHP-FPM
CMD service php8.2-fpm start && nginx -g "daemon off;"
