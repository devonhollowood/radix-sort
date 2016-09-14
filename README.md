# radix-sort
This is a (for-fun) implementation of an [LSD radix sort](https://en.wikipedia.org/wiki/Radix_sort#Least_significant_digit_radix_sorts). This is a linear-time sort, taking advantage of the lexographic structure of the positive integers. Run `cargo bench` to collect evidence of linearity:

```
running 6 tests
test bench_1000000_items ... bench: 148,509,325 ns/iter (+/- 42,071,732)
test bench_100000_items  ... bench:  13,389,821 ns/iter (+/- 4,853,113)
test bench_10000_items   ... bench:   1,343,814 ns/iter (+/- 804,762)
test bench_1000_items    ... bench:     134,194 ns/iter (+/- 61,719)
test bench_100_items     ... bench:      21,148 ns/iter (+/- 8,194)
test bench_10_items      ... bench:       4,772 ns/iter (+/- 2,112)
```
