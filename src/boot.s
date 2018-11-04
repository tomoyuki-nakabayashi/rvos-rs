.option norvc
.section .boot, "ax",@progbits
.global _start
.global abort

_start:
    addi    x1, zero, 0

    /* Set up stack pointer. */
    addi    sp, zero, 0
    lui     sp, %hi(stacks + 1024)
    ori     sp, sp, %lo(stacks + 1024)

    /* Now jump to the rust world; __start_rust.  */
    auipc   t0, %hi(__start_rust)
    addi    t0, t0, %lo(__start_rust)
    j       __start_rust

abort:

.bss

    .global stacks
stacks:
    .skip 1024