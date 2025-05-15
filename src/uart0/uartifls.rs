#[doc = "Register `UARTIFLS` reader"]
pub type R = crate::R<UartiflsSpec>;
#[doc = "Register `UARTIFLS` writer"]
pub type W = crate::W<UartiflsSpec>;
#[doc = "Transmit interrupt FIFO level select\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txiflsel {
    #[doc = "0: Transmit FIFO becomes less than or equal to 1/8 full"]
    _18full = 0,
    #[doc = "1: Transmit FIFO becomes less than or equal to 1/4 full"]
    _14full = 1,
    #[doc = "2: Transmit FIFO becomes less than or equal to 1/2 full"]
    _12full = 2,
    #[doc = "3: Transmit FIFO becomes less than or equal to 3/4 full"]
    _34full = 3,
    #[doc = "4: Transmit FIFO becomes less than or equal to 7/8 full"]
    _78full = 4,
}
impl From<Txiflsel> for u8 {
    #[inline(always)]
    fn from(variant: Txiflsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txiflsel {
    type Ux = u8;
}
impl crate::IsEnum for Txiflsel {}
#[doc = "Field `TXIFLSEL` reader - Transmit interrupt FIFO level select"]
pub type TxiflselR = crate::FieldReader<Txiflsel>;
impl TxiflselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txiflsel> {
        match self.bits {
            0 => Some(Txiflsel::_18full),
            1 => Some(Txiflsel::_14full),
            2 => Some(Txiflsel::_12full),
            3 => Some(Txiflsel::_34full),
            4 => Some(Txiflsel::_78full),
            _ => None,
        }
    }
    #[doc = "Transmit FIFO becomes less than or equal to 1/8 full"]
    #[inline(always)]
    pub fn is_18full(&self) -> bool {
        *self == Txiflsel::_18full
    }
    #[doc = "Transmit FIFO becomes less than or equal to 1/4 full"]
    #[inline(always)]
    pub fn is_14full(&self) -> bool {
        *self == Txiflsel::_14full
    }
    #[doc = "Transmit FIFO becomes less than or equal to 1/2 full"]
    #[inline(always)]
    pub fn is_12full(&self) -> bool {
        *self == Txiflsel::_12full
    }
    #[doc = "Transmit FIFO becomes less than or equal to 3/4 full"]
    #[inline(always)]
    pub fn is_34full(&self) -> bool {
        *self == Txiflsel::_34full
    }
    #[doc = "Transmit FIFO becomes less than or equal to 7/8 full"]
    #[inline(always)]
    pub fn is_78full(&self) -> bool {
        *self == Txiflsel::_78full
    }
}
#[doc = "Field `TXIFLSEL` writer - Transmit interrupt FIFO level select"]
pub type TxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Txiflsel>;
impl<'a, REG> TxiflselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Transmit FIFO becomes less than or equal to 1/8 full"]
    #[inline(always)]
    pub fn _18full(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::_18full)
    }
    #[doc = "Transmit FIFO becomes less than or equal to 1/4 full"]
    #[inline(always)]
    pub fn _14full(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::_14full)
    }
    #[doc = "Transmit FIFO becomes less than or equal to 1/2 full"]
    #[inline(always)]
    pub fn _12full(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::_12full)
    }
    #[doc = "Transmit FIFO becomes less than or equal to 3/4 full"]
    #[inline(always)]
    pub fn _34full(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::_34full)
    }
    #[doc = "Transmit FIFO becomes less than or equal to 7/8 full"]
    #[inline(always)]
    pub fn _78full(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::_78full)
    }
}
#[doc = "Receive interrupt FIFO level select\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxiflsel {
    #[doc = "0: Receive FIFO becomes greater than or equal to 1/8 full"]
    _18full = 0,
    #[doc = "1: Receive FIFO becomes greater than or equal to 1/4 full"]
    _14full = 1,
    #[doc = "2: Receive FIFO becomes greater than or equal to 1/2 full"]
    _12full = 2,
    #[doc = "3: Receive FIFO becomes greater than or equal to 3/4 full"]
    _34full = 3,
    #[doc = "4: Receive FIFO becomes greater than or equal to 7/8 full"]
    _78full = 4,
}
impl From<Rxiflsel> for u8 {
    #[inline(always)]
    fn from(variant: Rxiflsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxiflsel {
    type Ux = u8;
}
impl crate::IsEnum for Rxiflsel {}
#[doc = "Field `RXIFLSEL` reader - Receive interrupt FIFO level select"]
pub type RxiflselR = crate::FieldReader<Rxiflsel>;
impl RxiflselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rxiflsel> {
        match self.bits {
            0 => Some(Rxiflsel::_18full),
            1 => Some(Rxiflsel::_14full),
            2 => Some(Rxiflsel::_12full),
            3 => Some(Rxiflsel::_34full),
            4 => Some(Rxiflsel::_78full),
            _ => None,
        }
    }
    #[doc = "Receive FIFO becomes greater than or equal to 1/8 full"]
    #[inline(always)]
    pub fn is_18full(&self) -> bool {
        *self == Rxiflsel::_18full
    }
    #[doc = "Receive FIFO becomes greater than or equal to 1/4 full"]
    #[inline(always)]
    pub fn is_14full(&self) -> bool {
        *self == Rxiflsel::_14full
    }
    #[doc = "Receive FIFO becomes greater than or equal to 1/2 full"]
    #[inline(always)]
    pub fn is_12full(&self) -> bool {
        *self == Rxiflsel::_12full
    }
    #[doc = "Receive FIFO becomes greater than or equal to 3/4 full"]
    #[inline(always)]
    pub fn is_34full(&self) -> bool {
        *self == Rxiflsel::_34full
    }
    #[doc = "Receive FIFO becomes greater than or equal to 7/8 full"]
    #[inline(always)]
    pub fn is_78full(&self) -> bool {
        *self == Rxiflsel::_78full
    }
}
#[doc = "Field `RXIFLSEL` writer - Receive interrupt FIFO level select"]
pub type RxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rxiflsel>;
impl<'a, REG> RxiflselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Receive FIFO becomes greater than or equal to 1/8 full"]
    #[inline(always)]
    pub fn _18full(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::_18full)
    }
    #[doc = "Receive FIFO becomes greater than or equal to 1/4 full"]
    #[inline(always)]
    pub fn _14full(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::_14full)
    }
    #[doc = "Receive FIFO becomes greater than or equal to 1/2 full"]
    #[inline(always)]
    pub fn _12full(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::_12full)
    }
    #[doc = "Receive FIFO becomes greater than or equal to 3/4 full"]
    #[inline(always)]
    pub fn _34full(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::_34full)
    }
    #[doc = "Receive FIFO becomes greater than or equal to 7/8 full"]
    #[inline(always)]
    pub fn _78full(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::_78full)
    }
}
impl R {
    #[doc = "Bits 0:2 - Transmit interrupt FIFO level select"]
    #[inline(always)]
    pub fn txiflsel(&self) -> TxiflselR {
        TxiflselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Receive interrupt FIFO level select"]
    #[inline(always)]
    pub fn rxiflsel(&self) -> RxiflselR {
        RxiflselR::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Transmit interrupt FIFO level select"]
    #[inline(always)]
    pub fn txiflsel(&mut self) -> TxiflselW<UartiflsSpec> {
        TxiflselW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Receive interrupt FIFO level select"]
    #[inline(always)]
    pub fn rxiflsel(&mut self) -> RxiflselW<UartiflsSpec> {
        RxiflselW::new(self, 3)
    }
}
#[doc = "Interrupt FIFO level select register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartifls::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartifls::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartiflsSpec;
impl crate::RegisterSpec for UartiflsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartifls::R`](R) reader structure"]
impl crate::Readable for UartiflsSpec {}
#[doc = "`write(|w| ..)` method takes [`uartifls::W`](W) writer structure"]
impl crate::Writable for UartiflsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTIFLS to value 0x12"]
impl crate::Resettable for UartiflsSpec {
    const RESET_VALUE: u32 = 0x12;
}
