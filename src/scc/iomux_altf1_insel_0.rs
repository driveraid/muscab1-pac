#[doc = "Register `IOMUX_ALTF1_INSEL_0` reader"]
pub type R = crate::R<IomuxAltf1Insel0Spec>;
#[doc = "Register `IOMUX_ALTF1_INSEL_0` writer"]
pub type W = crate::W<IomuxAltf1Insel0Spec>;
#[doc = "Field `iomux_altf1_insel_0` reader - 0: Select ATF2 1: Select ATF1"]
pub type IomuxAltf1Insel0R = crate::FieldReader<u32>;
#[doc = "Field `iomux_altf1_insel_0` writer - 0: Select ATF2 1: Select ATF1"]
pub type IomuxAltf1Insel0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 0: Select ATF2 1: Select ATF1"]
    #[inline(always)]
    pub fn iomux_altf1_insel_0(&self) -> IomuxAltf1Insel0R {
        IomuxAltf1Insel0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 0: Select ATF2 1: Select ATF1"]
    #[inline(always)]
    pub fn iomux_altf1_insel_0(&mut self) -> IomuxAltf1Insel0W<IomuxAltf1Insel0Spec> {
        IomuxAltf1Insel0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_altf1_insel_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_altf1_insel_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IomuxAltf1Insel0Spec;
impl crate::RegisterSpec for IomuxAltf1Insel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iomux_altf1_insel_0::R`](R) reader structure"]
impl crate::Readable for IomuxAltf1Insel0Spec {}
#[doc = "`write(|w| ..)` method takes [`iomux_altf1_insel_0::W`](W) writer structure"]
impl crate::Writable for IomuxAltf1Insel0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IOMUX_ALTF1_INSEL_0 to value 0"]
impl crate::Resettable for IomuxAltf1Insel0Spec {}
