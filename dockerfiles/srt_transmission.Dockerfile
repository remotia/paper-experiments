FROM remotia:base

COPY . .

RUN apt-get -y install iproute2

# Compile the binary
RUN cargo build --profile experiments --example srt_transmission 

RUN mv target/experiments/examples/srt_transmission /bin/srt_transmission

ENTRYPOINT [ "auto_encoding" ]
