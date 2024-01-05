#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    lvd_ctl: LVD_CTL,
    _reserved1: [u8; 0x03],
    lvd_str: LVD_STR,
    _reserved2: [u8; 0x03],
    lvd_clr: LVD_CLR,
    _reserved3: [u8; 0x03],
    lvd_rlr: LVD_RLR,
    lvd_str2: LVD_STR2,
}
impl RegisterBlock {
    #[doc = "0x00 - Low-voltage Detection Voltage Control Register"]
    #[inline(always)]
    pub const fn lvd_ctl(&self) -> &LVD_CTL {
        &self.lvd_ctl
    }
    #[doc = "0x04 - Low-voltage Detection Interrupt Register"]
    #[inline(always)]
    pub const fn lvd_str(&self) -> &LVD_STR {
        &self.lvd_str
    }
    #[doc = "0x08 - Low-voltage Detection Interrupt Clear Register"]
    #[inline(always)]
    pub const fn lvd_clr(&self) -> &LVD_CLR {
        &self.lvd_clr
    }
    #[doc = "0x0c - Low-voltage Detection Voltage Protection Register"]
    #[inline(always)]
    pub const fn lvd_rlr(&self) -> &LVD_RLR {
        &self.lvd_rlr
    }
    #[doc = "0x10 - Low-voltage Detection Circuit Status Register"]
    #[inline(always)]
    pub const fn lvd_str2(&self) -> &LVD_STR2 {
        &self.lvd_str2
    }
}
#[doc = "LVD_CTL (rw) register accessor: Low-voltage Detection Voltage Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvd_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lvd_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvd_ctl`]
module"]
pub type LVD_CTL = crate::Reg<lvd_ctl::LVD_CTL_SPEC>;
#[doc = "Low-voltage Detection Voltage Control Register"]
pub mod lvd_ctl;
#[doc = "LVD_STR (r) register accessor: Low-voltage Detection Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvd_str::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvd_str`]
module"]
pub type LVD_STR = crate::Reg<lvd_str::LVD_STR_SPEC>;
#[doc = "Low-voltage Detection Interrupt Register"]
pub mod lvd_str;
#[doc = "LVD_CLR (rw) register accessor: Low-voltage Detection Interrupt Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvd_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lvd_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvd_clr`]
module"]
pub type LVD_CLR = crate::Reg<lvd_clr::LVD_CLR_SPEC>;
#[doc = "Low-voltage Detection Interrupt Clear Register"]
pub mod lvd_clr;
#[doc = "LVD_RLR (rw) register accessor: Low-voltage Detection Voltage Protection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvd_rlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lvd_rlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvd_rlr`]
module"]
pub type LVD_RLR = crate::Reg<lvd_rlr::LVD_RLR_SPEC>;
#[doc = "Low-voltage Detection Voltage Protection Register"]
pub mod lvd_rlr;
#[doc = "LVD_STR2 (r) register accessor: Low-voltage Detection Circuit Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvd_str2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvd_str2`]
module"]
pub type LVD_STR2 = crate::Reg<lvd_str2::LVD_STR2_SPEC>;
#[doc = "Low-voltage Detection Circuit Status Register"]
pub mod lvd_str2;
