#[doc = "Register `CLK_POSTDIV_CTRL_SD` reader"]
pub type R = crate::R<ClkPostdivCtrlSdSpec>;
#[doc = "Register `CLK_POSTDIV_CTRL_SD` writer"]
pub type W = crate::W<ClkPostdivCtrlSdSpec>;
#[doc = "Field `postdiv_ctrl_sd_div` reader - postdiv_ctrl_sd_div"]
pub type PostdivCtrlSdDivR = crate::FieldReader;
#[doc = "Field `postdiv_ctrl_sd_div` writer - postdiv_ctrl_sd_div"]
pub type PostdivCtrlSdDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - postdiv_ctrl_sd_div"]
    #[inline(always)]
    pub fn postdiv_ctrl_sd_div(&self) -> PostdivCtrlSdDivR {
        PostdivCtrlSdDivR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - postdiv_ctrl_sd_div"]
    #[inline(always)]
    pub fn postdiv_ctrl_sd_div(&mut self) -> PostdivCtrlSdDivW<ClkPostdivCtrlSdSpec> {
        PostdivCtrlSdDivW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_postdiv_ctrl_sd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_postdiv_ctrl_sd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkPostdivCtrlSdSpec;
impl crate::RegisterSpec for ClkPostdivCtrlSdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_postdiv_ctrl_sd::R`](R) reader structure"]
impl crate::Readable for ClkPostdivCtrlSdSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_postdiv_ctrl_sd::W`](W) writer structure"]
impl crate::Writable for ClkPostdivCtrlSdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_POSTDIV_CTRL_SD to value 0x01"]
impl crate::Resettable for ClkPostdivCtrlSdSpec {
    const RESET_VALUE: u32 = 0x01;
}
