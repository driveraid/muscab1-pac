#[doc = "Register `CTRL_BYPASS_DIV` reader"]
pub type R = crate::R<CtrlBypassDivSpec>;
#[doc = "Register `CTRL_BYPASS_DIV` writer"]
pub type W = crate::W<CtrlBypassDivSpec>;
#[doc = "Field `bypass_div_pll_div_prediv_clk` reader - 0: Not bypass 1: bypass"]
pub type BypassDivPllDivPredivClkR = crate::BitReader;
#[doc = "Field `bypass_div_pll_div_prediv_clk` writer - 0: Not bypass 1: bypass"]
pub type BypassDivPllDivPredivClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bypass_qspi_div_clk` reader - 0: Not bypass 1: bypass"]
pub type BypassQspiDivClkR = crate::BitReader;
#[doc = "Field `bypass_qspi_div_clk` writer - 0: Not bypass 1: bypass"]
pub type BypassQspiDivClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bypass_rtc_div_clk` reader - 0: Not bypass 1: bypass"]
pub type BypassRtcDivClkR = crate::BitReader;
#[doc = "Field `bypass_rtc_div_clk` writer - 0: Not bypass 1: bypass"]
pub type BypassRtcDivClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bypass_sd_div_clk` reader - 0: Not bypass 1: bypass"]
pub type BypassSdDivClkR = crate::BitReader;
#[doc = "Field `bypass_sd_div_clk` writer - 0: Not bypass 1: bypass"]
pub type BypassSdDivClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bypass_test_div_clk` reader - 0: Not bypass 1: bypass"]
pub type BypassTestDivClkR = crate::BitReader;
#[doc = "Field `bypass_test_div_clk` writer - 0: Not bypass 1: bypass"]
pub type BypassTestDivClkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: Not bypass 1: bypass"]
    #[inline(always)]
    pub fn bypass_div_pll_div_prediv_clk(&self) -> BypassDivPllDivPredivClkR {
        BypassDivPllDivPredivClkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - 0: Not bypass 1: bypass"]
    #[inline(always)]
    pub fn bypass_qspi_div_clk(&self) -> BypassQspiDivClkR {
        BypassQspiDivClkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0: Not bypass 1: bypass"]
    #[inline(always)]
    pub fn bypass_rtc_div_clk(&self) -> BypassRtcDivClkR {
        BypassRtcDivClkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0: Not bypass 1: bypass"]
    #[inline(always)]
    pub fn bypass_sd_div_clk(&self) -> BypassSdDivClkR {
        BypassSdDivClkR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0: Not bypass 1: bypass"]
    #[inline(always)]
    pub fn bypass_test_div_clk(&self) -> BypassTestDivClkR {
        BypassTestDivClkR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Not bypass 1: bypass"]
    #[inline(always)]
    pub fn bypass_div_pll_div_prediv_clk(
        &mut self,
    ) -> BypassDivPllDivPredivClkW<CtrlBypassDivSpec> {
        BypassDivPllDivPredivClkW::new(self, 0)
    }
    #[doc = "Bit 3 - 0: Not bypass 1: bypass"]
    #[inline(always)]
    pub fn bypass_qspi_div_clk(&mut self) -> BypassQspiDivClkW<CtrlBypassDivSpec> {
        BypassQspiDivClkW::new(self, 3)
    }
    #[doc = "Bit 4 - 0: Not bypass 1: bypass"]
    #[inline(always)]
    pub fn bypass_rtc_div_clk(&mut self) -> BypassRtcDivClkW<CtrlBypassDivSpec> {
        BypassRtcDivClkW::new(self, 4)
    }
    #[doc = "Bit 5 - 0: Not bypass 1: bypass"]
    #[inline(always)]
    pub fn bypass_sd_div_clk(&mut self) -> BypassSdDivClkW<CtrlBypassDivSpec> {
        BypassSdDivClkW::new(self, 5)
    }
    #[doc = "Bit 6 - 0: Not bypass 1: bypass"]
    #[inline(always)]
    pub fn bypass_test_div_clk(&mut self) -> BypassTestDivClkW<CtrlBypassDivSpec> {
        BypassTestDivClkW::new(self, 6)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_bypass_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_bypass_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlBypassDivSpec;
impl crate::RegisterSpec for CtrlBypassDivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_bypass_div::R`](R) reader structure"]
impl crate::Readable for CtrlBypassDivSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl_bypass_div::W`](W) writer structure"]
impl crate::Writable for CtrlBypassDivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL_BYPASS_DIV to value 0x01"]
impl crate::Resettable for CtrlBypassDivSpec {
    const RESET_VALUE: u32 = 0x01;
}
