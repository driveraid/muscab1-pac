#[doc = "Register `SECPPCINTEN` reader"]
pub type R = crate::R<SecppcintenSpec>;
#[doc = "Register `SECPPCINTEN` writer"]
pub type W = crate::W<SecppcintenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Secure PPC Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`secppcinten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secppcinten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecppcintenSpec;
impl crate::RegisterSpec for SecppcintenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secppcinten::R`](R) reader structure"]
impl crate::Readable for SecppcintenSpec {}
#[doc = "`write(|w| ..)` method takes [`secppcinten::W`](W) writer structure"]
impl crate::Writable for SecppcintenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECPPCINTEN to value 0"]
impl crate::Resettable for SecppcintenSpec {
    const RESET_VALUE: u32 = 0;
}
