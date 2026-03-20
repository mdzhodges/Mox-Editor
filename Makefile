# Use the actual name from your tree output
RUST_LIB = engine/target/release/libengine.a
OCAML_EXE = _build/default/parser/bin/main.exe

all: $(OCAML_EXE)

$(RUST_LIB): engine/src/lib.rs engine/src/buffer.rs
	@echo "Building Rust Engine..."
	@cd engine && cargo build --release

$(OCAML_EXE): $(RUST_LIB) parser/bin/main.ml
	@echo "Building OCaml Parser..."
	@dune build

run: all
	@./$(OCAML_EXE)

clean:
	@cd engine && cargo clean
	@dune clean

.PHONY: all clean run