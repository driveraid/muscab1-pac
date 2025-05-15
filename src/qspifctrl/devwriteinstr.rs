#[doc = "Register `DEVWRITEINSTR` reader"]
pub type R = crate::R<DevwriteinstrSpec>;
#[doc = "Register `DEVWRITEINSTR` writer"]
pub type W = crate::W<DevwriteinstrSpec>;
#[doc = "Field `WROPCODE` reader - Write Opcode"]
pub type WropcodeR = crate::FieldReader;
#[doc = "Field `WROPCODE` writer - Write Opcode"]
pub type WropcodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WELDISABLE` reader - WEL Disable"]
pub type WeldisableR = crate::BitReader;
#[doc = "Field `WELDISABLE` writer - WEL Disable"]
pub type WeldisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRTRTYPESSPI` reader - Address Transfer Type for Standard SPI modes"]
pub type AddrtrtypesspiR = crate::FieldReader;
#[doc = "Field `ADDRTRTYPESSPI` writer - Address Transfer Type for Standard SPI modes"]
pub type AddrtrtypesspiW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DATATRTYPESSPI` reader - Data Transfer Type for Standard SPI modes"]
pub type DatatrtypesspiR = crate::FieldReader;
#[doc = "Field `DATATRTYPESSPI` writer - Data Transfer Type for Standard SPI modes"]
pub type DatatrtypesspiW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WRITEDUMCLKCYCNUM` reader - Number of Dummy Clock Cycles required by device for Write Instruction"]
pub type WritedumclkcycnumR = crate::FieldReader;
#[doc = "Field `WRITEDUMCLKCYCNUM` writer - Number of Dummy Clock Cycles required by device for Write Instruction"]
pub type WritedumclkcycnumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - Write Opcode"]
    #[inline(always)]
    pub fn wropcode(&self) -> WropcodeR {
        WropcodeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - WEL Disable"]
    #[inline(always)]
    pub fn weldisable(&self) -> WeldisableR {
        WeldisableR::new(((self.bits >> 8) & 1) != 0)
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
    #[doc = "Bits 24:28 - Number of Dummy Clock Cycles required by device for Write Instruction"]
    #[inline(always)]
    pub fn writedumclkcycnum(&self) -> WritedumclkcycnumR {
        WritedumclkcycnumR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Write Opcode"]
    #[inline(always)]
    pub fn wropcode(&mut self) -> WropcodeW<DevwriteinstrSpec> {
        WropcodeW::new(self, 0)
    }
    #[doc = "Bit 8 - WEL Disable"]
    #[inline(always)]
    pub fn weldisable(&mut self) -> WeldisableW<DevwriteinstrSpec> {
        WeldisableW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Address Transfer Type for Standard SPI modes"]
    #[inline(always)]
    pub fn addrtrtypesspi(&mut self) -> AddrtrtypesspiW<DevwriteinstrSpec> {
        AddrtrtypesspiW::new(self, 12)
    }
    #[doc = "Bits 16:17 - Data Transfer Type for Standard SPI modes"]
    #[inline(always)]
    pub fn datatrtypesspi(&mut self) -> DatatrtypesspiW<DevwriteinstrSpec> {
        DatatrtypesspiW::new(self, 16)
    }
    #[doc = "Bits 24:28 - Number of Dummy Clock Cycles required by device for Write Instruction"]
    #[inline(always)]
    pub fn writedumclkcycnum(&mut self) -> WritedumclkcycnumW<DevwriteinstrSpec> {
        WritedumclkcycnumW::new(self, 24)
    }
}
#[doc = "Device Write Instruction Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devwriteinstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devwriteinstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevwriteinstrSpec;
impl crate::RegisterSpec for DevwriteinstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devwriteinstr::R`](R) reader structure"]
impl crate::Readable for DevwriteinstrSpec {}
#[doc = "`write(|w| ..)` method takes [`devwriteinstr::W`](W) writer structure"]
impl crate::Writable for DevwriteinstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVWRITEINSTR to value 0x02"]
impl crate::Resettable for DevwriteinstrSpec {
    const RESET_VALUE: u32 = 0x02;
}
