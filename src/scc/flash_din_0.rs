#[doc = "Register `FLASH_DIN_0` reader"]
pub type R = crate::R<FlashDin0Spec>;
#[doc = "Register `FLASH_DIN_0` writer"]
pub type W = crate::W<FlashDin0Spec>;
#[doc = "Field `scc_flash_din0` reader - eFlash 0 and eFlash 1 data input\\[31:0\\]"]
pub type SccFlashDin0R = crate::FieldReader<u32>;
#[doc = "Field `scc_flash_din0` writer - eFlash 0 and eFlash 1 data input\\[31:0\\]"]
pub type SccFlashDin0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - eFlash 0 and eFlash 1 data input\\[31:0\\]"]
    #[inline(always)]
    pub fn scc_flash_din0(&self) -> SccFlashDin0R {
        SccFlashDin0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - eFlash 0 and eFlash 1 data input\\[31:0\\]"]
    #[inline(always)]
    pub fn scc_flash_din0(&mut self) -> SccFlashDin0W<FlashDin0Spec> {
        SccFlashDin0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_din_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_din_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashDin0Spec;
impl crate::RegisterSpec for FlashDin0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_din_0::R`](R) reader structure"]
impl crate::Readable for FlashDin0Spec {}
#[doc = "`write(|w| ..)` method takes [`flash_din_0::W`](W) writer structure"]
impl crate::Writable for FlashDin0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASH_DIN_0 to value 0"]
impl crate::Resettable for FlashDin0Spec {}
