FROM rust:1.95-bookworm

ARG CARGO_LLVM_COV_VERSION=0.8.4

RUN apt-get update \
    && apt-get install --yes --no-install-recommends \
        fonts-noto-cjk \
        fonts-noto-color-emoji \
        libwayland-dev \
        libx11-dev \
        libxcursor-dev \
        libxi-dev \
        libxkbcommon-dev \
        libxrandr-dev \
        pkg-config \
        xauth \
        xfonts-base \
        xvfb \
    && rm -rf /var/lib/apt/lists/*

RUN rustup component add llvm-tools-preview \
    && cargo install cargo-llvm-cov --version "${CARGO_LLVM_COV_VERSION}" --locked
