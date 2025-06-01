FROM rustlang/rust:nightly-slim

# Install necessary tools
RUN apt-get update && \
    apt-get install -y curl git pkg-config libssl-dev openssh-client inotify-tools && \
    cargo install dioxus-cli

# Download Tailwind CSS Standalone CLI
RUN curl -LO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 && \
    chmod +x tailwindcss-linux-x64 && \
    mv tailwindcss-linux-x64 /usr/local/bin/tailwindcss

WORKDIR /app

# Copy configuration files (can be overridden with volume mount)
COPY Cargo.toml Cargo.lock ./
COPY tailwind.config.js ./
COPY src ./src
COPY public ./public

RUN cargo fetch
RUN cargo update

CMD ["bash"] 