#[doc = "Register `SYS_VERSION` reader"]
pub type R = crate::R<SysVersionSpec>;
#[doc = "Field `PART_NUMBER` reader - Part Number for the SSE-200"]
pub type PartNumberR = crate::FieldReader<u16>;
#[doc = "Field `DESIGNER_ID` reader - Arm Product with designer code 0x41"]
pub type DesignerIdR = crate::FieldReader;
#[doc = "Field `MINOR_REVISION` reader - Minor Revision"]
pub type MinorRevisionR = crate::FieldReader;
#[doc = "Field `MAJOR_REVISION` reader - Major Revision"]
pub type MajorRevisionR = crate::FieldReader;
#[doc = "Field `CONFIGURATION` reader - CONFIGURATION for SSE-200 r2: 0x2"]
pub type ConfigurationR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11 - Part Number for the SSE-200"]
    #[inline(always)]
    pub fn part_number(&self) -> PartNumberR {
        PartNumberR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:19 - Arm Product with designer code 0x41"]
    #[inline(always)]
    pub fn designer_id(&self) -> DesignerIdR {
        DesignerIdR::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:23 - Minor Revision"]
    #[inline(always)]
    pub fn minor_revision(&self) -> MinorRevisionR {
        MinorRevisionR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Major Revision"]
    #[inline(always)]
    pub fn major_revision(&self) -> MajorRevisionR {
        MajorRevisionR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - CONFIGURATION for SSE-200 r2: 0x2"]
    #[inline(always)]
    pub fn configuration(&self) -> ConfigurationR {
        ConfigurationR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "System Version Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_version::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysVersionSpec;
impl crate::RegisterSpec for SysVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_version::R`](R) reader structure"]
impl crate::Readable for SysVersionSpec {}
#[doc = "`reset()` method sets SYS_VERSION to value 0x2204_1743"]
impl crate::Resettable for SysVersionSpec {
    const RESET_VALUE: u32 = 0x2204_1743;
}
