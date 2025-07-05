#[doc = "Register `INTTYPECLR` reader"]
pub type R = crate::R<InttypeclrSpec>;
#[doc = "Register `INTTYPECLR` writer"]
pub type W = crate::W<InttypeclrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt type clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`inttypeclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inttypeclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InttypeclrSpec;
impl crate::RegisterSpec for InttypeclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inttypeclr::R`](R) reader structure"]
impl crate::Readable for InttypeclrSpec {}
#[doc = "`write(|w| ..)` method takes [`inttypeclr::W`](W) writer structure"]
impl crate::Writable for InttypeclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTTYPECLR to value 0"]
impl crate::Resettable for InttypeclrSpec {
    const RESET_VALUE: u32 = 0;
}
