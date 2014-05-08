cmake_minimum_required (VERSION 2.6)
project (witness)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/vendor/RustCMake/cmake")
find_package(rustc)
find_package(rustdoc)
include(Rust)

set(RUSTC_FLAGS -L ${CMAKE_BINARY_DIR}/lib)
set(RUSTDOC_FLAGS -L ${CMAKE_BINARY_DIR}/lib)

get_rust_deps(src/hello.rs WITNESS_DEPS)

rust_crate(src/hello.rs
          TARGET_NAME WITNESS
          DESTINATION bin
          DEPENDS "${WITNESS_DEPS}")

rust_crate(src/hello.rs
          TARGET_NAME WITNESS_TEST
          DESTINATION test
          DEPENDS "${WITNESS_DEPS}"
          OTHER_RUSTC_FLAGS --test)

add_custom_target(default
                  ALL
                  DEPENDS ${WITNESS_FULL_TARGET})

add_custom_target(test
                  COMMAND ./hello || true
                  WORKING_DIRECTORY "${CMAKE_BINARY_DIR}/test"
                  DEPENDS ${WITNESS_TEST_FULL_TARGET} ${WITNESS_FULL_TARGET})
