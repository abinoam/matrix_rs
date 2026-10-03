# AGENTS.md

Guidance for AI coding agents (Claude Code, OpenAI Codex, GitHub Copilot, etc.) working in this repository.

## Project

`matrix_rs` is a Ruby gem that aims to be a drop-in replacement for Ruby's stdlib `Matrix` class, implemented in Rust. It is an early-stage learning project: only `MatrixRs.[]`, `MatrixRs.empty` (still a stub returning nested Arrays), `#to_s` and `#*` (matrix product) exist.

## Commands

```sh
bin/setup                       # bundle install
git submodule update --init     # fetch vendor/matrix (Ruby Matrix gem, needed for its test suite)

bundle exec rake test           # builds the Rust cdylib (thermite:build), runs cargo tests (thermite:test), then the Minitest suite
bundle exec rake thermite:build # build only the Rust extension
cargo test                      # Rust unit tests only

# Single Ruby test (extension must be built first)
bundle exec ruby -Ilib:test test/matrix_rs_test.rb -n test_matrix_dot

# Also run the upstream Ruby Matrix test suite against MatrixRs (expected to fail heavily for now)
RUN_RUBY_MATRIX_TEST_SUITE=true bundle exec rake test

bundle exec rubocop             # Ruby lint (double-quoted strings enforced)
```

CI (`.travis.yml`) sets `NO_LINK_RUTIE=1` and runs the matrix both with and without `RUN_RUBY_MATRIX_TEST_SUITE`; the `true` job is allowed to fail.

## Architecture

- **Ruby ↔ Rust bridge**: uses [rutie](https://github.com/danielpclark/rutie) (Rust side) and the `rutie` gem (Ruby side), built via [thermite](https://github.com/malept/thermite) (`Rakefile`, `ext/Rakefile` for gem install). `lib/matrix_rs.rb` only declares `class MatrixRs` and calls `Rutie.new(:matrix_rs).init "Init_matrix_rs"`, which loads the compiled `cdylib` and defines all methods.
- **`src/lib.rs`**: `Init_matrix_rs` registers the Ruby methods. Most methods are declared with rutie's `methods!` macro (`pub_*` naming convention). `MatrixRs.[]` is a raw `extern "C"` variadic function using `rb_scan_args` because `methods!` cannot take splat args.
- **Wrapped data**: each `MatrixRs` Ruby object wraps a `WrappableMatrix` (`src/wrappable_matrix.rs`) via `wrappable_struct!` / `MATRIX_WRAPPER_INSTANCE`. It holds an `ndarray::Array2<f64>` — currently **only floats are supported** (integer input raises; the integer dot-product test is skipped).
- **Error handling**: `src/args_treating.rs` provides `unwrap_or_rb_raise()` on `Result`, converting rutie conversion errors into Ruby exceptions.

## Tests

- `test/matrix_rs_test.rb`: project's own Minitest tests.
- `test/matrix_rb_test.rb`: when `RUN_RUBY_MATRIX_TEST_SUITE=true`, loads `vendor/matrix/test/matrix/test_matrix.rb` (git submodule of the Ruby Matrix gem) unmodified.
- `test/test/unit.rb`: shim that makes that suite run under Minitest — defines `Test::Unit::TestCase = Minitest::Test`, aliases test-unit assertions, and rebinds the constant `Matrix` to `MatrixRs` inside the test case so the upstream tests exercise this implementation. Keep this shim instead of editing the submodule's tests.

## Commits

Do not add AI attribution trailers (e.g. `Co-Authored-By: Claude ...`, `Co-authored-by: Copilot ...`) or "Generated with ..." lines to commit messages or PR descriptions.
