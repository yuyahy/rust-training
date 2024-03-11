FROM rust:1.76.0-slim-bullseye
RUN apt-get -y update
RUN apt-get -y install git sudo wget curl zsh nano

RUN sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

# For rust formatter
RUN rustup component add rustfmt

# Prevent garbled text in "git log"
RUN echo "export LESSCHARSET=utf-8" >> ~/.zshrc