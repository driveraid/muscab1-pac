#[doc = "Register `CPU0INTR_SET` writer"]
pub type W = crate::W<Cpu0intrSetSpec>;
#[doc = "Field `CPU0INTR_SET` writer - CPU 0 Interrupt Set. When a 1 is written to CPU0INTR_SET\\[n\\], the corresponding CPU0INTR_STAT\\[n\\]
signal is set to HIGH."]
pub type Cpu0intrSetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl W {
    #[doc = "Bits 0:3 - CPU 0 Interrupt Set. When a 1 is written to CPU0INTR_SET\\[n\\], the corresponding CPU0INTR_STAT\\[n\\]
signal is set to HIGH."]
    #[inline(always)]
    pub fn cpu0intr_set(&mut self) -> Cpu0intrSetW<Cpu0intrSetSpec> {
        Cpu0intrSetW::new(self, 0)
    }
}
#[doc = "Core 0 interrupt set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0intr_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpu0intrSetSpec;
impl crate::RegisterSpec for Cpu0intrSetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cpu0intr_set::W`](W) writer structure"]
impl crate::Writable for Cpu0intrSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPU0INTR_SET to value 0"]
impl crate::Resettable for Cpu0intrSetSpec {
    const RESET_VALUE: u32 = 0;
}
