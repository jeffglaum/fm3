#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    mode0: MODE0,
    mode1: MODE1,
    mode2: MODE2,
    mode3: MODE3,
    mode4: MODE4,
    mode5: MODE5,
    mode6: MODE6,
    mode7: MODE7,
    tim0: TIM0,
    tim1: TIM1,
    tim2: TIM2,
    tim3: TIM3,
    tim4: TIM4,
    tim5: TIM5,
    tim6: TIM6,
    tim7: TIM7,
    area0: AREA0,
    area1: AREA1,
    area2: AREA2,
    area3: AREA3,
    area4: AREA4,
    area5: AREA5,
    area6: AREA6,
    area7: AREA7,
    atim0: ATIM0,
    atim1: ATIM1,
    atim2: ATIM2,
    atim3: ATIM3,
    atim4: ATIM4,
    atim5: ATIM5,
    atim6: ATIM6,
    atim7: ATIM7,
    _reserved32: [u8; 0x0280],
    dclkr: DCLKR,
}
impl RegisterBlock {
    #[doc = "0x00 - Mode Register 0"]
    #[inline(always)]
    pub const fn mode0(&self) -> &MODE0 {
        &self.mode0
    }
    #[doc = "0x04 - Mode Register 1"]
    #[inline(always)]
    pub const fn mode1(&self) -> &MODE1 {
        &self.mode1
    }
    #[doc = "0x08 - Mode Register 2"]
    #[inline(always)]
    pub const fn mode2(&self) -> &MODE2 {
        &self.mode2
    }
    #[doc = "0x0c - Mode Register 3"]
    #[inline(always)]
    pub const fn mode3(&self) -> &MODE3 {
        &self.mode3
    }
    #[doc = "0x10 - Mode Register 4"]
    #[inline(always)]
    pub const fn mode4(&self) -> &MODE4 {
        &self.mode4
    }
    #[doc = "0x14 - Mode Register 5"]
    #[inline(always)]
    pub const fn mode5(&self) -> &MODE5 {
        &self.mode5
    }
    #[doc = "0x18 - Mode Register 6"]
    #[inline(always)]
    pub const fn mode6(&self) -> &MODE6 {
        &self.mode6
    }
    #[doc = "0x1c - Mode Register 7"]
    #[inline(always)]
    pub const fn mode7(&self) -> &MODE7 {
        &self.mode7
    }
    #[doc = "0x20 - Timing Register 0"]
    #[inline(always)]
    pub const fn tim0(&self) -> &TIM0 {
        &self.tim0
    }
    #[doc = "0x24 - Timing Register 1"]
    #[inline(always)]
    pub const fn tim1(&self) -> &TIM1 {
        &self.tim1
    }
    #[doc = "0x28 - Timing Register 2"]
    #[inline(always)]
    pub const fn tim2(&self) -> &TIM2 {
        &self.tim2
    }
    #[doc = "0x2c - Timing Register 3"]
    #[inline(always)]
    pub const fn tim3(&self) -> &TIM3 {
        &self.tim3
    }
    #[doc = "0x30 - Timing Register 4"]
    #[inline(always)]
    pub const fn tim4(&self) -> &TIM4 {
        &self.tim4
    }
    #[doc = "0x34 - Timing Register 5"]
    #[inline(always)]
    pub const fn tim5(&self) -> &TIM5 {
        &self.tim5
    }
    #[doc = "0x38 - Timing Register 6"]
    #[inline(always)]
    pub const fn tim6(&self) -> &TIM6 {
        &self.tim6
    }
    #[doc = "0x3c - Timing Register 7"]
    #[inline(always)]
    pub const fn tim7(&self) -> &TIM7 {
        &self.tim7
    }
    #[doc = "0x40 - Area Register 0"]
    #[inline(always)]
    pub const fn area0(&self) -> &AREA0 {
        &self.area0
    }
    #[doc = "0x44 - Area Register 1"]
    #[inline(always)]
    pub const fn area1(&self) -> &AREA1 {
        &self.area1
    }
    #[doc = "0x48 - Area Register 2"]
    #[inline(always)]
    pub const fn area2(&self) -> &AREA2 {
        &self.area2
    }
    #[doc = "0x4c - Area Register 3"]
    #[inline(always)]
    pub const fn area3(&self) -> &AREA3 {
        &self.area3
    }
    #[doc = "0x50 - Area Register 4"]
    #[inline(always)]
    pub const fn area4(&self) -> &AREA4 {
        &self.area4
    }
    #[doc = "0x54 - Area Register 5"]
    #[inline(always)]
    pub const fn area5(&self) -> &AREA5 {
        &self.area5
    }
    #[doc = "0x58 - Area Register 6"]
    #[inline(always)]
    pub const fn area6(&self) -> &AREA6 {
        &self.area6
    }
    #[doc = "0x5c - Area Register 7"]
    #[inline(always)]
    pub const fn area7(&self) -> &AREA7 {
        &self.area7
    }
    #[doc = "0x60 - ALE Timing Register 0"]
    #[inline(always)]
    pub const fn atim0(&self) -> &ATIM0 {
        &self.atim0
    }
    #[doc = "0x64 - ALE Timing Register 1"]
    #[inline(always)]
    pub const fn atim1(&self) -> &ATIM1 {
        &self.atim1
    }
    #[doc = "0x68 - ALE Timing Register 2"]
    #[inline(always)]
    pub const fn atim2(&self) -> &ATIM2 {
        &self.atim2
    }
    #[doc = "0x6c - ALE Timing Register 3"]
    #[inline(always)]
    pub const fn atim3(&self) -> &ATIM3 {
        &self.atim3
    }
    #[doc = "0x70 - ALE Timing Register 4"]
    #[inline(always)]
    pub const fn atim4(&self) -> &ATIM4 {
        &self.atim4
    }
    #[doc = "0x74 - ALE Timing Register 5"]
    #[inline(always)]
    pub const fn atim5(&self) -> &ATIM5 {
        &self.atim5
    }
    #[doc = "0x78 - ALE Timing Register 6"]
    #[inline(always)]
    pub const fn atim6(&self) -> &ATIM6 {
        &self.atim6
    }
    #[doc = "0x7c - ALE Timing Register 7"]
    #[inline(always)]
    pub const fn atim7(&self) -> &ATIM7 {
        &self.atim7
    }
    #[doc = "0x300 - Division Clock Register"]
    #[inline(always)]
    pub const fn dclkr(&self) -> &DCLKR {
        &self.dclkr
    }
}
#[doc = "MODE0 (rw) register accessor: Mode Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode0`]
module"]
pub type MODE0 = crate::Reg<mode0::MODE0_SPEC>;
#[doc = "Mode Register 0"]
pub mod mode0;
pub use mode0 as mode1;
pub use mode0 as mode2;
pub use mode0 as mode3;
pub use mode0 as mode4;
pub use mode0 as mode5;
pub use mode0 as mode6;
pub use mode0 as mode7;
pub use MODE0 as MODE1;
pub use MODE0 as MODE2;
pub use MODE0 as MODE3;
pub use MODE0 as MODE4;
pub use MODE0 as MODE5;
pub use MODE0 as MODE6;
pub use MODE0 as MODE7;
#[doc = "TIM0 (rw) register accessor: Timing Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim0`]
module"]
pub type TIM0 = crate::Reg<tim0::TIM0_SPEC>;
#[doc = "Timing Register 0"]
pub mod tim0;
pub use tim0 as tim1;
pub use tim0 as tim2;
pub use tim0 as tim3;
pub use tim0 as tim4;
pub use tim0 as tim5;
pub use tim0 as tim6;
pub use tim0 as tim7;
pub use TIM0 as TIM1;
pub use TIM0 as TIM2;
pub use TIM0 as TIM3;
pub use TIM0 as TIM4;
pub use TIM0 as TIM5;
pub use TIM0 as TIM6;
pub use TIM0 as TIM7;
#[doc = "AREA0 (rw) register accessor: Area Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`area0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`area0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@area0`]
module"]
pub type AREA0 = crate::Reg<area0::AREA0_SPEC>;
#[doc = "Area Register 0"]
pub mod area0;
#[doc = "AREA1 (rw) register accessor: Area Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`area1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`area1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@area1`]
module"]
pub type AREA1 = crate::Reg<area1::AREA1_SPEC>;
#[doc = "Area Register 1"]
pub mod area1;
#[doc = "AREA2 (rw) register accessor: Area Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`area2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`area2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@area2`]
module"]
pub type AREA2 = crate::Reg<area2::AREA2_SPEC>;
#[doc = "Area Register 2"]
pub mod area2;
#[doc = "AREA3 (rw) register accessor: Area Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`area3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`area3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@area3`]
module"]
pub type AREA3 = crate::Reg<area3::AREA3_SPEC>;
#[doc = "Area Register 3"]
pub mod area3;
#[doc = "AREA4 (rw) register accessor: Area Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`area4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`area4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@area4`]
module"]
pub type AREA4 = crate::Reg<area4::AREA4_SPEC>;
#[doc = "Area Register 4"]
pub mod area4;
#[doc = "AREA5 (rw) register accessor: Area Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`area5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`area5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@area5`]
module"]
pub type AREA5 = crate::Reg<area5::AREA5_SPEC>;
#[doc = "Area Register 5"]
pub mod area5;
#[doc = "AREA6 (rw) register accessor: Area Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`area6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`area6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@area6`]
module"]
pub type AREA6 = crate::Reg<area6::AREA6_SPEC>;
#[doc = "Area Register 6"]
pub mod area6;
#[doc = "AREA7 (rw) register accessor: Area Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`area7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`area7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@area7`]
module"]
pub type AREA7 = crate::Reg<area7::AREA7_SPEC>;
#[doc = "Area Register 7"]
pub mod area7;
#[doc = "ATIM0 (rw) register accessor: ALE Timing Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atim0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atim0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atim0`]
module"]
pub type ATIM0 = crate::Reg<atim0::ATIM0_SPEC>;
#[doc = "ALE Timing Register 0"]
pub mod atim0;
pub use atim0 as atim1;
pub use atim0 as atim2;
pub use atim0 as atim3;
pub use atim0 as atim4;
pub use atim0 as atim5;
pub use atim0 as atim6;
pub use atim0 as atim7;
pub use ATIM0 as ATIM1;
pub use ATIM0 as ATIM2;
pub use ATIM0 as ATIM3;
pub use ATIM0 as ATIM4;
pub use ATIM0 as ATIM5;
pub use ATIM0 as ATIM6;
pub use ATIM0 as ATIM7;
#[doc = "DCLKR (rw) register accessor: Division Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dclkr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dclkr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dclkr`]
module"]
pub type DCLKR = crate::Reg<dclkr::DCLKR_SPEC>;
#[doc = "Division Clock Register"]
pub mod dclkr;
