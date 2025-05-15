#[doc = "Register `APBNSPPC1` reader"]
pub type R = crate::R<Apbnsppc1Spec>;
#[doc = "Register `APBNSPPC1` writer"]
pub type W = crate::W<Apbnsppc1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Non-Secure Access APB slave Peripheral Protection Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`apbnsppc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbnsppc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbnsppc1Spec;
impl crate::RegisterSpec for Apbnsppc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbnsppc1::R`](R) reader structure"]
impl crate::Readable for Apbnsppc1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbnsppc1::W`](W) writer structure"]
impl crate::Writable for Apbnsppc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBNSPPC1 to value 0"]
impl crate::Resettable for Apbnsppc1Spec {
    const RESET_VALUE: u32 = 0;
}
