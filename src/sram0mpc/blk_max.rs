#[doc = "Register `BLK_MAX` reader"]
pub type R = crate::R<BlkMaxSpec>;
#[doc = "Field `bit[3_0]` reader - Block size"]
pub type Bit3_0R = crate::FieldReader;
#[doc = "Field `bit[31]` reader - Initialization in progress"]
pub type Bit31R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Block size"]
    #[inline(always)]
    pub fn bit3_0(&self) -> Bit3_0R {
        Bit3_0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Initialization in progress"]
    #[inline(always)]
    pub fn bit31(&self) -> Bit31R {
        Bit31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Maximum value of block based index register\n\nYou can [`read`](crate::Reg::read) this register and get [`blk_max::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlkMaxSpec;
impl crate::RegisterSpec for BlkMaxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk_max::R`](R) reader structure"]
impl crate::Readable for BlkMaxSpec {}
#[doc = "`reset()` method sets BLK_MAX to value 0"]
impl crate::Resettable for BlkMaxSpec {}
