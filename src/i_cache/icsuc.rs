#[doc = "Register `ICSUC` reader"]
pub type R = crate::R<IcsucSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Instruction Cache Statistic Uncached Count register\n\nYou can [`read`](crate::Reg::read) this register and get [`icsuc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcsucSpec;
impl crate::RegisterSpec for IcsucSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icsuc::R`](R) reader structure"]
impl crate::Readable for IcsucSpec {}
#[doc = "`reset()` method sets ICSUC to value 0"]
impl crate::Resettable for IcsucSpec {
    const RESET_VALUE: u32 = 0;
}
