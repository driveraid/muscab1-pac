#[doc = "Register `INT_EN` reader"]
pub type R = crate::R<IntEnSpec>;
#[doc = "Register `INT_EN` writer"]
pub type W = crate::W<IntEnSpec>;
#[doc = "Field `bit[0]` reader - mpc_irq enable. Bits are valid when mpc_irq triggered is set"]
pub type Bit0R = crate::BitReader;
#[doc = "Field `bit[0]` writer - mpc_irq enable. Bits are valid when mpc_irq triggered is set"]
pub type Bit0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - mpc_irq enable. Bits are valid when mpc_irq triggered is set"]
    #[inline(always)]
    pub fn bit0(&self) -> Bit0R {
        Bit0R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - mpc_irq enable. Bits are valid when mpc_irq triggered is set"]
    #[inline(always)]
    pub fn bit0(&mut self) -> Bit0W<IntEnSpec> {
        Bit0W::new(self, 0)
    }
}
#[doc = "Interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEnSpec;
impl crate::RegisterSpec for IntEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_en::R`](R) reader structure"]
impl crate::Readable for IntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`int_en::W`](W) writer structure"]
impl crate::Writable for IntEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EN to value 0"]
impl crate::Resettable for IntEnSpec {
    const RESET_VALUE: u32 = 0;
}
