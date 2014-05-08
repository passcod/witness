Witness
=======

Implements a distributed non-timestamped witnessing service.

Build
-----

### External Dependencies

- [Rust](http://rust-lang.org/)
- [Rust-Bindgen](https://github.com/crabtw/rust-bindgen)
- CMake

### Building

```bash
$ git clone git://github.com/passcod/witness
$ cd witness
$ git submodule init
$ git submodule update
$ mkdir build && cd build
$ cmake ..
$ make
$ make test
$ bin/server
```
