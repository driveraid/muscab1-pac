#[doc = "Register `CLOCK_FORCE` reader"]
pub type R = crate::R<ClockForceSpec>;
#[doc = "Register `CLOCK_FORCE` writer"]
pub type W = crate::W<ClockForceSpec>;
#[doc = "Field `MAINCLK_FORCE` reader - Force MAINCLK to run when set to HIGH"]
pub type MainclkForceR = crate::BitReader;
#[doc = "Field `MAINCLK_FORCE` writer - Force MAINCLK to run when set to HIGH"]
pub type MainclkForceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSSYSCLK_FORCE` reader - Force Base element Local SYSCLK to run when set to HIGH"]
pub type SyssysclkForceR = crate::BitReader;
#[doc = "Field `SYSSYSCLK_FORCE` writer - Force Base element Local SYSCLK to run when set to HIGH"]
pub type SyssysclkForceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSFCLK_FORCE` reader - Force Base element Local FCLK to run when set to HIGH"]
pub type SysfclkForceR = crate::BitReader;
#[doc = "Field `SYSFCLK_FORCE` writer - Force Base element Local FCLK to run when set to HIGH"]
pub type SysfclkForceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAMSYSCLK_FORCE` reader - Force SRAM Local SYSCLK to run when set to HIGH"]
pub type SramsysclkForceR = crate::BitReader;
#[doc = "Field `SRAMSYSCLK_FORCE` writer - Force SRAM Local SYSCLK to run when set to HIGH"]
pub type SramsysclkForceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAMFCLK_FORCE` reader - Force SRAM Local FCLK to run when set to HIGH"]
pub type SramfclkForceR = crate::BitReader;
#[doc = "Field `SRAMFCLK_FORCE` writer - Force SRAM Local FCLK to run when set to HIGH"]
pub type SramfclkForceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPUSYSCLK_FORCE` reader - Force all CPU SYSCLK to run when set to HIGH"]
pub type CpusysclkForceR = crate::BitReader;
#[doc = "Field `CPUSYSCLK_FORCE` writer - Force all CPU SYSCLK to run when set to HIGH"]
pub type CpusysclkForceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPUFCLK_FORCE` reader - Force all CPU FCLK to run when set to HIGH"]
pub type CpufclkForceR = crate::BitReader;
#[doc = "Field `CPUFCLK_FORCE` writer - Force all CPU FCLK to run when set to HIGH"]
pub type CpufclkForceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTOSYSCLK_FORCE` reader - Force all CryptoCell clocks to run when set to HIGH"]
pub type CryptosysclkForceR = crate::BitReader;
#[doc = "Field `CRYPTOSYSCLK_FORCE` writer - Force all CryptoCell clocks to run when set to HIGH"]
pub type CryptosysclkForceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable FCLK gating by HINTSYSCLKEN when CPU 1 is OFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FclkhintgateEnable {
    #[doc = "1: Enable FCLK gating by HINTSYSCLKEN when CPU 1 is OFF"]
    Enable = 1,
    #[doc = "0: improve SRAM3 access latency at the cost of increased power consumption"]
    Latency = 0,
}
impl From<FclkhintgateEnable> for bool {
    #[inline(always)]
    fn from(variant: FclkhintgateEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCLKHINTGATE_ENABLE` reader - Enable FCLK gating by HINTSYSCLKEN when CPU 1 is OFF"]
pub type FclkhintgateEnableR = crate::BitReader<FclkhintgateEnable>;
impl FclkhintgateEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FclkhintgateEnable {
        match self.bits {
            true => FclkhintgateEnable::Enable,
            false => FclkhintgateEnable::Latency,
        }
    }
    #[doc = "Enable FCLK gating by HINTSYSCLKEN when CPU 1 is OFF"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FclkhintgateEnable::Enable
    }
    #[doc = "improve SRAM3 access latency at the cost of increased power consumption"]
    #[inline(always)]
    pub fn is_latency(&self) -> bool {
        *self == FclkhintgateEnable::Latency
    }
}
#[doc = "Field `FCLKHINTGATE_ENABLE` writer - Enable FCLK gating by HINTSYSCLKEN when CPU 1 is OFF"]
pub type FclkhintgateEnableW<'a, REG> = crate::BitWriter<'a, REG, FclkhintgateEnable>;
impl<'a, REG> FclkhintgateEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable FCLK gating by HINTSYSCLKEN when CPU 1 is OFF"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FclkhintgateEnable::Enable)
    }
    #[doc = "improve SRAM3 access latency at the cost of increased power consumption"]
    #[inline(always)]
    pub fn latency(self) -> &'a mut crate::W<REG> {
        self.variant(FclkhintgateEnable::Latency)
    }
}
impl R {
    #[doc = "Bit 0 - Force MAINCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn mainclk_force(&self) -> MainclkForceR {
        MainclkForceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force Base element Local SYSCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn syssysclk_force(&self) -> SyssysclkForceR {
        SyssysclkForceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force Base element Local FCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn sysfclk_force(&self) -> SysfclkForceR {
        SysfclkForceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force SRAM Local SYSCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn sramsysclk_force(&self) -> SramsysclkForceR {
        SramsysclkForceR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Force SRAM Local FCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn sramfclk_force(&self) -> SramfclkForceR {
        SramfclkForceR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Force all CPU SYSCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn cpusysclk_force(&self) -> CpusysclkForceR {
        CpusysclkForceR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Force all CPU FCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn cpufclk_force(&self) -> CpufclkForceR {
        CpufclkForceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Force all CryptoCell clocks to run when set to HIGH"]
    #[inline(always)]
    pub fn cryptosysclk_force(&self) -> CryptosysclkForceR {
        CryptosysclkForceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable FCLK gating by HINTSYSCLKEN when CPU 1 is OFF"]
    #[inline(always)]
    pub fn fclkhintgate_enable(&self) -> FclkhintgateEnableR {
        FclkhintgateEnableR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force MAINCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn mainclk_force(&mut self) -> MainclkForceW<ClockForceSpec> {
        MainclkForceW::new(self, 0)
    }
    #[doc = "Bit 1 - Force Base element Local SYSCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn syssysclk_force(&mut self) -> SyssysclkForceW<ClockForceSpec> {
        SyssysclkForceW::new(self, 1)
    }
    #[doc = "Bit 2 - Force Base element Local FCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn sysfclk_force(&mut self) -> SysfclkForceW<ClockForceSpec> {
        SysfclkForceW::new(self, 2)
    }
    #[doc = "Bit 3 - Force SRAM Local SYSCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn sramsysclk_force(&mut self) -> SramsysclkForceW<ClockForceSpec> {
        SramsysclkForceW::new(self, 3)
    }
    #[doc = "Bit 4 - Force SRAM Local FCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn sramfclk_force(&mut self) -> SramfclkForceW<ClockForceSpec> {
        SramfclkForceW::new(self, 4)
    }
    #[doc = "Bit 5 - Force all CPU SYSCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn cpusysclk_force(&mut self) -> CpusysclkForceW<ClockForceSpec> {
        CpusysclkForceW::new(self, 5)
    }
    #[doc = "Bit 6 - Force all CPU FCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn cpufclk_force(&mut self) -> CpufclkForceW<ClockForceSpec> {
        CpufclkForceW::new(self, 6)
    }
    #[doc = "Bit 7 - Force all CryptoCell clocks to run when set to HIGH"]
    #[inline(always)]
    pub fn cryptosysclk_force(&mut self) -> CryptosysclkForceW<ClockForceSpec> {
        CryptosysclkForceW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable FCLK gating by HINTSYSCLKEN when CPU 1 is OFF"]
    #[inline(always)]
    pub fn fclkhintgate_enable(&mut self) -> FclkhintgateEnableW<ClockForceSpec> {
        FclkhintgateEnableW::new(self, 8)
    }
}
#[doc = "Clock Force\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_force::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_force::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClockForceSpec;
impl crate::RegisterSpec for ClockForceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock_force::R`](R) reader structure"]
impl crate::Readable for ClockForceSpec {}
#[doc = "`write(|w| ..)` method takes [`clock_force::W`](W) writer structure"]
impl crate::Writable for ClockForceSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLOCK_FORCE to value 0"]
impl crate::Resettable for ClockForceSpec {}
