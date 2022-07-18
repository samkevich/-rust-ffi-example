# Rust ffi example

Small project to reproduce an issue compiling Rust and C together on MacOS with enabled address sanitiser.

## To compile:
```
RUSTFLAGS=-Zsanitizer=address cargo +nightly build -Z build-std --target x86_64-apple-darwin
```

## Linker error:
```
Undefined symbols for architecture x86_64:
            "___asan_version_mismatch_check_apple_clang_1316", referenced from:
                _asan.module_ctor in libfoo.a(foo.o)
          ld: symbol(s) not found for architecture x86_64
          clang: error: linker command failed with exit code 1 (use -v to see invocation)
```

## System:
* rustc 1.64.0-nightly (263edd43c 2022-07-17)
* apple clang version 13.1.6 (clang-1316.0.21.2)
