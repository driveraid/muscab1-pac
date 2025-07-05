#[doc = "Register `PLL_CTRL_PLL0_CLK` reader"]
pub type R = crate::R<PllCtrlPll0ClkSpec>;
#[doc = "Register `PLL_CTRL_PLL0_CLK` writer"]
pub type W = crate::W<PllCtrlPll0ClkSpec>;
#[doc = "Field `pd_pll0` reader - Power down PLL0"]
pub type PdPll0R = crate::BitReader;
#[doc = "Field `pd_pll0` writer - Power down PLL0"]
pub type PdPll0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pd_foutpostdiv1pd` reader - Power down FOUTPOSTDIV1PD:"]
pub type PdFoutpostdiv1pdR = crate::BitReader;
#[doc = "Field `pd_foutpostdiv1pd` writer - Power down FOUTPOSTDIV1PD:"]
pub type PdFoutpostdiv1pdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pd_foutpostdiv2pd` reader - Power down FOUTPOSTDIV2PD"]
pub type PdFoutpostdiv2pdR = crate::BitReader;
#[doc = "Field `pd_foutpostdiv2pd` writer - Power down FOUTPOSTDIV2PD"]
pub type PdFoutpostdiv2pdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pd_foutvcopd` reader - Power down FOUTVCOPD"]
pub type PdFoutvcopdR = crate::BitReader;
#[doc = "Field `pd_foutvcopd` writer - Power down FOUTVCOPD"]
pub type PdFoutvcopdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bypass_pll0` reader - Bypass PLL0"]
pub type BypassPll0R = crate::BitReader;
#[doc = "Field `bypass_pll0` writer - Bypass PLL0"]
pub type BypassPll0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power down PLL0"]
    #[inline(always)]
    pub fn pd_pll0(&self) -> PdPll0R {
        PdPll0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power down FOUTPOSTDIV1PD:"]
    #[inline(always)]
    pub fn pd_foutpostdiv1pd(&self) -> PdFoutpostdiv1pdR {
        PdFoutpostdiv1pdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power down FOUTPOSTDIV2PD"]
    #[inline(always)]
    pub fn pd_foutpostdiv2pd(&self) -> PdFoutpostdiv2pdR {
        PdFoutpostdiv2pdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Power down FOUTVCOPD"]
    #[inline(always)]
    pub fn pd_foutvcopd(&self) -> PdFoutvcopdR {
        PdFoutvcopdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bypass PLL0"]
    #[inline(always)]
    pub fn bypass_pll0(&self) -> BypassPll0R {
        BypassPll0R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power down PLL0"]
    #[inline(always)]
    pub fn pd_pll0(&mut self) -> PdPll0W<PllCtrlPll0ClkSpec> {
        PdPll0W::new(self, 0)
    }
    #[doc = "Bit 1 - Power down FOUTPOSTDIV1PD:"]
    #[inline(always)]
    pub fn pd_foutpostdiv1pd(&mut self) -> PdFoutpostdiv1pdW<PllCtrlPll0ClkSpec> {
        PdFoutpostdiv1pdW::new(self, 1)
    }
    #[doc = "Bit 2 - Power down FOUTPOSTDIV2PD"]
    #[inline(always)]
    pub fn pd_foutpostdiv2pd(&mut self) -> PdFoutpostdiv2pdW<PllCtrlPll0ClkSpec> {
        PdFoutpostdiv2pdW::new(self, 2)
    }
    #[doc = "Bit 3 - Power down FOUTVCOPD"]
    #[inline(always)]
    pub fn pd_foutvcopd(&mut self) -> PdFoutvcopdW<PllCtrlPll0ClkSpec> {
        PdFoutvcopdW::new(self, 3)
    }
    #[doc = "Bit 4 - Bypass PLL0"]
    #[inline(always)]
    pub fn bypass_pll0(&mut self) -> BypassPll0W<PllCtrlPll0ClkSpec> {
        BypassPll0W::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_ctrl_pll0_clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_ctrl_pll0_clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllCtrlPll0ClkSpec;
impl crate::RegisterSpec for PllCtrlPll0ClkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_ctrl_pll0_clk::R`](R) reader structure"]
impl crate::Readable for PllCtrlPll0ClkSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_ctrl_pll0_clk::W`](W) writer structure"]
impl crate::Writable for PllCtrlPll0ClkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL_CTRL_PLL0_CLK to value 0"]
impl crate::Resettable for PllCtrlPll0ClkSpec {
    const RESET_VALUE: u32 = 0;
}
