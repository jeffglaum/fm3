#[doc = "Register `rxipv4_nopay_octets` reader"]
pub type R = crate::R<RXIPV4_NOPAY_OCTETS_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXIPV4_NOPAY_OCTETS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\"Number of bytes received in IPv4 datagrams that did not have a TCP, UDP, or ICMP payload. The value in the IPv4 header's Length field is used to update this counter. \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_nopay_octets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXIPV4_NOPAY_OCTETS_SPEC;
impl crate::RegisterSpec for RXIPV4_NOPAY_OCTETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv4_nopay_octets::R`](R) reader structure"]
impl crate::Readable for RXIPV4_NOPAY_OCTETS_SPEC {}
#[doc = "`reset()` method sets rxipv4_nopay_octets to value 0"]
impl crate::Resettable for RXIPV4_NOPAY_OCTETS_SPEC {
    const RESET_VALUE: u32 = 0;
}
