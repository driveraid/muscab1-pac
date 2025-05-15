#[doc = "Register `SYSCLK_DIV` reader"]
pub type R = crate::R<SysclkDivSpec>;
#[doc = "Register `SYSCLK_DIV` writer"]
pub type W = crate::W<SysclkDivSpec>;
#[doc = "Field `SYSCLKDIV` reader - SYSCLK from FCLK Clock Divider Ratio Request"]
pub type SysclkdivR = crate::FieldReader;
#[doc = "Field `SYSCLKDIV` writer - SYSCLK from FCLK Clock Divider Ratio Request"]
pub type SysclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SYSCLKDIV_CUR` reader - Clock Divider Current Value"]
pub type SysclkdivCurR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - SYSCLK from FCLK Clock Divider Ratio Request"]
    #[inline(always)]
    pub fn sysclkdiv(&self) -> SysclkdivR {
        SysclkdivR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Clock Divider Current Value"]
    #[inline(always)]
    pub fn sysclkdiv_cur(&self) -> SysclkdivCurR {
        SysclkdivCurR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - SYSCLK from FCLK Clock Divider Ratio Request"]
    #[inline(always)]
    pub fn sysclkdiv(&mut self) -> SysclkdivW<SysclkDivSpec> {
        SysclkdivW::new(self, 0)
    }
}
#[doc = "System Clock Divider Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sysclk_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysclk_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysclkDivSpec;
impl crate::RegisterSpec for SysclkDivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysclk_div::R`](R) reader structure"]
impl crate::Readable for SysclkDivSpec {}
#[doc = "`write(|w| ..)` method takes [`sysclk_div::W`](W) writer structure"]
impl crate::Writable for SysclkDivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCLK_DIV to value 0"]
impl crate::Resettable for SysclkDivSpec {}
