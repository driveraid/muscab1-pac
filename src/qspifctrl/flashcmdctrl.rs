#[doc = "Register `FLASHCMDCTRL` reader"]
pub type R = crate::R<FlashcmdctrlSpec>;
#[doc = "Register `FLASHCMDCTRL` writer"]
pub type W = crate::W<FlashcmdctrlSpec>;
#[doc = "Field `CMDEXEC` reader - Execute the command"]
pub type CmdexecR = crate::BitReader;
#[doc = "Field `CMDEXEC` writer - Execute the command"]
pub type CmdexecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDEXINPROG` reader - Command execution in progress"]
pub type CmdexinprogR = crate::BitReader;
#[doc = "Field `CMDEXINPROG` writer - Command execution in progress"]
pub type CmdexinprogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUMCYCNUM` reader - Number of Dummy Cycles"]
pub type DumcycnumR = crate::FieldReader;
#[doc = "Field `DUMCYCNUM` writer - Number of Dummy Cycles"]
pub type DumcycnumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WRDATABYTENUM` reader - Number of Write Data Bytes"]
pub type WrdatabytenumR = crate::FieldReader;
#[doc = "Field `WRDATABYTENUM` writer - Number of Write Data Bytes"]
pub type WrdatabytenumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WRDATAEN` reader - Write Data Enable"]
pub type WrdataenR = crate::BitReader;
#[doc = "Field `WRDATAEN` writer - Write Data Enable"]
pub type WrdataenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRBYTENUM` reader - Number of Address Bytes"]
pub type AddrbytenumR = crate::FieldReader;
#[doc = "Field `ADDRBYTENUM` writer - Number of Address Bytes"]
pub type AddrbytenumW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODEBITEN` reader - Mode Bit Enable"]
pub type ModebitenR = crate::BitReader;
#[doc = "Field `MODEBITEN` writer - Mode Bit Enable"]
pub type ModebitenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDADDREN` reader - Command Address Enable"]
pub type CmdaddrenR = crate::BitReader;
#[doc = "Field `CMDADDREN` writer - Command Address Enable"]
pub type CmdaddrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDATABYTENUM` reader - Number of Read Data Bytes"]
pub type RdatabytenumR = crate::FieldReader;
#[doc = "Field `RDATABYTENUM` writer - Number of Read Data Bytes"]
pub type RdatabytenumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RDATAEN` reader - Read Data Enable"]
pub type RdataenR = crate::BitReader;
#[doc = "Field `RDATAEN` writer - Read Data Enable"]
pub type RdataenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDOPCODE` reader - Command Opcode"]
pub type CmdopcodeR = crate::FieldReader;
#[doc = "Field `CMDOPCODE` writer - Command Opcode"]
pub type CmdopcodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Execute the command"]
    #[inline(always)]
    pub fn cmdexec(&self) -> CmdexecR {
        CmdexecR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command execution in progress"]
    #[inline(always)]
    pub fn cmdexinprog(&self) -> CmdexinprogR {
        CmdexinprogR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 7:11 - Number of Dummy Cycles"]
    #[inline(always)]
    pub fn dumcycnum(&self) -> DumcycnumR {
        DumcycnumR::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:14 - Number of Write Data Bytes"]
    #[inline(always)]
    pub fn wrdatabytenum(&self) -> WrdatabytenumR {
        WrdatabytenumR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Write Data Enable"]
    #[inline(always)]
    pub fn wrdataen(&self) -> WrdataenR {
        WrdataenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Number of Address Bytes"]
    #[inline(always)]
    pub fn addrbytenum(&self) -> AddrbytenumR {
        AddrbytenumR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Mode Bit Enable"]
    #[inline(always)]
    pub fn modebiten(&self) -> ModebitenR {
        ModebitenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Command Address Enable"]
    #[inline(always)]
    pub fn cmdaddren(&self) -> CmdaddrenR {
        CmdaddrenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Number of Read Data Bytes"]
    #[inline(always)]
    pub fn rdatabytenum(&self) -> RdatabytenumR {
        RdatabytenumR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Read Data Enable"]
    #[inline(always)]
    pub fn rdataen(&self) -> RdataenR {
        RdataenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Command Opcode"]
    #[inline(always)]
    pub fn cmdopcode(&self) -> CmdopcodeR {
        CmdopcodeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Execute the command"]
    #[inline(always)]
    pub fn cmdexec(&mut self) -> CmdexecW<FlashcmdctrlSpec> {
        CmdexecW::new(self, 0)
    }
    #[doc = "Bit 1 - Command execution in progress"]
    #[inline(always)]
    pub fn cmdexinprog(&mut self) -> CmdexinprogW<FlashcmdctrlSpec> {
        CmdexinprogW::new(self, 1)
    }
    #[doc = "Bits 7:11 - Number of Dummy Cycles"]
    #[inline(always)]
    pub fn dumcycnum(&mut self) -> DumcycnumW<FlashcmdctrlSpec> {
        DumcycnumW::new(self, 7)
    }
    #[doc = "Bits 12:14 - Number of Write Data Bytes"]
    #[inline(always)]
    pub fn wrdatabytenum(&mut self) -> WrdatabytenumW<FlashcmdctrlSpec> {
        WrdatabytenumW::new(self, 12)
    }
    #[doc = "Bit 15 - Write Data Enable"]
    #[inline(always)]
    pub fn wrdataen(&mut self) -> WrdataenW<FlashcmdctrlSpec> {
        WrdataenW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Number of Address Bytes"]
    #[inline(always)]
    pub fn addrbytenum(&mut self) -> AddrbytenumW<FlashcmdctrlSpec> {
        AddrbytenumW::new(self, 16)
    }
    #[doc = "Bit 18 - Mode Bit Enable"]
    #[inline(always)]
    pub fn modebiten(&mut self) -> ModebitenW<FlashcmdctrlSpec> {
        ModebitenW::new(self, 18)
    }
    #[doc = "Bit 19 - Command Address Enable"]
    #[inline(always)]
    pub fn cmdaddren(&mut self) -> CmdaddrenW<FlashcmdctrlSpec> {
        CmdaddrenW::new(self, 19)
    }
    #[doc = "Bits 20:22 - Number of Read Data Bytes"]
    #[inline(always)]
    pub fn rdatabytenum(&mut self) -> RdatabytenumW<FlashcmdctrlSpec> {
        RdatabytenumW::new(self, 20)
    }
    #[doc = "Bit 23 - Read Data Enable"]
    #[inline(always)]
    pub fn rdataen(&mut self) -> RdataenW<FlashcmdctrlSpec> {
        RdataenW::new(self, 23)
    }
    #[doc = "Bits 24:31 - Command Opcode"]
    #[inline(always)]
    pub fn cmdopcode(&mut self) -> CmdopcodeW<FlashcmdctrlSpec> {
        CmdopcodeW::new(self, 24)
    }
}
#[doc = "Flash Command Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashcmdctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashcmdctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashcmdctrlSpec;
impl crate::RegisterSpec for FlashcmdctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashcmdctrl::R`](R) reader structure"]
impl crate::Readable for FlashcmdctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`flashcmdctrl::W`](W) writer structure"]
impl crate::Writable for FlashcmdctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASHCMDCTRL to value 0"]
impl crate::Resettable for FlashcmdctrlSpec {
    const RESET_VALUE: u32 = 0;
}
