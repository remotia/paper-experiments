FROM remotia:base

COPY . .

# Compile the binary
RUN cargo build --profile experiments --example auto_encoding

RUN mv target/experiments/examples/auto_encoding /bin/auto_encoding

ENTRYPOINT [ "auto_encoding" ]
