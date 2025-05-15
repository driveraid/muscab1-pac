#[doc = "Register `CLK_POSTDIV_CTRL_FLASH` reader"]
pub type R = crate::R<ClkPostdivCtrlFlashSpec>;
#[doc = "Register `CLK_POSTDIV_CTRL_FLASH` writer"]
pub type W = crate::W<ClkPostdivCtrlFlashSpec>;
#[doc = "Field `postdiv_ctrl_flash_div` reader - postdiv_ctrl_flash_div"]
pub type PostdivCtrlFlashDivR = crate::FieldReader;
#[doc = "Field `postdiv_ctrl_flash_div` writer - postdiv_ctrl_flash_div"]
pub type PostdivCtrlFlashDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - postdiv_ctrl_flash_div"]
    #[inline(always)]
    pub fn postdiv_ctrl_flash_div(&self) -> PostdivCtrlFlashDivR {
        PostdivCtrlFlashDivR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - postdiv_ctrl_flash_div"]
    #[inline(always)]
    pub fn postdiv_ctrl_flash_div(&mut self) -> PostdivCtrlFlashDivW<ClkPostdivCtrlFlashSpec> {
        PostdivCtrlFlashDivW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_postdiv_ctrl_flash::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_postdiv_ctrl_flash::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkPostdivCtrlFlashSpec;
impl crate::RegisterSpec for ClkPostdivCtrlFlashSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_postdiv_ctrl_flash::R`](R) reader structure"]
impl crate::Readable for ClkPostdivCtrlFlashSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_postdiv_ctrl_flash::W`](W) writer structure"]
impl crate::Writable for ClkPostdivCtrlFlashSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLK_POSTDIV_CTRL_FLASH to value 0x01"]
impl crate::Resettable for ClkPostdivCtrlFlashSpec {
    const RESET_VALUE: u32 = 0x01;
}
