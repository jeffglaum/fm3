#[doc = "Register `rxtcp_gd_octets` reader"]
pub type R = crate::R<RXTCP_GD_OCTETS_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXTCP_GD_OCTETS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of bytes received in a good TCP segment\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtcp_gd_octets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXTCP_GD_OCTETS_SPEC;
impl crate::RegisterSpec for RXTCP_GD_OCTETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxtcp_gd_octets::R`](R) reader structure"]
impl crate::Readable for RXTCP_GD_OCTETS_SPEC {}
#[doc = "`reset()` method sets rxtcp_gd_octets to value 0"]
impl crate::Resettable for RXTCP_GD_OCTETS_SPEC {
    const RESET_VALUE: u32 = 0;
}
