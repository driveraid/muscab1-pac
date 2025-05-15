#[doc = "Register `INTPOLCLR` reader"]
pub type R = crate::R<IntpolclrSpec>;
#[doc = "Register `INTPOLCLR` writer"]
pub type W = crate::W<IntpolclrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Polarity-level, edge interrupt configuration clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intpolclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpolclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntpolclrSpec;
impl crate::RegisterSpec for IntpolclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intpolclr::R`](R) reader structure"]
impl crate::Readable for IntpolclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intpolclr::W`](W) writer structure"]
impl crate::Writable for IntpolclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTPOLCLR to value 0"]
impl crate::Resettable for IntpolclrSpec {
    const RESET_VALUE: u32 = 0;
}
