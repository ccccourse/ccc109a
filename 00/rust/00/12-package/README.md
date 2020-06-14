

* https://kaisery.github.io/trpl-zh-cn/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html

## 記得加 pub


```rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

## 引用

```rs
use std::cmp::Ordering;
use std::io;
use std::io::Write;
use std::collections::*;
```

## 嵌套路徑


```rs
use std::{cmp::Ordering, io};
use std::io::{self, Write};
```