#[doc = "Register `RESET_CTRL` reader"]
pub type R = crate::R<ResetCtrlSpec>;
#[doc = "Register `RESET_CTRL` writer"]
pub type W = crate::W<ResetCtrlSpec>;
#[doc = "Field `GPTIMER_RESET` reader - Reset Active low"]
pub type GptimerResetR = crate::BitReader;
#[doc = "Field `GPTIMER_RESET` writer - Reset Active low"]
pub type GptimerResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0_RESET` reader - Reset Active low"]
pub type I2c0ResetR = crate::BitReader;
#[doc = "Field `I2C0_RESET` writer - Reset Active low"]
pub type I2c0ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_RESET` reader - Reset Active low"]
pub type I2c1ResetR = crate::BitReader;
#[doc = "Field `I2C1_RESET` writer - Reset Active low"]
pub type I2c1ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S_RESET` reader - Reset Active low"]
pub type I2sResetR = crate::BitReader;
#[doc = "Field `I2S_RESET` writer - Reset Active low"]
pub type I2sResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_RESET` reader - Reset Active low"]
pub type SpiResetR = crate::BitReader;
#[doc = "Field `SPI_RESET` writer - Reset Active low"]
pub type SpiResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI_RESET` reader - Reset Active low"]
pub type QspiResetR = crate::BitReader;
#[doc = "Field `QSPI_RESET` writer - Reset Active low"]
pub type QspiResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART0_RESET` reader - Reset Active low"]
pub type Uart0ResetR = crate::BitReader;
#[doc = "Field `UART0_RESET` writer - Reset Active low"]
pub type Uart0ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1_RESET` reader - Reset Active low"]
pub type Uart1ResetR = crate::BitReader;
#[doc = "Field `UART1_RESET` writer - Reset Active low"]
pub type Uart1ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO_RESET` reader - Reset Active low"]
pub type GpioResetR = crate::BitReader;
#[doc = "Field `GPIO_RESET` writer - Reset Active low"]
pub type GpioResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVT_RESET` reader - Reset Active low"]
pub type PvtResetR = crate::BitReader;
#[doc = "Field `PVT_RESET` writer - Reset Active low"]
pub type PvtResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM0_RESET` reader - Reset Active low"]
pub type Pwm0ResetR = crate::BitReader;
#[doc = "Field `PWM0_RESET` writer - Reset Active low"]
pub type Pwm0ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM1_RESET` reader - Reset Active low"]
pub type Pwm1ResetR = crate::BitReader;
#[doc = "Field `PWM1_RESET` writer - Reset Active low"]
pub type Pwm1ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM2_RESET` reader - Reset Active low"]
pub type Pwm2ResetR = crate::BitReader;
#[doc = "Field `PWM2_RESET` writer - Reset Active low"]
pub type Pwm2ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_RESET` reader - Reset Active low"]
pub type RtcResetR = crate::BitReader;
#[doc = "Field `RTC_RESET` writer - Reset Active low"]
pub type RtcResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Reset Active low"]
    #[inline(always)]
    pub fn gptimer_reset(&self) -> GptimerResetR {
        GptimerResetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset Active low"]
    #[inline(always)]
    pub fn i2c0_reset(&self) -> I2c0ResetR {
        I2c0ResetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset Active low"]
    #[inline(always)]
    pub fn i2c1_reset(&self) -> I2c1ResetR {
        I2c1ResetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset Active low"]
    #[inline(always)]
    pub fn i2s_reset(&self) -> I2sResetR {
        I2sResetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reset Active low"]
    #[inline(always)]
    pub fn spi_reset(&self) -> SpiResetR {
        SpiResetR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset Active low"]
    #[inline(always)]
    pub fn qspi_reset(&self) -> QspiResetR {
        QspiResetR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset Active low"]
    #[inline(always)]
    pub fn uart0_reset(&self) -> Uart0ResetR {
        Uart0ResetR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reset Active low"]
    #[inline(always)]
    pub fn uart1_reset(&self) -> Uart1ResetR {
        Uart1ResetR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reset Active low"]
    #[inline(always)]
    pub fn gpio_reset(&self) -> GpioResetR {
        GpioResetR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reset Active low"]
    #[inline(always)]
    pub fn pvt_reset(&self) -> PvtResetR {
        PvtResetR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reset Active low"]
    #[inline(always)]
    pub fn pwm0_reset(&self) -> Pwm0ResetR {
        Pwm0ResetR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reset Active low"]
    #[inline(always)]
    pub fn pwm1_reset(&self) -> Pwm1ResetR {
        Pwm1ResetR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reset Active low"]
    #[inline(always)]
    pub fn pwm2_reset(&self) -> Pwm2ResetR {
        Pwm2ResetR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Reset Active low"]
    #[inline(always)]
    pub fn rtc_reset(&self) -> RtcResetR {
        RtcResetR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Reset Active low"]
    #[inline(always)]
    pub fn gptimer_reset(&mut self) -> GptimerResetW<ResetCtrlSpec> {
        GptimerResetW::new(self, 1)
    }
    #[doc = "Bit 2 - Reset Active low"]
    #[inline(always)]
    pub fn i2c0_reset(&mut self) -> I2c0ResetW<ResetCtrlSpec> {
        I2c0ResetW::new(self, 2)
    }
    #[doc = "Bit 3 - Reset Active low"]
    #[inline(always)]
    pub fn i2c1_reset(&mut self) -> I2c1ResetW<ResetCtrlSpec> {
        I2c1ResetW::new(self, 3)
    }
    #[doc = "Bit 4 - Reset Active low"]
    #[inline(always)]
    pub fn i2s_reset(&mut self) -> I2sResetW<ResetCtrlSpec> {
        I2sResetW::new(self, 4)
    }
    #[doc = "Bit 5 - Reset Active low"]
    #[inline(always)]
    pub fn spi_reset(&mut self) -> SpiResetW<ResetCtrlSpec> {
        SpiResetW::new(self, 5)
    }
    #[doc = "Bit 6 - Reset Active low"]
    #[inline(always)]
    pub fn qspi_reset(&mut self) -> QspiResetW<ResetCtrlSpec> {
        QspiResetW::new(self, 6)
    }
    #[doc = "Bit 7 - Reset Active low"]
    #[inline(always)]
    pub fn uart0_reset(&mut self) -> Uart0ResetW<ResetCtrlSpec> {
        Uart0ResetW::new(self, 7)
    }
    #[doc = "Bit 8 - Reset Active low"]
    #[inline(always)]
    pub fn uart1_reset(&mut self) -> Uart1ResetW<ResetCtrlSpec> {
        Uart1ResetW::new(self, 8)
    }
    #[doc = "Bit 9 - Reset Active low"]
    #[inline(always)]
    pub fn gpio_reset(&mut self) -> GpioResetW<ResetCtrlSpec> {
        GpioResetW::new(self, 9)
    }
    #[doc = "Bit 10 - Reset Active low"]
    #[inline(always)]
    pub fn pvt_reset(&mut self) -> PvtResetW<ResetCtrlSpec> {
        PvtResetW::new(self, 10)
    }
    #[doc = "Bit 11 - Reset Active low"]
    #[inline(always)]
    pub fn pwm0_reset(&mut self) -> Pwm0ResetW<ResetCtrlSpec> {
        Pwm0ResetW::new(self, 11)
    }
    #[doc = "Bit 12 - Reset Active low"]
    #[inline(always)]
    pub fn pwm1_reset(&mut self) -> Pwm1ResetW<ResetCtrlSpec> {
        Pwm1ResetW::new(self, 12)
    }
    #[doc = "Bit 13 - Reset Active low"]
    #[inline(always)]
    pub fn pwm2_reset(&mut self) -> Pwm2ResetW<ResetCtrlSpec> {
        Pwm2ResetW::new(self, 13)
    }
    #[doc = "Bit 14 - Reset Active low"]
    #[inline(always)]
    pub fn rtc_reset(&mut self) -> RtcResetW<ResetCtrlSpec> {
        RtcResetW::new(self, 14)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reset_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetCtrlSpec;
impl crate::RegisterSpec for ResetCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset_ctrl::R`](R) reader structure"]
impl crate::Readable for ResetCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reset_ctrl::W`](W) writer structure"]
impl crate::Writable for ResetCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESET_CTRL to value 0xffff_ffff"]
impl crate::Resettable for ResetCtrlSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
