#[doc = "Register `UARTCR` reader"]
pub type R = crate::R<UartcrSpec>;
#[doc = "Register `UARTCR` writer"]
pub type W = crate::W<UartcrSpec>;
#[doc = "UART enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uarten {
    #[doc = "0: UART is disabled"]
    Disable = 0,
    #[doc = "1: UART is enabled"]
    Enable = 1,
}
impl From<Uarten> for bool {
    #[inline(always)]
    fn from(variant: Uarten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UARTEN` reader - UART enable"]
pub type UartenR = crate::BitReader<Uarten>;
impl UartenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uarten {
        match self.bits {
            false => Uarten::Disable,
            true => Uarten::Enable,
        }
    }
    #[doc = "UART is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Uarten::Disable
    }
    #[doc = "UART is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Uarten::Enable
    }
}
#[doc = "Field `UARTEN` writer - UART enable"]
pub type UartenW<'a, REG> = crate::BitWriter<'a, REG, Uarten>;
impl<'a, REG> UartenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UART is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Uarten::Disable)
    }
    #[doc = "UART is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Uarten::Enable)
    }
}
#[doc = "SIR enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Siren {
    #[doc = "0: SIR is disabled"]
    Disable = 0,
    #[doc = "1: SIR is enabled"]
    Enable = 1,
}
impl From<Siren> for bool {
    #[inline(always)]
    fn from(variant: Siren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIREN` reader - SIR enable"]
pub type SirenR = crate::BitReader<Siren>;
impl SirenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Siren {
        match self.bits {
            false => Siren::Disable,
            true => Siren::Enable,
        }
    }
    #[doc = "SIR is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Siren::Disable
    }
    #[doc = "SIR is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Siren::Enable
    }
}
#[doc = "Field `SIREN` writer - SIR enable"]
pub type SirenW<'a, REG> = crate::BitWriter<'a, REG, Siren>;
impl<'a, REG> SirenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SIR is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Siren::Disable)
    }
    #[doc = "SIR is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Siren::Enable)
    }
}
#[doc = "IrDA SIR low power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sirlp {
    #[doc = "0: SIR low power mode is disabled"]
    Disable = 0,
    #[doc = "1: SIR low power mode is enabled"]
    Enable = 1,
}
impl From<Sirlp> for bool {
    #[inline(always)]
    fn from(variant: Sirlp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIRLP` reader - IrDA SIR low power mode"]
pub type SirlpR = crate::BitReader<Sirlp>;
impl SirlpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sirlp {
        match self.bits {
            false => Sirlp::Disable,
            true => Sirlp::Enable,
        }
    }
    #[doc = "SIR low power mode is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sirlp::Disable
    }
    #[doc = "SIR low power mode is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sirlp::Enable
    }
}
#[doc = "Field `SIRLP` writer - IrDA SIR low power mode"]
pub type SirlpW<'a, REG> = crate::BitWriter<'a, REG, Sirlp>;
impl<'a, REG> SirlpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SIR low power mode is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sirlp::Disable)
    }
    #[doc = "SIR low power mode is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sirlp::Enable)
    }
}
#[doc = "Loop back enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbe {
    #[doc = "0: Loop back mode is disabled"]
    Disable = 0,
    #[doc = "1: Loop back mode is enabled"]
    Enable = 1,
}
impl From<Lbe> for bool {
    #[inline(always)]
    fn from(variant: Lbe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBE` reader - Loop back enable"]
pub type LbeR = crate::BitReader<Lbe>;
impl LbeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbe {
        match self.bits {
            false => Lbe::Disable,
            true => Lbe::Enable,
        }
    }
    #[doc = "Loop back mode is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Lbe::Disable
    }
    #[doc = "Loop back mode is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Lbe::Enable
    }
}
#[doc = "Field `LBE` writer - Loop back enable"]
pub type LbeW<'a, REG> = crate::BitWriter<'a, REG, Lbe>;
impl<'a, REG> LbeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Loop back mode is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Lbe::Disable)
    }
    #[doc = "Loop back mode is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lbe::Enable)
    }
}
#[doc = "Transmit enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txe {
    #[doc = "0: Transmission is disabled."]
    Disable = 0,
    #[doc = "1: Transmission is enabled."]
    Enable = 1,
}
impl From<Txe> for bool {
    #[inline(always)]
    fn from(variant: Txe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXE` reader - Transmit enable"]
pub type TxeR = crate::BitReader<Txe>;
impl TxeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txe {
        match self.bits {
            false => Txe::Disable,
            true => Txe::Enable,
        }
    }
    #[doc = "Transmission is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Txe::Disable
    }
    #[doc = "Transmission is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Txe::Enable
    }
}
#[doc = "Field `TXE` writer - Transmit enable"]
pub type TxeW<'a, REG> = crate::BitWriter<'a, REG, Txe>;
impl<'a, REG> TxeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmission is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Txe::Disable)
    }
    #[doc = "Transmission is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Txe::Enable)
    }
}
#[doc = "Receive enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxe {
    #[doc = "0: Reception is disabled"]
    Disable = 0,
    #[doc = "1: Reception is enabled"]
    Enable = 1,
}
impl From<Rxe> for bool {
    #[inline(always)]
    fn from(variant: Rxe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXE` reader - Receive enable"]
pub type RxeR = crate::BitReader<Rxe>;
impl RxeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxe {
        match self.bits {
            false => Rxe::Disable,
            true => Rxe::Enable,
        }
    }
    #[doc = "Reception is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rxe::Disable
    }
    #[doc = "Reception is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rxe::Enable
    }
}
#[doc = "Field `RXE` writer - Receive enable"]
pub type RxeW<'a, REG> = crate::BitWriter<'a, REG, Rxe>;
impl<'a, REG> RxeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reception is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rxe::Disable)
    }
    #[doc = "Reception is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rxe::Enable)
    }
}
#[doc = "Field `DTR` reader - Data transmit ready"]
pub type DtrR = crate::BitReader;
#[doc = "Field `DTR` writer - Data transmit ready"]
pub type DtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTS` reader - Request to send"]
pub type RtsR = crate::BitReader;
#[doc = "Field `RTS` writer - Request to send"]
pub type RtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Out1` reader - Complement of the UART Out1"]
pub type Out1R = crate::BitReader;
#[doc = "Field `Out1` writer - Complement of the UART Out1"]
pub type Out1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Out2` reader - Complement of the UART Out2"]
pub type Out2R = crate::BitReader;
#[doc = "Field `Out2` writer - Complement of the UART Out2"]
pub type Out2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "RTS hardware flow control enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtsen {
    #[doc = "0: RTS hardware flow control is disabled"]
    Disable = 0,
    #[doc = "1: RTS hardware flow control is enabled"]
    Enable = 1,
}
impl From<Rtsen> for bool {
    #[inline(always)]
    fn from(variant: Rtsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSEn` reader - RTS hardware flow control enable"]
pub type RtsenR = crate::BitReader<Rtsen>;
impl RtsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtsen {
        match self.bits {
            false => Rtsen::Disable,
            true => Rtsen::Enable,
        }
    }
    #[doc = "RTS hardware flow control is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rtsen::Disable
    }
    #[doc = "RTS hardware flow control is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rtsen::Enable
    }
}
#[doc = "Field `RTSEn` writer - RTS hardware flow control enable"]
pub type RtsenW<'a, REG> = crate::BitWriter<'a, REG, Rtsen>;
impl<'a, REG> RtsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTS hardware flow control is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsen::Disable)
    }
    #[doc = "RTS hardware flow control is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsen::Enable)
    }
}
#[doc = "CTS hardware flow control enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsen {
    #[doc = "0: CTS hardware flow control is disabled"]
    Disable = 0,
    #[doc = "1: CTS hardware flow control is enabled"]
    Enable = 1,
}
impl From<Ctsen> for bool {
    #[inline(always)]
    fn from(variant: Ctsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSEn` reader - CTS hardware flow control enable"]
pub type CtsenR = crate::BitReader<Ctsen>;
impl CtsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsen {
        match self.bits {
            false => Ctsen::Disable,
            true => Ctsen::Enable,
        }
    }
    #[doc = "CTS hardware flow control is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ctsen::Disable
    }
    #[doc = "CTS hardware flow control is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ctsen::Enable
    }
}
#[doc = "Field `CTSEn` writer - CTS hardware flow control enable"]
pub type CtsenW<'a, REG> = crate::BitWriter<'a, REG, Ctsen>;
impl<'a, REG> CtsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CTS hardware flow control is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsen::Disable)
    }
    #[doc = "CTS hardware flow control is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsen::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - UART enable"]
    #[inline(always)]
    pub fn uarten(&self) -> UartenR {
        UartenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SIR enable"]
    #[inline(always)]
    pub fn siren(&self) -> SirenR {
        SirenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IrDA SIR low power mode"]
    #[inline(always)]
    pub fn sirlp(&self) -> SirlpR {
        SirlpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Loop back enable"]
    #[inline(always)]
    pub fn lbe(&self) -> LbeR {
        LbeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit enable"]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive enable"]
    #[inline(always)]
    pub fn rxe(&self) -> RxeR {
        RxeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data transmit ready"]
    #[inline(always)]
    pub fn dtr(&self) -> DtrR {
        DtrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Request to send"]
    #[inline(always)]
    pub fn rts(&self) -> RtsR {
        RtsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Complement of the UART Out1"]
    #[inline(always)]
    pub fn out1(&self) -> Out1R {
        Out1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Complement of the UART Out2"]
    #[inline(always)]
    pub fn out2(&self) -> Out2R {
        Out2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RTS hardware flow control enable"]
    #[inline(always)]
    pub fn rtsen(&self) -> RtsenR {
        RtsenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CTS hardware flow control enable"]
    #[inline(always)]
    pub fn ctsen(&self) -> CtsenR {
        CtsenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART enable"]
    #[inline(always)]
    pub fn uarten(&mut self) -> UartenW<UartcrSpec> {
        UartenW::new(self, 0)
    }
    #[doc = "Bit 1 - SIR enable"]
    #[inline(always)]
    pub fn siren(&mut self) -> SirenW<UartcrSpec> {
        SirenW::new(self, 1)
    }
    #[doc = "Bit 2 - IrDA SIR low power mode"]
    #[inline(always)]
    pub fn sirlp(&mut self) -> SirlpW<UartcrSpec> {
        SirlpW::new(self, 2)
    }
    #[doc = "Bit 7 - Loop back enable"]
    #[inline(always)]
    pub fn lbe(&mut self) -> LbeW<UartcrSpec> {
        LbeW::new(self, 7)
    }
    #[doc = "Bit 8 - Transmit enable"]
    #[inline(always)]
    pub fn txe(&mut self) -> TxeW<UartcrSpec> {
        TxeW::new(self, 8)
    }
    #[doc = "Bit 9 - Receive enable"]
    #[inline(always)]
    pub fn rxe(&mut self) -> RxeW<UartcrSpec> {
        RxeW::new(self, 9)
    }
    #[doc = "Bit 10 - Data transmit ready"]
    #[inline(always)]
    pub fn dtr(&mut self) -> DtrW<UartcrSpec> {
        DtrW::new(self, 10)
    }
    #[doc = "Bit 11 - Request to send"]
    #[inline(always)]
    pub fn rts(&mut self) -> RtsW<UartcrSpec> {
        RtsW::new(self, 11)
    }
    #[doc = "Bit 12 - Complement of the UART Out1"]
    #[inline(always)]
    pub fn out1(&mut self) -> Out1W<UartcrSpec> {
        Out1W::new(self, 12)
    }
    #[doc = "Bit 13 - Complement of the UART Out2"]
    #[inline(always)]
    pub fn out2(&mut self) -> Out2W<UartcrSpec> {
        Out2W::new(self, 13)
    }
    #[doc = "Bit 14 - RTS hardware flow control enable"]
    #[inline(always)]
    pub fn rtsen(&mut self) -> RtsenW<UartcrSpec> {
        RtsenW::new(self, 14)
    }
    #[doc = "Bit 15 - CTS hardware flow control enable"]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CtsenW<UartcrSpec> {
        CtsenW::new(self, 15)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartcrSpec;
impl crate::RegisterSpec for UartcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartcr::R`](R) reader structure"]
impl crate::Readable for UartcrSpec {}
#[doc = "`write(|w| ..)` method takes [`uartcr::W`](W) writer structure"]
impl crate::Writable for UartcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTCR to value 0x0300"]
impl crate::Resettable for UartcrSpec {
    const RESET_VALUE: u32 = 0x0300;
}
