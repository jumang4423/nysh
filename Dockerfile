FROM rustlang/rust:nightly
RUN mkdir /nysh
WORKDIR /nysh
ADD . /nysh