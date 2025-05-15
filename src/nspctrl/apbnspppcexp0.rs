#[doc = "Register `APBNSPPPCEXP0` reader"]
pub type R = crate::R<Apbnspppcexp0Spec>;
#[doc = "Register `APBNSPPPCEXP0` writer"]
pub type W = crate::W<Apbnspppcexp0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Expansion 0 Non_Secure Unprivileged Access APB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`apbnspppcexp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbnspppcexp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbnspppcexp0Spec;
impl crate::RegisterSpec for Apbnspppcexp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbnspppcexp0::R`](R) reader structure"]
impl crate::Readable for Apbnspppcexp0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbnspppcexp0::W`](W) writer structure"]
impl crate::Writable for Apbnspppcexp0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APBNSPPPCEXP0 to value 0"]
impl crate::Resettable for Apbnspppcexp0Spec {}
