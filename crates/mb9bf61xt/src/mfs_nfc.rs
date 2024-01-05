#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    i2cdnf: I2CDNF,
}
impl RegisterBlock {
    #[doc = "0x00 - I2C Auxiliary Noise Filter Setting Register"]
    #[inline(always)]
    pub const fn i2cdnf(&self) -> &I2CDNF {
        &self.i2cdnf
    }
}
#[doc = "I2CDNF (rw) register accessor: I2C Auxiliary Noise Filter Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cdnf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cdnf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cdnf`]
module"]
pub type I2CDNF = crate::Reg<i2cdnf::I2CDNF_SPEC>;
#[doc = "I2C Auxiliary Noise Filter Setting Register"]
pub mod i2cdnf;
