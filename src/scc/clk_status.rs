#[doc = "Register `CLK_STATUS` reader"]
pub type R = crate::R<ClkStatusSpec>;
#[doc = "Field `status_out_clk_mainclk_ready` reader - Clock ready (active)"]
pub type StatusOutClkMainclkReadyR = crate::BitReader;
#[doc = "Field `status_lock_signal_pll0_clk` reader - PLL Lock Status"]
pub type StatusLockSignalPll0ClkR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Clock ready (active)"]
    #[inline(always)]
    pub fn status_out_clk_mainclk_ready(&self) -> StatusOutClkMainclkReadyR {
        StatusOutClkMainclkReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL Lock Status"]
    #[inline(always)]
    pub fn status_lock_signal_pll0_clk(&self) -> StatusLockSignalPll0ClkR {
        StatusLockSignalPll0ClkR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkStatusSpec;
impl crate::RegisterSpec for ClkStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_status::R`](R) reader structure"]
impl crate::Readable for ClkStatusSpec {}
#[doc = "`reset()` method sets CLK_STATUS to value 0x03"]
impl crate::Resettable for ClkStatusSpec {
    const RESET_VALUE: u32 = 0x03;
}
