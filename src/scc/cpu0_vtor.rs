#[doc = "Register `CPU0_VTOR` reader"]
pub type R = crate::R<Cpu0VtorSpec>;
#[doc = "Register `CPU0_VTOR` writer"]
pub type W = crate::W<Cpu0VtorSpec>;
#[doc = "Field `CPU0_VTOR_SECURE` reader - Reset vector for CPU0 secure mode"]
pub type Cpu0VtorSecureR = crate::FieldReader<u32>;
#[doc = "Field `CPU0_VTOR_SECURE` writer - Reset vector for CPU0 secure mode"]
pub type Cpu0VtorSecureW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 7:31 - Reset vector for CPU0 secure mode"]
    #[inline(always)]
    pub fn cpu0_vtor_secure(&self) -> Cpu0VtorSecureR {
        Cpu0VtorSecureR::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 7:31 - Reset vector for CPU0 secure mode"]
    #[inline(always)]
    pub fn cpu0_vtor_secure(&mut self) -> Cpu0VtorSecureW<Cpu0VtorSpec> {
        Cpu0VtorSecureW::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0_vtor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0_vtor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpu0VtorSpec;
impl crate::RegisterSpec for Cpu0VtorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu0_vtor::R`](R) reader structure"]
impl crate::Readable for Cpu0VtorSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu0_vtor::W`](W) writer structure"]
impl crate::Writable for Cpu0VtorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU0_VTOR to value 0x1000_0000"]
impl crate::Resettable for Cpu0VtorSpec {
    const RESET_VALUE: u32 = 0x1000_0000;
}
