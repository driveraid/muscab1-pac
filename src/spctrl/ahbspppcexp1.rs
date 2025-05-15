#[doc = "Register `AHBSPPPCEXP1` reader"]
pub type R = crate::R<Ahbspppcexp1Spec>;
#[doc = "Register `AHBSPPPCEXP1` writer"]
pub type W = crate::W<Ahbspppcexp1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Expansion 1 Secure Unprivileged Access AHB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbspppcexp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbspppcexp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahbspppcexp1Spec;
impl crate::RegisterSpec for Ahbspppcexp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbspppcexp1::R`](R) reader structure"]
impl crate::Readable for Ahbspppcexp1Spec {}
#[doc = "`write(|w| ..)` method takes [`ahbspppcexp1::W`](W) writer structure"]
impl crate::Writable for Ahbspppcexp1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBSPPPCEXP1 to value 0"]
impl crate::Resettable for Ahbspppcexp1Spec {
    const RESET_VALUE: u32 = 0;
}
