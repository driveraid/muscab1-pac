#[doc = "Register `APBSPPPCEXP1` reader"]
pub type R = crate::R<Apbspppcexp1Spec>;
#[doc = "Register `APBSPPPCEXP1` writer"]
pub type W = crate::W<Apbspppcexp1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Expansion 1 Secure Unprivileged Access APB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`apbspppcexp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbspppcexp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbspppcexp1Spec;
impl crate::RegisterSpec for Apbspppcexp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbspppcexp1::R`](R) reader structure"]
impl crate::Readable for Apbspppcexp1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbspppcexp1::W`](W) writer structure"]
impl crate::Writable for Apbspppcexp1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APBSPPPCEXP1 to value 0"]
impl crate::Resettable for Apbspppcexp1Spec {}
