#[doc = "Register `AHBSR` reader"]
pub type R = crate::R<AHBSR_SPEC>;
#[doc = "Field `AHBS` reader - AHB Status"]
pub type AHBS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - AHB Status"]
    #[inline(always)]
    pub fn ahbs(&self) -> AHBS_R {
        AHBS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "AHB Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBSR_SPEC;
impl crate::RegisterSpec for AHBSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbsr::R`](R) reader structure"]
impl crate::Readable for AHBSR_SPEC {}
#[doc = "`reset()` method sets AHBSR to value 0"]
impl crate::Resettable for AHBSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
