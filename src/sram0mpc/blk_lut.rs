#[doc = "Register `BLK_LUT` reader"]
pub type R = crate::R<BlkLutSpec>;
#[doc = "Register `BLK_LUT` writer"]
pub type W = crate::W<BlkLutSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Block based gating Look Up Table\n\nYou can [`read`](crate::Reg::read) this register and get [`blk_lut::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk_lut::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlkLutSpec;
impl crate::RegisterSpec for BlkLutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk_lut::R`](R) reader structure"]
impl crate::Readable for BlkLutSpec {}
#[doc = "`write(|w| ..)` method takes [`blk_lut::W`](W) writer structure"]
impl crate::Writable for BlkLutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLK_LUT to value 0"]
impl crate::Resettable for BlkLutSpec {
    const RESET_VALUE: u32 = 0;
}
