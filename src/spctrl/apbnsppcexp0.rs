#[doc = "Register `APBNSPPCEXP0` reader"]
pub type R = crate::R<Apbnsppcexp0Spec>;
#[doc = "Register `APBNSPPCEXP0` writer"]
pub type W = crate::W<Apbnsppcexp0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Expansion 0 Non_Secure Access APB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`apbnsppcexp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbnsppcexp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbnsppcexp0Spec;
impl crate::RegisterSpec for Apbnsppcexp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbnsppcexp0::R`](R) reader structure"]
impl crate::Readable for Apbnsppcexp0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbnsppcexp0::W`](W) writer structure"]
impl crate::Writable for Apbnsppcexp0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APBNSPPCEXP0 to value 0"]
impl crate::Resettable for Apbnsppcexp0Spec {}
