#[doc = "Register `CLK_POSTDIV_CTRL_RTC` reader"]
pub type R = crate::R<ClkPostdivCtrlRtcSpec>;
#[doc = "Register `CLK_POSTDIV_CTRL_RTC` writer"]
pub type W = crate::W<ClkPostdivCtrlRtcSpec>;
#[doc = "Field `postdiv_ctrl_rtc_div` reader - postdiv_ctrl_rtc_div"]
pub type PostdivCtrlRtcDivR = crate::FieldReader<u32>;
#[doc = "Field `postdiv_ctrl_rtc_div` writer - postdiv_ctrl_rtc_div"]
pub type PostdivCtrlRtcDivW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - postdiv_ctrl_rtc_div"]
    #[inline(always)]
    pub fn postdiv_ctrl_rtc_div(&self) -> PostdivCtrlRtcDivR {
        PostdivCtrlRtcDivR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - postdiv_ctrl_rtc_div"]
    #[inline(always)]
    pub fn postdiv_ctrl_rtc_div(&mut self) -> PostdivCtrlRtcDivW<ClkPostdivCtrlRtcSpec> {
        PostdivCtrlRtcDivW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_postdiv_ctrl_rtc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_postdiv_ctrl_rtc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkPostdivCtrlRtcSpec;
impl crate::RegisterSpec for ClkPostdivCtrlRtcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_postdiv_ctrl_rtc::R`](R) reader structure"]
impl crate::Readable for ClkPostdivCtrlRtcSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_postdiv_ctrl_rtc::W`](W) writer structure"]
impl crate::Writable for ClkPostdivCtrlRtcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_POSTDIV_CTRL_RTC to value 0xffff_ffff"]
impl crate::Resettable for ClkPostdivCtrlRtcSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
