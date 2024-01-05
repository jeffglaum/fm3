#[doc = "Register `rxipv4_hdrerr_octets` reader"]
pub type R = crate::R<RXIPV4_HDRERR_OCTETS_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXIPV4_HDRERR_OCTETS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\"Number of bytes received in IPv4 datagrams with header errors (checksum, length, version mismatch). The value in the Length field of IPv4 header is used to update this counter. \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_hdrerr_octets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXIPV4_HDRERR_OCTETS_SPEC;
impl crate::RegisterSpec for RXIPV4_HDRERR_OCTETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv4_hdrerr_octets::R`](R) reader structure"]
impl crate::Readable for RXIPV4_HDRERR_OCTETS_SPEC {}
#[doc = "`reset()` method sets rxipv4_hdrerr_octets to value 0"]
impl crate::Resettable for RXIPV4_HDRERR_OCTETS_SPEC {
    const RESET_VALUE: u32 = 0;
}
