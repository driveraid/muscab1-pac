#[doc = "Register `GRETREG` reader"]
pub type R = crate::R<GretregSpec>;
#[doc = "Register `GRETREG` writer"]
pub type W = crate::W<GretregSpec>;
#[doc = "Field `GRETREG` reader - General Purpose Retention Register"]
pub type GretregR = crate::FieldReader<u16>;
#[doc = "Field `GRETREG` writer - General Purpose Retention Register"]
pub type GretregW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - General Purpose Retention Register"]
    #[inline(always)]
    pub fn gretreg(&self) -> GretregR {
        GretregR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - General Purpose Retention Register"]
    #[inline(always)]
    pub fn gretreg(&mut self) -> GretregW<GretregSpec> {
        GretregW::new(self, 0)
    }
}
#[doc = "General Purpose Retention\n\nYou can [`read`](crate::Reg::read) this register and get [`gretreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gretreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GretregSpec;
impl crate::RegisterSpec for GretregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gretreg::R`](R) reader structure"]
impl crate::Readable for GretregSpec {}
#[doc = "`write(|w| ..)` method takes [`gretreg::W`](W) writer structure"]
impl crate::Writable for GretregSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRETREG to value 0"]
impl crate::Resettable for GretregSpec {
    const RESET_VALUE: u32 = 0;
}
