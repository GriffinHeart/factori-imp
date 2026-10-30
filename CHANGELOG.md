# Changelog

## Unreleased (0.9.4)

- Remove trailing semicolon from `create!` and `create_vec!` expansions, fixing the
  `semicolon_in_expressions_from_non_local_macros` warning ([rust-lang/rust#79813](https://github.com/rust-lang/rust/issues/79813))

## 0.9.3

- Support using `create*` macros inside of `factori!`

## 0.9.2

- Update readme with how to in-place replace factori

## 0.9.1

- fixes wrong doc link in readme

## 0.9.0

- First release
- Adds `create_vec!`
- Adds transient attributes
- Fixes clippy warnings
