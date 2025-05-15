#[doc = "Register `IOMUX_ALTF2_INSEL_1` reader"]
pub type R = crate::R<IomuxAltf2Insel1Spec>;
#[doc = "Register `IOMUX_ALTF2_INSEL_1` writer"]
pub type W = crate::W<IomuxAltf2Insel1Spec>;
#[doc = "Field `iomux_altf2_insel_1` reader - 0: Select ATF3 1: Select ATF2"]
pub type IomuxAltf2Insel1R = crate::FieldReader;
#[doc = "Field `iomux_altf2_insel_1` writer - 0: Select ATF3 1: Select ATF2"]
pub type IomuxAltf2Insel1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 0: Select ATF3 1: Select ATF2"]
    #[inline(always)]
    pub fn iomux_altf2_insel_1(&self) -> IomuxAltf2Insel1R {
        IomuxAltf2Insel1R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 0: Select ATF3 1: Select ATF2"]
    #[inline(always)]
    pub fn iomux_altf2_insel_1(&mut self) -> IomuxAltf2Insel1W<IomuxAltf2Insel1Spec> {
        IomuxAltf2Insel1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_altf2_insel_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_altf2_insel_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IomuxAltf2Insel1Spec;
impl crate::RegisterSpec for IomuxAltf2Insel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iomux_altf2_insel_1::R`](R) reader structure"]
impl crate::Readable for IomuxAltf2Insel1Spec {}
#[doc = "`write(|w| ..)` method takes [`iomux_altf2_insel_1::W`](W) writer structure"]
impl crate::Writable for IomuxAltf2Insel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IOMUX_ALTF2_INSEL_1 to value 0"]
impl crate::Resettable for IomuxAltf2Insel1Spec {}
