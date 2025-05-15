#[doc = "Register `SECPPCINTSTAT` reader"]
pub type R = crate::R<SecppcintstatSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Secure PPC Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`secppcintstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecppcintstatSpec;
impl crate::RegisterSpec for SecppcintstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secppcintstat::R`](R) reader structure"]
impl crate::Readable for SecppcintstatSpec {}
#[doc = "`reset()` method sets SECPPCINTSTAT to value 0"]
impl crate::Resettable for SecppcintstatSpec {
    const RESET_VALUE: u32 = 0;
}
