# Changelog

## 0.8.1 (`colorize-proc-macro` 0.2.1)

### Fixed
- Escaped braces now work like in `format!`: `colorize!("{{literal}} {}", Fg->"x")`
  previously failed with "args are not closed".
- Inline named arguments (`{name}`, `{name:?}`, ...) no longer take the style of the
  next positional argument. Explicit indices (`{0}`) and `.*` precision are resolved
  the same way `format!` does, so each style follows its argument.
- An invalid `tag =>` style is now reported even when no placeholder uses it.
- README examples now compile and match the actual output; they run as doctests.

### Changed
- Every argument is passed through to `format!`, so an argument without a placeholder
  is now a compile error (as with `format!`) instead of being silently dropped.
- Removed the unused `paste` dependency.
- Added `examples/demo.rs` and a screenshot to the README.
- CI runs fmt, clippy and tests across the whole workspace; the publish workflow now
  publishes both crates in dependency order.
