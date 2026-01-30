# Dockerfile for building Murmure on Ubuntu 24.04
# Based on .github/workflows/build-linux.yml

FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive

# Install system dependencies (matching build-linux.yml)
RUN apt-get update && apt-get install -y \
    libwebkit2gtk-4.1-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    patchelf \
    libasound2-dev \
    libxdo-dev \
    build-essential \
    pkg-config \
    libssl-dev \
    libgtk-3-dev \
    unzip \
    curl \
    ca-certificates \
    gnupg \
    xdg-utils \
    file \
    libfuse2 \
    librsvg2-bin \
    gtk-update-icon-cache \
    libgdk-pixbuf2.0-bin \
    shared-mime-info \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js LTS
RUN curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Install pnpm
RUN npm install -g pnpm

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /murmure
