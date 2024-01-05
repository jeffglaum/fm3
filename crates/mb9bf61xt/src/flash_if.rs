#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    faszr: FASZR,
    frwtr: FRWTR,
    fstr: FSTR,
    _reserved3: [u8; 0x04],
    fsyndn: FSYNDN,
    fbfcr: FBFCR,
    _reserved5: [u8; 0xe8],
    crtrmm: CRTRMM,
}
impl RegisterBlock {
    #[doc = "0x00 - Flash Access Size Register"]
    #[inline(always)]
    pub const fn faszr(&self) -> &FASZR {
        &self.faszr
    }
    #[doc = "0x04 - Flash Read Wait Register"]
    #[inline(always)]
    pub const fn frwtr(&self) -> &FRWTR {
        &self.frwtr
    }
    #[doc = "0x08 - Flash Status Register"]
    #[inline(always)]
    pub const fn fstr(&self) -> &FSTR {
        &self.fstr
    }
    #[doc = "0x10 - Flash Sync Down Register"]
    #[inline(always)]
    pub const fn fsyndn(&self) -> &FSYNDN {
        &self.fsyndn
    }
    #[doc = "0x14 - Flash Buffer Control Register"]
    #[inline(always)]
    pub const fn fbfcr(&self) -> &FBFCR {
        &self.fbfcr
    }
    #[doc = "0x100 - CR Trimming Data Mirror Register"]
    #[inline(always)]
    pub const fn crtrmm(&self) -> &CRTRMM {
        &self.crtrmm
    }
}
#[doc = "FASZR (rw) register accessor: Flash Access Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`faszr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`faszr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@faszr`]
module"]
pub type FASZR = crate::Reg<faszr::FASZR_SPEC>;
#[doc = "Flash Access Size Register"]
pub mod faszr;
#[doc = "FRWTR (rw) register accessor: Flash Read Wait Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frwtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frwtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frwtr`]
module"]
pub type FRWTR = crate::Reg<frwtr::FRWTR_SPEC>;
#[doc = "Flash Read Wait Register"]
pub mod frwtr;
#[doc = "FSTR (r) register accessor: Flash Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fstr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fstr`]
module"]
pub type FSTR = crate::Reg<fstr::FSTR_SPEC>;
#[doc = "Flash Status Register"]
pub mod fstr;
#[doc = "FSYNDN (rw) register accessor: Flash Sync Down Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsyndn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsyndn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsyndn`]
module"]
pub type FSYNDN = crate::Reg<fsyndn::FSYNDN_SPEC>;
#[doc = "Flash Sync Down Register"]
pub mod fsyndn;
#[doc = "FBFCR (rw) register accessor: Flash Buffer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbfcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbfcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fbfcr`]
module"]
pub type FBFCR = crate::Reg<fbfcr::FBFCR_SPEC>;
#[doc = "Flash Buffer Control Register"]
pub mod fbfcr;
#[doc = "CRTRMM (r) register accessor: CR Trimming Data Mirror Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crtrmm::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crtrmm`]
module"]
pub type CRTRMM = crate::Reg<crtrmm::CRTRMM_SPEC>;
#[doc = "CR Trimming Data Mirror Register"]
pub mod crtrmm;
