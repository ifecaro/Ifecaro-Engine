FROM archlinux:base-devel

# Install necessary tools
RUN pacman -Syu --noconfirm \
    curl \
    git \
    pkg-config \
    openssl \
    openssh \
    chromium \
    rustup \
    base-devel \
    procps-ng \
    mold \
    sccache

RUN pacman -Scc --noconfirm


# Download Tailwind CSS Standalone CLI
RUN curl -LO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 && \
    chmod +x tailwindcss-linux-x64 && \
    mv tailwindcss-linux-x64 /usr/local/bin/tailwindcss

# Add environment variables (persistent for all RUN and CMD steps)
ENV RUSTC_WRAPPER="sccache"
ENV SCCACHE_DIR="~/.cache/sccache"
ENV CHROME_BINARY="/usr/bin/chromium"
ENV PATH="/root/.cargo/bin:${PATH}"
# ENV RUST_LOG="error"

# Link chromium to common browser aliases (for headless testing etc.)
RUN ln -s /usr/bin/chromium /usr/bin/google-chrome || true
RUN ln -s /usr/bin/chromium /usr/bin/chromium-browser || true

# Setup Rust
RUN rustup default nightly && \
    rustup target add wasm32-unknown-unknown && \
    cargo install wasm-pack dioxus-cli cargo-edit

# Set working directory (will be overridden in docker-compose)
WORKDIR /app

# Copy minimal files (actual source code will be volume mounted)
COPY Cargo.toml Cargo.lock ./
COPY tailwind.config.js ./
COPY src ./src
COPY public ./public

# Pre-fetch dependencies to layer cache
RUN cargo fetch && cargo upgrade && cargo update

CMD ["bash"]
