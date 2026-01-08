/*-------------------------------------------------------
 Multiboot2 header + x86_64 entry point
 Compatible with modular kernel (framebuffer, keyboard, serial)
 AT&T syntax (GAS)
--------------------------------------------------------*/

.section .multiboot, "a"
.align 8

# -----------------------------
# Multiboot2 Header
# -----------------------------
.long 0xe85250d6                 # Multiboot2 magic
.long 0                          # architecture (0 = i386, required even for x86_64)
.long header_end - header_start  # total header length
.long -(0xe85250d6 + 0 + (header_end - header_start))  # checksum

header_start:

# ---- Framebuffer REQUEST TAG (type = 5) ----
.short 5                         # tag type: framebuffer request
.short 0                         # flags
.long 20                         # tag size
.long 1024                       # preferred width
.long 768                        # preferred height
.long 32                         # preferred bits per pixel

# ---- END TAG ----
.short 0                         # type = 0 (end)
.short 0                         # flags
.long 8                          # size

header_end:

# -----------------------------
# Kernel entry point
# -----------------------------
.section .text
.global _start
.extern kernel_main

_start:
    # Initialize a basic stack
    lea stack_top(%rip), %rsp

    # On x86_64 Multiboot2, RDI contains the multiboot info address
    call kernel_main

.hang:
    hlt
    jmp .hang

# -----------------------------
# Kernel stack
# -----------------------------
.section .bss
.align 16
stack:
    .skip 16384                  # 16 KiB stack (aligned)
stack_top:
