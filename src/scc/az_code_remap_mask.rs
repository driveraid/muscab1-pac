#[doc = "Register `AZ_CODE_REMAP_MASK` reader"]
pub type R = crate::R<AzCodeRemapMaskSpec>;
#[doc = "Register `AZ_CODE_REMAP_MASK` writer"]
pub type W = crate::W<AzCodeRemapMaskSpec>;
#[doc = "Field `az_code_remap_mask` reader - Alcatraz code remap mask"]
pub type AzCodeRemapMaskR = crate::FieldReader<u32>;
#[doc = "Field `az_code_remap_mask` writer - Alcatraz code remap mask"]
pub type AzCodeRemapMaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Alcatraz code remap mask"]
    #[inline(always)]
    pub fn az_code_remap_mask(&self) -> AzCodeRemapMaskR {
        AzCodeRemapMaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Alcatraz code remap mask"]
    #[inline(always)]
    pub fn az_code_remap_mask(&mut self) -> AzCodeRemapMaskW<AzCodeRemapMaskSpec> {
        AzCodeRemapMaskW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`az_code_remap_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`az_code_remap_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AzCodeRemapMaskSpec;
impl crate::RegisterSpec for AzCodeRemapMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`az_code_remap_mask::R`](R) reader structure"]
impl crate::Readable for AzCodeRemapMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`az_code_remap_mask::W`](W) writer structure"]
impl crate::Writable for AzCodeRemapMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AZ_CODE_REMAP_MASK to value 0x00ff_ffff"]
impl crate::Resettable for AzCodeRemapMaskSpec {
    const RESET_VALUE: u32 = 0x00ff_ffff;
}
