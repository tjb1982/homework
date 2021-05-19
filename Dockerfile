FROM rust

WORKDIR /usr/src/homework

COPY . .

RUN cargo install --path .

CMD python3 ./functional_tests/gen_people.py 1000 | /usr/local/cargo/bin/api --hostname 0.0.0.0:8082 -- -
