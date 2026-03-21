# libones

Ones' complement arithmetic implementations across four languages: Rust, Go, TypeScript, and C/C++. All implementations provide the same core operations — complement, add with end-around carry, and subtract — and agree on the same semantics.

License: MPL-2.0

---

## What this is

Ones' complement is a binary integer representation where negative numbers are the bitwise complement of their positive counterparts, rather than the two's-complement negation. It has two representations of zero (positive zero: all bits 0; negative zero: all bits 1), and addition requires an end-around carry: when a sum overflows the bit width, the carry bit is added back into the result.

This library does not emulate a historical machine. It provides the arithmetic operations needed when you must work with ones' complement bit patterns — for instance, certain network checksum algorithms (RFC 1071) use ones' complement addition.

---

## Repository layout

```
crates/ones/       Rust crate (workspace member)
ones/              Go package
packages/ones/     TypeScript package (@portal-solutions/libones)
include/ones/      C/C++ header-only library
```

---

## Implementations

### Rust (`crates/ones/`)

Crate name: `ones`, version `0.1.0`.

The central type is `OnesSigned<T>`, a `#[repr(transparent)]` newtype over an unsigned integer that stores a ones'-complement bit pattern. `T` must implement `OnesOne`, a marker trait sealed to `u8`, `u16`, `u32`, `u64`, `u128`, and `usize`.

- `Add` applies wrapping addition and then, if the result wrapped (detected by `result < left_operand`), adds 1 as the end-around carry.
- `Sub` is implemented as `a + !b` — addition of the bitwise complement — which is the standard ones'-complement subtraction identity.
- Positive zero is `T::default()` (0); negative zero is `!T::default()` (all bits set).

Optional feature `num_traits`: replaces the sealed `OnesOne` trait with a blanket implementation over `num_traits::Unsigned + WrappingAdd + WrappingSub + Not`, allowing the library to work with custom numeric types that satisfy those bounds.

The crate is `#![no_std]`.

### Go (`ones/`)

Module path: `github.com/portal-co/libones`.

Two types:

**`OnesSigned[T Signed]`** — generic wrapper over `int`, `int8`, `int16`, `int32`, or `int64`. It stores the value using the host language's signed representation rather than an unsigned bit pattern. Its `Add` and `Sub` correct for end-around carry/borrow by checking the sign of the raw result. Note that the arithmetic details differ slightly from the Rust implementation (which uses unsigned storage); the Go version works directly with two's-complement signed integers and applies a sign-based correction.

**`Bitwidth`** — a fixed-bit-width context operating on `uint64` values. Provides `Complement`, `Add` (with end-around carry), and `Subtract`. The bit width is set at construction and determines the modulus (`1 << bits`).

### TypeScript (`packages/ones/`)

Package name: `@portal-solutions/libones`, built with `zshy` from `index.ts`. Distributed as both ESM (`dist/index.js`) and CJS (`dist/index.cjs`).

Exports a single class `Bitwidth` with:
- `add(a, b)` / `complement(a)` / `subtract(a, b)` — operate on `number` (safe for up to 32-bit widths due to JavaScript bitwise operator limitations).
- `addBigint(a, b)` / `complementBigint(a)` / `subtractBigint(a, b)` — same operations on `bigint`, suitable for bit widths above 32.

The class is frozen (`Object.freeze`) after construction.

Build: `sh build.sh` from the repo root (delegates to `packages/ones/build.sh`, which runs `npx zshy`).

### C/C++ (`include/ones/ones.h`)

Header-only, no compilation step.

For C: a `ones_bitwidth_t` struct with inline functions `ones_complement`, `ones_add`, and `ones_subtract`, plus `_Generic`-based macros `ONES_COMPLEMENT`, `ONES_ADD`, and `ONES_SUBTRACT` that dispatch on operand type without requiring the caller to pass a bit-width context. Fixed-width convenience macros are also provided: `ONES_ADD_8`, `ONES_ADD_16`, `ONES_ADD_32`, `ONES_ADD_64`, and their complements/subtracts.

For C++: a `ones` namespace with function templates `complement<T>`, `add<T>`, and `subtract<T>` that work on any unsigned integer type using wrapping arithmetic. The same macro names (`ONES_ADD`, etc.) resolve to the templated versions when compiled as C++.

---

## Current state

- All four implementations are present with tests.
- The `crates/` and `packages/` directories are referenced in `Cargo.toml` and `package.json` workspaces respectively, but the `ones/` (Go) and `include/` (C/C++) components are standalone — no build system integration beyond the header itself.
- The `ones/` Go directory and `include/ones/` directory each contain a `_` placeholder file, suggesting these directories were set up but may not be fully integrated into a broader build.
- There is no published crate or npm package version tracked here; version is `0.1.0` across all package manifests.

---

## Arithmetic semantics

All implementations agree on the following:

- `complement(a)` = `(2^N - 1) - a` for an N-bit context, or `~a` for full-width types.
- `add(a, b)` = `(a + b) & mask` + carry, where carry is 1 if `a + b >= 2^N`, 0 otherwise. Equivalent to end-around carry.
- `subtract(a, b)` = `add(a, complement(b))`.
- `a - a` yields negative zero (all bits set), not positive zero.
