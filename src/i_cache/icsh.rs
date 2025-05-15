#[doc = "Register `ICSH` reader"]
pub type R = crate::R<IcshSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Instruction Cache Statistic Hit Count register\n\nYou can [`read`](crate::Reg::read) this register and get [`icsh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcshSpec;
impl crate::RegisterSpec for IcshSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icsh::R`](R) reader structure"]
impl crate::Readable for IcshSpec {}
#[doc = "`reset()` method sets ICSH to value 0"]
impl crate::Resettable for IcshSpec {
    const RESET_VALUE: u32 = 0;
}
