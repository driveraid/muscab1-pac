#[doc = "Register `IOMUX_ALTF2_DEFAULT_IN_0` reader"]
pub type R = crate::R<IomuxAltf2DefaultIn0Spec>;
#[doc = "Register `IOMUX_ALTF2_DEFAULT_IN_0` writer"]
pub type W = crate::W<IomuxAltf2DefaultIn0Spec>;
#[doc = "Field `iomux_altf2_default_in_0` reader - 0: Default to 0 1: Default to 1"]
pub type IomuxAltf2DefaultIn0R = crate::FieldReader<u32>;
#[doc = "Field `iomux_altf2_default_in_0` writer - 0: Default to 0 1: Default to 1"]
pub type IomuxAltf2DefaultIn0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 0: Default to 0 1: Default to 1"]
    #[inline(always)]
    pub fn iomux_altf2_default_in_0(&self) -> IomuxAltf2DefaultIn0R {
        IomuxAltf2DefaultIn0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 0: Default to 0 1: Default to 1"]
    #[inline(always)]
    pub fn iomux_altf2_default_in_0(&mut self) -> IomuxAltf2DefaultIn0W<IomuxAltf2DefaultIn0Spec> {
        IomuxAltf2DefaultIn0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_altf2_default_in_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_altf2_default_in_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IomuxAltf2DefaultIn0Spec;
impl crate::RegisterSpec for IomuxAltf2DefaultIn0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iomux_altf2_default_in_0::R`](R) reader structure"]
impl crate::Readable for IomuxAltf2DefaultIn0Spec {}
#[doc = "`write(|w| ..)` method takes [`iomux_altf2_default_in_0::W`](W) writer structure"]
impl crate::Writable for IomuxAltf2DefaultIn0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOMUX_ALTF2_DEFAULT_IN_0 to value 0"]
impl crate::Resettable for IomuxAltf2DefaultIn0Spec {
    const RESET_VALUE: u32 = 0;
}
