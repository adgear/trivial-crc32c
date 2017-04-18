#![feature(asm)]

#[cfg(any(target_arch = "x86_64"))]
pub fn crc32c(bytes: &[u8]) -> u32
{
    let csum: u32;
    unsafe {
        // This might not be the fastest, but it is nice and short.
        asm!("movq    %rsi, %rcx
              xorl    %edx, %edx
              orq     $$-1, %rax
              shrq    $$3, %rcx
         .L3:
              cmpq    %rcx, %rdx
              je      .L2
              crc32q  (%rdi,%rdx,8), %rax
              incq    %rdx
              jmp     .L3
         .L2:
              leaq    (%rdi,%rdx,8), %rcx
              andl    $$7, %esi
              xorl    %edx, %edx
         .L5:
              cmpq    %rdx, %rsi
              je      .L1
              crc32b  (%rcx,%rdx), %eax
              incq    %rdx
              jmp     .L5
         .L1: not     %eax"
             : "={eax}" (csum)
             : "{rdi}" (bytes.as_ptr()), "{rsi}" (bytes.len())
             : "~rcx", "~rdx"
             :);
    }
    csum
}

#[cfg(test)]
mod tests {
    use super::crc32c;

    // http://reveng.sourceforge.net/crc-catalogue/17plus.htm#crc.cat.crc-32c
    #[test]
    fn crc_catalog() {
        assert_eq!(0xe3069283, crc32c(b"123456789"))
    }

    #[test]
    fn rfc3270_all_zeros() {
        assert_eq!(0x8a9136aa, crc32c(&vec![0; 32]))
    }

    #[test]
    fn rfc3270_all_ones() {
        assert_eq!(0x62a8ab43, crc32c(&vec![0xff; 32]))
    }

    #[test]
    fn rfc3270_increasing_values() {
        assert_eq!(0x46dd794e,
                   crc32c(&(0..32).collect::<Vec<u8>>().as_slice()));
    }

    #[test]
    fn rfc3270_decreasing_values() {
        assert_eq!(0x113fdb5c,
                   crc32c(&(0..32).rev().collect::<Vec<u8>>().as_slice()));
    }
}
