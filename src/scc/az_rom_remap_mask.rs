#[doc = "Register `AZ_ROM_REMAP_MASK` reader"]
pub type R = crate::R<AzRomRemapMaskSpec>;
#[doc = "Register `AZ_ROM_REMAP_MASK` writer"]
pub type W = crate::W<AzRomRemapMaskSpec>;
#[doc = "Field `az_rom_remap_mask` reader - Alcatraz ROM remap mask"]
pub type AzRomRemapMaskR = crate::FieldReader<u32>;
#[doc = "Field `az_rom_remap_mask` writer - Alcatraz ROM remap mask"]
pub type AzRomRemapMaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Alcatraz ROM remap mask"]
    #[inline(always)]
    pub fn az_rom_remap_mask(&self) -> AzRomRemapMaskR {
        AzRomRemapMaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Alcatraz ROM remap mask"]
    #[inline(always)]
    pub fn az_rom_remap_mask(&mut self) -> AzRomRemapMaskW<AzRomRemapMaskSpec> {
        AzRomRemapMaskW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`az_rom_remap_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`az_rom_remap_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AzRomRemapMaskSpec;
impl crate::RegisterSpec for AzRomRemapMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`az_rom_remap_mask::R`](R) reader structure"]
impl crate::Readable for AzRomRemapMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`az_rom_remap_mask::W`](W) writer structure"]
impl crate::Writable for AzRomRemapMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AZ_ROM_REMAP_MASK to value 0x0001_ffff"]
impl crate::Resettable for AzRomRemapMaskSpec {
    const RESET_VALUE: u32 = 0x0001_ffff;
}
