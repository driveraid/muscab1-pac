#[doc = "Register `FLASH0_DOUT_1` reader"]
pub type R = crate::R<Flash0Dout1Spec>;
#[doc = "Field `scc_flash0_dout1` reader - eFlash 0 data output\\[63:32\\]"]
pub type SccFlash0Dout1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - eFlash 0 data output\\[63:32\\]"]
    #[inline(always)]
    pub fn scc_flash0_dout1(&self) -> SccFlash0Dout1R {
        SccFlash0Dout1R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`flash0_dout_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Flash0Dout1Spec;
impl crate::RegisterSpec for Flash0Dout1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash0_dout_1::R`](R) reader structure"]
impl crate::Readable for Flash0Dout1Spec {}
#[doc = "`reset()` method sets FLASH0_DOUT_1 to value 0xffff_ffff"]
impl crate::Resettable for Flash0Dout1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
