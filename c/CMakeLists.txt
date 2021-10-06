CMAKE_MINIMUM_REQUIRED(VERSION 3.12.0 FATAL_ERROR)
set(CMAKE_CXX_STANDARD 11)
project(libmerk_ffi_test)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# ${CMAKE_CURRENT_SOURCE_DIR}/../target is the current location of merk.h file
include_directories(
        ${INCLUDE_DIRECTORIES}
        ${CMAKE_CURRENT_SOURCE_DIR}/../target
)

# Tell linker to look for "libmerk_ios" in cargo build dir
link_directories(${CMAKE_SOURCE_DIR}/../target/release)

add_executable(main ${CMAKE_CURRENT_SOURCE_DIR}/main.c)
# Link cargo build to executable
target_link_libraries(main merk_ios)