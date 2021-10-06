THIS_SCRIPT_PATH=$(realpath "$0")
THIS_SCRIPT_DIRECTORY=$(dirname "$THIS_SCRIPT_PATH")
PROJECT_ROOT=$(dirname "$THIS_SCRIPT_DIRECTORY")

# shellcheck disable=SC2164
cd "${PROJECT_ROOT}"
cargo +nightly build --release
# shellcheck disable=SC2164
cd "${THIS_SCRIPT_DIRECTORY}"
cmake . && cmake --build . --
./bin/main