# Homework

A library written in [Rust](https://www.rust-lang.org/) that handles sorting and serializing a simple model of `Person` records. A `Person` looks like this:

```json
{
    "last_name": "Brennan",
    "first_name": "Tom",
    "email": "tjb1982@gmail.com",
    "favorite_color": "red",
    "dob": "8/19/1982"
}
```

- [Homework](#homework)
  * [CLI](#cli)
  * [API](#api)
  * [Quickstart](#quickstart)
  * [Rationale](#rationale)


There are two clients that use this library: a [CLI interface](src/bin/cli.rs), and an [ReST interface](src/bin/api).

## CLI

The CLI is designed to work exclusively with CSV, ingesting a list of input files and outputting a list of records in CSV format to stdout.

It can read from stdin as well as a list of files you provide, with or without headers, and works with any `char` separator you provide.

For example, in the following, the CLI reads a list of three files and outputs the results sorted in the default order (i.e., by `last_name`, `first_name`, ...; ascending).

```bash
cli file1.csv file2.csv file3.csv
```

Using the `-S` flag, the separator is set to read CSV with a `"|"` for the separator, and using the `-E` flag set, the CLI will assume that all of the files have a header.

```bash
cli -S"|" -E -- file1.csv file2.csv file3.csv
```

Reading from stdin is just a matter of providing a `"-"` in place of one of the files. For example, if you want to generate some test records using the functional test suite, and have the CLI read those in as input, do:

```bash
python3 ./functional_tests/gen_people.py 500 | cli - file1.csv file2.csv file3.csv
```

You can also provide a mapping of separator/has-header combinations for each input file using flags.

```bash
cli file1.csv -s"|" -e true file2.csv -s"," file3.csv -e true -s" "
```

The output can also contain a header:

```bash
cli file1.csv file2.csv file3.csv -t
```

Sorting is available by a sequence of flags. For example, to sort by `favorite_color` ascending, then `first_name` descending:

```bash
cli file1.csv file2.csv file3.csv -f favorite_color -f first_name -d desc
```

You can also discover what fields there are using `-a`.

## API

The API is a ReST API with the following endpoints:

- `POST /records` - Post a single data line in any of the 3 formats supported by your existing code
- `GET /records/:field_name` - returns records sorted by `:field_name`
- `GET /records/color` - alias for `/records/favorite_color`
- `GET /records/birthdate` - alias for `/records/dob`
- `GET /records/name` - alias for `/records/last_name`


## Quickstart

To run both the CLI and ReST API with minimal effort, a Dockerfile is provided that builds the repository and launches the API service on port 8082 with a pre-populated database of 1000 randomly generated records.

```bash
docker build -t homework .
docker run --name homework -p 8082:8082 homework
```

To run the functional test suite, log into the running container:

```bash
docker exec -it homework bash
```

Once logged in, you have access to the target binaries and python, etc. Run the test suite like:

```bash
python3 functional_tests/main.py cli | json_pp | less
```

> N.B. The test suite reports in JSON format, so piping to `json_pp` and `less` makes it easier to view the results.

The test suite also takes a couple of arguments which you can see by passing the `-h` or `--help` flag.

## Rationale

Although the requirements for the homework assignment were pretty small, and could easily have been done in a dynamic language (such as Python or Clojure), I chose to do it in Rust for a few reasons:

### I love learning

I wanted to take this as an opportunity to learn a new language

### The requirements said to "put your best foot forward"

Setting up a non-trivial proof-of-concept in an unfamiliar technology is a relatively common on-the-job exercise in the real world. And Rust is a seriously difficult language to learn. One of the best ways I know how to "put my best foot forward" is to show that I can go from zero to "productive" relatively quickly in pretty much any language.

The key word here is "productive," as opposed to "mastered." Rust has a lot of complex concepts around its ownership model, and it will take some time before I feel confidently creative with it, the way I do with, say, JavaScript, Python, or Clojure. Or even C/C++.

### Rust and its toolchain are a really good fit for the requirements

The Rust ecosystem may not be as mature as the JVM, or Python, ecosystems, etc., but the tools and libraries it does have are particularly well-suited to creating nice-looking, safe/correct, CLIs and microservices. What makes Rust particularly good for APIs is that it compiles to a static binary and doesn't need garbage collection, like C++; but unlike C++ is much better at producing memory-safe code without sacrificing performance.
