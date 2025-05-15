#[doc = "Register `FLASH_DIN_2` reader"]
pub type R = crate::R<FlashDin2Spec>;
#[doc = "Register `FLASH_DIN_2` writer"]
pub type W = crate::W<FlashDin2Spec>;
#[doc = "Field `scc_flash_din2` reader - eFlash 0 and eFlash 1 data input\\[95:64\\]"]
pub type SccFlashDin2R = crate::FieldReader<u32>;
#[doc = "Field `scc_flash_din2` writer - eFlash 0 and eFlash 1 data input\\[95:64\\]"]
pub type SccFlashDin2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - eFlash 0 and eFlash 1 data input\\[95:64\\]"]
    #[inline(always)]
    pub fn scc_flash_din2(&self) -> SccFlashDin2R {
        SccFlashDin2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - eFlash 0 and eFlash 1 data input\\[95:64\\]"]
    #[inline(always)]
    pub fn scc_flash_din2(&mut self) -> SccFlashDin2W<FlashDin2Spec> {
        SccFlashDin2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_din_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_din_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashDin2Spec;
impl crate::RegisterSpec for FlashDin2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_din_2::R`](R) reader structure"]
impl crate::Readable for FlashDin2Spec {}
#[doc = "`write(|w| ..)` method takes [`flash_din_2::W`](W) writer structure"]
impl crate::Writable for FlashDin2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASH_DIN_2 to value 0"]
impl crate::Resettable for FlashDin2Spec {}
