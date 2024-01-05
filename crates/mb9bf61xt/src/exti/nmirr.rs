#[doc = "Register `NMIRR` reader"]
pub type R = crate::R<NMIRR_SPEC>;
#[doc = "Field `NR` reader - NMI interrupt request detection bit"]
pub type NR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NMI interrupt request detection bit"]
    #[inline(always)]
    pub fn nr(&self) -> NR_R {
        NR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Non Maskable Interrupt Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nmirr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NMIRR_SPEC;
impl crate::RegisterSpec for NMIRR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`nmirr::R`](R) reader structure"]
impl crate::Readable for NMIRR_SPEC {}
#[doc = "`reset()` method sets NMIRR to value 0"]
impl crate::Resettable for NMIRR_SPEC {
    const RESET_VALUE: u8 = 0;
}
