# syntax=docker/dockerfile:1.6

########################
# 1) Build stage (Rust, NIGHTLY)
########################
FROM rustlang/rust:nightly-slim AS builder

WORKDIR /usr/src/app

# Build tooling needed by Songbird / audiopus_sys (Opus)
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    build-essential \
    ca-certificates \
    cmake \
    libopus-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo metadata first (cache)
COPY Cargo.toml Cargo.lock ./

# Then the rest of the source
COPY . .

# Build the release binary (adjust bin name if needed)
RUN cargo build --release --bin mango-tango


########################
# 2) Runtime stage (same base => same glibc)
########################
FROM rustlang/rust:nightly-slim AS runtime

WORKDIR /usr/local/app

# Runtime deps: ffmpeg nightly, yt-dlp nightly, Opus runtime, Python for yt-dlp
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    xz-utils \
    libopus0 \
    python3 \
    && rm -rf /var/lib/apt/lists/*


##################################
# Install nightly ffmpeg (git)
##################################
RUN curl -L https://johnvansickle.com/ffmpeg/builds/ffmpeg-git-amd64-static.tar.xz -o /tmp/ffmpeg.tar.xz \
    && mkdir -p /opt/ffmpeg \
    && tar -xJf /tmp/ffmpeg.tar.xz -C /opt/ffmpeg --strip-components=1 \
    && ln -s /opt/ffmpeg/ffmpeg /usr/local/bin/ffmpeg \
    && ln -s /opt/ffmpeg/ffprobe /usr/local/bin/ffprobe \
    && rm /tmp/ffmpeg.tar.xz

##################################
# Install nightly yt-dlp binary
##################################
RUN curl -L https://github.com/yt-dlp/yt-dlp-nightly-builds/releases/latest/download/yt-dlp \
         -o /usr/local/bin/yt-dlp \
    && chmod +x /usr/local/bin/yt-dlp

##################################
# Copy compiled Rust binary
##################################
COPY --from=builder /usr/src/app/target/release/mango-tango /usr/local/bin/mango-tango

# Create non-root user and give them ownership of the work dir
RUN useradd -m app \
    && mkdir -p /usr/local/app \
    && chown -R app:app /usr/local/app /usr/local/bin/mango-tango

USER app

WORKDIR /usr/local/app

EXPOSE 8080

CMD ["mango-tango"]
