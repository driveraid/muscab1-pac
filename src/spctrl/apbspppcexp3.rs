#[doc = "Register `APBSPPPCEXP3` reader"]
pub type R = crate::R<Apbspppcexp3Spec>;
#[doc = "Register `APBSPPPCEXP3` writer"]
pub type W = crate::W<Apbspppcexp3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Expansion 3 Secure Unprivileged Access APB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`apbspppcexp3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbspppcexp3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbspppcexp3Spec;
impl crate::RegisterSpec for Apbspppcexp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbspppcexp3::R`](R) reader structure"]
impl crate::Readable for Apbspppcexp3Spec {}
#[doc = "`write(|w| ..)` method takes [`apbspppcexp3::W`](W) writer structure"]
impl crate::Writable for Apbspppcexp3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBSPPPCEXP3 to value 0"]
impl crate::Resettable for Apbspppcexp3Spec {
    const RESET_VALUE: u32 = 0;
}
