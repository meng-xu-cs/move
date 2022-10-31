all: build

clean:
	cargo clean --profile fuzz
	rm -rf main.o main

build:
	RUSTFLAGS="-Clinker-plugin-lto -Clinker=clang -Clink-arg=-fuse-ld=lld" \
		cargo rustc --profile fuzz
	clang-14 -c -O2 -flto=thin -o main.o main.c
	clang-14 -flto=thin -fuse-ld=lld \
		-L ../../../../target/fuzz -l"bounded_fuzzing" \
		-o main -O2 main.o