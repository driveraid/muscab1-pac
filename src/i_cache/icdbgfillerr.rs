#[doc = "Register `ICDBGFILLERR` reader"]
pub type R = crate::R<IcdbgfillerrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Address where the latest fill error was seen\n\nYou can [`read`](crate::Reg::read) this register and get [`icdbgfillerr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcdbgfillerrSpec;
impl crate::RegisterSpec for IcdbgfillerrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icdbgfillerr::R`](R) reader structure"]
impl crate::Readable for IcdbgfillerrSpec {}
#[doc = "`reset()` method sets ICDBGFILLERR to value 0"]
impl crate::Resettable for IcdbgfillerrSpec {
    const RESET_VALUE: u32 = 0;
}
