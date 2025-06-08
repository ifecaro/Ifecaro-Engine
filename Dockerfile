FROM archlinux:base-devel

# Install necessary tools
RUN pacman -Syu --noconfirm && \
    pacman -S --noconfirm \
    curl \
    git \
    pkg-config \
    openssl \
    openssh \
    inotify-tools \
    chromium \
    rustup \
    base-devel

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

# Setup Rust
RUN rustup default nightly && \
    rustup target add wasm32-unknown-unknown && \
    cargo fetch && \
    cargo update && \
    cargo install wasm-pack && \
    cargo install dioxus-cli

ENV CHROME_BINARY=/usr/bin/chromium
ENV PATH="/root/.cargo/bin:${PATH}"
RUN ln -s /usr/bin/chromium /usr/bin/google-chrome || true
RUN ln -s /usr/bin/chromium /usr/bin/chromium-browser || true

ENV RUST_LOG=error

CMD ["bash"] 