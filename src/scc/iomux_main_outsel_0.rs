#[doc = "Register `IOMUX_MAIN_OUTSEL_0` reader"]
pub type R = crate::R<IomuxMainOutsel0Spec>;
#[doc = "Register `IOMUX_MAIN_OUTSEL_0` writer"]
pub type W = crate::W<IomuxMainOutsel0Spec>;
#[doc = "Field `iomux_main_outsel_0` reader - 0: Select ATF1 1: Select Main Function"]
pub type IomuxMainOutsel0R = crate::FieldReader<u32>;
#[doc = "Field `iomux_main_outsel_0` writer - 0: Select ATF1 1: Select Main Function"]
pub type IomuxMainOutsel0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 0: Select ATF1 1: Select Main Function"]
    #[inline(always)]
    pub fn iomux_main_outsel_0(&self) -> IomuxMainOutsel0R {
        IomuxMainOutsel0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 0: Select ATF1 1: Select Main Function"]
    #[inline(always)]
    pub fn iomux_main_outsel_0(&mut self) -> IomuxMainOutsel0W<IomuxMainOutsel0Spec> {
        IomuxMainOutsel0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_main_outsel_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_main_outsel_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IomuxMainOutsel0Spec;
impl crate::RegisterSpec for IomuxMainOutsel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iomux_main_outsel_0::R`](R) reader structure"]
impl crate::Readable for IomuxMainOutsel0Spec {}
#[doc = "`write(|w| ..)` method takes [`iomux_main_outsel_0::W`](W) writer structure"]
impl crate::Writable for IomuxMainOutsel0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IOMUX_MAIN_OUTSEL_0 to value 0xffff_ffff"]
impl crate::Resettable for IomuxMainOutsel0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
