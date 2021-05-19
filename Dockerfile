FROM rust

WORKDIR /usr/src/homework

COPY . .

RUN apt-get update -y && apt-get install less python3-pip -y
RUN pip3 install -r functional_tests/requirements.txt
RUN cargo install --path .

CMD python3 ./functional_tests/gen_people.py 1000 | /usr/local/cargo/bin/api --hostname 0.0.0.0:8082 -- -
