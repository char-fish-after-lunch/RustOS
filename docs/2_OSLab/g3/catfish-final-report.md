# RustOS多核移植与基于PARD框架的线程级Label管理 结课报告

2015011251 王纪霆

## 实验目标

以下是原定的实验目标：

- 完成RustOS在riscv32上的多核开启（完成）
- 使RustOS可在中科院计算所PARD RISCV硬件环境下运行（完成）
- 使RustOS能够在PARD上开启smp（完成）
- 添加控制功能，使得RustOS可以控制/查看PARD的寄存器（未完成）

## 工作内容

前八周完成的工作可见[此处](https://github.com/char-fish-after-lunch/RustOS/blob/rv32-smp-porting/docs/2_OSLab/g3/catfish-final.md)，以下只说明后八周进行的工作。

### PARD通讯

在第八周之后，调出了串口通讯的bug，可以在PARD上正常运行RustOS了。此外，在RustOS中添加了一个串口模块、在prm上添加了监听串口的脚本，这样一来，理论上RustOS中内核态可以调用接口访问PARD控制寄存器。但是实际上写入、读出寄存器并未成功，之后才发现是因为硬件实现已与[旧软件实现](https://github.com/maoyuchaxue/prm-sw/blob/master/apps/pardctl/pardctl.c)不相兼容。由于时间不够，到项目结束时最后也并没有修正过来。

目前，非线程级的标签管理中，控制系统（外部的pc端）是通过网络向prm的OpenOCD发送请求的。最简单的办法大概是将fpga中的串口作为OpenOCD的输入，但这样仅一次寄存器IO就将耗去大量时间，很不优雅。最好的办法还是用更轻量的、手写的“服务器”来响应请求。

为此，结合[riscv debug specification](https://github.com/riscv/riscv-debug-spec/blob/master/riscv-debug-release.pdf)，以及[可参照的jtag/riscv debug协议简单实现](https://github.com/maoyuchaxue/prm-sw/tree/master/common)，还有[修改后的debug模块硬件源码](https://github.com/char-fish-after-lunch/labeled-RISC-V/blob/master/src/main/scala/devices/debug/dm_registers.scala)，应该就可以并无太大困难地完成了。

### 硬件问题

中途遇到的另一个问题就是其实硬件并没有把所承诺的功能完全实现。虽然[文档](https://github.com/maoyuchaxue/prm-sw/blob/master/apps/pardctl/README.md)中声明了很多寄存器，但前12周我使用的版本实际上只支持最基础的几个（membase, memmask, bucket size, bucket frequency），并且可读写性、访问方式也与该文档不一致。在稳定下来、形成稳定文档之前，目前可能只能随时看[硬件源码](https://github.com/char-fish-after-lunch/labeled-RISC-V/blob/master/src/main/scala/devices/debug/Debug.scala)来判断现在的文档中有多少是准确的了。

在约13周时PARD又放出了一个新版本，增加了一些Cache相关寄存器读写的支持。但将这一版本合并之后，不仅没有完成功能开发，反而产生了更大的问题，RustOS难以稳定启动，有时可以正常工作，有时甚至在bbl中就会卡死。最终未能解决这个问题。

另外，即使现在的最新版本依旧存在一些问题，例如token bucket/traffic都还没有做到支持线程级别切换，而是每个核绑定一个寄存器，这使得即使完成了RustOS内对控制寄存器的修改，要做到随线程切换而切换控制寄存器还是一件不怎么优雅的事。

## 实验总结

总而言之，本次项目并不成功，由于硬件调试水平、交流积极性不够，再加上最后时间不足，没有能够交出一个令人满意的结果。现在反思下来，自己水平不足确实是一个问题，但另一个问题也在于选题可能稍嫌冒进。本学期中遇到的很多问题其实是相当无谓的，如果是同样的项目改到明年完成，RustOS完成RV64移植之后，就不再有需要更改硬件字长的问题，可以省去前几周遇到的一些困难；而PARD完成度更高之后，也会省去软硬件不兼容、功能未实现等麻烦。在两方都很不成熟的情况下对接，可能并不是最好的选择。
