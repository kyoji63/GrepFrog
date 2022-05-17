FROM alpine:3.15.4 AS builder

WORKDIR /home/grepfrog

COPY . .

RUN  apk add --no-cache gcc musl-dev rust cargo \
  && cargo build --release

FROM alpine:3.15.4

RUN  apk add --no-cache libgcc musl-dev \
  && adduser -D grepfrog

COPY --from=builder /home/grepfrog/target/release/grepfrog /opt/grepfrog/grepfrog

WORKDIR /home/grepfrog
USER grepfrog

ENTRYPOINT [ "/opt/grepfrog/grepfrog" ]