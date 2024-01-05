#[doc = "Register `LVD_STR2` reader"]
pub type R = crate::R<LVD_STR2_SPEC>;
#[doc = "Field `LVDIRDY` reader - Low-voltage detection interrupt status flag"]
pub type LVDIRDY_R = crate::BitReader;
impl R {
    #[doc = "Bit 7 - Low-voltage detection interrupt status flag"]
    #[inline(always)]
    pub fn lvdirdy(&self) -> LVDIRDY_R {
        LVDIRDY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Low-voltage Detection Circuit Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvd_str2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LVD_STR2_SPEC;
impl crate::RegisterSpec for LVD_STR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lvd_str2::R`](R) reader structure"]
impl crate::Readable for LVD_STR2_SPEC {}
#[doc = "`reset()` method sets LVD_STR2 to value 0x40"]
impl crate::Resettable for LVD_STR2_SPEC {
    const RESET_VALUE: u8 = 0x40;
}
