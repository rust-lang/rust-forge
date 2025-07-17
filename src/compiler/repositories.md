# Repositories maintained by the Compiler Team

The obvious focus of the Compiler Team is the [rustc] compiler itself, but the team also maintains a few other repositories:

- An experimental [C codegen backend][c-backend] for rustc, which lowers Rust MIR to C code and compiles it with a C compiler. Currently maintained by X and Y.
- A [Cranelift][cranelift-backend] experimental rustc backend, based on [Cranelift]. This has the potential to improve compilation times in debug mode. Currently maintained by X and Y.
- A Rust port of the [LLVM::APFloat][rustc_apfloat] library. Maintained by X abnd Y.
- A fork of [Enzyme], a high-performance automatic differentiator of LLVM and MLIR (more info at this [link][enzyme-mit]). The fork is currently maintained by X and Y.
- [Thorin], a DWARF packaging utility supporting GNU extension and DWARF 5 package formats. Maintained by X and Y.
- The [Rust Forge][forge], the documentation website you're reading. Maintained by the Rust project.

In addition, the Compiler team routinely contributes to other internal projects, [here the full list][other].

If you want to start (or are already) contributing to the Rust project and you have expertise or interest in any of these repositories, feel free to get in touch!

[rustc]: https://github.com/rust-lang/rust
[c-backend]: https://github.com/rust-lang/rustc_codegen_c
[cranelift-backend]: https://github.com/rust-lang/rustc_codegen_cranelift
[cranelift]: https://github.com/bytecodealliance/wasmtime/blob/main/cranelift
[rustc_apfloat]: https://github.com/rust-lang/rustc_apfloat
[enzyme]: https://github.com/rust-lang/enzyme
[enzyme-mit]: https://enzyme.mit.edu/
[thorin]: https://github.com/rust-lang/thorin
[forge]: https://github.com/rust-lang/rust-forge
[other]: https://github.com/orgs/rust-lang/teams/compiler/repositories
