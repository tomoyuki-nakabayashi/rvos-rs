.option norvc
.section .boot, "ax",@progbits
.global _start
.global abort

_start:
    addi    x1, zero, 0

    /* Now jump to the rust world; __start_rust.  */
    auipc   t0, 0x1
    addi    t0, t0, 0x234
    jr      t0

abort:

.bss