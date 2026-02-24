.global long_mode_start
.extern kernel_main_64

long_mode_start:
    cli

    # Here soon:
    # - create page tables
    # - enable PAE
    # - enable LME
    # - enable paging

    # PLACEHOLDER
.spin:
    hlt
    jmp .spin
