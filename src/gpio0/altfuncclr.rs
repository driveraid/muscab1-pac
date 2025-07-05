#[doc = "Register `ALTFUNCCLR` reader"]
pub type R = crate::R<AltfuncclrSpec>;
#[doc = "Register `ALTFUNCCLR` writer"]
pub type W = crate::W<AltfuncclrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Alternate function clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`altfuncclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`altfuncclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AltfuncclrSpec;
impl crate::RegisterSpec for AltfuncclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`altfuncclr::R`](R) reader structure"]
impl crate::Readable for AltfuncclrSpec {}
#[doc = "`write(|w| ..)` method takes [`altfuncclr::W`](W) writer structure"]
impl crate::Writable for AltfuncclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALTFUNCCLR to value 0"]
impl crate::Resettable for AltfuncclrSpec {
    const RESET_VALUE: u32 = 0;
}
