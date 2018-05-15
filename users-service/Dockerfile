FROM rust
WORKDIR /usr/src/app
COPY . . 
RUN apt-get update 
RUN apt-get install -y libzmq3-dev 
RUN cargo install --force
EXPOSE 3000
CMD ["users-service"]
