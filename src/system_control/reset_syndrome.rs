#[doc = "Register `RESET_SYNDROME` reader"]
pub type R = crate::R<ResetSyndromeSpec>;
#[doc = "Register `RESET_SYNDROME` writer"]
pub type W = crate::W<ResetSyndromeSpec>;
#[doc = "Field `PoR` writer - Power-on"]
pub type PoRW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSWD` writer - Non-secure watchdog"]
pub type NswdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWD` writer - Secure watchdog"]
pub type SwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S32KWD` writer - Watchdog on the S32KCLK clock"]
pub type S32kwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSRSTREQ0` writer - CPU 0 System Reset Request"]
pub type Sysrstreq0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSRSTREQ1` writer - CPU 1 System Reset Request"]
pub type Sysrstreq1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKUP0` writer - CPU 0 Lock-up Status"]
pub type Lockup0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKUP1` writer - CPU 1 Lock-up Status"]
pub type Lockup1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETREQ` writer - External Reset Request"]
pub type ResetreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRESETREQ` writer - Software Reset Request"]
pub type SwresetreqW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Power-on"]
    #[inline(always)]
    pub fn po_r(&mut self) -> PoRW<ResetSyndromeSpec> {
        PoRW::new(self, 0)
    }
    #[doc = "Bit 1 - Non-secure watchdog"]
    #[inline(always)]
    pub fn nswd(&mut self) -> NswdW<ResetSyndromeSpec> {
        NswdW::new(self, 1)
    }
    #[doc = "Bit 2 - Secure watchdog"]
    #[inline(always)]
    pub fn swd(&mut self) -> SwdW<ResetSyndromeSpec> {
        SwdW::new(self, 2)
    }
    #[doc = "Bit 3 - Watchdog on the S32KCLK clock"]
    #[inline(always)]
    pub fn s32kwd(&mut self) -> S32kwdW<ResetSyndromeSpec> {
        S32kwdW::new(self, 3)
    }
    #[doc = "Bit 4 - CPU 0 System Reset Request"]
    #[inline(always)]
    pub fn sysrstreq0(&mut self) -> Sysrstreq0W<ResetSyndromeSpec> {
        Sysrstreq0W::new(self, 4)
    }
    #[doc = "Bit 5 - CPU 1 System Reset Request"]
    #[inline(always)]
    pub fn sysrstreq1(&mut self) -> Sysrstreq1W<ResetSyndromeSpec> {
        Sysrstreq1W::new(self, 5)
    }
    #[doc = "Bit 6 - CPU 0 Lock-up Status"]
    #[inline(always)]
    pub fn lockup0(&mut self) -> Lockup0W<ResetSyndromeSpec> {
        Lockup0W::new(self, 6)
    }
    #[doc = "Bit 7 - CPU 1 Lock-up Status"]
    #[inline(always)]
    pub fn lockup1(&mut self) -> Lockup1W<ResetSyndromeSpec> {
        Lockup1W::new(self, 7)
    }
    #[doc = "Bit 8 - External Reset Request"]
    #[inline(always)]
    pub fn resetreq(&mut self) -> ResetreqW<ResetSyndromeSpec> {
        ResetreqW::new(self, 8)
    }
    #[doc = "Bit 9 - Software Reset Request"]
    #[inline(always)]
    pub fn swresetreq(&mut self) -> SwresetreqW<ResetSyndromeSpec> {
        SwresetreqW::new(self, 9)
    }
}
#[doc = "Reset Syndrome\n\nYou can [`read`](crate::Reg::read) this register and get [`reset_syndrome::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_syndrome::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetSyndromeSpec;
impl crate::RegisterSpec for ResetSyndromeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset_syndrome::R`](R) reader structure"]
impl crate::Readable for ResetSyndromeSpec {}
#[doc = "`write(|w| ..)` method takes [`reset_syndrome::W`](W) writer structure"]
impl crate::Writable for ResetSyndromeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESET_SYNDROME to value 0x01"]
impl crate::Resettable for ResetSyndromeSpec {
    const RESET_VALUE: u32 = 0x01;
}
