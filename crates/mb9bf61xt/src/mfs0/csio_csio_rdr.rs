#[doc = "Register `CSIO_RDR` reader"]
pub type R = crate::R<CSIO_CSIO_RDR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CSIO_CSIO_RDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Received Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csio_csio_rdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIO_CSIO_RDR_SPEC;
impl crate::RegisterSpec for CSIO_CSIO_RDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csio_csio_rdr::R`](R) reader structure"]
impl crate::Readable for CSIO_CSIO_RDR_SPEC {}
#[doc = "`reset()` method sets CSIO_RDR to value 0"]
impl crate::Resettable for CSIO_CSIO_RDR_SPEC {
    const RESET_VALUE: u16 = 0;
}
