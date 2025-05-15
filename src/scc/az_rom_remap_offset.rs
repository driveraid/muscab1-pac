#[doc = "Register `AZ_ROM_REMAP_OFFSET` reader"]
pub type R = crate::R<AzRomRemapOffsetSpec>;
#[doc = "Register `AZ_ROM_REMAP_OFFSET` writer"]
pub type W = crate::W<AzRomRemapOffsetSpec>;
#[doc = "Field `az_rom_remap_offset` reader - Alcatraz ROM remap offset"]
pub type AzRomRemapOffsetR = crate::FieldReader<u32>;
#[doc = "Field `az_rom_remap_offset` writer - Alcatraz ROM remap offset"]
pub type AzRomRemapOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Alcatraz ROM remap offset"]
    #[inline(always)]
    pub fn az_rom_remap_offset(&self) -> AzRomRemapOffsetR {
        AzRomRemapOffsetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Alcatraz ROM remap offset"]
    #[inline(always)]
    pub fn az_rom_remap_offset(&mut self) -> AzRomRemapOffsetW<AzRomRemapOffsetSpec> {
        AzRomRemapOffsetW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`az_rom_remap_offset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`az_rom_remap_offset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AzRomRemapOffsetSpec;
impl crate::RegisterSpec for AzRomRemapOffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`az_rom_remap_offset::R`](R) reader structure"]
impl crate::Readable for AzRomRemapOffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`az_rom_remap_offset::W`](W) writer structure"]
impl crate::Writable for AzRomRemapOffsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AZ_ROM_REMAP_OFFSET to value 0x1a20_0000"]
impl crate::Resettable for AzRomRemapOffsetSpec {
    const RESET_VALUE: u32 = 0x1a20_0000;
}
