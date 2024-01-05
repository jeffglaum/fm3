#[doc = "Register `rxtcp_err_octets` reader"]
pub type R = crate::R<RXTCP_ERR_OCTETS_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXTCP_ERR_OCTETS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of bytes received in a TCP segment with checksum errors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtcp_err_octets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXTCP_ERR_OCTETS_SPEC;
impl crate::RegisterSpec for RXTCP_ERR_OCTETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxtcp_err_octets::R`](R) reader structure"]
impl crate::Readable for RXTCP_ERR_OCTETS_SPEC {}
#[doc = "`reset()` method sets rxtcp_err_octets to value 0"]
impl crate::Resettable for RXTCP_ERR_OCTETS_SPEC {
    const RESET_VALUE: u32 = 0;
}
