# How to Install

```
git pull https://github.com/raposay/fizzbuzz/
cd fizzbuzz
cargo build -r
cargo run
```

# Why?
I wanted to experiment with maps and the COW (Copy-on-write) type.
This implementation of FizzBuzz uses a COW return type to prevent unneccessary string allocations. The string literals are never mutated so there is no need to cast them as String.