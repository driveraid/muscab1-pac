#[doc = "Register `CLK_TEST_CTRL` reader"]
pub type R = crate::R<ClkTestCtrlSpec>;
#[doc = "Register `CLK_TEST_CTRL` writer"]
pub type W = crate::W<ClkTestCtrlSpec>;
#[doc = "Field `CLK_TEST_SEL` reader - Select TESTMUX input"]
pub type ClkTestSelR = crate::FieldReader;
#[doc = "Field `CLK_TEST_SEL` writer - Select TESTMUX input"]
pub type ClkTestSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CLK_TEST_EN` reader - 0: Not enable 1: Enable"]
pub type ClkTestEnR = crate::BitReader;
#[doc = "Field `CLK_TEST_EN` writer - 0: Not enable 1: Enable"]
pub type ClkTestEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MAIN_FORCE_RDY` reader - CLK_MAIN_FORCE_RDY"]
pub type ClkMainForceRdyR = crate::BitReader;
#[doc = "Field `CLK_MAIN_FORCE_RDY` writer - CLK_MAIN_FORCE_RDY"]
pub type ClkMainForceRdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Select TESTMUX input"]
    #[inline(always)]
    pub fn clk_test_sel(&self) -> ClkTestSelR {
        ClkTestSelR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 0: Not enable 1: Enable"]
    #[inline(always)]
    pub fn clk_test_en(&self) -> ClkTestEnR {
        ClkTestEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CLK_MAIN_FORCE_RDY"]
    #[inline(always)]
    pub fn clk_main_force_rdy(&self) -> ClkMainForceRdyR {
        ClkMainForceRdyR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select TESTMUX input"]
    #[inline(always)]
    pub fn clk_test_sel(&mut self) -> ClkTestSelW<ClkTestCtrlSpec> {
        ClkTestSelW::new(self, 0)
    }
    #[doc = "Bit 5 - 0: Not enable 1: Enable"]
    #[inline(always)]
    pub fn clk_test_en(&mut self) -> ClkTestEnW<ClkTestCtrlSpec> {
        ClkTestEnW::new(self, 5)
    }
    #[doc = "Bit 6 - CLK_MAIN_FORCE_RDY"]
    #[inline(always)]
    pub fn clk_main_force_rdy(&mut self) -> ClkMainForceRdyW<ClkTestCtrlSpec> {
        ClkMainForceRdyW::new(self, 6)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_test_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_test_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkTestCtrlSpec;
impl crate::RegisterSpec for ClkTestCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_test_ctrl::R`](R) reader structure"]
impl crate::Readable for ClkTestCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_test_ctrl::W`](W) writer structure"]
impl crate::Writable for ClkTestCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_TEST_CTRL to value 0"]
impl crate::Resettable for ClkTestCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
