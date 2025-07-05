#[doc = "Register `IOMUX_ALTF2_DEFAULT_IN_1` reader"]
pub type R = crate::R<IomuxAltf2DefaultIn1Spec>;
#[doc = "Register `IOMUX_ALTF2_DEFAULT_IN_1` writer"]
pub type W = crate::W<IomuxAltf2DefaultIn1Spec>;
#[doc = "Field `iomux_altf2_default_in_1` reader - 0: Default to 0 1: Default to 1"]
pub type IomuxAltf2DefaultIn1R = crate::FieldReader;
#[doc = "Field `iomux_altf2_default_in_1` writer - 0: Default to 0 1: Default to 1"]
pub type IomuxAltf2DefaultIn1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 0: Default to 0 1: Default to 1"]
    #[inline(always)]
    pub fn iomux_altf2_default_in_1(&self) -> IomuxAltf2DefaultIn1R {
        IomuxAltf2DefaultIn1R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 0: Default to 0 1: Default to 1"]
    #[inline(always)]
    pub fn iomux_altf2_default_in_1(&mut self) -> IomuxAltf2DefaultIn1W<IomuxAltf2DefaultIn1Spec> {
        IomuxAltf2DefaultIn1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_altf2_default_in_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_altf2_default_in_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IomuxAltf2DefaultIn1Spec;
impl crate::RegisterSpec for IomuxAltf2DefaultIn1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iomux_altf2_default_in_1::R`](R) reader structure"]
impl crate::Readable for IomuxAltf2DefaultIn1Spec {}
#[doc = "`write(|w| ..)` method takes [`iomux_altf2_default_in_1::W`](W) writer structure"]
impl crate::Writable for IomuxAltf2DefaultIn1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOMUX_ALTF2_DEFAULT_IN_1 to value 0"]
impl crate::Resettable for IomuxAltf2DefaultIn1Spec {
    const RESET_VALUE: u32 = 0;
}
