#[doc = "Register `SECMSCINTSTAT` reader"]
pub type R = crate::R<SecmscintstatSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Secure MSC Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`secmscintstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecmscintstatSpec;
impl crate::RegisterSpec for SecmscintstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secmscintstat::R`](R) reader structure"]
impl crate::Readable for SecmscintstatSpec {}
#[doc = "`reset()` method sets SECMSCINTSTAT to value 0"]
impl crate::Resettable for SecmscintstatSpec {
    const RESET_VALUE: u32 = 0;
}
