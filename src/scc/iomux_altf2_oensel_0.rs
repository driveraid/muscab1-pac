#[doc = "Register `IOMUX_ALTF2_OENSEL_0` reader"]
pub type R = crate::R<IomuxAltf2Oensel0Spec>;
#[doc = "Register `IOMUX_ALTF2_OENSEL_0` writer"]
pub type W = crate::W<IomuxAltf2Oensel0Spec>;
#[doc = "Field `iomux_altf2_oensel_0` reader - 0: Select ATF3 1: Select ATF2"]
pub type IomuxAltf2Oensel0R = crate::FieldReader<u32>;
#[doc = "Field `iomux_altf2_oensel_0` writer - 0: Select ATF3 1: Select ATF2"]
pub type IomuxAltf2Oensel0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 0: Select ATF3 1: Select ATF2"]
    #[inline(always)]
    pub fn iomux_altf2_oensel_0(&self) -> IomuxAltf2Oensel0R {
        IomuxAltf2Oensel0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 0: Select ATF3 1: Select ATF2"]
    #[inline(always)]
    pub fn iomux_altf2_oensel_0(&mut self) -> IomuxAltf2Oensel0W<IomuxAltf2Oensel0Spec> {
        IomuxAltf2Oensel0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_altf2_oensel_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_altf2_oensel_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IomuxAltf2Oensel0Spec;
impl crate::RegisterSpec for IomuxAltf2Oensel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iomux_altf2_oensel_0::R`](R) reader structure"]
impl crate::Readable for IomuxAltf2Oensel0Spec {}
#[doc = "`write(|w| ..)` method takes [`iomux_altf2_oensel_0::W`](W) writer structure"]
impl crate::Writable for IomuxAltf2Oensel0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOMUX_ALTF2_OENSEL_0 to value 0xffff_ffff"]
impl crate::Resettable for IomuxAltf2Oensel0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
