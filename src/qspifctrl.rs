#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    qspicfg: Qspicfg,
    devreadinstr: Devreadinstr,
    devwriteinstr: Devwriteinstr,
    _reserved3: [u8; 0x08],
    devsize: Devsize,
    _reserved4: [u8; 0x0c],
    remapaddr: Remapaddr,
    _reserved5: [u8; 0x68],
    flashcmdctrl: Flashcmdctrl,
    flashcmdaddr: Flashcmdaddr,
    _reserved7: [u8; 0x08],
    flashcmdrdatalow: Flashcmdrdatalow,
    flashcmdrdataup: Flashcmdrdataup,
    flashcmdwrdatalow: Flashcmdwrdatalow,
    flashcmdwrdataup: Flashcmdwrdataup,
}
impl RegisterBlock {
    #[doc = "0x00 - QSPI Configuration Register"]
    #[inline(always)]
    pub const fn qspicfg(&self) -> &Qspicfg {
        &self.qspicfg
    }
    #[doc = "0x04 - Device Read Instruction Register"]
    #[inline(always)]
    pub const fn devreadinstr(&self) -> &Devreadinstr {
        &self.devreadinstr
    }
    #[doc = "0x08 - Device Write Instruction Configuration Register"]
    #[inline(always)]
    pub const fn devwriteinstr(&self) -> &Devwriteinstr {
        &self.devwriteinstr
    }
    #[doc = "0x14 - Device Size Configuration Register"]
    #[inline(always)]
    pub const fn devsize(&self) -> &Devsize {
        &self.devsize
    }
    #[doc = "0x24 - Remap Address Register"]
    #[inline(always)]
    pub const fn remapaddr(&self) -> &Remapaddr {
        &self.remapaddr
    }
    #[doc = "0x90 - Flash Command Control Register"]
    #[inline(always)]
    pub const fn flashcmdctrl(&self) -> &Flashcmdctrl {
        &self.flashcmdctrl
    }
    #[doc = "0x94 - Flash Command Address Register"]
    #[inline(always)]
    pub const fn flashcmdaddr(&self) -> &Flashcmdaddr {
        &self.flashcmdaddr
    }
    #[doc = "0xa0 - Flash Command Read Data Register (Lower)"]
    #[inline(always)]
    pub const fn flashcmdrdatalow(&self) -> &Flashcmdrdatalow {
        &self.flashcmdrdatalow
    }
    #[doc = "0xa4 - Flash Command Read Data Register (Upper)"]
    #[inline(always)]
    pub const fn flashcmdrdataup(&self) -> &Flashcmdrdataup {
        &self.flashcmdrdataup
    }
    #[doc = "0xa8 - Flash Command Write Data Register (Lower)"]
    #[inline(always)]
    pub const fn flashcmdwrdatalow(&self) -> &Flashcmdwrdatalow {
        &self.flashcmdwrdatalow
    }
    #[doc = "0xac - Flash Command Write Data Register (Upper)"]
    #[inline(always)]
    pub const fn flashcmdwrdataup(&self) -> &Flashcmdwrdataup {
        &self.flashcmdwrdataup
    }
}
#[doc = "QSPICFG (rw) register accessor: QSPI Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspicfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspicfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspicfg`] module"]
#[doc(alias = "QSPICFG")]
pub type Qspicfg = crate::Reg<qspicfg::QspicfgSpec>;
#[doc = "QSPI Configuration Register"]
pub mod qspicfg;
#[doc = "DEVREADINSTR (rw) register accessor: Device Read Instruction Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devreadinstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devreadinstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devreadinstr`] module"]
#[doc(alias = "DEVREADINSTR")]
pub type Devreadinstr = crate::Reg<devreadinstr::DevreadinstrSpec>;
#[doc = "Device Read Instruction Register"]
pub mod devreadinstr;
#[doc = "DEVWRITEINSTR (rw) register accessor: Device Write Instruction Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devwriteinstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devwriteinstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devwriteinstr`] module"]
#[doc(alias = "DEVWRITEINSTR")]
pub type Devwriteinstr = crate::Reg<devwriteinstr::DevwriteinstrSpec>;
#[doc = "Device Write Instruction Configuration Register"]
pub mod devwriteinstr;
#[doc = "DEVSIZE (rw) register accessor: Device Size Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devsize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devsize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devsize`] module"]
#[doc(alias = "DEVSIZE")]
pub type Devsize = crate::Reg<devsize::DevsizeSpec>;
#[doc = "Device Size Configuration Register"]
pub mod devsize;
#[doc = "REMAPADDR (rw) register accessor: Remap Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`remapaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remapaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remapaddr`] module"]
#[doc(alias = "REMAPADDR")]
pub type Remapaddr = crate::Reg<remapaddr::RemapaddrSpec>;
#[doc = "Remap Address Register"]
pub mod remapaddr;
#[doc = "FLASHCMDCTRL (rw) register accessor: Flash Command Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashcmdctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashcmdctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashcmdctrl`] module"]
#[doc(alias = "FLASHCMDCTRL")]
pub type Flashcmdctrl = crate::Reg<flashcmdctrl::FlashcmdctrlSpec>;
#[doc = "Flash Command Control Register"]
pub mod flashcmdctrl;
#[doc = "FLASHCMDADDR (rw) register accessor: Flash Command Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashcmdaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashcmdaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashcmdaddr`] module"]
#[doc(alias = "FLASHCMDADDR")]
pub type Flashcmdaddr = crate::Reg<flashcmdaddr::FlashcmdaddrSpec>;
#[doc = "Flash Command Address Register"]
pub mod flashcmdaddr;
#[doc = "FLASHCMDRDATALOW (r) register accessor: Flash Command Read Data Register (Lower)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashcmdrdatalow::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashcmdrdatalow`] module"]
#[doc(alias = "FLASHCMDRDATALOW")]
pub type Flashcmdrdatalow = crate::Reg<flashcmdrdatalow::FlashcmdrdatalowSpec>;
#[doc = "Flash Command Read Data Register (Lower)"]
pub mod flashcmdrdatalow;
#[doc = "FLASHCMDRDATAUP (r) register accessor: Flash Command Read Data Register (Upper)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashcmdrdataup::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashcmdrdataup`] module"]
#[doc(alias = "FLASHCMDRDATAUP")]
pub type Flashcmdrdataup = crate::Reg<flashcmdrdataup::FlashcmdrdataupSpec>;
#[doc = "Flash Command Read Data Register (Upper)"]
pub mod flashcmdrdataup;
#[doc = "FLASHCMDWRDATALOW (rw) register accessor: Flash Command Write Data Register (Lower)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashcmdwrdatalow::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashcmdwrdatalow::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashcmdwrdatalow`] module"]
#[doc(alias = "FLASHCMDWRDATALOW")]
pub type Flashcmdwrdatalow = crate::Reg<flashcmdwrdatalow::FlashcmdwrdatalowSpec>;
#[doc = "Flash Command Write Data Register (Lower)"]
pub mod flashcmdwrdatalow;
#[doc = "FLASHCMDWRDATAUP (rw) register accessor: Flash Command Write Data Register (Upper)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashcmdwrdataup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashcmdwrdataup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashcmdwrdataup`] module"]
#[doc(alias = "FLASHCMDWRDATAUP")]
pub type Flashcmdwrdataup = crate::Reg<flashcmdwrdataup::FlashcmdwrdataupSpec>;
#[doc = "Flash Command Write Data Register (Upper)"]
pub mod flashcmdwrdataup;
