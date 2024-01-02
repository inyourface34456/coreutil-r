use common::*;

fn main() {
    #[cfg(target_arch = "x86_64")]
    println!("x86_64");

    #[cfg(target_arch = "wsam32")]
    println!("wsam32");

    #[cfg(target_arch = "thumbv8m")]
    println!("thumbv8m");

    #[cfg(target_arch = "thumbv7neon")]
    println!("thumbv7neon");

    #[cfg(target_arch = "thumbv7m")]
    println!("thumbv7m");

    #[cfg(target_arch = "thumbv7em")]
    println!("thumbv7em");

    #[cfg(target_arch = "thumbv6m")]
    println!("thumbv6m");

    #[cfg(target_arch = "sparcv9")]
    println!("sparcv9");

    #[cfg(target_arch = "sparc64")]
    println!("sparc64");

    #[cfg(target_arch = "s390x")]
    println!("s390x");

    #[cfg(target_arch = "riscv64imac")]
    println!("riscv64imac");

    #[cfg(target_arch = "riscv64gc")]
    println!("riscv64gc");

    #[cfg(target_arch = "riscv32imc")]
    println!("riscv32imc");

    #[cfg(target_arch = "riscv32i")]
    println!("riscv32i");

    #[cfg(target_arch = "powerpc64le")]
    println!("powerpc64le");

    #[cfg(target_arch = "powerpc64")]
    println!("powerpc64");

    #[cfg(target_arch = "powerpc")]
    println!("powerpc");

    #[cfg(target_arch = "nvptx64")]
    println!("nvptx64");

    #[cfg(target_arch = "i686")]
    println!("i686");

    #[cfg(target_arch = "i586")]
    println!("i586");

    #[cfg(target_arch = "armv7")]
    println!("armv7");

    #[cfg(target_arch = "armv7r")]
    println!("armv7r");

    #[cfg(target_arch = "armv7a")]
    println!("armv7a");

    #[cfg(target_arch = "armv5te")]
    println!("armv5te");

    #[cfg(target_arch = "armebv7r")]
    println!("armebv7r");

    #[cfg(target_arch = "arm")]
    println!("arm");

    #[cfg(target_arch = "aarch64")]
    println!("aarch64");
}
