#[doc = "Register `LVD_STR` reader"]
pub type R = crate::R<LVD_STR_SPEC>;
#[doc = "Field `LVDIR` reader - Low-voltage detection interrupt bit"]
pub type LVDIR_R = crate::BitReader;
impl R {
    #[doc = "Bit 7 - Low-voltage detection interrupt bit"]
    #[inline(always)]
    pub fn lvdir(&self) -> LVDIR_R {
        LVDIR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Low-voltage Detection Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvd_str::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LVD_STR_SPEC;
impl crate::RegisterSpec for LVD_STR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lvd_str::R`](R) reader structure"]
impl crate::Readable for LVD_STR_SPEC {}
#[doc = "`reset()` method sets LVD_STR to value 0"]
impl crate::Resettable for LVD_STR_SPEC {
    const RESET_VALUE: u8 = 0;
}
