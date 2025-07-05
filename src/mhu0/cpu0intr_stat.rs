#[doc = "Register `CPU0INTR_STAT` reader"]
pub type R = crate::R<Cpu0intrStatSpec>;
#[doc = "Field `CPU0INTR_STAT` reader - CPU 0 Interrupt Status. When any bit is set to 1, the MHU interrupt signal to CPU 0 is set to HIGH."]
pub type Cpu0intrStatR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - CPU 0 Interrupt Status. When any bit is set to 1, the MHU interrupt signal to CPU 0 is set to HIGH."]
    #[inline(always)]
    pub fn cpu0intr_stat(&self) -> Cpu0intrStatR {
        Cpu0intrStatR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Core 0 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0intr_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpu0intrStatSpec;
impl crate::RegisterSpec for Cpu0intrStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu0intr_stat::R`](R) reader structure"]
impl crate::Readable for Cpu0intrStatSpec {}
#[doc = "`reset()` method sets CPU0INTR_STAT to value 0"]
impl crate::Resettable for Cpu0intrStatSpec {
    const RESET_VALUE: u32 = 0;
}
