#[doc = "Register `FLASH_DIN_3` reader"]
pub type R = crate::R<FlashDin3Spec>;
#[doc = "Register `FLASH_DIN_3` writer"]
pub type W = crate::W<FlashDin3Spec>;
#[doc = "Field `scc_flash_din3` reader - eFlash 0 and eFlash 1 data input\\[127:96\\]"]
pub type SccFlashDin3R = crate::FieldReader<u32>;
#[doc = "Field `scc_flash_din3` writer - eFlash 0 and eFlash 1 data input\\[127:96\\]"]
pub type SccFlashDin3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - eFlash 0 and eFlash 1 data input\\[127:96\\]"]
    #[inline(always)]
    pub fn scc_flash_din3(&self) -> SccFlashDin3R {
        SccFlashDin3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - eFlash 0 and eFlash 1 data input\\[127:96\\]"]
    #[inline(always)]
    pub fn scc_flash_din3(&mut self) -> SccFlashDin3W<FlashDin3Spec> {
        SccFlashDin3W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_din_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_din_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashDin3Spec;
impl crate::RegisterSpec for FlashDin3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_din_3::R`](R) reader structure"]
impl crate::Readable for FlashDin3Spec {}
#[doc = "`write(|w| ..)` method takes [`flash_din_3::W`](W) writer structure"]
impl crate::Writable for FlashDin3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_DIN_3 to value 0"]
impl crate::Resettable for FlashDin3Spec {
    const RESET_VALUE: u32 = 0;
}
