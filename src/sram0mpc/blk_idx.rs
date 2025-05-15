#[doc = "Register `BLK_IDX` reader"]
pub type R = crate::R<BlkIdxSpec>;
#[doc = "Register `BLK_IDX` writer"]
pub type W = crate::W<BlkIdxSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Index value for accessing block based look up table\n\nYou can [`read`](crate::Reg::read) this register and get [`blk_idx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk_idx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlkIdxSpec;
impl crate::RegisterSpec for BlkIdxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk_idx::R`](R) reader structure"]
impl crate::Readable for BlkIdxSpec {}
#[doc = "`write(|w| ..)` method takes [`blk_idx::W`](W) writer structure"]
impl crate::Writable for BlkIdxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLK_IDX to value 0"]
impl crate::Resettable for BlkIdxSpec {}
