
* 來源 -- https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

注意: cargo.toml 裏必須加入 

```
[dependencies]
rand = "0.5.5"
```

然後才能正確建置

```

user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/sp/rust/03-guessing/guessing_game (master)
$ cargo run
   Compiling guessing_game v0.6.0 (D:\ccc\ccc109a\sp\rust\03-guessing\guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.61s
     Running `target\debug\guessing_game.exe`
Guess the number!
Please input your guess.
38
You guessed: 38
Too small!
Please input your guess.
50
You guessed: 50
Too small!
Please input your guess.
70
You guessed: 70
Too small!
Please input your guess.
80
You guessed: 80
Too small!
Please input your guess.
90
You guessed: 90
Too big!
Please input your guess.
85
You guessed: 85
Too small!
Please input your guess.
87
You guessed: 87
Too small!
Please input your guess.
88
You guessed: 88
Too small!
Please input your guess.
89
You guessed: 89
You win!
```
