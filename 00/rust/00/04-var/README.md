

## var

```
user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/sp/rust/04-var (master)
$ rustc var1.rs

user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/sp/rust/04-var (master)
$ ./var1
The value of x is: 5
The value of x is: 6

user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/sp/rust/04-var (master)
$ rustc var2shadow.rs

user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/sp/rust/04-var (master)
$ ./var2shadow.exe 
The value of x is: 12

```

## type

Length | Signed | Unsigned
-------|-------|------------
8-bit	 | i8	   | u8
16-bit	 | i16	 | u16
32-bit	 | i32	 | u32
64-bit	 | i64	 | u64
128-bit	 | i128	 | u128
arch	 | isize	 | usize

Number literals	| Example
--------------|----------
Decimal    	| 98_222
Hex    	| 	0xff
Octal    	| 	0o77
Binary    	| 	0b1111_0000
Byte (u8 only)    	| 	b'A'