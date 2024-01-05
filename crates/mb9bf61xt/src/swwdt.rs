#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    wdogload: WDOGLOAD,
    wdogvalue: WDOGVALUE,
    wdogcontrol: WDOGCONTROL,
    _reserved3: [u8; 0x03],
    wdogintclr: WDOGINTCLR,
    wdogris: WDOGRIS,
    _reserved5: [u8; 0x0bef],
    wdoglock: WDOGLOCK,
}
impl RegisterBlock {
    #[doc = "0x00 - Software Watchdog Timer Load Register"]
    #[inline(always)]
    pub const fn wdogload(&self) -> &WDOGLOAD {
        &self.wdogload
    }
    #[doc = "0x04 - Software Watchdog Timer Value Register"]
    #[inline(always)]
    pub const fn wdogvalue(&self) -> &WDOGVALUE {
        &self.wdogvalue
    }
    #[doc = "0x08 - Software Watchdog Timer Control Register"]
    #[inline(always)]
    pub const fn wdogcontrol(&self) -> &WDOGCONTROL {
        &self.wdogcontrol
    }
    #[doc = "0x0c - Software Watchdog Timer Clear Register"]
    #[inline(always)]
    pub const fn wdogintclr(&self) -> &WDOGINTCLR {
        &self.wdogintclr
    }
    #[doc = "0x10 - Software Watchdog Timer Interrupt Status Register"]
    #[inline(always)]
    pub const fn wdogris(&self) -> &WDOGRIS {
        &self.wdogris
    }
    #[doc = "0xc00 - Software Watchdog Timer Lock Register"]
    #[inline(always)]
    pub const fn wdoglock(&self) -> &WDOGLOCK {
        &self.wdoglock
    }
}
#[doc = "WDOGLOAD (rw) register accessor: Software Watchdog Timer Load Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdogload::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdogload::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogload`]
module"]
pub type WDOGLOAD = crate::Reg<wdogload::WDOGLOAD_SPEC>;
#[doc = "Software Watchdog Timer Load Register"]
pub mod wdogload;
#[doc = "WDOGVALUE (r) register accessor: Software Watchdog Timer Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdogvalue::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogvalue`]
module"]
pub type WDOGVALUE = crate::Reg<wdogvalue::WDOGVALUE_SPEC>;
#[doc = "Software Watchdog Timer Value Register"]
pub mod wdogvalue;
#[doc = "WDOGCONTROL (rw) register accessor: Software Watchdog Timer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdogcontrol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdogcontrol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogcontrol`]
module"]
pub type WDOGCONTROL = crate::Reg<wdogcontrol::WDOGCONTROL_SPEC>;
#[doc = "Software Watchdog Timer Control Register"]
pub mod wdogcontrol;
#[doc = "WDOGINTCLR (rw) register accessor: Software Watchdog Timer Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdogintclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdogintclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogintclr`]
module"]
pub type WDOGINTCLR = crate::Reg<wdogintclr::WDOGINTCLR_SPEC>;
#[doc = "Software Watchdog Timer Clear Register"]
pub mod wdogintclr;
#[doc = "WDOGRIS (r) register accessor: Software Watchdog Timer Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdogris::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogris`]
module"]
pub type WDOGRIS = crate::Reg<wdogris::WDOGRIS_SPEC>;
#[doc = "Software Watchdog Timer Interrupt Status Register"]
pub mod wdogris;
#[doc = "WDOGLOCK (rw) register accessor: Software Watchdog Timer Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdoglock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdoglock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdoglock`]
module"]
pub type WDOGLOCK = crate::Reg<wdoglock::WDOGLOCK_SPEC>;
#[doc = "Software Watchdog Timer Lock Register"]
pub mod wdoglock;
