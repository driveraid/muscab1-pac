#[doc = "Register `FLASH_DIN_1` reader"]
pub type R = crate::R<FlashDin1Spec>;
#[doc = "Register `FLASH_DIN_1` writer"]
pub type W = crate::W<FlashDin1Spec>;
#[doc = "Field `scc_flash_din1` reader - eFlash 0 and eFlash 1 data input{63:32]"]
pub type SccFlashDin1R = crate::FieldReader<u32>;
#[doc = "Field `scc_flash_din1` writer - eFlash 0 and eFlash 1 data input{63:32]"]
pub type SccFlashDin1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - eFlash 0 and eFlash 1 data input{63:32]"]
    #[inline(always)]
    pub fn scc_flash_din1(&self) -> SccFlashDin1R {
        SccFlashDin1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - eFlash 0 and eFlash 1 data input{63:32]"]
    #[inline(always)]
    pub fn scc_flash_din1(&mut self) -> SccFlashDin1W<FlashDin1Spec> {
        SccFlashDin1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_din_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_din_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashDin1Spec;
impl crate::RegisterSpec for FlashDin1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_din_1::R`](R) reader structure"]
impl crate::Readable for FlashDin1Spec {}
#[doc = "`write(|w| ..)` method takes [`flash_din_1::W`](W) writer structure"]
impl crate::Writable for FlashDin1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASH_DIN_1 to value 0"]
impl crate::Resettable for FlashDin1Spec {}
