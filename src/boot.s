.option norvc
.section .boot, "ax",@progbits
.global _start
.global abort

_start:
    addi    x1, zero, 0

    /* Set up stack pointer. */
    auipc   sp, %hi(stacks + 1024)
    addi    sp, sp, %lo(stacks + 1024)

    /* Now jump to the rust world; __start_rust.  */
    auipc   t0, %hi(__start_rust)
    addi    t0, t0, %lo(__start_rust)
    jr      t0

abort:

.bss

    .global stacks
stacks:
    .skip 1024