#[doc = "Register `GPTINTM` reader"]
pub type R = crate::R<GptintmSpec>;
#[doc = "Register `GPTINTM` writer"]
pub type W = crate::W<GptintmSpec>;
#[doc = "Field `GPTINTM` reader - Current masked status of the interrupt"]
pub type GptintmR = crate::FieldReader;
#[doc = "Field `GPTINTM` writer - Current masked status of the interrupt"]
pub type GptintmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Current masked status of the interrupt"]
    #[inline(always)]
    pub fn gptintm(&self) -> GptintmR {
        GptintmR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Current masked status of the interrupt"]
    #[inline(always)]
    pub fn gptintm(&mut self) -> GptintmW<GptintmSpec> {
        GptintmW::new(self, 0)
    }
}
#[doc = "Masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`gptintm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gptintm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptintmSpec;
impl crate::RegisterSpec for GptintmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptintm::R`](R) reader structure"]
impl crate::Readable for GptintmSpec {}
#[doc = "`write(|w| ..)` method takes [`gptintm::W`](W) writer structure"]
impl crate::Writable for GptintmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPTINTM to value 0"]
impl crate::Resettable for GptintmSpec {
    const RESET_VALUE: u32 = 0;
}
