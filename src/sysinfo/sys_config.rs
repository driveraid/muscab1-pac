#[doc = "Register `SYS_CONFIG` reader"]
pub type R = crate::R<SysConfigSpec>;
#[doc = "Field `SRAM_NUM_BANK` reader - SRAM Number of Banks"]
pub type SramNumBankR = crate::FieldReader;
#[doc = "Field `SRAM_ADDR_WIDTH` reader - SRAM Bank Address Width"]
pub type SramAddrWidthR = crate::FieldReader;
#[doc = "CPU 0 has Data TCM:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpu0HasTcm {
    #[doc = "0: CPU 0 does not have Data TCM"]
    No = 0,
    #[doc = "1: CPU 0 has Data TCM"]
    Yes = 1,
}
impl From<Cpu0HasTcm> for bool {
    #[inline(always)]
    fn from(variant: Cpu0HasTcm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU0_HAS_TCM` reader - CPU 0 has Data TCM:"]
pub type Cpu0HasTcmR = crate::BitReader<Cpu0HasTcm>;
impl Cpu0HasTcmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu0HasTcm {
        match self.bits {
            false => Cpu0HasTcm::No,
            true => Cpu0HasTcm::Yes,
        }
    }
    #[doc = "CPU 0 does not have Data TCM"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Cpu0HasTcm::No
    }
    #[doc = "CPU 0 has Data TCM"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Cpu0HasTcm::Yes
    }
}
#[doc = "CPU 1 has Data TCM:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpu1HasTcm {
    #[doc = "0: CPU 1 does not have Data TCM"]
    No = 0,
    #[doc = "1: CPU 1 has Data TCM"]
    Yes = 1,
}
impl From<Cpu1HasTcm> for bool {
    #[inline(always)]
    fn from(variant: Cpu1HasTcm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU1_HAS_TCM` reader - CPU 1 has Data TCM:"]
pub type Cpu1HasTcmR = crate::BitReader<Cpu1HasTcm>;
impl Cpu1HasTcmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu1HasTcm {
        match self.bits {
            false => Cpu1HasTcm::No,
            true => Cpu1HasTcm::Yes,
        }
    }
    #[doc = "CPU 1 does not have Data TCM"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Cpu1HasTcm::No
    }
    #[doc = "CPU 1 has Data TCM"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Cpu1HasTcm::Yes
    }
}
#[doc = "Whether CryptoCell Included:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HasCrypto {
    #[doc = "0: CryptoCell Not Included"]
    No = 0,
    #[doc = "1: CryptoCell Included"]
    Yes = 1,
}
impl From<HasCrypto> for bool {
    #[inline(always)]
    fn from(variant: HasCrypto) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HAS_CRYPTO` reader - Whether CryptoCell Included:"]
pub type HasCryptoR = crate::BitReader<HasCrypto>;
impl HasCryptoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HasCrypto {
        match self.bits {
            false => HasCrypto::No,
            true => HasCrypto::Yes,
        }
    }
    #[doc = "CryptoCell Not Included"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == HasCrypto::No
    }
    #[doc = "CryptoCell Included"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == HasCrypto::Yes
    }
}
#[doc = "Field `CPU0_TCM_BANK_NUM` reader - The SRAM Bank that maps CPU0 Data TCM"]
pub type Cpu0TcmBankNumR = crate::FieldReader;
#[doc = "Number of SRAM banks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpu1TcmBankNum {
    #[doc = "3: 4 SRAM Banks"]
    Four = 3,
    #[doc = "2: 3 SRAM Banks"]
    Three = 2,
    #[doc = "1: 2 SRAM Banks"]
    Two = 1,
    #[doc = "0: Otherwise"]
    Otherwise = 0,
}
impl From<Cpu1TcmBankNum> for u8 {
    #[inline(always)]
    fn from(variant: Cpu1TcmBankNum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpu1TcmBankNum {
    type Ux = u8;
}
impl crate::IsEnum for Cpu1TcmBankNum {}
#[doc = "Field `CPU1_TCM_BANK_NUM` reader - Number of SRAM banks"]
pub type Cpu1TcmBankNumR = crate::FieldReader<Cpu1TcmBankNum>;
impl Cpu1TcmBankNumR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cpu1TcmBankNum> {
        match self.bits {
            3 => Some(Cpu1TcmBankNum::Four),
            2 => Some(Cpu1TcmBankNum::Three),
            1 => Some(Cpu1TcmBankNum::Two),
            0 => Some(Cpu1TcmBankNum::Otherwise),
            _ => None,
        }
    }
    #[doc = "4 SRAM Banks"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == Cpu1TcmBankNum::Four
    }
    #[doc = "3 SRAM Banks"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == Cpu1TcmBankNum::Three
    }
    #[doc = "2 SRAM Banks"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Cpu1TcmBankNum::Two
    }
    #[doc = "Otherwise"]
    #[inline(always)]
    pub fn is_otherwise(&self) -> bool {
        *self == Cpu1TcmBankNum::Otherwise
    }
}
#[doc = "CPU 0 Core Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpu0Type {
    #[doc = "0: Does Not Exist"]
    NotExist = 0,
    #[doc = "2: Cortex-M33 Core"]
    Cm33 = 2,
}
impl From<Cpu0Type> for u8 {
    #[inline(always)]
    fn from(variant: Cpu0Type) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpu0Type {
    type Ux = u8;
}
impl crate::IsEnum for Cpu0Type {}
#[doc = "Field `CPU0_TYPE` reader - CPU 0 Core Type"]
pub type Cpu0TypeR = crate::FieldReader<Cpu0Type>;
impl Cpu0TypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cpu0Type> {
        match self.bits {
            0 => Some(Cpu0Type::NotExist),
            2 => Some(Cpu0Type::Cm33),
            _ => None,
        }
    }
    #[doc = "Does Not Exist"]
    #[inline(always)]
    pub fn is_not_exist(&self) -> bool {
        *self == Cpu0Type::NotExist
    }
    #[doc = "Cortex-M33 Core"]
    #[inline(always)]
    pub fn is_cm33(&self) -> bool {
        *self == Cpu0Type::Cm33
    }
}
#[doc = "CPU 1 Core Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpu1Type {
    #[doc = "0: Does Not Exist"]
    NotExist = 0,
    #[doc = "2: Cortex-M33 Core"]
    Cm33 = 2,
}
impl From<Cpu1Type> for u8 {
    #[inline(always)]
    fn from(variant: Cpu1Type) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpu1Type {
    type Ux = u8;
}
impl crate::IsEnum for Cpu1Type {}
#[doc = "Field `CPU1_TYPE` reader - CPU 1 Core Type"]
pub type Cpu1TypeR = crate::FieldReader<Cpu1Type>;
impl Cpu1TypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cpu1Type> {
        match self.bits {
            0 => Some(Cpu1Type::NotExist),
            2 => Some(Cpu1Type::Cm33),
            _ => None,
        }
    }
    #[doc = "Does Not Exist"]
    #[inline(always)]
    pub fn is_not_exist(&self) -> bool {
        *self == Cpu1Type::NotExist
    }
    #[doc = "Cortex-M33 Core"]
    #[inline(always)]
    pub fn is_cm33(&self) -> bool {
        *self == Cpu1Type::Cm33
    }
}
impl R {
    #[doc = "Bits 0:3 - SRAM Number of Banks"]
    #[inline(always)]
    pub fn sram_num_bank(&self) -> SramNumBankR {
        SramNumBankR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - SRAM Bank Address Width"]
    #[inline(always)]
    pub fn sram_addr_width(&self) -> SramAddrWidthR {
        SramAddrWidthR::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 9 - CPU 0 has Data TCM:"]
    #[inline(always)]
    pub fn cpu0_has_tcm(&self) -> Cpu0HasTcmR {
        Cpu0HasTcmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU 1 has Data TCM:"]
    #[inline(always)]
    pub fn cpu1_has_tcm(&self) -> Cpu1HasTcmR {
        Cpu1HasTcmR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Whether CryptoCell Included:"]
    #[inline(always)]
    pub fn has_crypto(&self) -> HasCryptoR {
        HasCryptoR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - The SRAM Bank that maps CPU0 Data TCM"]
    #[inline(always)]
    pub fn cpu0_tcm_bank_num(&self) -> Cpu0TcmBankNumR {
        Cpu0TcmBankNumR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Number of SRAM banks"]
    #[inline(always)]
    pub fn cpu1_tcm_bank_num(&self) -> Cpu1TcmBankNumR {
        Cpu1TcmBankNumR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - CPU 0 Core Type"]
    #[inline(always)]
    pub fn cpu0_type(&self) -> Cpu0TypeR {
        Cpu0TypeR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - CPU 1 Core Type"]
    #[inline(always)]
    pub fn cpu1_type(&self) -> Cpu1TypeR {
        Cpu1TypeR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "System Hardware Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_config::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysConfigSpec;
impl crate::RegisterSpec for SysConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_config::R`](R) reader structure"]
impl crate::Readable for SysConfigSpec {}
#[doc = "`reset()` method sets SYS_CONFIG to value 0"]
impl crate::Resettable for SysConfigSpec {}
