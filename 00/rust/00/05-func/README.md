# rust 函數

```


user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/sp/rust/05-func (master)
$ rustc func1.rs

user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/sp/rust/05-func (master)
$ ./func1
Hello, world!
Another function.

user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/sp/rust/05-func (master)
$ rustc func2param.rs

user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/sp/rust/05-func (master)
$ ./func2param
The value of x is: 5

user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/sp/rust/05-func (master)
$ rustc func3param2.rs

user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/sp/rust/05-func (master)
$ ./func3param2
The value of x is: 5
The value of y is: 6

user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/sp/rust/05-func (master)
$ rustc func4block.rs
warning: unused variable: `x`
 --> func4block.rs:2:7
  |
2 |   let x = 5;
  |       ^ help: consider prefixing with an underscore: `_x`
  |
  = note: `#[warn(unused_variables)]` on by default


user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/sp/rust/05-func (master)
$ ./func4block
The value of y is: 4

user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/sp/rust/05-func (master)
$ rustc func5ret.rs

user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/sp/rust/05-func (master)
$ ./func5ret
The value of x is: 5

user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/sp/rust/05-func (master)
$ rustc func6ret.rs

user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/sp/rust/05-func (master)
$ ./func6ret
The value of x is: 6
```
