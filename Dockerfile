FROM messense/rust-musl-cross:x86_64-musl as builder
WORKDIR /aoc2023
RUN apt-get update && apt-get install nodejs -y
RUN apt-get install npm -y

# Copy the source code 
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

RUN rm -rf node_modules


FROM scratch
COPY --from=builder /aoc2023/target/x86_64-unknown-linux-musl/release/aoc2023-server /aoc2023-server
COPY --from=builder /aoc2023/assets /assets
COPY --from=builder /aoc2023/static /static
COPY --from=builder /aoc2023/input /input
COPY --from=builder /aoc2023/src /src

ENTRYPOINT ["/aoc2023-server"]
EXPOSE 80
