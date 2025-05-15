#[doc = "Register `AHBNSPPPCEXP3` reader"]
pub type R = crate::R<Ahbnspppcexp3Spec>;
#[doc = "Register `AHBNSPPPCEXP3` writer"]
pub type W = crate::W<Ahbnspppcexp3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Expansion 3 Non_Secure Unprivileged Access AHB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbnspppcexp3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbnspppcexp3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahbnspppcexp3Spec;
impl crate::RegisterSpec for Ahbnspppcexp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbnspppcexp3::R`](R) reader structure"]
impl crate::Readable for Ahbnspppcexp3Spec {}
#[doc = "`write(|w| ..)` method takes [`ahbnspppcexp3::W`](W) writer structure"]
impl crate::Writable for Ahbnspppcexp3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBNSPPPCEXP3 to value 0"]
impl crate::Resettable for Ahbnspppcexp3Spec {}
