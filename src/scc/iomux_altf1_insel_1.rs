#[doc = "Register `IOMUX_ALTF1_INSEL_1` reader"]
pub type R = crate::R<IomuxAltf1Insel1Spec>;
#[doc = "Register `IOMUX_ALTF1_INSEL_1` writer"]
pub type W = crate::W<IomuxAltf1Insel1Spec>;
#[doc = "Field `iomux_altf1_insel_1` reader - 0: Select ATF2 1: Select ATF1"]
pub type IomuxAltf1Insel1R = crate::FieldReader;
#[doc = "Field `iomux_altf1_insel_1` writer - 0: Select ATF2 1: Select ATF1"]
pub type IomuxAltf1Insel1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 0: Select ATF2 1: Select ATF1"]
    #[inline(always)]
    pub fn iomux_altf1_insel_1(&self) -> IomuxAltf1Insel1R {
        IomuxAltf1Insel1R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 0: Select ATF2 1: Select ATF1"]
    #[inline(always)]
    pub fn iomux_altf1_insel_1(&mut self) -> IomuxAltf1Insel1W<IomuxAltf1Insel1Spec> {
        IomuxAltf1Insel1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_altf1_insel_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_altf1_insel_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IomuxAltf1Insel1Spec;
impl crate::RegisterSpec for IomuxAltf1Insel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iomux_altf1_insel_1::R`](R) reader structure"]
impl crate::Readable for IomuxAltf1Insel1Spec {}
#[doc = "`write(|w| ..)` method takes [`iomux_altf1_insel_1::W`](W) writer structure"]
impl crate::Writable for IomuxAltf1Insel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IOMUX_ALTF1_INSEL_1 to value 0"]
impl crate::Resettable for IomuxAltf1Insel1Spec {}
