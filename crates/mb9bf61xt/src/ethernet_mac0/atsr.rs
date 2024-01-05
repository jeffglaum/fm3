#[doc = "Register `ATSR` reader"]
pub type R = crate::R<ATSR_SPEC>;
#[doc = "Field `ATS` reader - ATS"]
pub type ATS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ATS"]
    #[inline(always)]
    pub fn ats(&self) -> ATS_R {
        ATS_R::new(self.bits)
    }
}
#[doc = "Auxiliary Time Stamp - Seconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ATSR_SPEC;
impl crate::RegisterSpec for ATSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atsr::R`](R) reader structure"]
impl crate::Readable for ATSR_SPEC {}
#[doc = "`reset()` method sets ATSR to value 0"]
impl crate::Resettable for ATSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
