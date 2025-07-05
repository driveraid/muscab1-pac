#[doc = "Register `FLASH0_DOUT_3` reader"]
pub type R = crate::R<Flash0Dout3Spec>;
#[doc = "Field `scc_flash0_dout3` reader - eFlash 0 data output\\[127:96\\]"]
pub type SccFlash0Dout3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - eFlash 0 data output\\[127:96\\]"]
    #[inline(always)]
    pub fn scc_flash0_dout3(&self) -> SccFlash0Dout3R {
        SccFlash0Dout3R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`flash0_dout_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Flash0Dout3Spec;
impl crate::RegisterSpec for Flash0Dout3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash0_dout_3::R`](R) reader structure"]
impl crate::Readable for Flash0Dout3Spec {}
#[doc = "`reset()` method sets FLASH0_DOUT_3 to value 0xffff_ffff"]
impl crate::Resettable for Flash0Dout3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
