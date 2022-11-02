all: build

clean:
	cargo clean --profile fuzz
	rm -rf main.o main.bc main fuzz.o fuzz.bc fuzz

build:
	RUSTFLAGS="-Clinker-plugin-lto -Clinker=clang-15 -Clink-arg=-fuse-ld=lld-15 -Zemit-thin-lto=no" \
		cargo +nightly rustc \
			--profile fuzz \
			-Zbuild-std --target x86_64-unknown-linux-gnu
	clang-15 \
		-target x86_64-unknown-linux-gnu \
		-c -O2 -flto \
		-o main.o \
		main.c
	clang-15 \
		-O2 -flto -fuse-ld=lld \
		-L ../../../../target/x86_64-unknown-linux-gnu/fuzz -lbounded_fuzzing \
		-Wl,--plugin-opt=-lto-embed-bitcode=optimized \
		-Wl,-as-needed \
		-Wl,-Bdynamic -lgcc_s -lrt -lc -lpthread -ldl \
		-Wl,--eh-frame-hdr \
		-Wl,--gc-sections \
		-Wl,-znoexecstack \
		-Wl,-zrelro \
		-Wl,-znow \
		-nodefaultlibs \
		-no-pie \
		-o main \
		main.o
	llvm-objcopy-15 \
		main \
		--dump-section .llvmbc=main.bc
	opt-15 \
		-o fuzz.bc \
		main.bc
	llc-15 \
		-filetype=obj \
		-o fuzz.o \
		fuzz.bc
	clang-15 \
		-Wl,-as-needed \
		-Wl,-Bdynamic -lgcc_s -lrt -lc -lpthread -ldl \
		-Wl,--eh-frame-hdr \
		-Wl,--gc-sections \
		-Wl,-znoexecstack \
		-Wl,-zrelro \
		-Wl,-znow \
		-nodefaultlibs \
		-no-pie \
		-o fuzz \
		fuzz.o