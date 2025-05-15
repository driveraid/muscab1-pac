#[doc = "Register `SECMSCINTCLR` reader"]
pub type R = crate::R<SecmscintclrSpec>;
#[doc = "Register `SECMSCINTCLR` writer"]
pub type W = crate::W<SecmscintclrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Secure MSC Interrupt Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`secmscintclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secmscintclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecmscintclrSpec;
impl crate::RegisterSpec for SecmscintclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secmscintclr::R`](R) reader structure"]
impl crate::Readable for SecmscintclrSpec {}
#[doc = "`write(|w| ..)` method takes [`secmscintclr::W`](W) writer structure"]
impl crate::Writable for SecmscintclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECMSCINTCLR to value 0"]
impl crate::Resettable for SecmscintclrSpec {}
