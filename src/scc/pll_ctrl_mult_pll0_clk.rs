#[doc = "Register `PLL_CTRL_MULT_PLL0_CLK` reader"]
pub type R = crate::R<PllCtrlMultPll0ClkSpec>;
#[doc = "Register `PLL_CTRL_MULT_PLL0_CLK` writer"]
pub type W = crate::W<PllCtrlMultPll0ClkSpec>;
#[doc = "Field `pll_mult_ctrl_pll0_clk` reader - pll_mult_ctrl_pll0_clk"]
pub type PllMultCtrlPll0ClkR = crate::FieldReader<u16>;
#[doc = "Field `pll_mult_ctrl_pll0_clk` writer - pll_mult_ctrl_pll0_clk"]
pub type PllMultCtrlPll0ClkW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - pll_mult_ctrl_pll0_clk"]
    #[inline(always)]
    pub fn pll_mult_ctrl_pll0_clk(&self) -> PllMultCtrlPll0ClkR {
        PllMultCtrlPll0ClkR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - pll_mult_ctrl_pll0_clk"]
    #[inline(always)]
    pub fn pll_mult_ctrl_pll0_clk(&mut self) -> PllMultCtrlPll0ClkW<PllCtrlMultPll0ClkSpec> {
        PllMultCtrlPll0ClkW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_ctrl_mult_pll0_clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_ctrl_mult_pll0_clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllCtrlMultPll0ClkSpec;
impl crate::RegisterSpec for PllCtrlMultPll0ClkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_ctrl_mult_pll0_clk::R`](R) reader structure"]
impl crate::Readable for PllCtrlMultPll0ClkSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_ctrl_mult_pll0_clk::W`](W) writer structure"]
impl crate::Writable for PllCtrlMultPll0ClkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL_CTRL_MULT_PLL0_CLK to value 0x1388"]
impl crate::Resettable for PllCtrlMultPll0ClkSpec {
    const RESET_VALUE: u32 = 0x1388;
}
