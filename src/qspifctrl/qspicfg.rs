#[doc = "Register `QSPICFG` reader"]
pub type R = crate::R<QspicfgSpec>;
#[doc = "Register `QSPICFG` writer"]
pub type W = crate::W<QspicfgSpec>;
#[doc = "Field `QSPIEN` reader - QSPI Enable"]
pub type QspienR = crate::BitReader;
#[doc = "Field `QSPIEN` writer - QSPI Enable"]
pub type QspienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKPOLARITY` reader - Clock polarity outside SPI word. This maps to the standard SPI CPOL transfer format"]
pub type ClkpolarityR = crate::BitReader;
#[doc = "Field `CLKPOLARITY` writer - Clock polarity outside SPI word. This maps to the standard SPI CPOL transfer format"]
pub type ClkpolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKPHASE` reader - Clock phase, this maps to the standard SPI CPHA transfer format"]
pub type ClkphaseR = crate::BitReader;
#[doc = "Field `CLKPHASE` writer - Clock phase, this maps to the standard SPI CPHA transfer format"]
pub type ClkphaseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHYMODEEN` reader - PHY Mode enable"]
pub type PhymodeenR = crate::BitReader;
#[doc = "Field `PHYMODEEN` writer - PHY Mode enable"]
pub type PhymodeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDIRACCCTR` reader - Enable Direct Access Controller"]
pub type EndiraccctrR = crate::BitReader;
#[doc = "Field `ENDIRACCCTR` writer - Enable Direct Access Controller"]
pub type EndiraccctrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEGIPMODEEN` reader - Legacy IP Mode Enable"]
pub type LegipmodeenR = crate::BitReader;
#[doc = "Field `LEGIPMODEEN` writer - Legacy IP Mode Enable"]
pub type LegipmodeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Peripheral select decode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perseldec {
    #[doc = "0: Only 1 of 4 selects n_ss_out is active"]
    Disabled = 0,
    #[doc = "1: Allow external 4-to-16 decode"]
    Enabled = 1,
}
impl From<Perseldec> for bool {
    #[inline(always)]
    fn from(variant: Perseldec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERSELDEC` reader - Peripheral select decode"]
pub type PerseldecR = crate::BitReader<Perseldec>;
impl PerseldecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perseldec {
        match self.bits {
            false => Perseldec::Disabled,
            true => Perseldec::Enabled,
        }
    }
    #[doc = "Only 1 of 4 selects n_ss_out is active"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Perseldec::Disabled
    }
    #[doc = "Allow external 4-to-16 decode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Perseldec::Enabled
    }
}
#[doc = "Field `PERSELDEC` writer - Peripheral select decode"]
pub type PerseldecW<'a, REG> = crate::BitWriter<'a, REG, Perseldec>;
impl<'a, REG> PerseldecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Only 1 of 4 selects n_ss_out is active"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Perseldec::Disabled)
    }
    #[doc = "Allow external 4-to-16 decode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Perseldec::Enabled)
    }
}
#[doc = "Peripheral chip select lines\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Percslines {
    #[doc = "0: n_ss_out: 0b1110"]
    Ss0 = 0,
    #[doc = "1: n_ss_out: 0b1101"]
    Ss1 = 1,
    #[doc = "3: n_ss_out: 0b1011"]
    Ss2 = 3,
    #[doc = "7: n_ss_out: 0b0111"]
    Ss3 = 7,
    #[doc = "15: n_ss_out: 0b1111 (no peripheral selected)"]
    Ssinactive = 15,
}
impl From<Percslines> for u8 {
    #[inline(always)]
    fn from(variant: Percslines) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Percslines {
    type Ux = u8;
}
impl crate::IsEnum for Percslines {}
#[doc = "Field `PERCSLINES` reader - Peripheral chip select lines"]
pub type PercslinesR = crate::FieldReader<Percslines>;
impl PercslinesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Percslines> {
        match self.bits {
            0 => Some(Percslines::Ss0),
            1 => Some(Percslines::Ss1),
            3 => Some(Percslines::Ss2),
            7 => Some(Percslines::Ss3),
            15 => Some(Percslines::Ssinactive),
            _ => None,
        }
    }
    #[doc = "n_ss_out: 0b1110"]
    #[inline(always)]
    pub fn is_ss0(&self) -> bool {
        *self == Percslines::Ss0
    }
    #[doc = "n_ss_out: 0b1101"]
    #[inline(always)]
    pub fn is_ss1(&self) -> bool {
        *self == Percslines::Ss1
    }
    #[doc = "n_ss_out: 0b1011"]
    #[inline(always)]
    pub fn is_ss2(&self) -> bool {
        *self == Percslines::Ss2
    }
    #[doc = "n_ss_out: 0b0111"]
    #[inline(always)]
    pub fn is_ss3(&self) -> bool {
        *self == Percslines::Ss3
    }
    #[doc = "n_ss_out: 0b1111 (no peripheral selected)"]
    #[inline(always)]
    pub fn is_ssinactive(&self) -> bool {
        *self == Percslines::Ssinactive
    }
}
#[doc = "Field `PERCSLINES` writer - Peripheral chip select lines"]
pub type PercslinesW<'a, REG> = crate::FieldWriter<'a, REG, 4, Percslines>;
impl<'a, REG> PercslinesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "n_ss_out: 0b1110"]
    #[inline(always)]
    pub fn ss0(self) -> &'a mut crate::W<REG> {
        self.variant(Percslines::Ss0)
    }
    #[doc = "n_ss_out: 0b1101"]
    #[inline(always)]
    pub fn ss1(self) -> &'a mut crate::W<REG> {
        self.variant(Percslines::Ss1)
    }
    #[doc = "n_ss_out: 0b1011"]
    #[inline(always)]
    pub fn ss2(self) -> &'a mut crate::W<REG> {
        self.variant(Percslines::Ss2)
    }
    #[doc = "n_ss_out: 0b0111"]
    #[inline(always)]
    pub fn ss3(self) -> &'a mut crate::W<REG> {
        self.variant(Percslines::Ss3)
    }
    #[doc = "n_ss_out: 0b1111 (no peripheral selected)"]
    #[inline(always)]
    pub fn ssinactive(self) -> &'a mut crate::W<REG> {
        self.variant(Percslines::Ssinactive)
    }
}
#[doc = "Field `WPPINDRV` reader - Set to drive the WP pin of Flash device"]
pub type WppindrvR = crate::BitReader;
#[doc = "Field `WPPINDRV` writer - Set to drive the WP pin of Flash device"]
pub type WppindrvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDMAPIF` reader - Enable DMA Peripheral Interface"]
pub type EndmapifR = crate::BitReader;
#[doc = "Field `ENDMAPIF` writer - Enable DMA Peripheral Interface"]
pub type EndmapifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENAHBADDRRM` reader - Enable AHB Address Re-mapping"]
pub type EnahbaddrrmR = crate::BitReader;
#[doc = "Field `ENAHBADDRRM` writer - Enable AHB Address Re-mapping"]
pub type EnahbaddrrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENTRXIPMODEONR` reader - Enter XIP Mode on next READ"]
pub type EntrxipmodeonrR = crate::BitReader;
#[doc = "Field `ENTRXIPMODEONR` writer - Enter XIP Mode on next READ"]
pub type EntrxipmodeonrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENTRXIPMODEIMM` reader - Enter XIP Mode immediately"]
pub type EntrxipmodeimmR = crate::BitReader;
#[doc = "Field `ENTRXIPMODEIMM` writer - Enter XIP Mode immediately"]
pub type EntrxipmodeimmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAMOBRDIV` reader - Master mode baud rate divisor (2 to 32)"]
pub type MamobrdivR = crate::FieldReader;
#[doc = "Field `MAMOBRDIV` writer - Master mode baud rate divisor (2 to 32)"]
pub type MamobrdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AHBDECEN` reader - Enable AHB Decoder"]
pub type AhbdecenR = crate::BitReader;
#[doc = "Field `AHBDECEN` writer - Enable AHB Decoder"]
pub type AhbdecenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTREN` reader - Enable DTR Protocol"]
pub type DtrenR = crate::BitReader;
#[doc = "Field `DTREN` writer - Enable DTR Protocol"]
pub type DtrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIPLPHYEN` reader - Pipeline PHY Mode enable"]
pub type PiplphyenR = crate::BitReader;
#[doc = "Field `PIPLPHYEN` writer - Pipeline PHY Mode enable"]
pub type PiplphyenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIPLIDLE` reader - Serial Interface and QSPI pipeline is IDLE"]
pub type PiplidleR = crate::BitReader;
#[doc = "Field `PIPLIDLE` writer - Serial Interface and QSPI pipeline is IDLE"]
pub type PiplidleW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - QSPI Enable"]
    #[inline(always)]
    pub fn qspien(&self) -> QspienR {
        QspienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock polarity outside SPI word. This maps to the standard SPI CPOL transfer format"]
    #[inline(always)]
    pub fn clkpolarity(&self) -> ClkpolarityR {
        ClkpolarityR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock phase, this maps to the standard SPI CPHA transfer format"]
    #[inline(always)]
    pub fn clkphase(&self) -> ClkphaseR {
        ClkphaseR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PHY Mode enable"]
    #[inline(always)]
    pub fn phymodeen(&self) -> PhymodeenR {
        PhymodeenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Direct Access Controller"]
    #[inline(always)]
    pub fn endiraccctr(&self) -> EndiraccctrR {
        EndiraccctrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Legacy IP Mode Enable"]
    #[inline(always)]
    pub fn legipmodeen(&self) -> LegipmodeenR {
        LegipmodeenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Peripheral select decode"]
    #[inline(always)]
    pub fn perseldec(&self) -> PerseldecR {
        PerseldecR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Peripheral chip select lines"]
    #[inline(always)]
    pub fn percslines(&self) -> PercslinesR {
        PercslinesR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Set to drive the WP pin of Flash device"]
    #[inline(always)]
    pub fn wppindrv(&self) -> WppindrvR {
        WppindrvR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable DMA Peripheral Interface"]
    #[inline(always)]
    pub fn endmapif(&self) -> EndmapifR {
        EndmapifR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable AHB Address Re-mapping"]
    #[inline(always)]
    pub fn enahbaddrrm(&self) -> EnahbaddrrmR {
        EnahbaddrrmR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enter XIP Mode on next READ"]
    #[inline(always)]
    pub fn entrxipmodeonr(&self) -> EntrxipmodeonrR {
        EntrxipmodeonrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enter XIP Mode immediately"]
    #[inline(always)]
    pub fn entrxipmodeimm(&self) -> EntrxipmodeimmR {
        EntrxipmodeimmR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - Master mode baud rate divisor (2 to 32)"]
    #[inline(always)]
    pub fn mamobrdiv(&self) -> MamobrdivR {
        MamobrdivR::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - Enable AHB Decoder"]
    #[inline(always)]
    pub fn ahbdecen(&self) -> AhbdecenR {
        AhbdecenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable DTR Protocol"]
    #[inline(always)]
    pub fn dtren(&self) -> DtrenR {
        DtrenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Pipeline PHY Mode enable"]
    #[inline(always)]
    pub fn piplphyen(&self) -> PiplphyenR {
        PiplphyenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - Serial Interface and QSPI pipeline is IDLE"]
    #[inline(always)]
    pub fn piplidle(&self) -> PiplidleR {
        PiplidleR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - QSPI Enable"]
    #[inline(always)]
    pub fn qspien(&mut self) -> QspienW<QspicfgSpec> {
        QspienW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock polarity outside SPI word. This maps to the standard SPI CPOL transfer format"]
    #[inline(always)]
    pub fn clkpolarity(&mut self) -> ClkpolarityW<QspicfgSpec> {
        ClkpolarityW::new(self, 1)
    }
    #[doc = "Bit 2 - Clock phase, this maps to the standard SPI CPHA transfer format"]
    #[inline(always)]
    pub fn clkphase(&mut self) -> ClkphaseW<QspicfgSpec> {
        ClkphaseW::new(self, 2)
    }
    #[doc = "Bit 3 - PHY Mode enable"]
    #[inline(always)]
    pub fn phymodeen(&mut self) -> PhymodeenW<QspicfgSpec> {
        PhymodeenW::new(self, 3)
    }
    #[doc = "Bit 7 - Enable Direct Access Controller"]
    #[inline(always)]
    pub fn endiraccctr(&mut self) -> EndiraccctrW<QspicfgSpec> {
        EndiraccctrW::new(self, 7)
    }
    #[doc = "Bit 8 - Legacy IP Mode Enable"]
    #[inline(always)]
    pub fn legipmodeen(&mut self) -> LegipmodeenW<QspicfgSpec> {
        LegipmodeenW::new(self, 8)
    }
    #[doc = "Bit 9 - Peripheral select decode"]
    #[inline(always)]
    pub fn perseldec(&mut self) -> PerseldecW<QspicfgSpec> {
        PerseldecW::new(self, 9)
    }
    #[doc = "Bits 10:13 - Peripheral chip select lines"]
    #[inline(always)]
    pub fn percslines(&mut self) -> PercslinesW<QspicfgSpec> {
        PercslinesW::new(self, 10)
    }
    #[doc = "Bit 14 - Set to drive the WP pin of Flash device"]
    #[inline(always)]
    pub fn wppindrv(&mut self) -> WppindrvW<QspicfgSpec> {
        WppindrvW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable DMA Peripheral Interface"]
    #[inline(always)]
    pub fn endmapif(&mut self) -> EndmapifW<QspicfgSpec> {
        EndmapifW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable AHB Address Re-mapping"]
    #[inline(always)]
    pub fn enahbaddrrm(&mut self) -> EnahbaddrrmW<QspicfgSpec> {
        EnahbaddrrmW::new(self, 16)
    }
    #[doc = "Bit 17 - Enter XIP Mode on next READ"]
    #[inline(always)]
    pub fn entrxipmodeonr(&mut self) -> EntrxipmodeonrW<QspicfgSpec> {
        EntrxipmodeonrW::new(self, 17)
    }
    #[doc = "Bit 18 - Enter XIP Mode immediately"]
    #[inline(always)]
    pub fn entrxipmodeimm(&mut self) -> EntrxipmodeimmW<QspicfgSpec> {
        EntrxipmodeimmW::new(self, 18)
    }
    #[doc = "Bits 19:22 - Master mode baud rate divisor (2 to 32)"]
    #[inline(always)]
    pub fn mamobrdiv(&mut self) -> MamobrdivW<QspicfgSpec> {
        MamobrdivW::new(self, 19)
    }
    #[doc = "Bit 23 - Enable AHB Decoder"]
    #[inline(always)]
    pub fn ahbdecen(&mut self) -> AhbdecenW<QspicfgSpec> {
        AhbdecenW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable DTR Protocol"]
    #[inline(always)]
    pub fn dtren(&mut self) -> DtrenW<QspicfgSpec> {
        DtrenW::new(self, 24)
    }
    #[doc = "Bit 25 - Pipeline PHY Mode enable"]
    #[inline(always)]
    pub fn piplphyen(&mut self) -> PiplphyenW<QspicfgSpec> {
        PiplphyenW::new(self, 25)
    }
    #[doc = "Bit 31 - Serial Interface and QSPI pipeline is IDLE"]
    #[inline(always)]
    pub fn piplidle(&mut self) -> PiplidleW<QspicfgSpec> {
        PiplidleW::new(self, 31)
    }
}
#[doc = "QSPI Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspicfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspicfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QspicfgSpec;
impl crate::RegisterSpec for QspicfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qspicfg::R`](R) reader structure"]
impl crate::Readable for QspicfgSpec {}
#[doc = "`write(|w| ..)` method takes [`qspicfg::W`](W) writer structure"]
impl crate::Writable for QspicfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QSPICFG to value 0x8078_0081"]
impl crate::Resettable for QspicfgSpec {
    const RESET_VALUE: u32 = 0x8078_0081;
}
