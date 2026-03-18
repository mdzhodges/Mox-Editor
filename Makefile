# Root Makefile for Mox-Editor

RUST_LIB = engine/target/release/libmox_engine.a
OCAML_EXE = _build/default/parser/bin/main.exe

all: $(OCAML_EXE)

# 1. Build the Rust Static Library
$(RUST_LIB): engine/src/lib.rs engine/src/buffer.rs
	@echo "Building Rust Engine..."
	@cd engine && cargo build --release

# 2. Build the OCaml Parser (Links against the Rust .a)
$(OCAML_EXE): $(RUST_LIB) parser/bin/main.ml
	@echo "Building OCaml Parser..."
	@dune build

# Run the editor
run: all
	@dune exec parser/bin/mox.exe

clean:
	@cd engine && cargo clean
	@dune clean

.PHONY: all clean run