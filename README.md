# Codewars Solutions

This repository contains my solutions to various [Codewars](https://www.codewars.com/) kata challenges, implemented in Rust.

## Challenges

### 6 kyu - Multiples of 3 or 5

- **Directory:** `multiples_3_or_5`
- **Description:** If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23. Finish the solution so that it returns the sum of all the multiples of 3 or 5 below the number passed in. Additionally, if the number is negative, return 0. If the number is a multiple of both 3 and 5, only count it once.

## How to run tests

To run the tests for all challenges:

```bash
cargo test
```

To run tests for a specific challenge:

```bash
cargo test -p multiples_3_or_5
```
