# QEMU 安裝

* https://www.qemu.org/download/#windows

安裝後位置:
1. windows : C:\Program Files\qemu

## 載入系統 -- 以 xv6 為例

```
PS D:\ccc\course\sp\code\c\12-os3mini\xv6\img> qemu-system-i386 -nographic -drive file=fs.img,index=1,media=disk,format=raw -drive file=xv6.img,index=0,media=disk,format=raw -smp 2 -m 512
```

然後就出現了

```
SeaBIOS (version rel-1.12.0-59-gc9ba5276e321-prebuilt.qemu.org)


iPXE (http://ipxe.org) 00:03.0 CA00 PCI2.10 PnP PMM+1FF908D0+1FEF08D0 CA00



Booting from Hard Disk...
xv6...
cpu1: starting 1
cpu0: starting 0
sb: size 1000 nblocks 941 ninodes 200 nlog 30 logstart 2 inodestart 32 bmap start 58
init: starting sh
$



  ls
.              1 1 512
..             1 1 512
README         2 2 2170
cat            2 3 13628
echo           2 4 12640
forktest       2 5 8068
grep           2 6 15504
init           2 7 13220
kill           2 8 12692
ln             2 9 12588
ls             2 10 14776
mkdir          2 11 12772
rm             2 12 12748
sh             2 13 23236
stressfs       2 14 13420
usertests      2 15 56352
wc             2 16 14168
zombie         2 17 12412
console        3 18 0
$ mkdir ccc
$ ls
.              1 1 512
..             1 1 512
README         2 2 2170
cat            2 3 13628
echo           2 4 12640
forktest       2 5 8068
grep           2 6 15504
init           2 7 13220
kill           2 8 12692
ln             2 9 12588
ls             2 10 14776
mkdir          2 11 12772
rm             2 12 12748
sh             2 13 23236
stressfs       2 14 13420
usertests      2 15 56352
wc             2 16 14168
zombie         2 17 12412
console        3 18 0
ccc            1 19 32
$ cat README
xv6 is a re-implementation of Dennis Ritchie's and Ken Thompson's Unix
Version 6 (v6).  xv6 loosely follows the structure and style of v6,
but is implemented for a modern x86-based multiprocessor using ANSI C.

ACKNOWLEDGMENTS

xv6 is inspired by John Lions's Commentary on UNIX 6th Edition (Peer
to Peer Communications; ISBN: 1-57398-013-7; 1st edition (June 14,
2000)). See also https://pdos.csail.mit.edu/6.828/, which
provides pointers to on-line resources for v6.

xv6 borrows code from the following sources:
    JOS (asm.h, elf.h, mmu.h, bootasm.S, ide.c, console.c, and others)
    Plan 9 (entryother.S, mp.h, mp.c, lapic.c)
    FreeBSD (ioapic.c)
    NetBSD (console.c)

The following people have made contributions: Russ Cox (context switching,
locking), Cliff Frey (MP), Xiao Yu (MP), Nickolai Zeldovich, and Austin
Clements.

We are also grateful for the bug reports and patches contributed by Silas
Boyd-Wickizer, Anton Burtsev, Cody Cutler, Mike CAT, Tej Chajed, eyalz800,
Nelson Elhage, Saar Ettinger, Alice Ferrazzi, Nathaniel Filardo, Peter
Froehlich, Yakir Goaron,Shivam Handa, Bryan Henry, Jim Huang, Alexander
Kapshuk, Anders Kaseorg, kehao95, Wolfgang Keller, Eddie Kohler, Austin
Liew, Imbar Marinescu, Yandong Mao, Matan Shabtay, Hitoshi Mitake, Carmi
Merimovich, Mark Morrissey, mtasm, Joel Nider, Greg Price, Ayan Shafqat,
Eldar Sehayek, Yongming Shen, Cam Tenny, tyfkda, Rafael Ubal, Warren
Toomey, Stephen Tu, Pablo Ventura, Xi Wang, Keiichi Watanabe, Nicolas
Wolovick, wxdao, Grant Wu, Jindong Zhang, Icenowy Zheng, and Zou Chang Wei.

The code in the files that constitute xv6 is
Copyright 2006-2018 Frans Kaashoek, Robert Morris, and Russ Cox.

ERROR REPORTS

We switched our focus to xv6 on RISC-V; see the mit-pdos/xv6-riscv.git
repository on github.com.

BUILDING AND RUNNING XV6

To build xv6 on an x86 ELF machine (like Linux or FreeBSD), run
"make". On non-x86 or non-ELF machines (like OS X, even on x86), you
will need to install a cross-compiler gcc suite capable of producing
x86 ELF binaries (see https://pdos.csail.mit.edu/6.828/).
Then run "make TOOLPREFIX=i386-jos-elf-". Now install the QEMU PC
simulator and run "make qemu".$ ls
.              1 1 512
..             1 1 512
README         2 2 2170
cat            2 3 13628
echo           2 4 12640
forktest       2 5 8068
grep           2 6 15504
init           2 7 13220
kill           2 8 12692
ln             2 9 12588
ls             2 10 14776
mkdir          2 11 12772
rm             2 12 12748
sh             2 13 23236
stressfs       2 14 13420
usertests      2 15 56352
wc             2 16 14168
zombie         2 17 12412
console        3 18 0
ccc            1 19 32
```

## 載入系統 -- 以 MINIX 為例

```
C:\Program Files\qemu\qemu-img.exe: kilobytes, megabytes, gigabytes, terabytes, petabytes and exabytes.
PS D:\install\minix> qemu-img create -f qcow2 minix.img 1G
Formatting 'minix.img', fmt=qcow2 size=1073741824 cluster_size=65536 lazy_refcounts=off refcount_bits=16

PS D:\install\minix> qemu-system-x86_64 -m 1024 minix.img -cdrom .\minix_R3.3.0-588a35b.iso
```

然後就啟動完畢了 (需要不少時間，請耐心等候) !

## 參考文獻


* [QEMU-IMG入门教程](https://blog.gavinzh.com/2017/08/02/qemu-img-tutorial-commands/)
* [使用qemu安装虚拟机](https://blog.csdn.net/RichardYSteven/article/details/54645328) (讚!)
* [QEMU (正體中文)](https://wiki.archlinux.org/index.php/QEMU_(%E6%AD%A3%E9%AB%94%E4%B8%AD%E6%96%87))
* [QEMU 1: 使用QEMU创建虚拟机](https://my.oschina.net/kelvinxupt/blog/265108)
* [一文读懂 Qemu 模拟器](https://www.jianshu.com/p/db8c20aa6a69)
* [五分鐘開始玩 qemu-kvm 虛擬機](https://newtoypia.blogspot.com/2015/02/qemu-kvm.html)

* [QEMU Emulator User Documentation](http://people.redhat.com/pbonzini/qemu-test-doc/_build/html/index.html)
* https://www.qemu.org/docs/

* https://askubuntu.com/questions/138140/how-do-i-install-qemu
