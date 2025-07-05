#[doc = "Register `IOMUX_ALTF1_OENSEL_0` reader"]
pub type R = crate::R<IomuxAltf1Oensel0Spec>;
#[doc = "Register `IOMUX_ALTF1_OENSEL_0` writer"]
pub type W = crate::W<IomuxAltf1Oensel0Spec>;
#[doc = "Field `iomux_altf1_oensel_0` reader - 0: Select ATF2 1: Select ATF1"]
pub type IomuxAltf1Oensel0R = crate::FieldReader<u32>;
#[doc = "Field `iomux_altf1_oensel_0` writer - 0: Select ATF2 1: Select ATF1"]
pub type IomuxAltf1Oensel0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 0: Select ATF2 1: Select ATF1"]
    #[inline(always)]
    pub fn iomux_altf1_oensel_0(&self) -> IomuxAltf1Oensel0R {
        IomuxAltf1Oensel0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 0: Select ATF2 1: Select ATF1"]
    #[inline(always)]
    pub fn iomux_altf1_oensel_0(&mut self) -> IomuxAltf1Oensel0W<IomuxAltf1Oensel0Spec> {
        IomuxAltf1Oensel0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_altf1_oensel_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_altf1_oensel_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IomuxAltf1Oensel0Spec;
impl crate::RegisterSpec for IomuxAltf1Oensel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iomux_altf1_oensel_0::R`](R) reader structure"]
impl crate::Readable for IomuxAltf1Oensel0Spec {}
#[doc = "`write(|w| ..)` method takes [`iomux_altf1_oensel_0::W`](W) writer structure"]
impl crate::Writable for IomuxAltf1Oensel0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOMUX_ALTF1_OENSEL_0 to value 0xffff_ffff"]
impl crate::Resettable for IomuxAltf1Oensel0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
