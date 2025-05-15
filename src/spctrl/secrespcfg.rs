#[doc = "Register `SECRESPCFG` reader"]
pub type R = crate::R<SecrespcfgSpec>;
#[doc = "Register `SECRESPCFG` writer"]
pub type W = crate::W<SecrespcfgSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Security Violation Response Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`secrespcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secrespcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecrespcfgSpec;
impl crate::RegisterSpec for SecrespcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secrespcfg::R`](R) reader structure"]
impl crate::Readable for SecrespcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`secrespcfg::W`](W) writer structure"]
impl crate::Writable for SecrespcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECRESPCFG to value 0"]
impl crate::Resettable for SecrespcfgSpec {
    const RESET_VALUE: u32 = 0;
}
