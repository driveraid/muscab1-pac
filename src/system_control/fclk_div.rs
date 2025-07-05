#[doc = "Register `FCLK_DIV` reader"]
pub type R = crate::R<FclkDivSpec>;
#[doc = "Register `FCLK_DIV` writer"]
pub type W = crate::W<FclkDivSpec>;
#[doc = "Field `FCLKDIV` reader - FCLK from MAINCLK Clock Divider Ratio Request"]
pub type FclkdivR = crate::FieldReader;
#[doc = "Field `FCLKDIV` writer - FCLK from MAINCLK Clock Divider Ratio Request"]
pub type FclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FCLKDIV_CUR` reader - Clock Divider Current Value."]
pub type FclkdivCurR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - FCLK from MAINCLK Clock Divider Ratio Request"]
    #[inline(always)]
    pub fn fclkdiv(&self) -> FclkdivR {
        FclkdivR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Clock Divider Current Value."]
    #[inline(always)]
    pub fn fclkdiv_cur(&self) -> FclkdivCurR {
        FclkdivCurR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - FCLK from MAINCLK Clock Divider Ratio Request"]
    #[inline(always)]
    pub fn fclkdiv(&mut self) -> FclkdivW<FclkDivSpec> {
        FclkdivW::new(self, 0)
    }
}
#[doc = "Fast Clock Divider Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`fclk_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fclk_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FclkDivSpec;
impl crate::RegisterSpec for FclkDivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fclk_div::R`](R) reader structure"]
impl crate::Readable for FclkDivSpec {}
#[doc = "`write(|w| ..)` method takes [`fclk_div::W`](W) writer structure"]
impl crate::Writable for FclkDivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCLK_DIV to value 0"]
impl crate::Resettable for FclkDivSpec {
    const RESET_VALUE: u32 = 0;
}
