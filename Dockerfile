FROM rustlang/rust:nightly-slim

# 安裝必要工具
RUN apt-get update && \
    apt-get install -y curl git pkg-config libssl-dev openssh-client && \
    cargo install dioxus-cli

# 下載 Tailwind CSS Standalone CLI
RUN curl -LO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 && \
    chmod +x tailwindcss-linux-x64 && \
    mv tailwindcss-linux-x64 /usr/local/bin/tailwindcss

WORKDIR /app

# 複製設定檔（可用 volume 掛載覆蓋）
COPY Cargo.toml Cargo.lock ./
COPY tailwind.config.js ./
COPY src ./src
COPY public ./public

RUN cargo fetch
RUN cargo update

CMD ["bash"] 