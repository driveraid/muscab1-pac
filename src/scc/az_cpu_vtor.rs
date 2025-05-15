#[doc = "Register `AZ_CPU_VTOR` reader"]
pub type R = crate::R<AzCpuVtorSpec>;
#[doc = "Register `AZ_CPU_VTOR` writer"]
pub type W = crate::W<AzCpuVtorSpec>;
#[doc = "Field `AZ_ROM_REMAP` reader - Remap vector for Alcatraz ROM address space."]
pub type AzRomRemapR = crate::FieldReader;
#[doc = "Field `AZ_ROM_REMAP` writer - Remap vector for Alcatraz ROM address space."]
pub type AzRomRemapW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AZ_CODE_REMAP` reader - Remap vector for Alcatraz Code address space"]
pub type AzCodeRemapR = crate::FieldReader;
#[doc = "Field `AZ_CODE_REMAP` writer - Remap vector for Alcatraz Code address space"]
pub type AzCodeRemapW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AZ_SYS_REMAP` reader - Remap vector for Alcatraz System address space"]
pub type AzSysRemapR = crate::FieldReader;
#[doc = "Field `AZ_SYS_REMAP` writer - Remap vector for Alcatraz System address space"]
pub type AzSysRemapW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Remap vector for Alcatraz ROM address space."]
    #[inline(always)]
    pub fn az_rom_remap(&self) -> AzRomRemapR {
        AzRomRemapR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Remap vector for Alcatraz Code address space"]
    #[inline(always)]
    pub fn az_code_remap(&self) -> AzCodeRemapR {
        AzCodeRemapR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Remap vector for Alcatraz System address space"]
    #[inline(always)]
    pub fn az_sys_remap(&self) -> AzSysRemapR {
        AzSysRemapR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Remap vector for Alcatraz ROM address space."]
    #[inline(always)]
    pub fn az_rom_remap(&mut self) -> AzRomRemapW<AzCpuVtorSpec> {
        AzRomRemapW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Remap vector for Alcatraz Code address space"]
    #[inline(always)]
    pub fn az_code_remap(&mut self) -> AzCodeRemapW<AzCpuVtorSpec> {
        AzCodeRemapW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Remap vector for Alcatraz System address space"]
    #[inline(always)]
    pub fn az_sys_remap(&mut self) -> AzSysRemapW<AzCpuVtorSpec> {
        AzSysRemapW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`az_cpu_vtor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`az_cpu_vtor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AzCpuVtorSpec;
impl crate::RegisterSpec for AzCpuVtorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`az_cpu_vtor::R`](R) reader structure"]
impl crate::Readable for AzCpuVtorSpec {}
#[doc = "`write(|w| ..)` method takes [`az_cpu_vtor::W`](W) writer structure"]
impl crate::Writable for AzCpuVtorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AZ_CPU_VTOR to value 0x00a0_3800"]
impl crate::Resettable for AzCpuVtorSpec {
    const RESET_VALUE: u32 = 0x00a0_3800;
}
