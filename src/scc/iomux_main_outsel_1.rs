#[doc = "Register `IOMUX_MAIN_OUTSEL_1` reader"]
pub type R = crate::R<IomuxMainOutsel1Spec>;
#[doc = "Register `IOMUX_MAIN_OUTSEL_1` writer"]
pub type W = crate::W<IomuxMainOutsel1Spec>;
#[doc = "Field `iomux_main_outsel_1` reader - 0: Select ATF1 1: Select Main Function"]
pub type IomuxMainOutsel1R = crate::FieldReader;
#[doc = "Field `iomux_main_outsel_1` writer - 0: Select ATF1 1: Select Main Function"]
pub type IomuxMainOutsel1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 0: Select ATF1 1: Select Main Function"]
    #[inline(always)]
    pub fn iomux_main_outsel_1(&self) -> IomuxMainOutsel1R {
        IomuxMainOutsel1R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 0: Select ATF1 1: Select Main Function"]
    #[inline(always)]
    pub fn iomux_main_outsel_1(&mut self) -> IomuxMainOutsel1W<IomuxMainOutsel1Spec> {
        IomuxMainOutsel1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_main_outsel_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_main_outsel_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IomuxMainOutsel1Spec;
impl crate::RegisterSpec for IomuxMainOutsel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iomux_main_outsel_1::R`](R) reader structure"]
impl crate::Readable for IomuxMainOutsel1Spec {}
#[doc = "`write(|w| ..)` method takes [`iomux_main_outsel_1::W`](W) writer structure"]
impl crate::Writable for IomuxMainOutsel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IOMUX_MAIN_OUTSEL_1 to value 0xffff_ffff"]
impl crate::Resettable for IomuxMainOutsel1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
