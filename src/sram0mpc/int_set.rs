#[doc = "Register `INT_SET` writer"]
pub type W = crate::W<IntSetSpec>;
#[doc = "Field `bit[0]` writer - mpc_irq set. Debug purpose only"]
pub type Bit0W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - mpc_irq set. Debug purpose only"]
    #[inline(always)]
    pub fn bit0(&mut self) -> Bit0W<IntSetSpec> {
        Bit0W::new(self, 0)
    }
}
#[doc = "Interrupt set. Debug purpose only\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntSetSpec;
impl crate::RegisterSpec for IntSetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_set::W`](W) writer structure"]
impl crate::Writable for IntSetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_SET to value 0"]
impl crate::Resettable for IntSetSpec {}
