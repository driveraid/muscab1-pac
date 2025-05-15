#[doc = "Register `CLK_CTRL_ENABLE` reader"]
pub type R = crate::R<ClkCtrlEnableSpec>;
#[doc = "Register `CLK_CTRL_ENABLE` writer"]
pub type W = crate::W<ClkCtrlEnableSpec>;
#[doc = "Field `ctrl_enable_1hz` reader - 0: Disable; 1: Enable"]
pub type CtrlEnable1hzR = crate::BitReader;
#[doc = "Field `ctrl_enable_1hz` writer - 0: Disable; 1: Enable"]
pub type CtrlEnable1hzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ctrl_enable_dapswclk` reader - 0: Disable; 1: Enable"]
pub type CtrlEnableDapswclkR = crate::BitReader;
#[doc = "Field `ctrl_enable_dapswclk` writer - 0: Disable; 1: Enable"]
pub type CtrlEnableDapswclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ctrl_enable_gpiohclk` reader - 0: Disable; 1: Enable"]
pub type CtrlEnableGpiohclkR = crate::BitReader;
#[doc = "Field `ctrl_enable_gpiohclk` writer - 0: Disable; 1: Enable"]
pub type CtrlEnableGpiohclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ctrl_enable_i2sclk0` reader - 0: Disable; 1: Enable"]
pub type CtrlEnableI2sclk0R = crate::BitReader;
#[doc = "Field `ctrl_enable_i2sclk0` writer - 0: Disable; 1: Enable"]
pub type CtrlEnableI2sclk0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ctrl_enable_i2sclk1` reader - 0: Disable; 1: Enable"]
pub type CtrlEnableI2sclk1R = crate::BitReader;
#[doc = "Field `ctrl_enable_i2sclk1` writer - 0: Disable; 1: Enable"]
pub type CtrlEnableI2sclk1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ctrl_enable_i2sclk2` reader - 0: Disable; 1: Enable"]
pub type CtrlEnableI2sclk2R = crate::BitReader;
#[doc = "Field `ctrl_enable_i2sclk2` writer - 0: Disable; 1: Enable"]
pub type CtrlEnableI2sclk2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ctrl_enable_mainclk` reader - 0: Disable; 1: Enable"]
pub type CtrlEnableMainclkR = crate::BitReader;
#[doc = "Field `ctrl_enable_mainclk` writer - 0: Disable; 1: Enable"]
pub type CtrlEnableMainclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ctrl_enable_qspi_phy_clk` reader - 0: Disable; 1: Enable"]
pub type CtrlEnableQspiPhyClkR = crate::BitReader;
#[doc = "Field `ctrl_enable_qspi_phy_clk` writer - 0: Disable; 1: Enable"]
pub type CtrlEnableQspiPhyClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ctrl_enable_refclk` reader - 0: Disable; 1: Enable"]
pub type CtrlEnableRefclkR = crate::BitReader;
#[doc = "Field `ctrl_enable_refclk` writer - 0: Disable; 1: Enable"]
pub type CtrlEnableRefclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ctrl_enable_rm38kclk` reader - 0: Disable; 1: Enable"]
pub type CtrlEnableRm38kclkR = crate::BitReader;
#[doc = "Field `ctrl_enable_rm38kclk` writer - 0: Disable; 1: Enable"]
pub type CtrlEnableRm38kclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ctrl_enable_sccclk` reader - 0: Disable; 1: Enable"]
pub type CtrlEnableSccclkR = crate::BitReader;
#[doc = "Field `ctrl_enable_sccclk` writer - 0: Disable; 1: Enable"]
pub type CtrlEnableSccclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ctrl_enable_sdphyclk` reader - 0: Disable; 1: Enable"]
pub type CtrlEnableSdphyclkR = crate::BitReader;
#[doc = "Field `ctrl_enable_sdphyclk` writer - 0: Disable; 1: Enable"]
pub type CtrlEnableSdphyclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ctrl_enable_testclk` reader - 0: Disable; 1: Enable"]
pub type CtrlEnableTestclkR = crate::BitReader;
#[doc = "Field `ctrl_enable_testclk` writer - 0: Disable; 1: Enable"]
pub type CtrlEnableTestclkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_1hz(&self) -> CtrlEnable1hzR {
        CtrlEnable1hzR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_dapswclk(&self) -> CtrlEnableDapswclkR {
        CtrlEnableDapswclkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_gpiohclk(&self) -> CtrlEnableGpiohclkR {
        CtrlEnableGpiohclkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_i2sclk0(&self) -> CtrlEnableI2sclk0R {
        CtrlEnableI2sclk0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_i2sclk1(&self) -> CtrlEnableI2sclk1R {
        CtrlEnableI2sclk1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_i2sclk2(&self) -> CtrlEnableI2sclk2R {
        CtrlEnableI2sclk2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_mainclk(&self) -> CtrlEnableMainclkR {
        CtrlEnableMainclkR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_qspi_phy_clk(&self) -> CtrlEnableQspiPhyClkR {
        CtrlEnableQspiPhyClkR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_refclk(&self) -> CtrlEnableRefclkR {
        CtrlEnableRefclkR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_rm38kclk(&self) -> CtrlEnableRm38kclkR {
        CtrlEnableRm38kclkR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_sccclk(&self) -> CtrlEnableSccclkR {
        CtrlEnableSccclkR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_sdphyclk(&self) -> CtrlEnableSdphyclkR {
        CtrlEnableSdphyclkR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_testclk(&self) -> CtrlEnableTestclkR {
        CtrlEnableTestclkR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_1hz(&mut self) -> CtrlEnable1hzW<ClkCtrlEnableSpec> {
        CtrlEnable1hzW::new(self, 0)
    }
    #[doc = "Bit 1 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_dapswclk(&mut self) -> CtrlEnableDapswclkW<ClkCtrlEnableSpec> {
        CtrlEnableDapswclkW::new(self, 1)
    }
    #[doc = "Bit 2 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_gpiohclk(&mut self) -> CtrlEnableGpiohclkW<ClkCtrlEnableSpec> {
        CtrlEnableGpiohclkW::new(self, 2)
    }
    #[doc = "Bit 3 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_i2sclk0(&mut self) -> CtrlEnableI2sclk0W<ClkCtrlEnableSpec> {
        CtrlEnableI2sclk0W::new(self, 3)
    }
    #[doc = "Bit 4 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_i2sclk1(&mut self) -> CtrlEnableI2sclk1W<ClkCtrlEnableSpec> {
        CtrlEnableI2sclk1W::new(self, 4)
    }
    #[doc = "Bit 5 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_i2sclk2(&mut self) -> CtrlEnableI2sclk2W<ClkCtrlEnableSpec> {
        CtrlEnableI2sclk2W::new(self, 5)
    }
    #[doc = "Bit 8 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_mainclk(&mut self) -> CtrlEnableMainclkW<ClkCtrlEnableSpec> {
        CtrlEnableMainclkW::new(self, 8)
    }
    #[doc = "Bit 9 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_qspi_phy_clk(&mut self) -> CtrlEnableQspiPhyClkW<ClkCtrlEnableSpec> {
        CtrlEnableQspiPhyClkW::new(self, 9)
    }
    #[doc = "Bit 10 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_refclk(&mut self) -> CtrlEnableRefclkW<ClkCtrlEnableSpec> {
        CtrlEnableRefclkW::new(self, 10)
    }
    #[doc = "Bit 11 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_rm38kclk(&mut self) -> CtrlEnableRm38kclkW<ClkCtrlEnableSpec> {
        CtrlEnableRm38kclkW::new(self, 11)
    }
    #[doc = "Bit 12 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_sccclk(&mut self) -> CtrlEnableSccclkW<ClkCtrlEnableSpec> {
        CtrlEnableSccclkW::new(self, 12)
    }
    #[doc = "Bit 13 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_sdphyclk(&mut self) -> CtrlEnableSdphyclkW<ClkCtrlEnableSpec> {
        CtrlEnableSdphyclkW::new(self, 13)
    }
    #[doc = "Bit 15 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_testclk(&mut self) -> CtrlEnableTestclkW<ClkCtrlEnableSpec> {
        CtrlEnableTestclkW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_ctrl_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_ctrl_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkCtrlEnableSpec;
impl crate::RegisterSpec for ClkCtrlEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_ctrl_enable::R`](R) reader structure"]
impl crate::Readable for ClkCtrlEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_ctrl_enable::W`](W) writer structure"]
impl crate::Writable for ClkCtrlEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLK_CTRL_ENABLE to value 0xffff"]
impl crate::Resettable for ClkCtrlEnableSpec {
    const RESET_VALUE: u32 = 0xffff;
}
