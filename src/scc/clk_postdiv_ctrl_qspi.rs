#[doc = "Register `CLK_POSTDIV_CTRL_QSPI` reader"]
pub type R = crate::R<ClkPostdivCtrlQspiSpec>;
#[doc = "Register `CLK_POSTDIV_CTRL_QSPI` writer"]
pub type W = crate::W<ClkPostdivCtrlQspiSpec>;
#[doc = "Field `postdiv_ctrl_qspi_div` reader - postdiv_ctrl_qspi_div"]
pub type PostdivCtrlQspiDivR = crate::FieldReader;
#[doc = "Field `postdiv_ctrl_qspi_div` writer - postdiv_ctrl_qspi_div"]
pub type PostdivCtrlQspiDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - postdiv_ctrl_qspi_div"]
    #[inline(always)]
    pub fn postdiv_ctrl_qspi_div(&self) -> PostdivCtrlQspiDivR {
        PostdivCtrlQspiDivR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - postdiv_ctrl_qspi_div"]
    #[inline(always)]
    pub fn postdiv_ctrl_qspi_div(&mut self) -> PostdivCtrlQspiDivW<ClkPostdivCtrlQspiSpec> {
        PostdivCtrlQspiDivW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_postdiv_ctrl_qspi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_postdiv_ctrl_qspi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkPostdivCtrlQspiSpec;
impl crate::RegisterSpec for ClkPostdivCtrlQspiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_postdiv_ctrl_qspi::R`](R) reader structure"]
impl crate::Readable for ClkPostdivCtrlQspiSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_postdiv_ctrl_qspi::W`](W) writer structure"]
impl crate::Writable for ClkPostdivCtrlQspiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_POSTDIV_CTRL_QSPI to value 0x01"]
impl crate::Resettable for ClkPostdivCtrlQspiSpec {
    const RESET_VALUE: u32 = 0x01;
}
