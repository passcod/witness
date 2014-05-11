cmake_minimum_required (VERSION 2.6)
project (witness)

add_library(mongoose ${CMAKE_SOURCE_DIR}/vendor/mongoose/mongoose.c)
add_custom_target(binding
  COMMAND bindgen -builtins -l mongoose -o mongoose/binds -I /usr/lib/gcc/x86_64-unknown-linux-gnu/4.9.0/include/ ${CMAKE_SOURCE_DIR}/vendor/mongoose/mongoose.h
  COMMAND cat mongoose/header mongoose/binds > mongoose/lib.rs
  COMMAND rustc mongoose/lib.rs
  COMMAND mkdir -p ${CMAKE_BINARY_DIR}/lib && mv libmongoose* ${CMAKE_BINARY_DIR}/lib/
  WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}/src"
  DEPENDS mongoose)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/vendor/RustCMake/cmake")
find_package(rustc)
find_package(rustdoc)
include(Rust)

set(RUSTC_FLAGS -L ${CMAKE_BINARY_DIR}/lib)
set(RUSTDOC_FLAGS -L ${CMAKE_BINARY_DIR}/lib)

get_rust_deps(src/server.rs WITNESS_DEPS)

rust_crate(src/server.rs
          TARGET_NAME WITNESS
          DESTINATION bin
          DEPENDS "${WITNESS_DEPS}" binding)

rust_crate(src/server.rs
          TARGET_NAME WITNESS_TEST
          DESTINATION test
          DEPENDS "${WITNESS_DEPS}" binding
          OTHER_RUSTC_FLAGS --test)

add_custom_target(default
                  ALL
                  DEPENDS ${WITNESS_FULL_TARGET})

add_custom_target(test
                  COMMAND ./server || true
                  WORKING_DIRECTORY "${CMAKE_BINARY_DIR}/test"
                  DEPENDS ${WITNESS_TEST_FULL_TARGET} ${WITNESS_FULL_TARGET})
