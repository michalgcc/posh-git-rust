FROM quay.io/pypa/manylinux_2_24_x86_64
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y 
ENV PATH="/root/.cargo/bin:${PATH}"
COPY . /app
WORKDIR /app
RUN ["cargo", "build", "--release"]
RUN ["strip", "target/release/posh-git-rust"]