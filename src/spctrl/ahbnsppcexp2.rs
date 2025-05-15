#[doc = "Register `AHBNSPPCEXP2` reader"]
pub type R = crate::R<Ahbnsppcexp2Spec>;
#[doc = "Register `AHBNSPPCEXP2` writer"]
pub type W = crate::W<Ahbnsppcexp2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Expansion 2 Non_Secure Access AHB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbnsppcexp2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbnsppcexp2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahbnsppcexp2Spec;
impl crate::RegisterSpec for Ahbnsppcexp2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbnsppcexp2::R`](R) reader structure"]
impl crate::Readable for Ahbnsppcexp2Spec {}
#[doc = "`write(|w| ..)` method takes [`ahbnsppcexp2::W`](W) writer structure"]
impl crate::Writable for Ahbnsppcexp2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBNSPPCEXP2 to value 0"]
impl crate::Resettable for Ahbnsppcexp2Spec {}
