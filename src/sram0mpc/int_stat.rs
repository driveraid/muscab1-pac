#[doc = "Register `INT_STAT` reader"]
pub type R = crate::R<IntStatSpec>;
#[doc = "Field `bit[0]` reader - mpc_irq triggered"]
pub type Bit0R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - mpc_irq triggered"]
    #[inline(always)]
    pub fn bit0(&self) -> Bit0R {
        Bit0R::new((self.bits & 1) != 0)
    }
}
#[doc = "Interrupt state\n\nYou can [`read`](crate::Reg::read) this register and get [`int_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStatSpec;
impl crate::RegisterSpec for IntStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_stat::R`](R) reader structure"]
impl crate::Readable for IntStatSpec {}
#[doc = "`reset()` method sets INT_STAT to value 0"]
impl crate::Resettable for IntStatSpec {
    const RESET_VALUE: u32 = 0;
}
