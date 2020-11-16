ENTRY(_start);
SECTIONS {
         . = 0x100000;
         .text : { *(.text) *(.text.*)
                   *(.rodata) *(.rodata.*) }
         .data : { *(.data) }
         .bss : { *(.bss) }
         
}
