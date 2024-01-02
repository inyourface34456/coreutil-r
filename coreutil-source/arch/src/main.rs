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
}
