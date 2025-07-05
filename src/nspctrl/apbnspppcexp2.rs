#[doc = "Register `APBNSPPPCEXP2` reader"]
pub type R = crate::R<Apbnspppcexp2Spec>;
#[doc = "Register `APBNSPPPCEXP2` writer"]
pub type W = crate::W<Apbnspppcexp2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Expansion 2 Non_Secure Unprivileged Access APB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`apbnspppcexp2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbnspppcexp2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbnspppcexp2Spec;
impl crate::RegisterSpec for Apbnspppcexp2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbnspppcexp2::R`](R) reader structure"]
impl crate::Readable for Apbnspppcexp2Spec {}
#[doc = "`write(|w| ..)` method takes [`apbnspppcexp2::W`](W) writer structure"]
impl crate::Writable for Apbnspppcexp2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBNSPPPCEXP2 to value 0"]
impl crate::Resettable for Apbnspppcexp2Spec {
    const RESET_VALUE: u32 = 0;
}
