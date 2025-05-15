#[doc = "Register `IOMUX_MAIN_DEFAULT_IN_1` reader"]
pub type R = crate::R<IomuxMainDefaultIn1Spec>;
#[doc = "Register `IOMUX_MAIN_DEFAULT_IN_1` writer"]
pub type W = crate::W<IomuxMainDefaultIn1Spec>;
#[doc = "Field `iomux_main_default_in_1` reader - 0: Default to 0 1: Default to 1"]
pub type IomuxMainDefaultIn1R = crate::FieldReader;
#[doc = "Field `iomux_main_default_in_1` writer - 0: Default to 0 1: Default to 1"]
pub type IomuxMainDefaultIn1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 0: Default to 0 1: Default to 1"]
    #[inline(always)]
    pub fn iomux_main_default_in_1(&self) -> IomuxMainDefaultIn1R {
        IomuxMainDefaultIn1R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 0: Default to 0 1: Default to 1"]
    #[inline(always)]
    pub fn iomux_main_default_in_1(&mut self) -> IomuxMainDefaultIn1W<IomuxMainDefaultIn1Spec> {
        IomuxMainDefaultIn1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_main_default_in_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_main_default_in_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IomuxMainDefaultIn1Spec;
impl crate::RegisterSpec for IomuxMainDefaultIn1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iomux_main_default_in_1::R`](R) reader structure"]
impl crate::Readable for IomuxMainDefaultIn1Spec {}
#[doc = "`write(|w| ..)` method takes [`iomux_main_default_in_1::W`](W) writer structure"]
impl crate::Writable for IomuxMainDefaultIn1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOMUX_MAIN_DEFAULT_IN_1 to value 0"]
impl crate::Resettable for IomuxMainDefaultIn1Spec {
    const RESET_VALUE: u32 = 0;
}
