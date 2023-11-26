FROM mwader/static-ffmpeg as ffmpeg_base
FROM rust:1.71.1 as builder

RUN apt-get update && apt-get install -y \
    clang \
	libavcodec-dev \
	libavformat-dev \
	libavutil-dev \
	libavfilter-dev \
	libavdevice-dev \
	pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

# Create the final image
FROM debian:bullseye-slim

COPY --from=mwader/static-ffmpeg:6.0 /ffmpeg /usr/local/bin/
COPY --from=mwader/static-ffmpeg:6.0 /ffprobe /usr/local/bin/

# Install runtime dependencies

COPY --from=builder /usr/src/app/target/release/av1-encoder-service /usr/local/bin/av1-encoder-service