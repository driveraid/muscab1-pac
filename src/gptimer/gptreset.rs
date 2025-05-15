#[doc = "Register `GPTRESET` reader"]
pub type R = crate::R<GptresetSpec>;
#[doc = "Field `GPTRESET` reader - CPU0 interrupt status"]
pub type GptresetR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - CPU0 interrupt status"]
    #[inline(always)]
    pub fn gptreset(&self) -> GptresetR {
        GptresetR::new((self.bits & 3) as u8)
    }
}
#[doc = "Control Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gptreset::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptresetSpec;
impl crate::RegisterSpec for GptresetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptreset::R`](R) reader structure"]
impl crate::Readable for GptresetSpec {}
#[doc = "`reset()` method sets GPTRESET to value 0"]
impl crate::Resettable for GptresetSpec {
    const RESET_VALUE: u32 = 0;
}
