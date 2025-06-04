FROM rustlang/rust:nightly-slim

# Install necessary tools
RUN apt-get update && \
    apt-get install -y \
    curl git pkg-config libssl-dev openssh-client inotify-tools chromium chromium-sandbox chromium-common chromium-driver && \
    apt-get install -y \
    libglib2.0-0 libnss3:amd64 libnss3-dev libnssutil3 libnspr4 \
    wget ca-certificates fonts-liberation libappindicator3-1 libasound2 \
    libatk-bridge2.0-0 libatk1.0-0 libcups2 libdbus-1-3 libdrm2 libgbm1 \
    libu2f-udev libvulkan1 libxcomposite1 libxdamage1 libxrandr2 xdg-utils \
    libgconf-2-4 libfontconfig1 libxss1 libxtst6 libgtk-3-0 libpango-1.0-0 \
    libpangocairo-1.0-0 libcairo2 libatspi2.0-0 libwayland-client0 \
    libwayland-cursor0 libwayland-egl1 libxkbcommon0 libdbus-glib-1-2 \
    libgdk-pixbuf2.0-0 libx11-6 libxext6 libxi6 libxrender1 libxfixes3 \
    libxcursor1 libxinerama1 libc-bin && \
    apt-get clean && rm -rf /var/lib/apt/lists/* && \
    cargo install dioxus-cli wasm-pack && \
    ln -s /usr/lib/x86_64-linux-gnu/libglib-2.0.so.0 /usr/lib/libglib-2.0.so.0 || true

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

ENV CHROME_BINARY=/usr/bin/chromium
RUN ln -s /usr/bin/chromium /usr/bin/google-chrome || true
RUN ln -s /usr/bin/chromium /usr/bin/chromium-browser || true

ENV RUST_LOG=error

CMD ["bash"] 