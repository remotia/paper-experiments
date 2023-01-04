FROM ubuntu:20.04

# Install dependencies
ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=Etc/UTC
RUN apt-get update
RUN apt-get install -y git curl cmake yasm libclang-dev linux-libc-dev  \
    libc6-dev gcc g++ pkg-config libx264-dev libx265-dev \
    libxcb1-dev libxcb-shm0-dev libxcb-randr0-dev

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup default nightly

# Copy source code
WORKDIR /home/paper-experiments/
COPY utils/linux_ffmpeg.rs utils/linux_ffmpeg.rs

# Compile ffmpeg and libav
RUN bash utils/linux_ffmpeg.rs
ENV FFMPEG_PKG_CONFIG_PATH="/home/paper-experiments/tmp/ffmpeg_build/lib/pkgconfig"
ENV FFMPEG_INCLUDE_DIR="/home/paper-experiments/tmp/ffmpeg_build/include"
