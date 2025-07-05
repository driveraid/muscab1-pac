#[doc = "Register `INT_CLEAR` writer"]
pub type W = crate::W<IntClearSpec>;
#[doc = "Field `bit[0]` writer - mpc_irq clear (cleared automatically)"]
pub type Bit0W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - mpc_irq clear (cleared automatically)"]
    #[inline(always)]
    pub fn bit0(&mut self) -> Bit0W<IntClearSpec> {
        Bit0W::new(self, 0)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntClearSpec;
impl crate::RegisterSpec for IntClearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clear::W`](W) writer structure"]
impl crate::Writable for IntClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_CLEAR to value 0"]
impl crate::Resettable for IntClearSpec {
    const RESET_VALUE: u32 = 0;
}
