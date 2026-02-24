fn main() {
    cc::Build::new()
        .files(&["arch/x86_64/boot/multiboot.S", "arch/x86_64/boot/long_mode.S"])
        .flag("-m64")
        .compile("multiboot");
}        
