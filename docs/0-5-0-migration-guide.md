# Migration Guide for Breaking Changes in v0.5.0

This document gives an overview of the breaking API changes in v0.5.0 and how
to update your code accordingly. Mostly, follow the error messages the compiler
will give you after updating to the new RustSAT version.

This fix contains some changes that technically would call for a major version
bump, but at the given stage of this project, we do not want to release version
1.0.0 yet, we publish this as a new minor version with breaking changes
instead.

## Error Handling

Error handling in the `Solve` trait, and file parsers now uses the
[`anyhow`](https://docs.rs/anyhow/latest/anyhow/) crate. This allows for better
error messages, and better tracing. In the process, some of the error types or
variants that are not needed any more have been removed:

- `rustsat::solvers::SolverError` has been removed and only
  `rustsat::solvers::StateError` remains
- `rustsat::instances::fio::opb::Error` has been removed
- `rustsat::instances::fio::dimacs::Error` has been removed
- `rustsat::instances::fio::ParsingError` has been removed
- `rustsat::solvers::SolverState::Error` has also been removed as no error
  state is needed with proper error returns

If you need to handle a specific error, you can use `anyhow`'s
[`downcast`](https://docs.rs/anyhow/latest/anyhow/struct.Error.html#method.downcast)
(e.g., on `solvers::StateError`), but I imagine most often these errors are
anyhow just propagated outwards and displayed.

## Changes to Improve API Ergonomics

There have been some API changes to improve usability, even though they are breaking.

- `to_dimacs` (and variants) on the `SatInstance`, `OptInstance`, and
  `MultiOptInstance` types now take a mutable reference rather than a value. This
  way the instance does not have to cloned when writing. For notes on mutability,
  see the documentation.
- `to_opb` (and variants) on the `SatInstance` type now takes by reference
- `to_opb` (and variants) on the `OptInstance` and `MultiOptInstance` types now
  take by mutable reference. For notes on mutability, see the documentation.
