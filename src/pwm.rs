#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pwmcr: Pwmcr,
    pwmpr: Pwmpr,
    pwmhr: Pwmhr,
    _reserved3: [u8; 0x04],
    pwmei: Pwmei,
    pwmdi: Pwmdi,
    pwmri: Pwmri,
    pwmis: Pwmis,
}
impl RegisterBlock {
    #[doc = "0x00 - PWM Control Register"]
    #[inline(always)]
    pub const fn pwmcr(&self) -> &Pwmcr {
        &self.pwmcr
    }
    #[doc = "0x04 - PWM Period Register. Number of system clock cycles indicating the period of PWM cycle.The minimum and maximum values have special significance. 0x0: pwm_output continually high 0xFFFFFFFF: pwm_output continually low"]
    #[inline(always)]
    pub const fn pwmpr(&self) -> &Pwmpr {
        &self.pwmpr
    }
    #[doc = "0x08 - PWM High Iime Register. This register contains the number of system clock cycles for during which the pwm_output should be kept high in a PWM cycle"]
    #[inline(always)]
    pub const fn pwmhr(&self) -> &Pwmhr {
        &self.pwmhr
    }
    #[doc = "0x10 - PWM Enable Interrupt Register"]
    #[inline(always)]
    pub const fn pwmei(&self) -> &Pwmei {
        &self.pwmei
    }
    #[doc = "0x14 - PWM Disable Interrupt Register"]
    #[inline(always)]
    pub const fn pwmdi(&self) -> &Pwmdi {
        &self.pwmdi
    }
    #[doc = "0x18 - PWM Read Intr Enable Register.Reading from this address accesses the current state of the interrupt control registers"]
    #[inline(always)]
    pub const fn pwmri(&self) -> &Pwmri {
        &self.pwmri
    }
    #[doc = "0x1c - PWM Read Interrupt Status Register"]
    #[inline(always)]
    pub const fn pwmis(&self) -> &Pwmis {
        &self.pwmis
    }
}
#[doc = "PWMCR (rw) register accessor: PWM Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmcr`]
module"]
#[doc(alias = "PWMCR")]
pub type Pwmcr = crate::Reg<pwmcr::PwmcrSpec>;
#[doc = "PWM Control Register"]
pub mod pwmcr;
#[doc = "PWMPR (rw) register accessor: PWM Period Register. Number of system clock cycles indicating the period of PWM cycle.The minimum and maximum values have special significance. 0x0: pwm_output continually high 0xFFFFFFFF: pwm_output continually low\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmpr`]
module"]
#[doc(alias = "PWMPR")]
pub type Pwmpr = crate::Reg<pwmpr::PwmprSpec>;
#[doc = "PWM Period Register. Number of system clock cycles indicating the period of PWM cycle.The minimum and maximum values have special significance. 0x0: pwm_output continually high 0xFFFFFFFF: pwm_output continually low"]
pub mod pwmpr;
#[doc = "PWMHR (rw) register accessor: PWM High Iime Register. This register contains the number of system clock cycles for during which the pwm_output should be kept high in a PWM cycle\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmhr`]
module"]
#[doc(alias = "PWMHR")]
pub type Pwmhr = crate::Reg<pwmhr::PwmhrSpec>;
#[doc = "PWM High Iime Register. This register contains the number of system clock cycles for during which the pwm_output should be kept high in a PWM cycle"]
pub mod pwmhr;
#[doc = "PWMEI (w) register accessor: PWM Enable Interrupt Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmei::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmei`]
module"]
#[doc(alias = "PWMEI")]
pub type Pwmei = crate::Reg<pwmei::PwmeiSpec>;
#[doc = "PWM Enable Interrupt Register"]
pub mod pwmei;
#[doc = "PWMDI (w) register accessor: PWM Disable Interrupt Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmdi::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmdi`]
module"]
#[doc(alias = "PWMDI")]
pub type Pwmdi = crate::Reg<pwmdi::PwmdiSpec>;
#[doc = "PWM Disable Interrupt Register"]
pub mod pwmdi;
#[doc = "PWMRI (r) register accessor: PWM Read Intr Enable Register.Reading from this address accesses the current state of the interrupt control registers\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmri::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmri`]
module"]
#[doc(alias = "PWMRI")]
pub type Pwmri = crate::Reg<pwmri::PwmriSpec>;
#[doc = "PWM Read Intr Enable Register.Reading from this address accesses the current state of the interrupt control registers"]
pub mod pwmri;
#[doc = "PWMIS (r) register accessor: PWM Read Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmis`]
module"]
#[doc(alias = "PWMIS")]
pub type Pwmis = crate::Reg<pwmis::PwmisSpec>;
#[doc = "PWM Read Interrupt Status Register"]
pub mod pwmis;
