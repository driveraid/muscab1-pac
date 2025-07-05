#[doc = "Register `CPU1_VTOR` reader"]
pub type R = crate::R<Cpu1VtorSpec>;
#[doc = "Register `CPU1_VTOR` writer"]
pub type W = crate::W<Cpu1VtorSpec>;
#[doc = "Field `CPU1_VTOR_SECURE` reader - Reset vector for CPU1 secure mode"]
pub type Cpu1VtorSecureR = crate::FieldReader<u32>;
#[doc = "Field `CPU1_VTOR_SECURE` writer - Reset vector for CPU1 secure mode"]
pub type Cpu1VtorSecureW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 7:31 - Reset vector for CPU1 secure mode"]
    #[inline(always)]
    pub fn cpu1_vtor_secure(&self) -> Cpu1VtorSecureR {
        Cpu1VtorSecureR::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 7:31 - Reset vector for CPU1 secure mode"]
    #[inline(always)]
    pub fn cpu1_vtor_secure(&mut self) -> Cpu1VtorSecureW<Cpu1VtorSpec> {
        Cpu1VtorSecureW::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu1_vtor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1_vtor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpu1VtorSpec;
impl crate::RegisterSpec for Cpu1VtorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu1_vtor::R`](R) reader structure"]
impl crate::Readable for Cpu1VtorSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu1_vtor::W`](W) writer structure"]
impl crate::Writable for Cpu1VtorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPU1_VTOR to value 0x1a40_0000"]
impl crate::Resettable for Cpu1VtorSpec {
    const RESET_VALUE: u32 = 0x1a40_0000;
}
