#[doc = "Register `SPARE_CTRL0` reader"]
pub type R = crate::R<SpareCtrl0Spec>;
#[doc = "Register `SPARE_CTRL0` writer"]
pub type W = crate::W<SpareCtrl0Spec>;
#[doc = "Field `spare_ctrl0` reader - Spare control register"]
pub type SpareCtrl0R = crate::FieldReader<u32>;
#[doc = "Field `spare_ctrl0` writer - Spare control register"]
pub type SpareCtrl0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Spare control register"]
    #[inline(always)]
    pub fn spare_ctrl0(&self) -> SpareCtrl0R {
        SpareCtrl0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Spare control register"]
    #[inline(always)]
    pub fn spare_ctrl0(&mut self) -> SpareCtrl0W<SpareCtrl0Spec> {
        SpareCtrl0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spare_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spare_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpareCtrl0Spec;
impl crate::RegisterSpec for SpareCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spare_ctrl0::R`](R) reader structure"]
impl crate::Readable for SpareCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`spare_ctrl0::W`](W) writer structure"]
impl crate::Writable for SpareCtrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPARE_CTRL0 to value 0"]
impl crate::Resettable for SpareCtrl0Spec {
    const RESET_VALUE: u32 = 0;
}
