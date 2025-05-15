#[doc = "Register `SPARE0` reader"]
pub type R = crate::R<Spare0Spec>;
#[doc = "Register `SPARE0` writer"]
pub type W = crate::W<Spare0Spec>;
#[doc = "Field `spare0` reader - Spare read-write register for software"]
pub type Spare0R = crate::FieldReader<u32>;
#[doc = "Field `spare0` writer - Spare read-write register for software"]
pub type Spare0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Spare read-write register for software"]
    #[inline(always)]
    pub fn spare0(&self) -> Spare0R {
        Spare0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Spare read-write register for software"]
    #[inline(always)]
    pub fn spare0(&mut self) -> Spare0W<Spare0Spec> {
        Spare0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spare0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spare0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spare0Spec;
impl crate::RegisterSpec for Spare0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spare0::R`](R) reader structure"]
impl crate::Readable for Spare0Spec {}
#[doc = "`write(|w| ..)` method takes [`spare0::W`](W) writer structure"]
impl crate::Writable for Spare0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPARE0 to value 0"]
impl crate::Resettable for Spare0Spec {
    const RESET_VALUE: u32 = 0;
}
