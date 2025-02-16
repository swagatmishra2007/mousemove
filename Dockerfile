FROM ubuntu:24.04

RUN apt-get update && apt-get install -y \
    curl wget tar xz-utils git cmake pkg-config libssl-dev zlib1g-dev clang

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup target add x86_64-apple-darwin

RUN git clone https://github.com/tpoechtrager/osxcross.git /osxcross

RUN ls -l /
RUN ls -l /osxcross

WORKDIR /osxcross
RUN cd /osxcross && ls -l
RUN chmod +x /osxcross/build.sh
RUN ./build.sh

# Set up Rust environment (adjust as needed for your project)
# ENV PATH="/root/.cargo/bin:${PATH}"  <--  No longer needed here, set above