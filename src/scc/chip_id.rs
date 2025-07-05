#[doc = "Register `CHIP_ID` reader"]
pub type R = crate::R<ChipIdSpec>;
#[doc = "Field `chip_id` reader - Component ID information"]
pub type ChipIdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Component ID information"]
    #[inline(always)]
    pub fn chip_id(&self) -> ChipIdR {
        ChipIdR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`chip_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChipIdSpec;
impl crate::RegisterSpec for ChipIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chip_id::R`](R) reader structure"]
impl crate::Readable for ChipIdSpec {}
#[doc = "`reset()` method sets CHIP_ID to value 0x07d0_0477"]
impl crate::Resettable for ChipIdSpec {
    const RESET_VALUE: u32 = 0x07d0_0477;
}
