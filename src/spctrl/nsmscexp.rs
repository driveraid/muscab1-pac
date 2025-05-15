#[doc = "Register `NSMSCEXP` reader"]
pub type R = crate::R<NsmscexpSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Expansion MSC Non-Secure Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`nsmscexp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NsmscexpSpec;
impl crate::RegisterSpec for NsmscexpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nsmscexp::R`](R) reader structure"]
impl crate::Readable for NsmscexpSpec {}
#[doc = "`reset()` method sets NSMSCEXP to value 0"]
impl crate::Resettable for NsmscexpSpec {}
