ENTRY(_start)

SECTIONS {
    . = 0x0;

    _stack_start = .;
    . = 0x7c00;
    _stack_end = .;

    .bootloader :
    {
        *(.boot)
        *(.text .text.*)
        . = ALIGN(512);
    }
}