#[doc = "Register `CPU1INTR_CLR` writer"]
pub type W = crate::W<Cpu1intrClrSpec>;
#[doc = "Field `CPU1INTR_CLR` writer - CPU 1 Interrupt Clear. When a 1 is written to CPU1INTR_CLR\\[n\\], the corresponding CPU1INTR_STAT\\[n\\]
signal is set to LOW."]
pub type Cpu1intrClrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl W {
    #[doc = "Bits 0:3 - CPU 1 Interrupt Clear. When a 1 is written to CPU1INTR_CLR\\[n\\], the corresponding CPU1INTR_STAT\\[n\\]
signal is set to LOW."]
    #[inline(always)]
    pub fn cpu1intr_clr(&mut self) -> Cpu1intrClrW<Cpu1intrClrSpec> {
        Cpu1intrClrW::new(self, 0)
    }
}
#[doc = "Core 1 interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1intr_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpu1intrClrSpec;
impl crate::RegisterSpec for Cpu1intrClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cpu1intr_clr::W`](W) writer structure"]
impl crate::Writable for Cpu1intrClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPU1INTR_CLR to value 0"]
impl crate::Resettable for Cpu1intrClrSpec {
    const RESET_VALUE: u32 = 0;
}
