# Rikan

This project is an implementation of MikanOS in Rust.

## Build
This project depends on [mikanos-build](https://github.com/uchan-nos/mikanos-build).

```console
$ git clone git@github.com:bean1310/Rikan.git
$ git clone git@github.com:uchan-nos/mikanos-build.git
$ cd Rikan
$ echo "MIKANOS_BUILD_PATH=$(dirname $PWD)/mikanos-build" > .env
$ make
```

## Run

```console
$ make run
```
