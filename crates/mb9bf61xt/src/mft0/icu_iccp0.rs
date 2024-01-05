#[doc = "Register `ICU_ICCP0` reader"]
pub type R = crate::R<ICU_ICCP0_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<ICU_ICCP0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "ICU ch.0 Capture value store register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icu_iccp0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICU_ICCP0_SPEC;
impl crate::RegisterSpec for ICU_ICCP0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`icu_iccp0::R`](R) reader structure"]
impl crate::Readable for ICU_ICCP0_SPEC {}
#[doc = "`reset()` method sets ICU_ICCP0 to value 0"]
impl crate::Resettable for ICU_ICCP0_SPEC {
    const RESET_VALUE: u16 = 0;
}
