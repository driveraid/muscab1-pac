#[doc = "Register `APBNSPPPCEXP3` reader"]
pub type R = crate::R<Apbnspppcexp3Spec>;
#[doc = "Register `APBNSPPPCEXP3` writer"]
pub type W = crate::W<Apbnspppcexp3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Expansion 3 Non_Secure Unprivileged Access APB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`apbnspppcexp3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbnspppcexp3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbnspppcexp3Spec;
impl crate::RegisterSpec for Apbnspppcexp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbnspppcexp3::R`](R) reader structure"]
impl crate::Readable for Apbnspppcexp3Spec {}
#[doc = "`write(|w| ..)` method takes [`apbnspppcexp3::W`](W) writer structure"]
impl crate::Writable for Apbnspppcexp3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APBNSPPPCEXP3 to value 0"]
impl crate::Resettable for Apbnspppcexp3Spec {}
