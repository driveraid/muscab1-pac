#[doc = "Register `SFSR` reader"]
pub type R = crate::R<SfsrSpec>;
#[doc = "Register `SFSR` writer"]
pub type W = crate::W<SfsrSpec>;
#[doc = "Field `INVEP` reader - Invalid entry pointd"]
pub type InvepR = crate::BitReader;
#[doc = "Field `INVEP` writer - Invalid entry pointd"]
pub type InvepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVIS` reader - Invalid integrity signature flag"]
pub type InvisR = crate::BitReader;
#[doc = "Field `INVIS` writer - Invalid integrity signature flag"]
pub type InvisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVER` reader - Invalid exception return flag"]
pub type InverR = crate::BitReader;
#[doc = "Field `INVER` writer - Invalid exception return flag"]
pub type InverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUVIOL` reader - Attribution unit violation flag"]
pub type AuviolR = crate::BitReader;
#[doc = "Field `AUVIOL` writer - Attribution unit violation flag"]
pub type AuviolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVTRAN` reader - Invalid transition flag"]
pub type InvtranR = crate::BitReader;
#[doc = "Field `INVTRAN` writer - Invalid transition flag"]
pub type InvtranW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSPERR` reader - Lazy state preservation error flag"]
pub type LsperrR = crate::BitReader;
#[doc = "Field `LSPERR` writer - Lazy state preservation error flag"]
pub type LsperrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFARVALID` reader - Secure fault address valid"]
pub type SfarvalidR = crate::BitReader;
#[doc = "Field `SFARVALID` writer - Secure fault address valid"]
pub type SfarvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERR` reader - Lazy state error flag"]
pub type LserrR = crate::BitReader;
#[doc = "Field `LSERR` writer - Lazy state error flag"]
pub type LserrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Invalid entry pointd"]
    #[inline(always)]
    pub fn invep(&self) -> InvepR {
        InvepR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Invalid integrity signature flag"]
    #[inline(always)]
    pub fn invis(&self) -> InvisR {
        InvisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Invalid exception return flag"]
    #[inline(always)]
    pub fn inver(&self) -> InverR {
        InverR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Attribution unit violation flag"]
    #[inline(always)]
    pub fn auviol(&self) -> AuviolR {
        AuviolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Invalid transition flag"]
    #[inline(always)]
    pub fn invtran(&self) -> InvtranR {
        InvtranR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Lazy state preservation error flag"]
    #[inline(always)]
    pub fn lsperr(&self) -> LsperrR {
        LsperrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Secure fault address valid"]
    #[inline(always)]
    pub fn sfarvalid(&self) -> SfarvalidR {
        SfarvalidR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Lazy state error flag"]
    #[inline(always)]
    pub fn lserr(&self) -> LserrR {
        LserrR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Invalid entry pointd"]
    #[inline(always)]
    pub fn invep(&mut self) -> InvepW<SfsrSpec> {
        InvepW::new(self, 0)
    }
    #[doc = "Bit 1 - Invalid integrity signature flag"]
    #[inline(always)]
    pub fn invis(&mut self) -> InvisW<SfsrSpec> {
        InvisW::new(self, 1)
    }
    #[doc = "Bit 2 - Invalid exception return flag"]
    #[inline(always)]
    pub fn inver(&mut self) -> InverW<SfsrSpec> {
        InverW::new(self, 2)
    }
    #[doc = "Bit 3 - Attribution unit violation flag"]
    #[inline(always)]
    pub fn auviol(&mut self) -> AuviolW<SfsrSpec> {
        AuviolW::new(self, 3)
    }
    #[doc = "Bit 4 - Invalid transition flag"]
    #[inline(always)]
    pub fn invtran(&mut self) -> InvtranW<SfsrSpec> {
        InvtranW::new(self, 4)
    }
    #[doc = "Bit 5 - Lazy state preservation error flag"]
    #[inline(always)]
    pub fn lsperr(&mut self) -> LsperrW<SfsrSpec> {
        LsperrW::new(self, 5)
    }
    #[doc = "Bit 6 - Secure fault address valid"]
    #[inline(always)]
    pub fn sfarvalid(&mut self) -> SfarvalidW<SfsrSpec> {
        SfarvalidW::new(self, 6)
    }
    #[doc = "Bit 7 - Lazy state error flag"]
    #[inline(always)]
    pub fn lserr(&mut self) -> LserrW<SfsrSpec> {
        LserrW::new(self, 7)
    }
}
#[doc = "Secure Fault Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sfsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfsrSpec;
impl crate::RegisterSpec for SfsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfsr::R`](R) reader structure"]
impl crate::Readable for SfsrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfsr::W`](W) writer structure"]
impl crate::Writable for SfsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFSR to value 0"]
impl crate::Resettable for SfsrSpec {}
