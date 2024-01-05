#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    enir: ENIR,
    eirr: EIRR,
    eicl: EICL,
    elvr: ELVR,
    elvr1: ELVR1,
    nmirr: NMIRR,
    _reserved6: [u8; 0x03],
    nmicl: NMICL,
}
impl RegisterBlock {
    #[doc = "0x00 - Enable Interrupt Request Register"]
    #[inline(always)]
    pub const fn enir(&self) -> &ENIR {
        &self.enir
    }
    #[doc = "0x04 - External Interrupt Request Register"]
    #[inline(always)]
    pub const fn eirr(&self) -> &EIRR {
        &self.eirr
    }
    #[doc = "0x08 - External Interrupt Clear Register"]
    #[inline(always)]
    pub const fn eicl(&self) -> &EICL {
        &self.eicl
    }
    #[doc = "0x0c - External Interrupt Level Register"]
    #[inline(always)]
    pub const fn elvr(&self) -> &ELVR {
        &self.elvr
    }
    #[doc = "0x10 - External Interrupt Level Register 1"]
    #[inline(always)]
    pub const fn elvr1(&self) -> &ELVR1 {
        &self.elvr1
    }
    #[doc = "0x14 - Non Maskable Interrupt Request Register"]
    #[inline(always)]
    pub const fn nmirr(&self) -> &NMIRR {
        &self.nmirr
    }
    #[doc = "0x18 - Non Maskable Interrupt Clear Register"]
    #[inline(always)]
    pub const fn nmicl(&self) -> &NMICL {
        &self.nmicl
    }
}
#[doc = "ENIR (rw) register accessor: Enable Interrupt Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enir`]
module"]
pub type ENIR = crate::Reg<enir::ENIR_SPEC>;
#[doc = "Enable Interrupt Request Register"]
pub mod enir;
#[doc = "EIRR (r) register accessor: External Interrupt Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eirr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirr`]
module"]
pub type EIRR = crate::Reg<eirr::EIRR_SPEC>;
#[doc = "External Interrupt Request Register"]
pub mod eirr;
#[doc = "EICL (rw) register accessor: External Interrupt Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eicl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eicl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eicl`]
module"]
pub type EICL = crate::Reg<eicl::EICL_SPEC>;
#[doc = "External Interrupt Clear Register"]
pub mod eicl;
#[doc = "ELVR (rw) register accessor: External Interrupt Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`elvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`elvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@elvr`]
module"]
pub type ELVR = crate::Reg<elvr::ELVR_SPEC>;
#[doc = "External Interrupt Level Register"]
pub mod elvr;
#[doc = "ELVR1 (rw) register accessor: External Interrupt Level Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`elvr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`elvr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@elvr1`]
module"]
pub type ELVR1 = crate::Reg<elvr1::ELVR1_SPEC>;
#[doc = "External Interrupt Level Register 1"]
pub mod elvr1;
#[doc = "NMIRR (r) register accessor: Non Maskable Interrupt Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nmirr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmirr`]
module"]
pub type NMIRR = crate::Reg<nmirr::NMIRR_SPEC>;
#[doc = "Non Maskable Interrupt Request Register"]
pub mod nmirr;
#[doc = "NMICL (rw) register accessor: Non Maskable Interrupt Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nmicl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nmicl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmicl`]
module"]
pub type NMICL = crate::Reg<nmicl::NMICL_SPEC>;
#[doc = "Non Maskable Interrupt Clear Register"]
pub mod nmicl;
