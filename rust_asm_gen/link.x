MEMORY
{
  /* Code memory begins at 0x0000_0000 and has a size of 1kB*/
  CODE : ORIGIN = 0x00000000, LENGTH = 1K
  
  /* RAM begins at 0x0001_0000 and has a size of 1kB
    Since we just address a subset o the address space 
    the used address will start at address 0 
     
    Notice, this is a workaround, overlapping sections should be possible 
    using OVERLAY but seems not to be working in LLVM (LLD) 
  */
  RAM : ORIGIN = 0x00010000, LENGTH = 1K
} ENTRY(reset) 

SECTIONS
{
  .reset :
  {
    *(.reset);
  } > CODE

  .text :
  {
    *(.text .text.*);
  } > CODE

  .rodata :
  {
    *(.rodata .rodata.*);
  } > RAM

  .data :
  {
    *(.data .data.*);
  } > RAM
}
