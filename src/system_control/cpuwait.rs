#[doc = "Register `CPUWAIT` reader"]
pub type R = crate::R<CpuwaitSpec>;
#[doc = "Register `CPUWAIT` writer"]
pub type W = crate::W<CpuwaitSpec>;
#[doc = "CPU 0 waits at boot and whether CPU1 powers up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpu0wait {
    #[doc = "0: CPU0 boot normally. From Power ON reset, nSRST reset or Watchdog Reset, CPU 1 powers up"]
    Normallyorpowerup = 0,
    #[doc = "1: CPU0 wait. From Power ON reset, nSRST reset or Watchdog Reset, CPU 1 do not power up"]
    Waitornopowerup = 1,
}
impl From<Cpu0wait> for bool {
    #[inline(always)]
    fn from(variant: Cpu0wait) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU0WAIT` reader - CPU 0 waits at boot and whether CPU1 powers up"]
pub type Cpu0waitR = crate::BitReader<Cpu0wait>;
impl Cpu0waitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu0wait {
        match self.bits {
            false => Cpu0wait::Normallyorpowerup,
            true => Cpu0wait::Waitornopowerup,
        }
    }
    #[doc = "CPU0 boot normally. From Power ON reset, nSRST reset or Watchdog Reset, CPU 1 powers up"]
    #[inline(always)]
    pub fn is_normallyorpowerup(&self) -> bool {
        *self == Cpu0wait::Normallyorpowerup
    }
    #[doc = "CPU0 wait. From Power ON reset, nSRST reset or Watchdog Reset, CPU 1 do not power up"]
    #[inline(always)]
    pub fn is_waitornopowerup(&self) -> bool {
        *self == Cpu0wait::Waitornopowerup
    }
}
#[doc = "Field `CPU0WAIT` writer - CPU 0 waits at boot and whether CPU1 powers up"]
pub type Cpu0waitW<'a, REG> = crate::BitWriter<'a, REG, Cpu0wait>;
impl<'a, REG> Cpu0waitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPU0 boot normally. From Power ON reset, nSRST reset or Watchdog Reset, CPU 1 powers up"]
    #[inline(always)]
    pub fn normallyorpowerup(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu0wait::Normallyorpowerup)
    }
    #[doc = "CPU0 wait. From Power ON reset, nSRST reset or Watchdog Reset, CPU 1 do not power up"]
    #[inline(always)]
    pub fn waitornopowerup(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu0wait::Waitornopowerup)
    }
}
#[doc = "CPU 1 waits at boot and whether CPU0 powers up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpu1wait {
    #[doc = "0: CPU1 boot normally. From Power ON reset, nSRST reset or Watchdog Reset, CPU 0 powers up"]
    Normallyorpowerup = 0,
    #[doc = "1: CPU1 wait. From Power ON reset, nSRST reset or Watchdog Reset, CPU 0 do not power up"]
    Waitornopowerup = 1,
}
impl From<Cpu1wait> for bool {
    #[inline(always)]
    fn from(variant: Cpu1wait) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU1WAIT` reader - CPU 1 waits at boot and whether CPU0 powers up"]
pub type Cpu1waitR = crate::BitReader<Cpu1wait>;
impl Cpu1waitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu1wait {
        match self.bits {
            false => Cpu1wait::Normallyorpowerup,
            true => Cpu1wait::Waitornopowerup,
        }
    }
    #[doc = "CPU1 boot normally. From Power ON reset, nSRST reset or Watchdog Reset, CPU 0 powers up"]
    #[inline(always)]
    pub fn is_normallyorpowerup(&self) -> bool {
        *self == Cpu1wait::Normallyorpowerup
    }
    #[doc = "CPU1 wait. From Power ON reset, nSRST reset or Watchdog Reset, CPU 0 do not power up"]
    #[inline(always)]
    pub fn is_waitornopowerup(&self) -> bool {
        *self == Cpu1wait::Waitornopowerup
    }
}
#[doc = "Field `CPU1WAIT` writer - CPU 1 waits at boot and whether CPU0 powers up"]
pub type Cpu1waitW<'a, REG> = crate::BitWriter<'a, REG, Cpu1wait>;
impl<'a, REG> Cpu1waitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPU1 boot normally. From Power ON reset, nSRST reset or Watchdog Reset, CPU 0 powers up"]
    #[inline(always)]
    pub fn normallyorpowerup(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1wait::Normallyorpowerup)
    }
    #[doc = "CPU1 wait. From Power ON reset, nSRST reset or Watchdog Reset, CPU 0 do not power up"]
    #[inline(always)]
    pub fn waitornopowerup(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1wait::Waitornopowerup)
    }
}
impl R {
    #[doc = "Bit 0 - CPU 0 waits at boot and whether CPU1 powers up"]
    #[inline(always)]
    pub fn cpu0wait(&self) -> Cpu0waitR {
        Cpu0waitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU 1 waits at boot and whether CPU0 powers up"]
    #[inline(always)]
    pub fn cpu1wait(&self) -> Cpu1waitR {
        Cpu1waitR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU 0 waits at boot and whether CPU1 powers up"]
    #[inline(always)]
    pub fn cpu0wait(&mut self) -> Cpu0waitW<CpuwaitSpec> {
        Cpu0waitW::new(self, 0)
    }
    #[doc = "Bit 1 - CPU 1 waits at boot and whether CPU0 powers up"]
    #[inline(always)]
    pub fn cpu1wait(&mut self) -> Cpu1waitW<CpuwaitSpec> {
        Cpu1waitW::new(self, 1)
    }
}
#[doc = "CPU Boot wait control after reset\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuwait::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuwait::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuwaitSpec;
impl crate::RegisterSpec for CpuwaitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuwait::R`](R) reader structure"]
impl crate::Readable for CpuwaitSpec {}
#[doc = "`write(|w| ..)` method takes [`cpuwait::W`](W) writer structure"]
impl crate::Writable for CpuwaitSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPUWAIT to value 0"]
impl crate::Resettable for CpuwaitSpec {}
