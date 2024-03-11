FROM rust:1.76.0-slim-bullseye
RUN apt-get -y update
RUN apt-get -y install git sudo wget curl zsh

RUN sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

# For rust formatter
RUN rustup component add rustfmt