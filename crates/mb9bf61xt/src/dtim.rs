#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    dual_timer1_timer1load: DUAL_TIMER1_TIMER1LOAD,
    timer1value: TIMER1VALUE,
    timer1control: TIMER1CONTROL,
    timer1intclr: TIMER1INTCLR,
    timer1ris: TIMER1RIS,
    timer1mis: TIMER1MIS,
    timer1bgload: TIMER1BGLOAD,
    _reserved7: [u8; 0x04],
    timer2load: TIMER2LOAD,
    timer2value: TIMER2VALUE,
    timer2control: TIMER2CONTROL,
    timer2intclr: TIMER2INTCLR,
    timer2ris: TIMER2RIS,
    timer2mis: TIMER2MIS,
    timer2bgload: TIMER2BGLOAD,
}
impl RegisterBlock {
    #[doc = "0x00 - Load Register"]
    #[inline(always)]
    pub const fn dual_timer1_timer1load(&self) -> &DUAL_TIMER1_TIMER1LOAD {
        &self.dual_timer1_timer1load
    }
    #[doc = "0x04 - Value Register"]
    #[inline(always)]
    pub const fn timer1value(&self) -> &TIMER1VALUE {
        &self.timer1value
    }
    #[doc = "0x08 - Control Register"]
    #[inline(always)]
    pub const fn timer1control(&self) -> &TIMER1CONTROL {
        &self.timer1control
    }
    #[doc = "0x0c - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn timer1intclr(&self) -> &TIMER1INTCLR {
        &self.timer1intclr
    }
    #[doc = "0x10 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn timer1ris(&self) -> &TIMER1RIS {
        &self.timer1ris
    }
    #[doc = "0x14 - Masked Interrupt Status Register"]
    #[inline(always)]
    pub const fn timer1mis(&self) -> &TIMER1MIS {
        &self.timer1mis
    }
    #[doc = "0x18 - Background Load Register"]
    #[inline(always)]
    pub const fn timer1bgload(&self) -> &TIMER1BGLOAD {
        &self.timer1bgload
    }
    #[doc = "0x20 - Load Register"]
    #[inline(always)]
    pub const fn timer2load(&self) -> &TIMER2LOAD {
        &self.timer2load
    }
    #[doc = "0x24 - Value Register"]
    #[inline(always)]
    pub const fn timer2value(&self) -> &TIMER2VALUE {
        &self.timer2value
    }
    #[doc = "0x28 - Control Register"]
    #[inline(always)]
    pub const fn timer2control(&self) -> &TIMER2CONTROL {
        &self.timer2control
    }
    #[doc = "0x2c - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn timer2intclr(&self) -> &TIMER2INTCLR {
        &self.timer2intclr
    }
    #[doc = "0x30 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn timer2ris(&self) -> &TIMER2RIS {
        &self.timer2ris
    }
    #[doc = "0x34 - Masked Interrupt Status Register"]
    #[inline(always)]
    pub const fn timer2mis(&self) -> &TIMER2MIS {
        &self.timer2mis
    }
    #[doc = "0x38 - Background Load Register"]
    #[inline(always)]
    pub const fn timer2bgload(&self) -> &TIMER2BGLOAD {
        &self.timer2bgload
    }
}
#[doc = "DualTimer1_TIMER1LOAD (rw) register accessor: Load Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dual_timer1_timer1load::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dual_timer1_timer1load::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dual_timer1_timer1load`]
module"]
pub type DUAL_TIMER1_TIMER1LOAD = crate::Reg<dual_timer1_timer1load::DUAL_TIMER1_TIMER1LOAD_SPEC>;
#[doc = "Load Register"]
pub mod dual_timer1_timer1load;
#[doc = "TIMER1VALUE (r) register accessor: Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1value::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1value`]
module"]
pub type TIMER1VALUE = crate::Reg<timer1value::TIMER1VALUE_SPEC>;
#[doc = "Value Register"]
pub mod timer1value;
#[doc = "TIMER1CONTROL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer1control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1control`]
module"]
pub type TIMER1CONTROL = crate::Reg<timer1control::TIMER1CONTROL_SPEC>;
#[doc = "Control Register"]
pub mod timer1control;
#[doc = "TIMER1INTCLR (w) register accessor: Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer1intclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1intclr`]
module"]
pub type TIMER1INTCLR = crate::Reg<timer1intclr::TIMER1INTCLR_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod timer1intclr;
#[doc = "TIMER1RIS (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1ris::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1ris`]
module"]
pub type TIMER1RIS = crate::Reg<timer1ris::TIMER1RIS_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod timer1ris;
#[doc = "TIMER1MIS (r) register accessor: Masked Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1mis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1mis`]
module"]
pub type TIMER1MIS = crate::Reg<timer1mis::TIMER1MIS_SPEC>;
#[doc = "Masked Interrupt Status Register"]
pub mod timer1mis;
#[doc = "TIMER1BGLOAD (rw) register accessor: Background Load Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1bgload::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer1bgload::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1bgload`]
module"]
pub type TIMER1BGLOAD = crate::Reg<timer1bgload::TIMER1BGLOAD_SPEC>;
#[doc = "Background Load Register"]
pub mod timer1bgload;
pub use dual_timer1_timer1load as timer2load;
pub use timer1bgload as timer2bgload;
pub use timer1control as timer2control;
pub use timer1intclr as timer2intclr;
pub use timer1mis as timer2mis;
pub use timer1ris as timer2ris;
pub use timer1value as timer2value;
pub use DUAL_TIMER1_TIMER1LOAD as TIMER2LOAD;
pub use TIMER1BGLOAD as TIMER2BGLOAD;
pub use TIMER1CONTROL as TIMER2CONTROL;
pub use TIMER1INTCLR as TIMER2INTCLR;
pub use TIMER1MIS as TIMER2MIS;
pub use TIMER1RIS as TIMER2RIS;
pub use TIMER1VALUE as TIMER2VALUE;
