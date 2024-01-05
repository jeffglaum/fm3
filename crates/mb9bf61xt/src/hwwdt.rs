#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    wdg_ldr: WDG_LDR,
    wdg_vlr: WDG_VLR,
    wdg_ctl: WDG_CTL,
    _reserved3: [u8; 0x03],
    wdg_icl: WDG_ICL,
    _reserved4: [u8; 0x03],
    wdg_ris: WDG_RIS,
    _reserved5: [u8; 0x0bef],
    wdg_lck: WDG_LCK,
}
impl RegisterBlock {
    #[doc = "0x00 - Hardware Watchdog Timer Load Register"]
    #[inline(always)]
    pub const fn wdg_ldr(&self) -> &WDG_LDR {
        &self.wdg_ldr
    }
    #[doc = "0x04 - Hardware Watchdog Timer Value Register"]
    #[inline(always)]
    pub const fn wdg_vlr(&self) -> &WDG_VLR {
        &self.wdg_vlr
    }
    #[doc = "0x08 - Hardware Watchdog Timer Control Register"]
    #[inline(always)]
    pub const fn wdg_ctl(&self) -> &WDG_CTL {
        &self.wdg_ctl
    }
    #[doc = "0x0c - Hardware Watchdog Timer Clear Register"]
    #[inline(always)]
    pub const fn wdg_icl(&self) -> &WDG_ICL {
        &self.wdg_icl
    }
    #[doc = "0x10 - Hardware Watchdog Timer Interrupt Status Register"]
    #[inline(always)]
    pub const fn wdg_ris(&self) -> &WDG_RIS {
        &self.wdg_ris
    }
    #[doc = "0xc00 - Hardware Watchdog Timer Lock Register"]
    #[inline(always)]
    pub const fn wdg_lck(&self) -> &WDG_LCK {
        &self.wdg_lck
    }
}
#[doc = "WDG_LDR (rw) register accessor: Hardware Watchdog Timer Load Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdg_ldr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdg_ldr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdg_ldr`]
module"]
pub type WDG_LDR = crate::Reg<wdg_ldr::WDG_LDR_SPEC>;
#[doc = "Hardware Watchdog Timer Load Register"]
pub mod wdg_ldr;
#[doc = "WDG_VLR (r) register accessor: Hardware Watchdog Timer Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdg_vlr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdg_vlr`]
module"]
pub type WDG_VLR = crate::Reg<wdg_vlr::WDG_VLR_SPEC>;
#[doc = "Hardware Watchdog Timer Value Register"]
pub mod wdg_vlr;
#[doc = "WDG_CTL (rw) register accessor: Hardware Watchdog Timer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdg_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdg_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdg_ctl`]
module"]
pub type WDG_CTL = crate::Reg<wdg_ctl::WDG_CTL_SPEC>;
#[doc = "Hardware Watchdog Timer Control Register"]
pub mod wdg_ctl;
#[doc = "WDG_ICL (rw) register accessor: Hardware Watchdog Timer Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdg_icl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdg_icl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdg_icl`]
module"]
pub type WDG_ICL = crate::Reg<wdg_icl::WDG_ICL_SPEC>;
#[doc = "Hardware Watchdog Timer Clear Register"]
pub mod wdg_icl;
#[doc = "WDG_RIS (r) register accessor: Hardware Watchdog Timer Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdg_ris::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdg_ris`]
module"]
pub type WDG_RIS = crate::Reg<wdg_ris::WDG_RIS_SPEC>;
#[doc = "Hardware Watchdog Timer Interrupt Status Register"]
pub mod wdg_ris;
#[doc = "WDG_LCK (rw) register accessor: Hardware Watchdog Timer Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdg_lck::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdg_lck::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdg_lck`]
module"]
pub type WDG_LCK = crate::Reg<wdg_lck::WDG_LCK_SPEC>;
#[doc = "Hardware Watchdog Timer Lock Register"]
pub mod wdg_lck;
