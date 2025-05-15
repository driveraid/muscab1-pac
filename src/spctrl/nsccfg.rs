#[doc = "Register `NSCCFG` reader"]
pub type R = crate::R<NsccfgSpec>;
#[doc = "Register `NSCCFG` writer"]
pub type W = crate::W<NsccfgSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Non Secure Callable Configuration for IDAU\n\nYou can [`read`](crate::Reg::read) this register and get [`nsccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NsccfgSpec;
impl crate::RegisterSpec for NsccfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nsccfg::R`](R) reader structure"]
impl crate::Readable for NsccfgSpec {}
#[doc = "`write(|w| ..)` method takes [`nsccfg::W`](W) writer structure"]
impl crate::Writable for NsccfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NSCCFG to value 0"]
impl crate::Resettable for NsccfgSpec {}
