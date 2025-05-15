#[doc = "Register `SECPPCINTCLR` writer"]
pub type W = crate::W<SecppcintclrSpec>;
impl core::fmt::Debug for crate::generic::Reg<SecppcintclrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Secure PPC Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secppcintclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecppcintclrSpec;
impl crate::RegisterSpec for SecppcintclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`secppcintclr::W`](W) writer structure"]
impl crate::Writable for SecppcintclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECPPCINTCLR to value 0"]
impl crate::Resettable for SecppcintclrSpec {
    const RESET_VALUE: u32 = 0;
}
