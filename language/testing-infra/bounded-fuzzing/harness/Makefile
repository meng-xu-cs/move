all: build

clean:
	cargo clean --profile fuzz
	rm -rf main.o main.bc main fuzz.o fuzz.bc fuzz

build:
	RUSTFLAGS="--emit=llvm-bc" \
		cargo +nightly rustc \
			--profile fuzz \
			-Zbuild-std \
			--target x86_64-unknown-linux-gnu
	rm \
		../../../../target/x86_64-unknown-linux-gnu/fuzz/deps/panic_abort-*.bc
	clang-15 \
		-target x86_64-unknown-linux-gnu \
		-c -emit-llvm \
		-o main.bc \
		main.c
	llvm-link-15 \
		../../../../target/x86_64-unknown-linux-gnu/fuzz/deps/*.bc \
		main.bc \
		-o fuzz.bc
