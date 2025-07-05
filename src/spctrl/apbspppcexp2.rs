#[doc = "Register `APBSPPPCEXP2` reader"]
pub type R = crate::R<Apbspppcexp2Spec>;
#[doc = "Register `APBSPPPCEXP2` writer"]
pub type W = crate::W<Apbspppcexp2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Expansion 2 Secure Unprivileged Access APB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`apbspppcexp2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbspppcexp2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbspppcexp2Spec;
impl crate::RegisterSpec for Apbspppcexp2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbspppcexp2::R`](R) reader structure"]
impl crate::Readable for Apbspppcexp2Spec {}
#[doc = "`write(|w| ..)` method takes [`apbspppcexp2::W`](W) writer structure"]
impl crate::Writable for Apbspppcexp2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBSPPPCEXP2 to value 0"]
impl crate::Resettable for Apbspppcexp2Spec {
    const RESET_VALUE: u32 = 0;
}
