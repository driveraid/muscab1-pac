#[doc = "Register `APBSPPPCEXP0` reader"]
pub type R = crate::R<Apbspppcexp0Spec>;
#[doc = "Register `APBSPPPCEXP0` writer"]
pub type W = crate::W<Apbspppcexp0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Expansion 0 Secure Unprivileged Access APB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`apbspppcexp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbspppcexp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbspppcexp0Spec;
impl crate::RegisterSpec for Apbspppcexp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbspppcexp0::R`](R) reader structure"]
impl crate::Readable for Apbspppcexp0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbspppcexp0::W`](W) writer structure"]
impl crate::Writable for Apbspppcexp0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APBSPPPCEXP0 to value 0"]
impl crate::Resettable for Apbspppcexp0Spec {}
