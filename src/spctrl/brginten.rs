#[doc = "Register `BRGINTEN` reader"]
pub type R = crate::R<BrgintenSpec>;
#[doc = "Register `BRGINTEN` writer"]
pub type W = crate::W<BrgintenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Bridge Buffer Error Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`brginten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brginten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrgintenSpec;
impl crate::RegisterSpec for BrgintenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brginten::R`](R) reader structure"]
impl crate::Readable for BrgintenSpec {}
#[doc = "`write(|w| ..)` method takes [`brginten::W`](W) writer structure"]
impl crate::Writable for BrgintenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BRGINTEN to value 0"]
impl crate::Resettable for BrgintenSpec {
    const RESET_VALUE: u32 = 0;
}
