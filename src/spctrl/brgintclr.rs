#[doc = "Register `BRGINTCLR` writer"]
pub type W = crate::W<BrgintclrSpec>;
impl core::fmt::Debug for crate::generic::Reg<BrgintclrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Bridge Buffer Error Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brgintclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrgintclrSpec;
impl crate::RegisterSpec for BrgintclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`brgintclr::W`](W) writer structure"]
impl crate::Writable for BrgintclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BRGINTCLR to value 0"]
impl crate::Resettable for BrgintclrSpec {}
