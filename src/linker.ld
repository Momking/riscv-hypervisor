OUTPUT_ARCH(riscv)
ENTRY(_start)

SECTIONS
{
    /* OpenSBI jumps here */
    . = 0x80200000;

    .text : {
        *(.text.entry) /* Our entry point assembly */
        *(.text .text.*)
    }

    .rodata : { *(.rodata .rodata.*) }
    .data : { *(.data .data.*) }
    .bss : {
        *(.bss .bss.*)
        *(.bss.stack) /* Our stack */
        stack_top = .;
    }

    /DISCARD/ : { *(.eh_frame) }
}
