#[doc = "Register `AHBSPPPCEXP2` reader"]
pub type R = crate::R<Ahbspppcexp2Spec>;
#[doc = "Register `AHBSPPPCEXP2` writer"]
pub type W = crate::W<Ahbspppcexp2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Expansion 2 Secure Unprivileged Access AHB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbspppcexp2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbspppcexp2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahbspppcexp2Spec;
impl crate::RegisterSpec for Ahbspppcexp2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbspppcexp2::R`](R) reader structure"]
impl crate::Readable for Ahbspppcexp2Spec {}
#[doc = "`write(|w| ..)` method takes [`ahbspppcexp2::W`](W) writer structure"]
impl crate::Writable for Ahbspppcexp2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBSPPPCEXP2 to value 0"]
impl crate::Resettable for Ahbspppcexp2Spec {
    const RESET_VALUE: u32 = 0;
}
