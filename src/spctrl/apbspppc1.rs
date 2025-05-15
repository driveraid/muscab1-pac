#[doc = "Register `APBSPPPC1` reader"]
pub type R = crate::R<Apbspppc1Spec>;
#[doc = "Register `APBSPPPC1` writer"]
pub type W = crate::W<Apbspppc1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Secure Unprivileged Access APB slave Peripheral Protection Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`apbspppc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbspppc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbspppc1Spec;
impl crate::RegisterSpec for Apbspppc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbspppc1::R`](R) reader structure"]
impl crate::Readable for Apbspppc1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbspppc1::W`](W) writer structure"]
impl crate::Writable for Apbspppc1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APBSPPPC1 to value 0"]
impl crate::Resettable for Apbspppc1Spec {}
