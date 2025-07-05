#[doc = "Register `AZ_CODE_REMAP_OFFSET` reader"]
pub type R = crate::R<AzCodeRemapOffsetSpec>;
#[doc = "Register `AZ_CODE_REMAP_OFFSET` writer"]
pub type W = crate::W<AzCodeRemapOffsetSpec>;
#[doc = "Field `az_code_remap_offset` reader - Alcatraz code remap offset"]
pub type AzCodeRemapOffsetR = crate::FieldReader<u32>;
#[doc = "Field `az_code_remap_offset` writer - Alcatraz code remap offset"]
pub type AzCodeRemapOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Alcatraz code remap offset"]
    #[inline(always)]
    pub fn az_code_remap_offset(&self) -> AzCodeRemapOffsetR {
        AzCodeRemapOffsetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Alcatraz code remap offset"]
    #[inline(always)]
    pub fn az_code_remap_offset(&mut self) -> AzCodeRemapOffsetW<AzCodeRemapOffsetSpec> {
        AzCodeRemapOffsetW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`az_code_remap_offset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`az_code_remap_offset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AzCodeRemapOffsetSpec;
impl crate::RegisterSpec for AzCodeRemapOffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`az_code_remap_offset::R`](R) reader structure"]
impl crate::Readable for AzCodeRemapOffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`az_code_remap_offset::W`](W) writer structure"]
impl crate::Writable for AzCodeRemapOffsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AZ_CODE_REMAP_OFFSET to value 0"]
impl crate::Resettable for AzCodeRemapOffsetSpec {
    const RESET_VALUE: u32 = 0;
}
