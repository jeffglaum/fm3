#[doc = "Register `PWC_DTBF` reader"]
pub type R = crate::R<PWC_PWC_DTBF_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PWC_PWC_DTBF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Data Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwc_pwc_dtbf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWC_PWC_DTBF_SPEC;
impl crate::RegisterSpec for PWC_PWC_DTBF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pwc_pwc_dtbf::R`](R) reader structure"]
impl crate::Readable for PWC_PWC_DTBF_SPEC {}
#[doc = "`reset()` method sets PWC_DTBF to value 0"]
impl crate::Resettable for PWC_PWC_DTBF_SPEC {
    const RESET_VALUE: u16 = 0;
}
