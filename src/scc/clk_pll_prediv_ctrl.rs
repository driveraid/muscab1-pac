#[doc = "Register `CLK_PLL_PREDIV_CTRL` reader"]
pub type R = crate::R<ClkPllPredivCtrlSpec>;
#[doc = "Register `CLK_PLL_PREDIV_CTRL` writer"]
pub type W = crate::W<ClkPllPredivCtrlSpec>;
#[doc = "Field `prediv_ctrl` reader - prediv_ctrl"]
pub type PredivCtrlR = crate::FieldReader<u16>;
#[doc = "Field `prediv_ctrl` writer - prediv_ctrl"]
pub type PredivCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - prediv_ctrl"]
    #[inline(always)]
    pub fn prediv_ctrl(&self) -> PredivCtrlR {
        PredivCtrlR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - prediv_ctrl"]
    #[inline(always)]
    pub fn prediv_ctrl(&mut self) -> PredivCtrlW<ClkPllPredivCtrlSpec> {
        PredivCtrlW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_pll_prediv_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_pll_prediv_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkPllPredivCtrlSpec;
impl crate::RegisterSpec for ClkPllPredivCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_pll_prediv_ctrl::R`](R) reader structure"]
impl crate::Readable for ClkPllPredivCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_pll_prediv_ctrl::W`](W) writer structure"]
impl crate::Writable for ClkPllPredivCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLK_PLL_PREDIV_CTRL to value 0"]
impl crate::Resettable for ClkPllPredivCtrlSpec {}
