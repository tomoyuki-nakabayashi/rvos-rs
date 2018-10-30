.section .boot, "awx"
.global _start

_start:
    addi    x1, zero, 0

    /* Now jump to the rust world; __start_rust.  */
    auipc   t0, __start_rust
    addi    t0, t0, __start_rust
    jr      t0