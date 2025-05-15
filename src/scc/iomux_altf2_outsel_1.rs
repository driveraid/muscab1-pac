#[doc = "Register `IOMUX_ALTF2_OUTSEL_1` reader"]
pub type R = crate::R<IomuxAltf2Outsel1Spec>;
#[doc = "Register `IOMUX_ALTF2_OUTSEL_1` writer"]
pub type W = crate::W<IomuxAltf2Outsel1Spec>;
#[doc = "Field `iomux_altf2_outsel_1` reader - 0: Select ATF3 1: Select ATF2"]
pub type IomuxAltf2Outsel1R = crate::FieldReader;
#[doc = "Field `iomux_altf2_outsel_1` writer - 0: Select ATF3 1: Select ATF2"]
pub type IomuxAltf2Outsel1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 0: Select ATF3 1: Select ATF2"]
    #[inline(always)]
    pub fn iomux_altf2_outsel_1(&self) -> IomuxAltf2Outsel1R {
        IomuxAltf2Outsel1R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 0: Select ATF3 1: Select ATF2"]
    #[inline(always)]
    pub fn iomux_altf2_outsel_1(&mut self) -> IomuxAltf2Outsel1W<IomuxAltf2Outsel1Spec> {
        IomuxAltf2Outsel1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_altf2_outsel_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_altf2_outsel_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IomuxAltf2Outsel1Spec;
impl crate::RegisterSpec for IomuxAltf2Outsel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iomux_altf2_outsel_1::R`](R) reader structure"]
impl crate::Readable for IomuxAltf2Outsel1Spec {}
#[doc = "`write(|w| ..)` method takes [`iomux_altf2_outsel_1::W`](W) writer structure"]
impl crate::Writable for IomuxAltf2Outsel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IOMUX_ALTF2_OUTSEL_1 to value 0xffff_ffff"]
impl crate::Resettable for IomuxAltf2Outsel1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
