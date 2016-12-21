MEMORY
{
  /* TODO You must correct these values */
  FLASH : ORIGIN = 0x0BAAAD00, LENGTH = 2K
  RAM : ORIGIN = 0x2BAAAD00, LENGTH = 0
}

ENTRY(_reset)

SECTIONS
{
  .text ORIGIN(FLASH) :
  {
    /* Vector table */
    _VECTOR_TABLE = .;
    LONG(ORIGIN(RAM) + LENGTH(RAM));
    LONG(_reset + 1);
    KEEP(*(.rodata._EXCEPTIONS));
    _eexceptions = .;
    KEEP(*(.rodata._INTERRUPTS));
    _einterrupts = .;

    /* Entry point: reset handler */
    _reset = .;
    *(.text._reset);

    *(.text.*);
    *(.rodata.*);
  } > FLASH

  .bss : ALIGN(4)
  {
    _sbss = .;
    *(.bss.*);
    _ebss = ALIGN(4);
  } > RAM

  .data : ALIGN(4)
  {
    _sdata = .;
    *(.data.*);
    _edata = ALIGN(4);
  } > RAM AT > FLASH

  _sidata = LOADADDR(.data);

  /DISCARD/ :
  {
    *(.ARM.exidx.*)
    *(.note.gnu.build-id.*)
  }
}

/* Exceptions */
PROVIDE(_nmi = _default_exception_handler);
PROVIDE(_hard_fault = _default_exception_handler);
PROVIDE(_memmanage_fault = _default_exception_handler);
PROVIDE(_bus_fault = _default_exception_handler);
PROVIDE(_usage_fault = _default_exception_handler);
PROVIDE(_svcall = _default_exception_handler);
PROVIDE(_pendsv = _default_exception_handler);
PROVIDE(_systick = _default_exception_handler);

/* TODO Interrupts (if you need them) */
/* PROVIDE(_wwdg = _default_exception_handler); */
/* PROVIDE(_pvd = _default_exception_handler); */
/* ... */

ASSERT(_eexceptions - ORIGIN(FLASH) == 0x40, "exceptions not linked where expected");
