# gcc 

## 安裝

* MSYS2 -- https://www.msys2.org/
* 或者裝 DevC++, CodeBlocks 之後設好 path 到 gcc 資料夾裏
* 在 linux 裏預設就有
* mac 中要先安裝 Command Line Tools for Xcode

## 基本用法

```
gcc hello.c -o hello
```

## 進階用法

```
gcc main.c sum.c -o sum  // 多檔案一起編譯

gcc -S sum.c -o sum.s // 產生組合語言

gcc --fverbose-asm -S sum.c -o sum.s // 產生有註解的組合語言

gcc -c sum.c -o sum.o // 只編譯不連結

gcc main.c sum.s -o sum // 組合語言和 C 語言混合編譯

gcc -E sum.c -o sum.i // 巨集展開

```

## 進階

* make + Makefile 專案建置
* objdump 觀看目的檔
* ar 製作函式庫
* strings 看字串表
* nm 看符號表
* gdb 除錯

