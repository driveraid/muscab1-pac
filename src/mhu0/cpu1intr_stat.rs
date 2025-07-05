#[doc = "Register `CPU1INTR_STAT` reader"]
pub type R = crate::R<Cpu1intrStatSpec>;
#[doc = "Field `CPU1INTR_STAT` reader - CPU 1 Interrupt Status. When any bit is set to 1, the MHU interrupt signal to CPU 1 is set to HIGH."]
pub type Cpu1intrStatR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - CPU 1 Interrupt Status. When any bit is set to 1, the MHU interrupt signal to CPU 1 is set to HIGH."]
    #[inline(always)]
    pub fn cpu1intr_stat(&self) -> Cpu1intrStatR {
        Cpu1intrStatR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Core 1 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu1intr_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpu1intrStatSpec;
impl crate::RegisterSpec for Cpu1intrStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu1intr_stat::R`](R) reader structure"]
impl crate::Readable for Cpu1intrStatSpec {}
#[doc = "`reset()` method sets CPU1INTR_STAT to value 0"]
impl crate::Resettable for Cpu1intrStatSpec {
    const RESET_VALUE: u32 = 0;
}
