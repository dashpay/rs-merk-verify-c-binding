cargo +nightly build --release &&
gcc ./c/main.c -o ./c/build/main -Itarget -Ltarget/release -lmerk_ios &&
./c/build/main