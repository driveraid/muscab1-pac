#[doc = "Register `DEVREADINSTR` reader"]
pub type R = crate::R<DevreadinstrSpec>;
#[doc = "Register `DEVREADINSTR` writer"]
pub type W = crate::W<DevreadinstrSpec>;
#[doc = "Field `ROPCODE` reader - Read Opcode to use when not in XIP mode"]
pub type RopcodeR = crate::FieldReader;
#[doc = "Field `ROPCODE` writer - Read Opcode to use when not in XIP mode"]
pub type RopcodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INSTRTYPE` reader - Instruction Type"]
pub type InstrtypeR = crate::FieldReader;
#[doc = "Field `INSTRTYPE` writer - Instruction Type"]
pub type InstrtypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DDRBITEN` reader - DDR Bit Enable"]
pub type DdrbitenR = crate::BitReader;
#[doc = "Field `DDRBITEN` writer - DDR Bit Enable"]
pub type DdrbitenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRTRTYPESSPI` reader - Address Transfer Type for Standard SPI modes"]
pub type AddrtrtypesspiR = crate::FieldReader;
#[doc = "Field `ADDRTRTYPESSPI` writer - Address Transfer Type for Standard SPI modes"]
pub type AddrtrtypesspiW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DATATRTYPESSPI` reader - Data Transfer Type for Standard SPI modes"]
pub type DatatrtypesspiR = crate::FieldReader;
#[doc = "Field `DATATRTYPESSPI` writer - Data Transfer Type for Standard SPI modes"]
pub type DatatrtypesspiW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODEBITEN` reader - Mode Bit Enable"]
pub type ModebitenR = crate::BitReader;
#[doc = "Field `MODEBITEN` writer - Mode Bit Enable"]
pub type ModebitenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READDUMCLKCYCNUM` reader - Number of Dummy Clock Cycles required by device for Read Instruction"]
pub type ReaddumclkcycnumR = crate::FieldReader;
#[doc = "Field `READDUMCLKCYCNUM` writer - Number of Dummy Clock Cycles required by device for Read Instruction"]
pub type ReaddumclkcycnumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - Read Opcode to use when not in XIP mode"]
    #[inline(always)]
    pub fn ropcode(&self) -> RopcodeR {
        RopcodeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Instruction Type"]
    #[inline(always)]
    pub fn instrtype(&self) -> InstrtypeR {
        InstrtypeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - DDR Bit Enable"]
    #[inline(always)]
    pub fn ddrbiten(&self) -> DdrbitenR {
        DdrbitenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Address Transfer Type for Standard SPI modes"]
    #[inline(always)]
    pub fn addrtrtypesspi(&self) -> AddrtrtypesspiR {
        AddrtrtypesspiR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Data Transfer Type for Standard SPI modes"]
    #[inline(always)]
    pub fn datatrtypesspi(&self) -> DatatrtypesspiR {
        DatatrtypesspiR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - Mode Bit Enable"]
    #[inline(always)]
    pub fn modebiten(&self) -> ModebitenR {
        ModebitenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Number of Dummy Clock Cycles required by device for Read Instruction"]
    #[inline(always)]
    pub fn readdumclkcycnum(&self) -> ReaddumclkcycnumR {
        ReaddumclkcycnumR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read Opcode to use when not in XIP mode"]
    #[inline(always)]
    pub fn ropcode(&mut self) -> RopcodeW<DevreadinstrSpec> {
        RopcodeW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Instruction Type"]
    #[inline(always)]
    pub fn instrtype(&mut self) -> InstrtypeW<DevreadinstrSpec> {
        InstrtypeW::new(self, 8)
    }
    #[doc = "Bit 10 - DDR Bit Enable"]
    #[inline(always)]
    pub fn ddrbiten(&mut self) -> DdrbitenW<DevreadinstrSpec> {
        DdrbitenW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Address Transfer Type for Standard SPI modes"]
    #[inline(always)]
    pub fn addrtrtypesspi(&mut self) -> AddrtrtypesspiW<DevreadinstrSpec> {
        AddrtrtypesspiW::new(self, 12)
    }
    #[doc = "Bits 16:17 - Data Transfer Type for Standard SPI modes"]
    #[inline(always)]
    pub fn datatrtypesspi(&mut self) -> DatatrtypesspiW<DevreadinstrSpec> {
        DatatrtypesspiW::new(self, 16)
    }
    #[doc = "Bit 20 - Mode Bit Enable"]
    #[inline(always)]
    pub fn modebiten(&mut self) -> ModebitenW<DevreadinstrSpec> {
        ModebitenW::new(self, 20)
    }
    #[doc = "Bits 24:28 - Number of Dummy Clock Cycles required by device for Read Instruction"]
    #[inline(always)]
    pub fn readdumclkcycnum(&mut self) -> ReaddumclkcycnumW<DevreadinstrSpec> {
        ReaddumclkcycnumW::new(self, 24)
    }
}
#[doc = "Device Read Instruction Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devreadinstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devreadinstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevreadinstrSpec;
impl crate::RegisterSpec for DevreadinstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devreadinstr::R`](R) reader structure"]
impl crate::Readable for DevreadinstrSpec {}
#[doc = "`write(|w| ..)` method takes [`devreadinstr::W`](W) writer structure"]
impl crate::Writable for DevreadinstrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVREADINSTR to value 0x03"]
impl crate::Resettable for DevreadinstrSpec {
    const RESET_VALUE: u32 = 0x03;
}
