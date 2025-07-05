#[doc = "Register `GPTCOUNTER` reader"]
pub type R = crate::R<GptcounterSpec>;
#[doc = "Field `GPTCOUNTER` reader - Current value of 32-bit Timer Counter"]
pub type GptcounterR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current value of 32-bit Timer Counter"]
    #[inline(always)]
    pub fn gptcounter(&self) -> GptcounterR {
        GptcounterR::new(self.bits)
    }
}
#[doc = "Counter data value register\n\nYou can [`read`](crate::Reg::read) this register and get [`gptcounter::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptcounterSpec;
impl crate::RegisterSpec for GptcounterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptcounter::R`](R) reader structure"]
impl crate::Readable for GptcounterSpec {}
#[doc = "`reset()` method sets GPTCOUNTER to value 0"]
impl crate::Resettable for GptcounterSpec {
    const RESET_VALUE: u32 = 0;
}
