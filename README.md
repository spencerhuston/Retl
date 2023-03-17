# Retl
Minimal CSV ETL language created as a project to learn Rust

Supported features in-progress:
- REPL
- CSV read/write
- stdin/out
- integer, boolean, char, string, null, list, tuple, and dictionary types
- range construction for integer type
- schema type
- list concant operator
- type inference
- strong, dynamic typing
- type-aliasing
- basic integer arithmetic
- if/else branching
- pattern matching with:
  * match by type
  * multi-match: |
  * range: ..
  * predicates
- Builtin functions: map, filter, foreach, etc.
- user-defined functions as lambdas