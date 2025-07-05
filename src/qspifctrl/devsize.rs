#[doc = "Register `DEVSIZE` reader"]
pub type R = crate::R<DevsizeSpec>;
#[doc = "Register `DEVSIZE` writer"]
pub type W = crate::W<DevsizeSpec>;
#[doc = "Field `ADDRBYTENUM` reader - Number of address bytes"]
pub type AddrbytenumR = crate::FieldReader;
#[doc = "Field `ADDRBYTENUM` writer - Number of address bytes"]
pub type AddrbytenumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BYTEPERDEVPGNUM` reader - Number of bytes per device page"]
pub type ByteperdevpgnumR = crate::FieldReader<u16>;
#[doc = "Field `BYTEPERDEVPGNUM` writer - Number of bytes per device page"]
pub type ByteperdevpgnumW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BYTEPERBLKNUM` reader - Number of bytes per block"]
pub type ByteperblknumR = crate::FieldReader;
#[doc = "Field `BYTEPERBLKNUM` writer - Number of bytes per block"]
pub type ByteperblknumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FDEVSIZECS0` reader - Size of Flash Device connected to CS\\[0\\]
pin"]
pub type Fdevsizecs0R = crate::FieldReader;
#[doc = "Field `FDEVSIZECS0` writer - Size of Flash Device connected to CS\\[0\\]
pin"]
pub type Fdevsizecs0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FDEVSIZECS1` reader - Size of Flash Device connected to CS\\[1\\]
pin"]
pub type Fdevsizecs1R = crate::FieldReader;
#[doc = "Field `FDEVSIZECS1` writer - Size of Flash Device connected to CS\\[1\\]
pin"]
pub type Fdevsizecs1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FDEVSIZECS2` reader - Size of Flash Device connected to CS\\[2\\]
pin"]
pub type Fdevsizecs2R = crate::FieldReader;
#[doc = "Field `FDEVSIZECS2` writer - Size of Flash Device connected to CS\\[2\\]
pin"]
pub type Fdevsizecs2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FDEVSIZECS3` reader - Size of Flash Device connected to CS\\[3\\]
pin"]
pub type Fdevsizecs3R = crate::FieldReader;
#[doc = "Field `FDEVSIZECS3` writer - Size of Flash Device connected to CS\\[3\\]
pin"]
pub type Fdevsizecs3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Number of address bytes"]
    #[inline(always)]
    pub fn addrbytenum(&self) -> AddrbytenumR {
        AddrbytenumR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Number of bytes per device page"]
    #[inline(always)]
    pub fn byteperdevpgnum(&self) -> ByteperdevpgnumR {
        ByteperdevpgnumR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:20 - Number of bytes per block"]
    #[inline(always)]
    pub fn byteperblknum(&self) -> ByteperblknumR {
        ByteperblknumR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - Size of Flash Device connected to CS\\[0\\]
pin"]
    #[inline(always)]
    pub fn fdevsizecs0(&self) -> Fdevsizecs0R {
        Fdevsizecs0R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - Size of Flash Device connected to CS\\[1\\]
pin"]
    #[inline(always)]
    pub fn fdevsizecs1(&self) -> Fdevsizecs1R {
        Fdevsizecs1R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:26 - Size of Flash Device connected to CS\\[2\\]
pin"]
    #[inline(always)]
    pub fn fdevsizecs2(&self) -> Fdevsizecs2R {
        Fdevsizecs2R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:28 - Size of Flash Device connected to CS\\[3\\]
pin"]
    #[inline(always)]
    pub fn fdevsizecs3(&self) -> Fdevsizecs3R {
        Fdevsizecs3R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Number of address bytes"]
    #[inline(always)]
    pub fn addrbytenum(&mut self) -> AddrbytenumW<DevsizeSpec> {
        AddrbytenumW::new(self, 0)
    }
    #[doc = "Bits 4:15 - Number of bytes per device page"]
    #[inline(always)]
    pub fn byteperdevpgnum(&mut self) -> ByteperdevpgnumW<DevsizeSpec> {
        ByteperdevpgnumW::new(self, 4)
    }
    #[doc = "Bits 16:20 - Number of bytes per block"]
    #[inline(always)]
    pub fn byteperblknum(&mut self) -> ByteperblknumW<DevsizeSpec> {
        ByteperblknumW::new(self, 16)
    }
    #[doc = "Bits 21:22 - Size of Flash Device connected to CS\\[0\\]
pin"]
    #[inline(always)]
    pub fn fdevsizecs0(&mut self) -> Fdevsizecs0W<DevsizeSpec> {
        Fdevsizecs0W::new(self, 21)
    }
    #[doc = "Bits 23:24 - Size of Flash Device connected to CS\\[1\\]
pin"]
    #[inline(always)]
    pub fn fdevsizecs1(&mut self) -> Fdevsizecs1W<DevsizeSpec> {
        Fdevsizecs1W::new(self, 23)
    }
    #[doc = "Bits 25:26 - Size of Flash Device connected to CS\\[2\\]
pin"]
    #[inline(always)]
    pub fn fdevsizecs2(&mut self) -> Fdevsizecs2W<DevsizeSpec> {
        Fdevsizecs2W::new(self, 25)
    }
    #[doc = "Bits 27:28 - Size of Flash Device connected to CS\\[3\\]
pin"]
    #[inline(always)]
    pub fn fdevsizecs3(&mut self) -> Fdevsizecs3W<DevsizeSpec> {
        Fdevsizecs3W::new(self, 27)
    }
}
#[doc = "Device Size Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devsize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devsize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevsizeSpec;
impl crate::RegisterSpec for DevsizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devsize::R`](R) reader structure"]
impl crate::Readable for DevsizeSpec {}
#[doc = "`write(|w| ..)` method takes [`devsize::W`](W) writer structure"]
impl crate::Writable for DevsizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVSIZE to value 0x0010_1002"]
impl crate::Resettable for DevsizeSpec {
    const RESET_VALUE: u32 = 0x0010_1002;
}
