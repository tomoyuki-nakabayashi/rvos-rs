.equ DRAM_BASE, 0x80000000

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
    lui     t0, %hi(DRAM_BASE)
    ori     t0, t0, %lo(DRAM_BASE)
    jr      t0

abort:

.bss

    .global stacks
stacks:
    .skip 1024
