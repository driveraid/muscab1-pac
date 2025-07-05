#[doc = "Register `PLL_POSTDIV_CTRL_PLL0_CLK` reader"]
pub type R = crate::R<PllPostdivCtrlPll0ClkSpec>;
#[doc = "Register `PLL_POSTDIV_CTRL_PLL0_CLK` writer"]
pub type W = crate::W<PllPostdivCtrlPll0ClkSpec>;
#[doc = "Field `pll_postdiv_ctrl_pll0_clk` reader - pll_postdiv_ctrl_pll0_clk"]
pub type PllPostdivCtrlPll0ClkR = crate::FieldReader;
#[doc = "Field `pll_postdiv_ctrl_pll0_clk` writer - pll_postdiv_ctrl_pll0_clk"]
pub type PllPostdivCtrlPll0ClkW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - pll_postdiv_ctrl_pll0_clk"]
    #[inline(always)]
    pub fn pll_postdiv_ctrl_pll0_clk(&self) -> PllPostdivCtrlPll0ClkR {
        PllPostdivCtrlPll0ClkR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - pll_postdiv_ctrl_pll0_clk"]
    #[inline(always)]
    pub fn pll_postdiv_ctrl_pll0_clk(
        &mut self,
    ) -> PllPostdivCtrlPll0ClkW<PllPostdivCtrlPll0ClkSpec> {
        PllPostdivCtrlPll0ClkW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_postdiv_ctrl_pll0_clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_postdiv_ctrl_pll0_clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllPostdivCtrlPll0ClkSpec;
impl crate::RegisterSpec for PllPostdivCtrlPll0ClkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_postdiv_ctrl_pll0_clk::R`](R) reader structure"]
impl crate::Readable for PllPostdivCtrlPll0ClkSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_postdiv_ctrl_pll0_clk::W`](W) writer structure"]
impl crate::Writable for PllPostdivCtrlPll0ClkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL_POSTDIV_CTRL_PLL0_CLK to value 0x01"]
impl crate::Resettable for PllPostdivCtrlPll0ClkSpec {
    const RESET_VALUE: u32 = 0x01;
}
