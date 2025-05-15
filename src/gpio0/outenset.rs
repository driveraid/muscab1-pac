#[doc = "Register `OUTENSET` reader"]
pub type R = crate::R<OutensetSpec>;
#[doc = "Register `OUTENSET` writer"]
pub type W = crate::W<OutensetSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ouptut enable set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`outenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutensetSpec;
impl crate::RegisterSpec for OutensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outenset::R`](R) reader structure"]
impl crate::Readable for OutensetSpec {}
#[doc = "`write(|w| ..)` method takes [`outenset::W`](W) writer structure"]
impl crate::Writable for OutensetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUTENSET to value 0"]
impl crate::Resettable for OutensetSpec {}
