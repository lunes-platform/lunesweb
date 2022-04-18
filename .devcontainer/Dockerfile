FROM rust:slim

RUN apt update -y && apt install zsh git curl -y
RUN sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
RUN rustup component add rustfmt