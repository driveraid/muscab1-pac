#[doc = "Register `AZ_SYS_REMAP_OFFSET` reader"]
pub type R = crate::R<AzSysRemapOffsetSpec>;
#[doc = "Register `AZ_SYS_REMAP_OFFSET` writer"]
pub type W = crate::W<AzSysRemapOffsetSpec>;
#[doc = "Field `az_sys_remap_offset` reader - Alcatraz system remap offset"]
pub type AzSysRemapOffsetR = crate::FieldReader<u32>;
#[doc = "Field `az_sys_remap_offset` writer - Alcatraz system remap offset"]
pub type AzSysRemapOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Alcatraz system remap offset"]
    #[inline(always)]
    pub fn az_sys_remap_offset(&self) -> AzSysRemapOffsetR {
        AzSysRemapOffsetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Alcatraz system remap offset"]
    #[inline(always)]
    pub fn az_sys_remap_offset(&mut self) -> AzSysRemapOffsetW<AzSysRemapOffsetSpec> {
        AzSysRemapOffsetW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`az_sys_remap_offset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`az_sys_remap_offset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AzSysRemapOffsetSpec;
impl crate::RegisterSpec for AzSysRemapOffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`az_sys_remap_offset::R`](R) reader structure"]
impl crate::Readable for AzSysRemapOffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`az_sys_remap_offset::W`](W) writer structure"]
impl crate::Writable for AzSysRemapOffsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AZ_SYS_REMAP_OFFSET to value 0x4001_0000"]
impl crate::Resettable for AzSysRemapOffsetSpec {
    const RESET_VALUE: u32 = 0x4001_0000;
}
