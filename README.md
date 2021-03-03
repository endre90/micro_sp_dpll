# micro_sp_dpll

A simple DPLL SAT solver for micro_sp implemented in Rust.

Read about DPLL here: https://en.wikipedia.org/wiki/DPLL_algorithm

The input formula can be provided either in a DIMACS CNF format, or as an arbitrary predicate. In the latter case, Tseitin's transformation is applied to transform the formula into CNF. Read about Tseitin's transformation here: https://en.wikipedia.org/wiki/Tseytin_transformation

### Ask for help
```
cargo run -- --help
```

### Example 1
Located in main.rs: `(x and (not y)) or (z or (x and (not w)))`

```
cargo run --release
```

### Example 2

```
cargo run --release -- -f dimacs -i /location/instance_name.cnf
```