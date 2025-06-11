# Advent Of Code

Let's have a monorepo with multiple languages per year.

Project structure is shamelessly stolen from Christopher Biscardi.

# Usage

Starting a day:

```sh
just create day-0N
```

Working:

```sh
just work day-02 part2
```

Benchmarking:

```sh
just bench-criterion day-0N partN
just bench-divan day-0N part N
```

or

```sh
just bench-all
```

Running:

```sh
cd day-0N
cargo run --bin partN
```
