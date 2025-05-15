#[doc = "Register `GPTINTR` reader"]
pub type R = crate::R<GptintrSpec>;
#[doc = "Field `GPTINTR` reader - Raw interrupt state, before masking of GPTINTR interrupt"]
pub type GptintrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Raw interrupt state, before masking of GPTINTR interrupt"]
    #[inline(always)]
    pub fn gptintr(&self) -> GptintrR {
        GptintrR::new((self.bits & 7) as u8)
    }
}
#[doc = "Raw interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`gptintr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptintrSpec;
impl crate::RegisterSpec for GptintrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptintr::R`](R) reader structure"]
impl crate::Readable for GptintrSpec {}
#[doc = "`reset()` method sets GPTINTR to value 0"]
impl crate::Resettable for GptintrSpec {
    const RESET_VALUE: u32 = 0;
}
