#[doc = "Register `CLK_CTRL_SEL` reader"]
pub type R = crate::R<ClkCtrlSelSpec>;
#[doc = "Register `CLK_CTRL_SEL` writer"]
pub type W = crate::W<ClkCtrlSelSpec>;
#[doc = "Field `sel_premux_clk` reader - 0: 32k 1: FASTCLK"]
pub type SelPremuxClkR = crate::BitReader;
#[doc = "Field `sel_premux_clk` writer - 0: 32k 1: FASTCLK"]
pub type SelPremuxClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sel_dapswmux_clk` reader - 0: PRE_MUX_CLK 1: TCK"]
pub type SelDapswmuxClkR = crate::BitReader;
#[doc = "Field `sel_dapswmux_clk` writer - 0: PRE_MUX_CLK 1: TCK"]
pub type SelDapswmuxClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sel_mainmux_clk` reader - 0: PLL0_CLK 1: PRE_MUX_CLK"]
pub type SelMainmuxClkR = crate::BitReader;
#[doc = "Field `sel_mainmux_clk` writer - 0: PLL0_CLK 1: PRE_MUX_CLK"]
pub type SelMainmuxClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sel_refmux_clk` reader - 0: PRE_MUX_CLK 1: PRE_PLL_CLK"]
pub type SelRefmuxClkR = crate::BitReader;
#[doc = "Field `sel_refmux_clk` writer - 0: PRE_MUX_CLK 1: PRE_PLL_CLK"]
pub type SelRefmuxClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sel_rm38kmux_clk` reader - 0: REF_MUX_CLK 1: RM38K"]
pub type SelRm38kmuxClkR = crate::BitReader;
#[doc = "Field `sel_rm38kmux_clk` writer - 0: REF_MUX_CLK 1: RM38K"]
pub type SelRm38kmuxClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sel_sccmux_clk` reader - 0: SCCCLK 1: PRE_MUX_CLK"]
pub type SelSccmuxClkR = crate::BitReader;
#[doc = "Field `sel_sccmux_clk` writer - 0: SCCCLK 1: PRE_MUX_CLK"]
pub type SelSccmuxClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sel_rm38p4_premux_clk` reader - 0: SYSSYSSUGCLK 1: NRM138P4"]
pub type SelRm38p4PremuxClkR = crate::BitReader;
#[doc = "Field `sel_rm38p4_premux_clk` writer - 0: SYSSYSSUGCLK 1: NRM138P4"]
pub type SelRm38p4PremuxClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ctrl_sel_test_mux_clk` reader - ctrl_sel_test_mux_clk"]
pub type CtrlSelTestMuxClkR = crate::FieldReader;
#[doc = "Field `ctrl_sel_test_mux_clk` writer - ctrl_sel_test_mux_clk"]
pub type CtrlSelTestMuxClkW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - 0: 32k 1: FASTCLK"]
    #[inline(always)]
    pub fn sel_premux_clk(&self) -> SelPremuxClkR {
        SelPremuxClkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: PRE_MUX_CLK 1: TCK"]
    #[inline(always)]
    pub fn sel_dapswmux_clk(&self) -> SelDapswmuxClkR {
        SelDapswmuxClkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0: PLL0_CLK 1: PRE_MUX_CLK"]
    #[inline(always)]
    pub fn sel_mainmux_clk(&self) -> SelMainmuxClkR {
        SelMainmuxClkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0: PRE_MUX_CLK 1: PRE_PLL_CLK"]
    #[inline(always)]
    pub fn sel_refmux_clk(&self) -> SelRefmuxClkR {
        SelRefmuxClkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0: REF_MUX_CLK 1: RM38K"]
    #[inline(always)]
    pub fn sel_rm38kmux_clk(&self) -> SelRm38kmuxClkR {
        SelRm38kmuxClkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0: SCCCLK 1: PRE_MUX_CLK"]
    #[inline(always)]
    pub fn sel_sccmux_clk(&self) -> SelSccmuxClkR {
        SelSccmuxClkR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0: SYSSYSSUGCLK 1: NRM138P4"]
    #[inline(always)]
    pub fn sel_rm38p4_premux_clk(&self) -> SelRm38p4PremuxClkR {
        SelRm38p4PremuxClkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:11 - ctrl_sel_test_mux_clk"]
    #[inline(always)]
    pub fn ctrl_sel_test_mux_clk(&self) -> CtrlSelTestMuxClkR {
        CtrlSelTestMuxClkR::new(((self.bits >> 7) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0: 32k 1: FASTCLK"]
    #[inline(always)]
    pub fn sel_premux_clk(&mut self) -> SelPremuxClkW<ClkCtrlSelSpec> {
        SelPremuxClkW::new(self, 0)
    }
    #[doc = "Bit 1 - 0: PRE_MUX_CLK 1: TCK"]
    #[inline(always)]
    pub fn sel_dapswmux_clk(&mut self) -> SelDapswmuxClkW<ClkCtrlSelSpec> {
        SelDapswmuxClkW::new(self, 1)
    }
    #[doc = "Bit 2 - 0: PLL0_CLK 1: PRE_MUX_CLK"]
    #[inline(always)]
    pub fn sel_mainmux_clk(&mut self) -> SelMainmuxClkW<ClkCtrlSelSpec> {
        SelMainmuxClkW::new(self, 2)
    }
    #[doc = "Bit 3 - 0: PRE_MUX_CLK 1: PRE_PLL_CLK"]
    #[inline(always)]
    pub fn sel_refmux_clk(&mut self) -> SelRefmuxClkW<ClkCtrlSelSpec> {
        SelRefmuxClkW::new(self, 3)
    }
    #[doc = "Bit 4 - 0: REF_MUX_CLK 1: RM38K"]
    #[inline(always)]
    pub fn sel_rm38kmux_clk(&mut self) -> SelRm38kmuxClkW<ClkCtrlSelSpec> {
        SelRm38kmuxClkW::new(self, 4)
    }
    #[doc = "Bit 5 - 0: SCCCLK 1: PRE_MUX_CLK"]
    #[inline(always)]
    pub fn sel_sccmux_clk(&mut self) -> SelSccmuxClkW<ClkCtrlSelSpec> {
        SelSccmuxClkW::new(self, 5)
    }
    #[doc = "Bit 6 - 0: SYSSYSSUGCLK 1: NRM138P4"]
    #[inline(always)]
    pub fn sel_rm38p4_premux_clk(&mut self) -> SelRm38p4PremuxClkW<ClkCtrlSelSpec> {
        SelRm38p4PremuxClkW::new(self, 6)
    }
    #[doc = "Bits 7:11 - ctrl_sel_test_mux_clk"]
    #[inline(always)]
    pub fn ctrl_sel_test_mux_clk(&mut self) -> CtrlSelTestMuxClkW<ClkCtrlSelSpec> {
        CtrlSelTestMuxClkW::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_ctrl_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_ctrl_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkCtrlSelSpec;
impl crate::RegisterSpec for ClkCtrlSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_ctrl_sel::R`](R) reader structure"]
impl crate::Readable for ClkCtrlSelSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_ctrl_sel::W`](W) writer structure"]
impl crate::Writable for ClkCtrlSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_CTRL_SEL to value 0x72"]
impl crate::Resettable for ClkCtrlSelSpec {
    const RESET_VALUE: u32 = 0x72;
}
