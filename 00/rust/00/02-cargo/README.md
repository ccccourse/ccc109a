# Cargo -- Hello

* 來源 -- https://doc.rust-lang.org/book/ch01-03-hello-cargo.html

```
user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/sp/rust/02-cargo (master)
$ cargo new hello_cargo
     Created binary (application) `hello_cargo` package

user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/sp/rust/02-cargo (master)
$ cd hello_cargo

user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/sp/rust/02-cargo/hello_cargo (master)
$ cargo build
   Compiling hello_cargo v0.1.0 (D:\ccc\ccc109a\sp\rust\02-cargo\hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 3.66s                                                 

user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/sp/rust/02-cargo/hello_cargo (master)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running `target\debug\hello_cargo.exe`
Hello, world!

user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/sp/rust/02-cargo/hello_cargo (master)
$ cargo check
    Checking hello_cargo v0.1.0 (D:\ccc\ccc109a\sp\rust\02-cargo\hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.71s

```
