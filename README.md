# Retl
Minimal CSV ETL language created as a project to learn Rust

Supported features in-progress:
- REPL
- CSV read/write
- stdin/out
- integer, boolean, char, string, list, and dictionary types
- range construction for int and char types
- list operators: ++, ===, []
- type inference
- basic integer arithmetic
- if/else branching
- pattern matching with:
  * multi-match: |
  * range: ..
  * named match: @
- Builtin functions: map, filter, foreach, etc.
- Lazy evaluation
- *parallel* keyword
