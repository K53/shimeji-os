FROM ubuntu:latest

RUN apt update && apt upgrade -y && apt install -y curl build-essential

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN apt install lld file -y

ENV PATH /root/.cargo/bin:$PATH

WORKDIR /rust

RUN rustup toolchain install nightly-2022-06-18 && rustup default nightly-2022-06-18

RUN rustup component add rust-src --toolchain nightly-2022-06-18-x86_64-unknown-linux-gnu

# COPY ./bootloader /rust/bootloader/
# COPY ./kernel /rust/kernel/
# COPY ./shared /rust/shared/

# CMD cd /rust/bootloader \
#     && cargo +nightly-2022-06-18 rustc --target x86_64-unknown-uefi -Z build-std  \
#     && cd /rust/kernel \
#     && cargo build


