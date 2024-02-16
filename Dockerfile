FROM rust

ADD target/release/rinha-2024-q1 rinha-2024-q1
ADD src/config/env src/config/env

ENTRYPOINT ["./rinha-2024-q1"]