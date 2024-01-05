#[doc = "Register `rxipv6_hdrerr_octets` reader"]
pub type R = crate::R<RXIPV6_HDRERR_OCTETS_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXIPV6_HDRERR_OCTETS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\"Number of bytes received in IPv6 datagrams with header errors (length, version mismatch). The value in the IPv6 header's Length field is used to update this counter. \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv6_hdrerr_octets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXIPV6_HDRERR_OCTETS_SPEC;
impl crate::RegisterSpec for RXIPV6_HDRERR_OCTETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv6_hdrerr_octets::R`](R) reader structure"]
impl crate::Readable for RXIPV6_HDRERR_OCTETS_SPEC {}
#[doc = "`reset()` method sets rxipv6_hdrerr_octets to value 0"]
impl crate::Resettable for RXIPV6_HDRERR_OCTETS_SPEC {
    const RESET_VALUE: u32 = 0;
}
