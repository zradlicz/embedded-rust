MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  /* TODO Adjust these memory regions to match your device memory layout */
  FLASH : ORIGIN = 0x08008000, LENGTH = 96K
  RAM : ORIGIN = 0x20000000, LENGTH = 32K
}