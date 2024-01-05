#[doc = "Register `CRTRMM` reader"]
pub type R = crate::R<CRTRMM_SPEC>;
#[doc = "Field `TRMM` reader - CR Trimming Data Mirror"]
pub type TRMM_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - CR Trimming Data Mirror"]
    #[inline(always)]
    pub fn trmm(&self) -> TRMM_R {
        TRMM_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "CR Trimming Data Mirror Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crtrmm::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRTRMM_SPEC;
impl crate::RegisterSpec for CRTRMM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crtrmm::R`](R) reader structure"]
impl crate::Readable for CRTRMM_SPEC {}
#[doc = "`reset()` method sets CRTRMM to value 0"]
impl crate::Resettable for CRTRMM_SPEC {
    const RESET_VALUE: u32 = 0;
}
