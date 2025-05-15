#[doc = "Register `APBNSPPCEXP1` reader"]
pub type R = crate::R<Apbnsppcexp1Spec>;
#[doc = "Register `APBNSPPCEXP1` writer"]
pub type W = crate::W<Apbnsppcexp1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Expansion 1 Non_Secure Access APB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`apbnsppcexp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbnsppcexp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbnsppcexp1Spec;
impl crate::RegisterSpec for Apbnsppcexp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbnsppcexp1::R`](R) reader structure"]
impl crate::Readable for Apbnsppcexp1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbnsppcexp1::W`](W) writer structure"]
impl crate::Writable for Apbnsppcexp1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBNSPPCEXP1 to value 0"]
impl crate::Resettable for Apbnsppcexp1Spec {
    const RESET_VALUE: u32 = 0;
}
