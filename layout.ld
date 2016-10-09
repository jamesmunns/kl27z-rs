MEMORY
{
  INTERRUPTS : ORIGIN = 0x00000000, LENGTH = 0x00000100
  FLASH      : ORIGIN = 0x00000410, LENGTH = 0x0000FBF0
  RAM        : ORIGIN = 0x1FFFF000, LENGTH = 0x00004000
}

SECTIONS
{
  .vector_table ORIGIN(INTERRUPTS) :
  {
    LONG(ORIGIN(RAM) + LENGTH(RAM))
    KEEP(*(.reset))
    KEEP(*(.exceptions))
  } > INTERRUPTS

  .text :
  {
    *(.text*)
  } > FLASH

  /DISCARD/ :
  {
    *(.ARM.exidx*)
    *(.note.gnu.build-id*)
  }
}
