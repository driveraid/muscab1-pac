#[doc = "Register `INITSVRTOR0` reader"]
pub type R = crate::R<Initsvrtor0Spec>;
#[doc = "Register `INITSVRTOR0` writer"]
pub type W = crate::W<Initsvrtor0Spec>;
#[doc = "Field `INITSVTOR0` reader - Default Secure Vector table offset at reset for CPU 0"]
pub type Initsvtor0R = crate::FieldReader<u32>;
#[doc = "Field `INITSVTOR0` writer - Default Secure Vector table offset at reset for CPU 0"]
pub type Initsvtor0W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 7:31 - Default Secure Vector table offset at reset for CPU 0"]
    #[inline(always)]
    pub fn initsvtor0(&self) -> Initsvtor0R {
        Initsvtor0R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 7:31 - Default Secure Vector table offset at reset for CPU 0"]
    #[inline(always)]
    pub fn initsvtor0(&mut self) -> Initsvtor0W<Initsvrtor0Spec> {
        Initsvtor0W::new(self, 7)
    }
}
#[doc = "Initial Secure Reset Vector Register For CPU 0\n\nYou can [`read`](crate::Reg::read) this register and get [`initsvrtor0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`initsvrtor0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Initsvrtor0Spec;
impl crate::RegisterSpec for Initsvrtor0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`initsvrtor0::R`](R) reader structure"]
impl crate::Readable for Initsvrtor0Spec {}
#[doc = "`write(|w| ..)` method takes [`initsvrtor0::W`](W) writer structure"]
impl crate::Writable for Initsvrtor0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INITSVRTOR0 to value 0"]
impl crate::Resettable for Initsvrtor0Spec {
    const RESET_VALUE: u32 = 0;
}
