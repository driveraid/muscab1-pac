#[doc = "Register `TIMER1CONTROL` reader"]
pub type R = crate::R<Timer1controlSpec>;
#[doc = "Register `TIMER1CONTROL` writer"]
pub type W = crate::W<Timer1controlSpec>;
#[doc = "Selects one-shot or wrapping counter mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OneShotCount {
    #[doc = "0: Wrapping counter mode"]
    Wrapping = 0,
    #[doc = "1: One-shot counter mode"]
    OneShot = 1,
}
impl From<OneShotCount> for bool {
    #[inline(always)]
    fn from(variant: OneShotCount) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OneShotCount` reader - Selects one-shot or wrapping counter mode."]
pub type OneShotCountR = crate::BitReader<OneShotCount>;
impl OneShotCountR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OneShotCount {
        match self.bits {
            false => OneShotCount::Wrapping,
            true => OneShotCount::OneShot,
        }
    }
    #[doc = "Wrapping counter mode"]
    #[inline(always)]
    pub fn is_wrapping(&self) -> bool {
        *self == OneShotCount::Wrapping
    }
    #[doc = "One-shot counter mode"]
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        *self == OneShotCount::OneShot
    }
}
#[doc = "Field `OneShotCount` writer - Selects one-shot or wrapping counter mode."]
pub type OneShotCountW<'a, REG> = crate::BitWriter<'a, REG, OneShotCount>;
impl<'a, REG> OneShotCountW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wrapping counter mode"]
    #[inline(always)]
    pub fn wrapping(self) -> &'a mut crate::W<REG> {
        self.variant(OneShotCount::Wrapping)
    }
    #[doc = "One-shot counter mode"]
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut crate::W<REG> {
        self.variant(OneShotCount::OneShot)
    }
}
#[doc = "Selects 16-bit or 32- bit counter operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerSize {
    #[doc = "0: 16-bit counter mode"]
    _16bit = 0,
    #[doc = "1: 32-bit counter mode"]
    _32bit = 1,
}
impl From<TimerSize> for bool {
    #[inline(always)]
    fn from(variant: TimerSize) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TimerSize` reader - Selects 16-bit or 32- bit counter operation."]
pub type TimerSizeR = crate::BitReader<TimerSize>;
impl TimerSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TimerSize {
        match self.bits {
            false => TimerSize::_16bit,
            true => TimerSize::_32bit,
        }
    }
    #[doc = "16-bit counter mode"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == TimerSize::_16bit
    }
    #[doc = "32-bit counter mode"]
    #[inline(always)]
    pub fn is_32bit(&self) -> bool {
        *self == TimerSize::_32bit
    }
}
#[doc = "Field `TimerSize` writer - Selects 16-bit or 32- bit counter operation."]
pub type TimerSizeW<'a, REG> = crate::BitWriter<'a, REG, TimerSize>;
impl<'a, REG> TimerSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "16-bit counter mode"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut crate::W<REG> {
        self.variant(TimerSize::_16bit)
    }
    #[doc = "32-bit counter mode"]
    #[inline(always)]
    pub fn _32bit(self) -> &'a mut crate::W<REG> {
        self.variant(TimerSize::_32bit)
    }
}
#[doc = "Timer prescale bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TimerPre {
    #[doc = "0: clock is divided by 1"]
    Dividedby1 = 0,
    #[doc = "1: clock is divided by 16"]
    Dividedby16 = 1,
    #[doc = "2: clock is divided by 256"]
    Dividedby256 = 2,
}
impl From<TimerPre> for u8 {
    #[inline(always)]
    fn from(variant: TimerPre) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TimerPre {
    type Ux = u8;
}
impl crate::IsEnum for TimerPre {}
#[doc = "Field `TimerPre` reader - Timer prescale bits."]
pub type TimerPreR = crate::FieldReader<TimerPre>;
impl TimerPreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TimerPre> {
        match self.bits {
            0 => Some(TimerPre::Dividedby1),
            1 => Some(TimerPre::Dividedby16),
            2 => Some(TimerPre::Dividedby256),
            _ => None,
        }
    }
    #[doc = "clock is divided by 1"]
    #[inline(always)]
    pub fn is_dividedby1(&self) -> bool {
        *self == TimerPre::Dividedby1
    }
    #[doc = "clock is divided by 16"]
    #[inline(always)]
    pub fn is_dividedby16(&self) -> bool {
        *self == TimerPre::Dividedby16
    }
    #[doc = "clock is divided by 256"]
    #[inline(always)]
    pub fn is_dividedby256(&self) -> bool {
        *self == TimerPre::Dividedby256
    }
}
#[doc = "Field `TimerPre` writer - Timer prescale bits."]
pub type TimerPreW<'a, REG> = crate::FieldWriter<'a, REG, 2, TimerPre>;
impl<'a, REG> TimerPreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "clock is divided by 1"]
    #[inline(always)]
    pub fn dividedby1(self) -> &'a mut crate::W<REG> {
        self.variant(TimerPre::Dividedby1)
    }
    #[doc = "clock is divided by 16"]
    #[inline(always)]
    pub fn dividedby16(self) -> &'a mut crate::W<REG> {
        self.variant(TimerPre::Dividedby16)
    }
    #[doc = "clock is divided by 256"]
    #[inline(always)]
    pub fn dividedby256(self) -> &'a mut crate::W<REG> {
        self.variant(TimerPre::Dividedby256)
    }
}
#[doc = "Interrupt Enable bit.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InterruptEnable {
    #[doc = "0: Interrupt is disabled."]
    Disable = 0,
    #[doc = "1: Interrupt is enabled."]
    Enable = 1,
}
impl From<InterruptEnable> for bool {
    #[inline(always)]
    fn from(variant: InterruptEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `InterruptEnable` reader - Interrupt Enable bit."]
pub type InterruptEnableR = crate::BitReader<InterruptEnable>;
impl InterruptEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InterruptEnable {
        match self.bits {
            false => InterruptEnable::Disable,
            true => InterruptEnable::Enable,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == InterruptEnable::Disable
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == InterruptEnable::Enable
    }
}
#[doc = "Field `InterruptEnable` writer - Interrupt Enable bit."]
pub type InterruptEnableW<'a, REG> = crate::BitWriter<'a, REG, InterruptEnable>;
impl<'a, REG> InterruptEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(InterruptEnable::Disable)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(InterruptEnable::Enable)
    }
}
#[doc = "Timer Mode bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerMode {
    #[doc = "0: Free-Running timer mode."]
    FreeRunning = 0,
    #[doc = "1: Periodic timer mode."]
    Periodic = 1,
}
impl From<TimerMode> for bool {
    #[inline(always)]
    fn from(variant: TimerMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TimerMode` reader - Timer Mode bit."]
pub type TimerModeR = crate::BitReader<TimerMode>;
impl TimerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TimerMode {
        match self.bits {
            false => TimerMode::FreeRunning,
            true => TimerMode::Periodic,
        }
    }
    #[doc = "Free-Running timer mode."]
    #[inline(always)]
    pub fn is_free_running(&self) -> bool {
        *self == TimerMode::FreeRunning
    }
    #[doc = "Periodic timer mode."]
    #[inline(always)]
    pub fn is_periodic(&self) -> bool {
        *self == TimerMode::Periodic
    }
}
#[doc = "Field `TimerMode` writer - Timer Mode bit."]
pub type TimerModeW<'a, REG> = crate::BitWriter<'a, REG, TimerMode>;
impl<'a, REG> TimerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Free-Running timer mode."]
    #[inline(always)]
    pub fn free_running(self) -> &'a mut crate::W<REG> {
        self.variant(TimerMode::FreeRunning)
    }
    #[doc = "Periodic timer mode."]
    #[inline(always)]
    pub fn periodic(self) -> &'a mut crate::W<REG> {
        self.variant(TimerMode::Periodic)
    }
}
#[doc = "Timer Enable Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerEnable {
    #[doc = "0: Timer is disabled."]
    Disable = 0,
    #[doc = "1: Timer is enabled."]
    Enable = 1,
}
impl From<TimerEnable> for bool {
    #[inline(always)]
    fn from(variant: TimerEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TimerEnable` reader - Timer Enable Enable bit."]
pub type TimerEnableR = crate::BitReader<TimerEnable>;
impl TimerEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TimerEnable {
        match self.bits {
            false => TimerEnable::Disable,
            true => TimerEnable::Enable,
        }
    }
    #[doc = "Timer is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TimerEnable::Disable
    }
    #[doc = "Timer is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TimerEnable::Enable
    }
}
#[doc = "Field `TimerEnable` writer - Timer Enable Enable bit."]
pub type TimerEnableW<'a, REG> = crate::BitWriter<'a, REG, TimerEnable>;
impl<'a, REG> TimerEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TimerEnable::Disable)
    }
    #[doc = "Timer is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TimerEnable::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Selects one-shot or wrapping counter mode."]
    #[inline(always)]
    pub fn one_shot_count(&self) -> OneShotCountR {
        OneShotCountR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects 16-bit or 32- bit counter operation."]
    #[inline(always)]
    pub fn timer_size(&self) -> TimerSizeR {
        TimerSizeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Timer prescale bits."]
    #[inline(always)]
    pub fn timer_pre(&self) -> TimerPreR {
        TimerPreR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - Interrupt Enable bit."]
    #[inline(always)]
    pub fn interrupt_enable(&self) -> InterruptEnableR {
        InterruptEnableR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer Mode bit."]
    #[inline(always)]
    pub fn timer_mode(&self) -> TimerModeR {
        TimerModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer Enable Enable bit."]
    #[inline(always)]
    pub fn timer_enable(&self) -> TimerEnableR {
        TimerEnableR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects one-shot or wrapping counter mode."]
    #[inline(always)]
    pub fn one_shot_count(&mut self) -> OneShotCountW<Timer1controlSpec> {
        OneShotCountW::new(self, 0)
    }
    #[doc = "Bit 1 - Selects 16-bit or 32- bit counter operation."]
    #[inline(always)]
    pub fn timer_size(&mut self) -> TimerSizeW<Timer1controlSpec> {
        TimerSizeW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Timer prescale bits."]
    #[inline(always)]
    pub fn timer_pre(&mut self) -> TimerPreW<Timer1controlSpec> {
        TimerPreW::new(self, 2)
    }
    #[doc = "Bit 5 - Interrupt Enable bit."]
    #[inline(always)]
    pub fn interrupt_enable(&mut self) -> InterruptEnableW<Timer1controlSpec> {
        InterruptEnableW::new(self, 5)
    }
    #[doc = "Bit 6 - Timer Mode bit."]
    #[inline(always)]
    pub fn timer_mode(&mut self) -> TimerModeW<Timer1controlSpec> {
        TimerModeW::new(self, 6)
    }
    #[doc = "Bit 7 - Timer Enable Enable bit."]
    #[inline(always)]
    pub fn timer_enable(&mut self) -> TimerEnableW<Timer1controlSpec> {
        TimerEnableW::new(self, 7)
    }
}
#[doc = "Timer 1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer1controlSpec;
impl crate::RegisterSpec for Timer1controlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer1control::R`](R) reader structure"]
impl crate::Readable for Timer1controlSpec {}
#[doc = "`write(|w| ..)` method takes [`timer1control::W`](W) writer structure"]
impl crate::Writable for Timer1controlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER1CONTROL to value 0x20"]
impl crate::Resettable for Timer1controlSpec {
    const RESET_VALUE: u32 = 0x20;
}
