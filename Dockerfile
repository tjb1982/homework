FROM rust

WORKDIR /usr/src/homework

COPY . .

RUN cargo install --path .

# python ./functional_tests/gen_people.py 10 | target/release/api --hostname localhost:8082 -- -
CMD python3 ./functional_tests/gen_people.py 10 | /usr/local/cargo/bin/api --hostname 0.0.0.0:8082 -- -
