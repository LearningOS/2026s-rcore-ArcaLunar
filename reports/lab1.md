# Chapter 3 练习

## 功能实现

在 `task/mod.rs` 里添加 `num_called[][]` 并添加 API，利用 `inner.current_task` 实现当前任务系统调用次数统计，并暴露 `increase_counter, get_counter` API 供 `syscall/mod.rs:syscall` 和 `syscall/process.rs:sys_trace` 进行调用．

除此之外，`sys_trace` 利用 unsafe 类型转换实现了读取与写入数据．

## 简答作业

1. 会产生 PageFault/IllegalInstruction 从而被 kernel kill

```text
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.

版本信息：
[rustsbi] RustSBI version 0.3.0-alpha.2, adapting to RISC-V SBI v1.0.0
[rustsbi] Implementation     : RustSBI-QEMU Version 0.2.0-alpha.2
[rustsbi] Platform Name      : riscv-virtio,qemu
[rustsbi] Platform SMP       : 1
[rustsbi] Platform Memory    : 0x80000000..0x88000000
[rustsbi] Boot HART          : 0
[rustsbi] Device Tree Region : 0x87000000..0x87000ef2
[rustsbi] Firmware Address   : 0x80000000
[rustsbi] Supervisor Address : 0x80200000
[rustsbi] pmp01: 0x00000000..0x80000000 (-wr)
[rustsbi] pmp02: 0x80000000..0x80200000 (---)
[rustsbi] pmp03: 0x80200000..0x88000000 (xwr)
[rustsbi] pmp04: 0x88000000..0x00000000 (-wr)
```

2. `__restore()`
  1. `sp` 代表的是内核栈上某个 TrapContext 的栈顶．`__restore` 既可以用于任务切换（切换 Task A/B 的上下文），也可以用于 Trap 切换（切换用户栈/内核栈的上下文）

  2. 特殊处理了 `sstatus, sepc, sscratch`.

      `sstatus` 记录 trap 发生前的特权级 (`SPP`)，方便内核态退回用户态时直接设置特权级。
      `sepc` 记录返回用户态后 `PC` 应该跳转到的地址。
      `sscratch` 记录用户栈，方便内核态退出后，通过与 `sp` 交换直接切回用户栈

  3. `x2` 是 `sp`，而 `sp` 不能用普通寄存器的方式恢复。因为当前 `sp` 还需要指向内核栈 TrapContext 来进行释放栈（`addi sp,sp,34*8`），并且把内核栈、用户栈交换
     `x4` 则不能被 application 使用，不能使用那么也就没必要保存

  4. `sp` 此时指向任务的用户栈，继续执行后面的指令。`sscratch` 指向任务的内核栈，为下一次 Trap Switch 做准备

  5. 发生在 `sret`，是 RISC-V 从 supervisor mode 退回 user mode 的指令

  6. 这一步之后，`sp` 指向任务的内核栈，便于内核分配 TrapContext、保存寄存器、执行指令。`sscratch` 指向任务的用户栈，便于保存 TrapContext．

  7. 由硬件自动完成，通常是在 `ecall` 的时候

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

> N/A

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

> N/A

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

