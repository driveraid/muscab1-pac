#[doc = "Register `INITSVRTOR1` reader"]
pub type R = crate::R<Initsvrtor1Spec>;
#[doc = "Register `INITSVRTOR1` writer"]
pub type W = crate::W<Initsvrtor1Spec>;
#[doc = "Field `INITSVTOR1` reader - Default Secure Vector table offset at reset for CPU 1"]
pub type Initsvtor1R = crate::FieldReader<u32>;
#[doc = "Field `INITSVTOR1` writer - Default Secure Vector table offset at reset for CPU 1"]
pub type Initsvtor1W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 7:31 - Default Secure Vector table offset at reset for CPU 1"]
    #[inline(always)]
    pub fn initsvtor1(&self) -> Initsvtor1R {
        Initsvtor1R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 7:31 - Default Secure Vector table offset at reset for CPU 1"]
    #[inline(always)]
    pub fn initsvtor1(&mut self) -> Initsvtor1W<Initsvrtor1Spec> {
        Initsvtor1W::new(self, 7)
    }
}
#[doc = "Initial Secure Reset Vector Register For CPU 1\n\nYou can [`read`](crate::Reg::read) this register and get [`initsvrtor1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`initsvrtor1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Initsvrtor1Spec;
impl crate::RegisterSpec for Initsvrtor1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`initsvrtor1::R`](R) reader structure"]
impl crate::Readable for Initsvrtor1Spec {}
#[doc = "`write(|w| ..)` method takes [`initsvrtor1::W`](W) writer structure"]
impl crate::Writable for Initsvrtor1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INITSVRTOR1 to value 0"]
impl crate::Resettable for Initsvrtor1Spec {
    const RESET_VALUE: u32 = 0;
}
