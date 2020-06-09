# QEMU 研究報告

編譯器 qemu 的初始創作者是 [Fabrice Bellard](https://bellard.org/) ，以下是他的作品網頁：

* https://bellard.org/

QEMU 是一種跨平台、跨處理器的虛擬機，可以在 x86 上模擬出 ARM 處理器的行為，也可以反過來在 ARM 上模擬出 x86 的行為。

其主要採用技術是將 img 二進位碼指令轉換成目標平台的微指令組合，這些微指令會呼叫背後的 C 語言程式，然後採用 gcc 為工具快速的將這些微指令編譯執行。

對於 QEMU 的設計原理，Fabrice Bellard 曾經寫過一篇論文描述，該論文網址如下：

* https://www.usenix.org/legacy/event/usenix05/tech/freenix/full_papers/bellard/bellard.pdf

論文中描述了 QEMU 的指令翻譯原理，以下是其中的一個 x86 指令範例：

> addi r1,r1,-16 # r1 = r1 - 16

若我們用 qemu 在 powerPC 處理器上執行該指令時，會被翻譯成下列三條微指令：

```asm
movl_T0_r1 # T0 = r1
addl_T0_im -16 # T0 = T0 - 16
movl_r1_T0 # r1 = T0
```

論文中說第一條微指令會翻譯成下列 C 語言呼叫。

```c
void op_movl_T0_r1(void) {
  T0 = env->regs[1];
}
```

我猜測第二、三條微指令應該是翻譯成：

```c
void add_T0_im(int num) {
  T0 = env->regs[1];
  T0 = T0 + num;
  env->regs[1] = T0
}

void op_movl_T0_r1(void) {
  env->regs[1] = T0;
}
```

論文中說上述 C 語言程式中的 env 在 PowerPC 中應該對應到 32 個暫存器。

> env is a structure containing the target CPU state. The 32 PowerPC registers are stored in the array env->regs[32].

Fabrice Bellard 設計時刻意把微指令數量降得很低，該論文中說道：

> The number of micro operations is minimized without impacting the quality of the generated code much. For example, instead of generating every possible move between every 32 PowerPC registers, we just generate moves to and from a few temporary registers. These registers T0, T1, T2 are typically stored in host registers by using the GCC static register variable extension.

這樣就可以把暫存器的安排與優化等任務交給 gcc ，降低 qemu 的實作負擔。 

(改進想法：現在有 LLVM ，或許可以改翻成 LLVM IR 中間碼，然後由 llc 將中間碼轉為目標平台的指令，或者交由 lli 中間碼解譯器去執行也行)

但是為了效能更快， op_addl_T0_im() 函數把常數參數放到全域變數中的 op_param1，而不是作為參數傳入，

```c
extern int __op_param1;
void op_addl_T0_im(void)
{
T0 = T0 + ((long)(&__op_param1));
}
```

QEMU 在翻譯指令時，會動態的將《來源平台的機器指令》翻譯成《目標平台的微指令》，以下是其程式片段：


```
[...]
for(;;) {
  switch(*opc_ptr++) {
    [...]
    case INDEX_op_movl_T0_r1:
    {
      extern void op_movl_T0_r1();
      memcpy(gen_code_ptr, (char *)&op_movl_T0_r1+0, 3);
      gen_code_ptr += 3;
      break;
    }
    case INDEX_op_addl_T0_im:
    {
      long param1;
      extern void op_addl_T0_im();
      memcpy(gen_code_ptr, (char *)&op_addl_T0_im+0, 6);
      param1 = *opparam_ptr++;
      *(uint32_t *)(gen_code_ptr + 2) = param1;
      gen_code_ptr += 6;
      break;
    }
    [...]
  }
}
[...]

```



