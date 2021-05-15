FROM --platform=arm64 rustlang/rust:nightly
RUN mkdir /nysh
WORKDIR /nysh
ADD . /nysh