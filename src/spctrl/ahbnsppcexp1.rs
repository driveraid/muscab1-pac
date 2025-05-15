#[doc = "Register `AHBNSPPCEXP1` reader"]
pub type R = crate::R<Ahbnsppcexp1Spec>;
#[doc = "Register `AHBNSPPCEXP1` writer"]
pub type W = crate::W<Ahbnsppcexp1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Expansion 1 Non_Secure Access AHB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbnsppcexp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbnsppcexp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahbnsppcexp1Spec;
impl crate::RegisterSpec for Ahbnsppcexp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbnsppcexp1::R`](R) reader structure"]
impl crate::Readable for Ahbnsppcexp1Spec {}
#[doc = "`write(|w| ..)` method takes [`ahbnsppcexp1::W`](W) writer structure"]
impl crate::Writable for Ahbnsppcexp1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBNSPPCEXP1 to value 0"]
impl crate::Resettable for Ahbnsppcexp1Spec {}
