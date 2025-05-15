#[doc = "Register `SECMPCINTSTATUS` reader"]
pub type R = crate::R<SecmpcintstatusSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Secure MPC Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`secmpcintstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecmpcintstatusSpec;
impl crate::RegisterSpec for SecmpcintstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secmpcintstatus::R`](R) reader structure"]
impl crate::Readable for SecmpcintstatusSpec {}
#[doc = "`reset()` method sets SECMPCINTSTATUS to value 0"]
impl crate::Resettable for SecmpcintstatusSpec {}
