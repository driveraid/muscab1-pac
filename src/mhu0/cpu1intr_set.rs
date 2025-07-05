#[doc = "Register `CPU1INTR_SET` writer"]
pub type W = crate::W<Cpu1intrSetSpec>;
#[doc = "Field `CPU1INTR_SET` writer - CPU 1 Interrupt Set. When a 1 is written to CPU1INTR_SET\\[n\\], the corresponding CPU1INTR_STAT\\[n\\]
signal is set to HIGH."]
pub type Cpu1intrSetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl W {
    #[doc = "Bits 0:3 - CPU 1 Interrupt Set. When a 1 is written to CPU1INTR_SET\\[n\\], the corresponding CPU1INTR_STAT\\[n\\]
signal is set to HIGH."]
    #[inline(always)]
    pub fn cpu1intr_set(&mut self) -> Cpu1intrSetW<Cpu1intrSetSpec> {
        Cpu1intrSetW::new(self, 0)
    }
}
#[doc = "Core 1 interrupt set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1intr_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpu1intrSetSpec;
impl crate::RegisterSpec for Cpu1intrSetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cpu1intr_set::W`](W) writer structure"]
impl crate::Writable for Cpu1intrSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPU1INTR_SET to value 0"]
impl crate::Resettable for Cpu1intrSetSpec {
    const RESET_VALUE: u32 = 0;
}
