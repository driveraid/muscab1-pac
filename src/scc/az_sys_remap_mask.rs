#[doc = "Register `AZ_SYS_REMAP_MASK` reader"]
pub type R = crate::R<AzSysRemapMaskSpec>;
#[doc = "Register `AZ_SYS_REMAP_MASK` writer"]
pub type W = crate::W<AzSysRemapMaskSpec>;
#[doc = "Field `az_sys_remap_mask` reader - Alcatraz system remap mask"]
pub type AzSysRemapMaskR = crate::FieldReader<u32>;
#[doc = "Field `az_sys_remap_mask` writer - Alcatraz system remap mask"]
pub type AzSysRemapMaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Alcatraz system remap mask"]
    #[inline(always)]
    pub fn az_sys_remap_mask(&self) -> AzSysRemapMaskR {
        AzSysRemapMaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Alcatraz system remap mask"]
    #[inline(always)]
    pub fn az_sys_remap_mask(&mut self) -> AzSysRemapMaskW<AzSysRemapMaskSpec> {
        AzSysRemapMaskW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`az_sys_remap_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`az_sys_remap_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AzSysRemapMaskSpec;
impl crate::RegisterSpec for AzSysRemapMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`az_sys_remap_mask::R`](R) reader structure"]
impl crate::Readable for AzSysRemapMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`az_sys_remap_mask::W`](W) writer structure"]
impl crate::Writable for AzSysRemapMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AZ_SYS_REMAP_MASK to value 0x0003_ffff"]
impl crate::Resettable for AzSysRemapMaskSpec {
    const RESET_VALUE: u32 = 0x0003_ffff;
}
