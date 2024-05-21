# sudokubrute

_A simple and fast implementation of backtrack solving of Sudokus_

## Instructions
The executable runs the solver on the hardest problem found so far. Which takes around 10s to solve.
```
cargo run -r
```

Tests checks whether it correctly solves a bunch of different sudokus.
```
cargo test
```

Benchmarks test how quickly a few sudokus are solved
```
cargo bench
```

## Technical details
This is a backtracking solver, see [here](https://en.wikipedia.org/wiki/Sudoku_solving_algorithms) for explanation on that part.

The most notable part is this code, found [here](https://github.com/jamadaha/sudokubrute/blob/d3b2de092274a04ba79b8147c80732233e52ed3e/src/board/mod.rs#L114):
```rust
pub fn candidates(&self, x: usize, y: usize) -> usize {
  self.valid_ranks[y] & self.valid_files[x] & self.get_box(x, y)
}
```
This gives for any square the valid values that can be placed there. It does so by finding the intersection between the valid values for rank, file, and the box.


## Benchmark
Benchmarked on a AMD Ryzen 5 5600X 6-Core Processor × 6 with [Criterion](https://github.com/bheisler/criterion.rs)

The benchmarks can be seen [here](https://github.com/jamadaha/sudokubrute/tree/master/benches)

![image](https://github.com/jamadaha/sudokubrute/assets/53914190/f8040cb4-42cd-4b3a-a019-7676856aafe6)

Solves most Sudokus in a few micro seonds and does around 159 million state modifications each second.
