#[doc = "Register `SPARE_CTRL1` reader"]
pub type R = crate::R<SpareCtrl1Spec>;
#[doc = "Register `SPARE_CTRL1` writer"]
pub type W = crate::W<SpareCtrl1Spec>;
#[doc = "Field `spare_ctrl1` reader - Spare control register"]
pub type SpareCtrl1R = crate::FieldReader<u32>;
#[doc = "Field `spare_ctrl1` writer - Spare control register"]
pub type SpareCtrl1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Spare control register"]
    #[inline(always)]
    pub fn spare_ctrl1(&self) -> SpareCtrl1R {
        SpareCtrl1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Spare control register"]
    #[inline(always)]
    pub fn spare_ctrl1(&mut self) -> SpareCtrl1W<SpareCtrl1Spec> {
        SpareCtrl1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spare_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spare_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpareCtrl1Spec;
impl crate::RegisterSpec for SpareCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spare_ctrl1::R`](R) reader structure"]
impl crate::Readable for SpareCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`spare_ctrl1::W`](W) writer structure"]
impl crate::Writable for SpareCtrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPARE_CTRL1 to value 0"]
impl crate::Resettable for SpareCtrl1Spec {
    const RESET_VALUE: u32 = 0;
}
