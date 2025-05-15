#[doc = "Register `BLK_CFG` reader"]
pub type R = crate::R<BlkCfgSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Block Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`blk_cfg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlkCfgSpec;
impl crate::RegisterSpec for BlkCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk_cfg::R`](R) reader structure"]
impl crate::Readable for BlkCfgSpec {}
#[doc = "`reset()` method sets BLK_CFG to value 0"]
impl crate::Resettable for BlkCfgSpec {}
