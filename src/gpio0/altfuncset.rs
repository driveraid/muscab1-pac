#[doc = "Register `ALTFUNCSET` reader"]
pub type R = crate::R<AltfuncsetSpec>;
#[doc = "Register `ALTFUNCSET` writer"]
pub type W = crate::W<AltfuncsetSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Alternate function set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`altfuncset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`altfuncset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AltfuncsetSpec;
impl crate::RegisterSpec for AltfuncsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`altfuncset::R`](R) reader structure"]
impl crate::Readable for AltfuncsetSpec {}
#[doc = "`write(|w| ..)` method takes [`altfuncset::W`](W) writer structure"]
impl crate::Writable for AltfuncsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALTFUNCSET to value 0"]
impl crate::Resettable for AltfuncsetSpec {}
