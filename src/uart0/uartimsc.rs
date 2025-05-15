#[doc = "Register `UARTIMSC` reader"]
pub type R = crate::R<UartimscSpec>;
#[doc = "Register `UARTIMSC` writer"]
pub type W = crate::W<UartimscSpec>;
#[doc = "nUARTRI modem interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rimim {
    #[doc = "0: Clears the mask"]
    Clear = 0,
    #[doc = "1: Sets the mask"]
    Set = 1,
}
impl From<Rimim> for bool {
    #[inline(always)]
    fn from(variant: Rimim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIMIM` reader - nUARTRI modem interrupt mask"]
pub type RimimR = crate::BitReader<Rimim>;
impl RimimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rimim {
        match self.bits {
            false => Rimim::Clear,
            true => Rimim::Set,
        }
    }
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Rimim::Clear
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rimim::Set
    }
}
#[doc = "Field `RIMIM` writer - nUARTRI modem interrupt mask"]
pub type RimimW<'a, REG> = crate::BitWriter<'a, REG, Rimim>;
impl<'a, REG> RimimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Rimim::Clear)
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rimim::Set)
    }
}
#[doc = "nUARTCTS modem interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsmim {
    #[doc = "0: Clears the mask"]
    Clear = 0,
    #[doc = "1: Sets the mask"]
    Set = 1,
}
impl From<Ctsmim> for bool {
    #[inline(always)]
    fn from(variant: Ctsmim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSMIM` reader - nUARTCTS modem interrupt mask."]
pub type CtsmimR = crate::BitReader<Ctsmim>;
impl CtsmimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsmim {
        match self.bits {
            false => Ctsmim::Clear,
            true => Ctsmim::Set,
        }
    }
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Ctsmim::Clear
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ctsmim::Set
    }
}
#[doc = "Field `CTSMIM` writer - nUARTCTS modem interrupt mask."]
pub type CtsmimW<'a, REG> = crate::BitWriter<'a, REG, Ctsmim>;
impl<'a, REG> CtsmimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsmim::Clear)
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsmim::Set)
    }
}
#[doc = "nUARTDCD modem interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcdmim {
    #[doc = "0: Clears the mask"]
    Clear = 0,
    #[doc = "1: Sets the mask"]
    Set = 1,
}
impl From<Dcdmim> for bool {
    #[inline(always)]
    fn from(variant: Dcdmim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCDMIM` reader - nUARTDCD modem interrupt mask"]
pub type DcdmimR = crate::BitReader<Dcdmim>;
impl DcdmimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcdmim {
        match self.bits {
            false => Dcdmim::Clear,
            true => Dcdmim::Set,
        }
    }
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Dcdmim::Clear
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dcdmim::Set
    }
}
#[doc = "Field `DCDMIM` writer - nUARTDCD modem interrupt mask"]
pub type DcdmimW<'a, REG> = crate::BitWriter<'a, REG, Dcdmim>;
impl<'a, REG> DcdmimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdmim::Clear)
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdmim::Set)
    }
}
#[doc = "nUARTDSR modem interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsrmim {
    #[doc = "0: Clears the mask"]
    Clear = 0,
    #[doc = "1: Sets the mask"]
    Set = 1,
}
impl From<Dsrmim> for bool {
    #[inline(always)]
    fn from(variant: Dsrmim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSRMIM` reader - nUARTDSR modem interrupt mask"]
pub type DsrmimR = crate::BitReader<Dsrmim>;
impl DsrmimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsrmim {
        match self.bits {
            false => Dsrmim::Clear,
            true => Dsrmim::Set,
        }
    }
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Dsrmim::Clear
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dsrmim::Set
    }
}
#[doc = "Field `DSRMIM` writer - nUARTDSR modem interrupt mask"]
pub type DsrmimW<'a, REG> = crate::BitWriter<'a, REG, Dsrmim>;
impl<'a, REG> DsrmimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Dsrmim::Clear)
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dsrmim::Set)
    }
}
#[doc = "Receive interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxim {
    #[doc = "0: Clears the mask"]
    Clear = 0,
    #[doc = "1: Sets the mask"]
    Set = 1,
}
impl From<Rxim> for bool {
    #[inline(always)]
    fn from(variant: Rxim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIM` reader - Receive interrupt mask"]
pub type RximR = crate::BitReader<Rxim>;
impl RximR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxim {
        match self.bits {
            false => Rxim::Clear,
            true => Rxim::Set,
        }
    }
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Rxim::Clear
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rxim::Set
    }
}
#[doc = "Field `RXIM` writer - Receive interrupt mask"]
pub type RximW<'a, REG> = crate::BitWriter<'a, REG, Rxim>;
impl<'a, REG> RximW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Rxim::Clear)
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rxim::Set)
    }
}
#[doc = "Transmit interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txim {
    #[doc = "0: Clears the mask"]
    Clear = 0,
    #[doc = "1: Sets the mask"]
    Set = 1,
}
impl From<Txim> for bool {
    #[inline(always)]
    fn from(variant: Txim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXIM` reader - Transmit interrupt mask"]
pub type TximR = crate::BitReader<Txim>;
impl TximR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txim {
        match self.bits {
            false => Txim::Clear,
            true => Txim::Set,
        }
    }
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Txim::Clear
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Txim::Set
    }
}
#[doc = "Field `TXIM` writer - Transmit interrupt mask"]
pub type TximW<'a, REG> = crate::BitWriter<'a, REG, Txim>;
impl<'a, REG> TximW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Txim::Clear)
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Txim::Set)
    }
}
#[doc = "Receive timeout interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtim {
    #[doc = "0: Clears the mask"]
    Clear = 0,
    #[doc = "1: Sets the mask"]
    Set = 1,
}
impl From<Rtim> for bool {
    #[inline(always)]
    fn from(variant: Rtim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTIM` reader - Receive timeout interrupt mask"]
pub type RtimR = crate::BitReader<Rtim>;
impl RtimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtim {
        match self.bits {
            false => Rtim::Clear,
            true => Rtim::Set,
        }
    }
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Rtim::Clear
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rtim::Set
    }
}
#[doc = "Field `RTIM` writer - Receive timeout interrupt mask"]
pub type RtimW<'a, REG> = crate::BitWriter<'a, REG, Rtim>;
impl<'a, REG> RtimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Rtim::Clear)
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rtim::Set)
    }
}
#[doc = "Framing error interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Feim {
    #[doc = "0: Clears the mask"]
    Clear = 0,
    #[doc = "1: Sets the mask"]
    Set = 1,
}
impl From<Feim> for bool {
    #[inline(always)]
    fn from(variant: Feim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEIM` reader - Framing error interrupt mask"]
pub type FeimR = crate::BitReader<Feim>;
impl FeimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Feim {
        match self.bits {
            false => Feim::Clear,
            true => Feim::Set,
        }
    }
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Feim::Clear
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Feim::Set
    }
}
#[doc = "Field `FEIM` writer - Framing error interrupt mask"]
pub type FeimW<'a, REG> = crate::BitWriter<'a, REG, Feim>;
impl<'a, REG> FeimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Feim::Clear)
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Feim::Set)
    }
}
#[doc = "Parity error interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peim {
    #[doc = "0: Clears the mask"]
    Clear = 0,
    #[doc = "1: Sets the mask"]
    Set = 1,
}
impl From<Peim> for bool {
    #[inline(always)]
    fn from(variant: Peim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEIM` reader - Parity error interrupt mask"]
pub type PeimR = crate::BitReader<Peim>;
impl PeimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peim {
        match self.bits {
            false => Peim::Clear,
            true => Peim::Set,
        }
    }
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Peim::Clear
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Peim::Set
    }
}
#[doc = "Field `PEIM` writer - Parity error interrupt mask"]
pub type PeimW<'a, REG> = crate::BitWriter<'a, REG, Peim>;
impl<'a, REG> PeimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Peim::Clear)
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Peim::Set)
    }
}
#[doc = "Break error interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Beim {
    #[doc = "0: Clears the mask"]
    Clear = 0,
    #[doc = "1: Sets the mask"]
    Set = 1,
}
impl From<Beim> for bool {
    #[inline(always)]
    fn from(variant: Beim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BEIM` reader - Break error interrupt mask"]
pub type BeimR = crate::BitReader<Beim>;
impl BeimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Beim {
        match self.bits {
            false => Beim::Clear,
            true => Beim::Set,
        }
    }
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Beim::Clear
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Beim::Set
    }
}
#[doc = "Field `BEIM` writer - Break error interrupt mask"]
pub type BeimW<'a, REG> = crate::BitWriter<'a, REG, Beim>;
impl<'a, REG> BeimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Beim::Clear)
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Beim::Set)
    }
}
#[doc = "Overrun error interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oeim {
    #[doc = "0: Clears the mask"]
    Clear = 0,
    #[doc = "1: Sets the mask"]
    Set = 1,
}
impl From<Oeim> for bool {
    #[inline(always)]
    fn from(variant: Oeim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OEIM` reader - Overrun error interrupt mask"]
pub type OeimR = crate::BitReader<Oeim>;
impl OeimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oeim {
        match self.bits {
            false => Oeim::Clear,
            true => Oeim::Set,
        }
    }
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Oeim::Clear
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Oeim::Set
    }
}
#[doc = "Field `OEIM` writer - Overrun error interrupt mask"]
pub type OeimW<'a, REG> = crate::BitWriter<'a, REG, Oeim>;
impl<'a, REG> OeimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Oeim::Clear)
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Oeim::Set)
    }
}
impl R {
    #[doc = "Bit 0 - nUARTRI modem interrupt mask"]
    #[inline(always)]
    pub fn rimim(&self) -> RimimR {
        RimimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - nUARTCTS modem interrupt mask."]
    #[inline(always)]
    pub fn ctsmim(&self) -> CtsmimR {
        CtsmimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - nUARTDCD modem interrupt mask"]
    #[inline(always)]
    pub fn dcdmim(&self) -> DcdmimR {
        DcdmimR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - nUARTDSR modem interrupt mask"]
    #[inline(always)]
    pub fn dsrmim(&self) -> DsrmimR {
        DsrmimR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive interrupt mask"]
    #[inline(always)]
    pub fn rxim(&self) -> RximR {
        RximR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit interrupt mask"]
    #[inline(always)]
    pub fn txim(&self) -> TximR {
        TximR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive timeout interrupt mask"]
    #[inline(always)]
    pub fn rtim(&self) -> RtimR {
        RtimR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Framing error interrupt mask"]
    #[inline(always)]
    pub fn feim(&self) -> FeimR {
        FeimR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity error interrupt mask"]
    #[inline(always)]
    pub fn peim(&self) -> PeimR {
        PeimR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Break error interrupt mask"]
    #[inline(always)]
    pub fn beim(&self) -> BeimR {
        BeimR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Overrun error interrupt mask"]
    #[inline(always)]
    pub fn oeim(&self) -> OeimR {
        OeimR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - nUARTRI modem interrupt mask"]
    #[inline(always)]
    pub fn rimim(&mut self) -> RimimW<UartimscSpec> {
        RimimW::new(self, 0)
    }
    #[doc = "Bit 1 - nUARTCTS modem interrupt mask."]
    #[inline(always)]
    pub fn ctsmim(&mut self) -> CtsmimW<UartimscSpec> {
        CtsmimW::new(self, 1)
    }
    #[doc = "Bit 2 - nUARTDCD modem interrupt mask"]
    #[inline(always)]
    pub fn dcdmim(&mut self) -> DcdmimW<UartimscSpec> {
        DcdmimW::new(self, 2)
    }
    #[doc = "Bit 3 - nUARTDSR modem interrupt mask"]
    #[inline(always)]
    pub fn dsrmim(&mut self) -> DsrmimW<UartimscSpec> {
        DsrmimW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive interrupt mask"]
    #[inline(always)]
    pub fn rxim(&mut self) -> RximW<UartimscSpec> {
        RximW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit interrupt mask"]
    #[inline(always)]
    pub fn txim(&mut self) -> TximW<UartimscSpec> {
        TximW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive timeout interrupt mask"]
    #[inline(always)]
    pub fn rtim(&mut self) -> RtimW<UartimscSpec> {
        RtimW::new(self, 6)
    }
    #[doc = "Bit 7 - Framing error interrupt mask"]
    #[inline(always)]
    pub fn feim(&mut self) -> FeimW<UartimscSpec> {
        FeimW::new(self, 7)
    }
    #[doc = "Bit 8 - Parity error interrupt mask"]
    #[inline(always)]
    pub fn peim(&mut self) -> PeimW<UartimscSpec> {
        PeimW::new(self, 8)
    }
    #[doc = "Bit 9 - Break error interrupt mask"]
    #[inline(always)]
    pub fn beim(&mut self) -> BeimW<UartimscSpec> {
        BeimW::new(self, 9)
    }
    #[doc = "Bit 10 - Overrun error interrupt mask"]
    #[inline(always)]
    pub fn oeim(&mut self) -> OeimW<UartimscSpec> {
        OeimW::new(self, 10)
    }
}
#[doc = "Interrupt mask set/clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartimsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartimsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartimscSpec;
impl crate::RegisterSpec for UartimscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartimsc::R`](R) reader structure"]
impl crate::Readable for UartimscSpec {}
#[doc = "`write(|w| ..)` method takes [`uartimsc::W`](W) writer structure"]
impl crate::Writable for UartimscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTIMSC to value 0"]
impl crate::Resettable for UartimscSpec {}
