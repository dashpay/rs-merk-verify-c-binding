cargo +nightly build --release &&
gcc -o ./c/build/main ./c/main.c -Isrc  -L. -l:target/release/libmerk_ios.so &&
./c/build/main