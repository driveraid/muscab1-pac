#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Security error response configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bit4 {
    #[doc = "0: Read-As-Zero - Writes ignored"]
    Razwi = 0,
    #[doc = "1: Bus Error"]
    Buserror = 1,
}
impl From<Bit4> for bool {
    #[inline(always)]
    fn from(variant: Bit4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `bit[4]` reader - Security error response configuration"]
pub type Bit4R = crate::BitReader<Bit4>;
impl Bit4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bit4 {
        match self.bits {
            false => Bit4::Razwi,
            true => Bit4::Buserror,
        }
    }
    #[doc = "Read-As-Zero - Writes ignored"]
    #[inline(always)]
    pub fn is_razwi(&self) -> bool {
        *self == Bit4::Razwi
    }
    #[doc = "Bus Error"]
    #[inline(always)]
    pub fn is_buserror(&self) -> bool {
        *self == Bit4::Buserror
    }
}
#[doc = "Field `bit[4]` writer - Security error response configuration"]
pub type Bit4W<'a, REG> = crate::BitWriter<'a, REG, Bit4>;
impl<'a, REG> Bit4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read-As-Zero - Writes ignored"]
    #[inline(always)]
    pub fn razwi(self) -> &'a mut crate::W<REG> {
        self.variant(Bit4::Razwi)
    }
    #[doc = "Bus Error"]
    #[inline(always)]
    pub fn buserror(self) -> &'a mut crate::W<REG> {
        self.variant(Bit4::Buserror)
    }
}
#[doc = "Field `bit[6]` reader - Data interface gating request"]
pub type Bit6R = crate::BitReader;
#[doc = "Field `bit[6]` writer - Data interface gating request"]
pub type Bit6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit[7]` reader - Data interface gating acknowledge (RO)"]
pub type Bit7R = crate::BitReader;
#[doc = "Field `bit[7]` writer - Data interface gating acknowledge (RO)"]
pub type Bit7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit[8]` reader - Auto-increment"]
pub type Bit8R = crate::BitReader;
#[doc = "Field `bit[8]` writer - Auto-increment"]
pub type Bit8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit[31]` reader - Security lockdown"]
pub type Bit31R = crate::BitReader;
#[doc = "Field `bit[31]` writer - Security lockdown"]
pub type Bit31W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Security error response configuration"]
    #[inline(always)]
    pub fn bit4(&self) -> Bit4R {
        Bit4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Data interface gating request"]
    #[inline(always)]
    pub fn bit6(&self) -> Bit6R {
        Bit6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data interface gating acknowledge (RO)"]
    #[inline(always)]
    pub fn bit7(&self) -> Bit7R {
        Bit7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Auto-increment"]
    #[inline(always)]
    pub fn bit8(&self) -> Bit8R {
        Bit8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 31 - Security lockdown"]
    #[inline(always)]
    pub fn bit31(&self) -> Bit31R {
        Bit31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Security error response configuration"]
    #[inline(always)]
    pub fn bit4(&mut self) -> Bit4W<CtrlSpec> {
        Bit4W::new(self, 4)
    }
    #[doc = "Bit 6 - Data interface gating request"]
    #[inline(always)]
    pub fn bit6(&mut self) -> Bit6W<CtrlSpec> {
        Bit6W::new(self, 6)
    }
    #[doc = "Bit 7 - Data interface gating acknowledge (RO)"]
    #[inline(always)]
    pub fn bit7(&mut self) -> Bit7W<CtrlSpec> {
        Bit7W::new(self, 7)
    }
    #[doc = "Bit 8 - Auto-increment"]
    #[inline(always)]
    pub fn bit8(&mut self) -> Bit8W<CtrlSpec> {
        Bit8W::new(self, 8)
    }
    #[doc = "Bit 31 - Security lockdown"]
    #[inline(always)]
    pub fn bit31(&mut self) -> Bit31W<CtrlSpec> {
        Bit31W::new(self, 31)
    }
}
#[doc = "MPC Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
