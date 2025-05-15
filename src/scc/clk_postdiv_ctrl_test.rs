#[doc = "Register `CLK_POSTDIV_CTRL_TEST` reader"]
pub type R = crate::R<ClkPostdivCtrlTestSpec>;
#[doc = "Register `CLK_POSTDIV_CTRL_TEST` writer"]
pub type W = crate::W<ClkPostdivCtrlTestSpec>;
#[doc = "Field `postdiv_ctrl_test_div` reader - postdiv_ctrl_test_div"]
pub type PostdivCtrlTestDivR = crate::FieldReader;
#[doc = "Field `postdiv_ctrl_test_div` writer - postdiv_ctrl_test_div"]
pub type PostdivCtrlTestDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - postdiv_ctrl_test_div"]
    #[inline(always)]
    pub fn postdiv_ctrl_test_div(&self) -> PostdivCtrlTestDivR {
        PostdivCtrlTestDivR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - postdiv_ctrl_test_div"]
    #[inline(always)]
    pub fn postdiv_ctrl_test_div(&mut self) -> PostdivCtrlTestDivW<ClkPostdivCtrlTestSpec> {
        PostdivCtrlTestDivW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_postdiv_ctrl_test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_postdiv_ctrl_test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkPostdivCtrlTestSpec;
impl crate::RegisterSpec for ClkPostdivCtrlTestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_postdiv_ctrl_test::R`](R) reader structure"]
impl crate::Readable for ClkPostdivCtrlTestSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_postdiv_ctrl_test::W`](W) writer structure"]
impl crate::Writable for ClkPostdivCtrlTestSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_POSTDIV_CTRL_TEST to value 0x0a"]
impl crate::Resettable for ClkPostdivCtrlTestSpec {
    const RESET_VALUE: u32 = 0x0a;
}
