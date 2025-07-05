#[doc = "Register `SWRESET` writer"]
pub type W = crate::W<SwresetSpec>;
#[doc = "Field `SWRESETREQ` writer - High Active Software Reset Request"]
pub type SwresetreqW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 9 - High Active Software Reset Request"]
    #[inline(always)]
    pub fn swresetreq(&mut self) -> SwresetreqW<SwresetSpec> {
        SwresetreqW::new(self, 9)
    }
}
#[doc = "Software Reset\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwresetSpec;
impl crate::RegisterSpec for SwresetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swreset::W`](W) writer structure"]
impl crate::Writable for SwresetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWRESET to value 0"]
impl crate::Resettable for SwresetSpec {
    const RESET_VALUE: u32 = 0;
}
