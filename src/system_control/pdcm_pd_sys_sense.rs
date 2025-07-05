#[doc = "Register `PDCM_PD_SYS_SENSE` reader"]
pub type R = crate::R<PdcmPdSysSenseSpec>;
#[doc = "Register `PDCM_PD_SYS_SENSE` writer"]
pub type W = crate::W<PdcmPdSysSenseSpec>;
#[doc = "Enable PD_SYS ON Sensitivity\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPdSysOn {
    #[doc = "1: Keep PD_SYS awake after powered ON"]
    Enable = 1,
}
impl From<SPdSysOn> for bool {
    #[inline(always)]
    fn from(variant: SPdSysOn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S_PD_SYS_ON` reader - Enable PD_SYS ON Sensitivity"]
pub type SPdSysOnR = crate::BitReader<SPdSysOn>;
impl SPdSysOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPdSysOn> {
        match self.bits {
            true => Some(SPdSysOn::Enable),
            _ => None,
        }
    }
    #[doc = "Keep PD_SYS awake after powered ON"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SPdSysOn::Enable
    }
}
#[doc = "Field `S_PD_SYS_ON` writer - Enable PD_SYS ON Sensitivity"]
pub type SPdSysOnW<'a, REG> = crate::BitWriter<'a, REG, SPdSysOn>;
impl<'a, REG> SPdSysOnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Keep PD_SYS awake after powered ON"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SPdSysOn::Enable)
    }
}
#[doc = "Tied to HIGH\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPdCpu0coreOn {
    #[doc = "1: PD_SYS always tries to stay ON if PD_CPU0CORE is ON"]
    High = 1,
}
impl From<SPdCpu0coreOn> for bool {
    #[inline(always)]
    fn from(variant: SPdCpu0coreOn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S_PD_CPU0CORE_ON` reader - Tied to HIGH"]
pub type SPdCpu0coreOnR = crate::BitReader<SPdCpu0coreOn>;
impl SPdCpu0coreOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPdCpu0coreOn> {
        match self.bits {
            true => Some(SPdCpu0coreOn::High),
            _ => None,
        }
    }
    #[doc = "PD_SYS always tries to stay ON if PD_CPU0CORE is ON"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPdCpu0coreOn::High
    }
}
#[doc = "Tied to HIGH\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPdCpu1coreOn {
    #[doc = "1: PD_SYS always tries to stay ON if PD_CPU1CORE is ON"]
    High = 1,
}
impl From<SPdCpu1coreOn> for bool {
    #[inline(always)]
    fn from(variant: SPdCpu1coreOn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S_PD_CPU1CORE_ON` reader - Tied to HIGH"]
pub type SPdCpu1coreOnR = crate::BitReader<SPdCpu1coreOn>;
impl SPdCpu1coreOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPdCpu1coreOn> {
        match self.bits {
            true => Some(SPdCpu1coreOn::High),
            _ => None,
        }
    }
    #[doc = "PD_SYS always tries to stay ON if PD_CPU1CORE is ON"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPdCpu1coreOn::High
    }
}
#[doc = "Tied to HIGH\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPdSram0On {
    #[doc = "1: PD_SYS always tries to keep ON if SRAM0 power domain is ON"]
    High = 1,
}
impl From<SPdSram0On> for bool {
    #[inline(always)]
    fn from(variant: SPdSram0On) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S_PD_SRAM0_ON` reader - Tied to HIGH"]
pub type SPdSram0OnR = crate::BitReader<SPdSram0On>;
impl SPdSram0OnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPdSram0On> {
        match self.bits {
            true => Some(SPdSram0On::High),
            _ => None,
        }
    }
    #[doc = "PD_SYS always tries to keep ON if SRAM0 power domain is ON"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPdSram0On::High
    }
}
#[doc = "Tied to HIGH\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPdSram1On {
    #[doc = "1: PD_SYS always tries to keep ON if SRAM1 power domain is ON"]
    High = 1,
}
impl From<SPdSram1On> for bool {
    #[inline(always)]
    fn from(variant: SPdSram1On) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S_PD_SRAM1_ON` reader - Tied to HIGH"]
pub type SPdSram1OnR = crate::BitReader<SPdSram1On>;
impl SPdSram1OnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPdSram1On> {
        match self.bits {
            true => Some(SPdSram1On::High),
            _ => None,
        }
    }
    #[doc = "PD_SYS always tries to keep ON if SRAM1 power domain is ON"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPdSram1On::High
    }
}
#[doc = "Tied to HIGH\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPdSram2On {
    #[doc = "1: PD_SYS always tries to keep ON if SRAM2 power domain is ON"]
    High = 1,
}
impl From<SPdSram2On> for bool {
    #[inline(always)]
    fn from(variant: SPdSram2On) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S_PD_SRAM2_ON` reader - Tied to HIGH"]
pub type SPdSram2OnR = crate::BitReader<SPdSram2On>;
impl SPdSram2OnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPdSram2On> {
        match self.bits {
            true => Some(SPdSram2On::High),
            _ => None,
        }
    }
    #[doc = "PD_SYS always tries to keep ON if SRAM2 power domain is ON"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPdSram2On::High
    }
}
#[doc = "Tied to HIGH\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPdSram3On {
    #[doc = "1: PD_SYS always tries to keep ON if SRAM3 power domain is ON"]
    High = 1,
}
impl From<SPdSram3On> for bool {
    #[inline(always)]
    fn from(variant: SPdSram3On) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S_PD_SRAM3_ON` reader - Tied to HIGH"]
pub type SPdSram3OnR = crate::BitReader<SPdSram3On>;
impl SPdSram3OnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPdSram3On> {
        match self.bits {
            true => Some(SPdSram3On::High),
            _ => None,
        }
    }
    #[doc = "PD_SYS always tries to keep ON if SRAM3 power domain is ON"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPdSram3On::High
    }
}
#[doc = "Tied to HIGH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPdCryptoOn {
    #[doc = "1: PD_SYS always tries to keep ON if S_PD_CRYPTO_ON is ON"]
    High = 1,
}
impl From<SPdCryptoOn> for bool {
    #[inline(always)]
    fn from(variant: SPdCryptoOn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S_PD_CRYPTO_ON` reader - Tied to HIGH"]
pub type SPdCryptoOnR = crate::BitReader<SPdCryptoOn>;
impl SPdCryptoOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPdCryptoOn> {
        match self.bits {
            true => Some(SPdCryptoOn::High),
            _ => None,
        }
    }
    #[doc = "PD_SYS always tries to keep ON if S_PD_CRYPTO_ON is ON"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPdCryptoOn::High
    }
}
#[doc = "Enable PDEXPIN\\[0\\]
signal Sensitivity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPdExp0In {
    #[doc = "1: Enable PDEXPIN\\[0\\]
signal Sensitivity."]
    Enable = 1,
    #[doc = "0: Disable PDEXPIN\\[0\\]
signal Sensitivity."]
    Disabled = 0,
}
impl From<SPdExp0In> for bool {
    #[inline(always)]
    fn from(variant: SPdExp0In) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S_PD_EXP0_IN` reader - Enable PDEXPIN\\[0\\]
signal Sensitivity"]
pub type SPdExp0InR = crate::BitReader<SPdExp0In>;
impl SPdExp0InR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPdExp0In {
        match self.bits {
            true => SPdExp0In::Enable,
            false => SPdExp0In::Disabled,
        }
    }
    #[doc = "Enable PDEXPIN\\[0\\]
signal Sensitivity."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SPdExp0In::Enable
    }
    #[doc = "Disable PDEXPIN\\[0\\]
signal Sensitivity."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPdExp0In::Disabled
    }
}
#[doc = "Field `S_PD_EXP0_IN` writer - Enable PDEXPIN\\[0\\]
signal Sensitivity"]
pub type SPdExp0InW<'a, REG> = crate::BitWriter<'a, REG, SPdExp0In>;
impl<'a, REG> SPdExp0InW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable PDEXPIN\\[0\\]
signal Sensitivity."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SPdExp0In::Enable)
    }
    #[doc = "Disable PDEXPIN\\[0\\]
signal Sensitivity."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPdExp0In::Disabled)
    }
}
#[doc = "Enable PDEXPIN\\[1\\]
signal Sensitivity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPdExp1In {
    #[doc = "1: Enable PDEXPIN\\[1\\]
signal Sensitivity."]
    Enable = 1,
    #[doc = "0: Disable PDEXPIN\\[1\\]
signal Sensitivity."]
    Disabled = 0,
}
impl From<SPdExp1In> for bool {
    #[inline(always)]
    fn from(variant: SPdExp1In) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S_PD_EXP1_IN` reader - Enable PDEXPIN\\[1\\]
signal Sensitivity"]
pub type SPdExp1InR = crate::BitReader<SPdExp1In>;
impl SPdExp1InR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPdExp1In {
        match self.bits {
            true => SPdExp1In::Enable,
            false => SPdExp1In::Disabled,
        }
    }
    #[doc = "Enable PDEXPIN\\[1\\]
signal Sensitivity."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SPdExp1In::Enable
    }
    #[doc = "Disable PDEXPIN\\[1\\]
signal Sensitivity."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPdExp1In::Disabled
    }
}
#[doc = "Field `S_PD_EXP1_IN` writer - Enable PDEXPIN\\[1\\]
signal Sensitivity"]
pub type SPdExp1InW<'a, REG> = crate::BitWriter<'a, REG, SPdExp1In>;
impl<'a, REG> SPdExp1InW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable PDEXPIN\\[1\\]
signal Sensitivity."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SPdExp1In::Enable)
    }
    #[doc = "Disable PDEXPIN\\[1\\]
signal Sensitivity."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPdExp1In::Disabled)
    }
}
#[doc = "Enable PDEXPIN\\[2\\]
signal Sensitivity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPdExp2In {
    #[doc = "1: Enable PDEXPIN\\[2\\]
signal Sensitivity."]
    Enable = 1,
    #[doc = "0: Disable PDEXPIN\\[2\\]
signal Sensitivity."]
    Disabled = 0,
}
impl From<SPdExp2In> for bool {
    #[inline(always)]
    fn from(variant: SPdExp2In) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S_PD_EXP2_IN` reader - Enable PDEXPIN\\[2\\]
signal Sensitivity"]
pub type SPdExp2InR = crate::BitReader<SPdExp2In>;
impl SPdExp2InR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPdExp2In {
        match self.bits {
            true => SPdExp2In::Enable,
            false => SPdExp2In::Disabled,
        }
    }
    #[doc = "Enable PDEXPIN\\[2\\]
signal Sensitivity."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SPdExp2In::Enable
    }
    #[doc = "Disable PDEXPIN\\[2\\]
signal Sensitivity."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPdExp2In::Disabled
    }
}
#[doc = "Field `S_PD_EXP2_IN` writer - Enable PDEXPIN\\[2\\]
signal Sensitivity"]
pub type SPdExp2InW<'a, REG> = crate::BitWriter<'a, REG, SPdExp2In>;
impl<'a, REG> SPdExp2InW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable PDEXPIN\\[2\\]
signal Sensitivity."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SPdExp2In::Enable)
    }
    #[doc = "Disable PDEXPIN\\[2\\]
signal Sensitivity."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPdExp2In::Disabled)
    }
}
#[doc = "Enable PDEXPIN\\[3\\]
signal Sensitivity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPdExp3In {
    #[doc = "1: Enable PDEXPIN\\[3\\]
signal Sensitivity."]
    Enable = 1,
    #[doc = "0: Disable PDEXPIN\\[3\\]
signal Sensitivity."]
    Disabled = 0,
}
impl From<SPdExp3In> for bool {
    #[inline(always)]
    fn from(variant: SPdExp3In) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S_PD_EXP3_IN` reader - Enable PDEXPIN\\[3\\]
signal Sensitivity"]
pub type SPdExp3InR = crate::BitReader<SPdExp3In>;
impl SPdExp3InR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPdExp3In {
        match self.bits {
            true => SPdExp3In::Enable,
            false => SPdExp3In::Disabled,
        }
    }
    #[doc = "Enable PDEXPIN\\[3\\]
signal Sensitivity."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SPdExp3In::Enable
    }
    #[doc = "Disable PDEXPIN\\[3\\]
signal Sensitivity."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPdExp3In::Disabled
    }
}
#[doc = "Field `S_PD_EXP3_IN` writer - Enable PDEXPIN\\[3\\]
signal Sensitivity"]
pub type SPdExp3InW<'a, REG> = crate::BitWriter<'a, REG, SPdExp3In>;
impl<'a, REG> SPdExp3InW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable PDEXPIN\\[3\\]
signal Sensitivity."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SPdExp3In::Enable)
    }
    #[doc = "Disable PDEXPIN\\[3\\]
signal Sensitivity."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPdExp3In::Disabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable PD_SYS ON Sensitivity"]
    #[inline(always)]
    pub fn s_pd_sys_on(&self) -> SPdSysOnR {
        SPdSysOnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tied to HIGH"]
    #[inline(always)]
    pub fn s_pd_cpu0core_on(&self) -> SPdCpu0coreOnR {
        SPdCpu0coreOnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tied to HIGH"]
    #[inline(always)]
    pub fn s_pd_cpu1core_on(&self) -> SPdCpu1coreOnR {
        SPdCpu1coreOnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tied to HIGH"]
    #[inline(always)]
    pub fn s_pd_sram0_on(&self) -> SPdSram0OnR {
        SPdSram0OnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tied to HIGH"]
    #[inline(always)]
    pub fn s_pd_sram1_on(&self) -> SPdSram1OnR {
        SPdSram1OnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Tied to HIGH"]
    #[inline(always)]
    pub fn s_pd_sram2_on(&self) -> SPdSram2OnR {
        SPdSram2OnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Tied to HIGH"]
    #[inline(always)]
    pub fn s_pd_sram3_on(&self) -> SPdSram3OnR {
        SPdSram3OnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - Tied to HIGH"]
    #[inline(always)]
    pub fn s_pd_crypto_on(&self) -> SPdCryptoOnR {
        SPdCryptoOnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable PDEXPIN\\[0\\]
signal Sensitivity"]
    #[inline(always)]
    pub fn s_pd_exp0_in(&self) -> SPdExp0InR {
        SPdExp0InR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable PDEXPIN\\[1\\]
signal Sensitivity"]
    #[inline(always)]
    pub fn s_pd_exp1_in(&self) -> SPdExp1InR {
        SPdExp1InR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable PDEXPIN\\[2\\]
signal Sensitivity"]
    #[inline(always)]
    pub fn s_pd_exp2_in(&self) -> SPdExp2InR {
        SPdExp2InR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable PDEXPIN\\[3\\]
signal Sensitivity"]
    #[inline(always)]
    pub fn s_pd_exp3_in(&self) -> SPdExp3InR {
        SPdExp3InR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable PD_SYS ON Sensitivity"]
    #[inline(always)]
    pub fn s_pd_sys_on(&mut self) -> SPdSysOnW<PdcmPdSysSenseSpec> {
        SPdSysOnW::new(self, 0)
    }
    #[doc = "Bit 16 - Enable PDEXPIN\\[0\\]
signal Sensitivity"]
    #[inline(always)]
    pub fn s_pd_exp0_in(&mut self) -> SPdExp0InW<PdcmPdSysSenseSpec> {
        SPdExp0InW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable PDEXPIN\\[1\\]
signal Sensitivity"]
    #[inline(always)]
    pub fn s_pd_exp1_in(&mut self) -> SPdExp1InW<PdcmPdSysSenseSpec> {
        SPdExp1InW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable PDEXPIN\\[2\\]
signal Sensitivity"]
    #[inline(always)]
    pub fn s_pd_exp2_in(&mut self) -> SPdExp2InW<PdcmPdSysSenseSpec> {
        SPdExp2InW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable PDEXPIN\\[3\\]
signal Sensitivity"]
    #[inline(always)]
    pub fn s_pd_exp3_in(&mut self) -> SPdExp3InW<PdcmPdSysSenseSpec> {
        SPdExp3InW::new(self, 19)
    }
}
#[doc = "External Wakeup Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pdcm_pd_sys_sense::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcm_pd_sys_sense::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdcmPdSysSenseSpec;
impl crate::RegisterSpec for PdcmPdSysSenseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdcm_pd_sys_sense::R`](R) reader structure"]
impl crate::Readable for PdcmPdSysSenseSpec {}
#[doc = "`write(|w| ..)` method takes [`pdcm_pd_sys_sense::W`](W) writer structure"]
impl crate::Writable for PdcmPdSysSenseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDCM_PD_SYS_SENSE to value 0x7f"]
impl crate::Resettable for PdcmPdSysSenseSpec {
    const RESET_VALUE: u32 = 0x7f;
}
