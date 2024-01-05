#[doc = "Register `rxipv6_nopay_frms` reader"]
pub type R = crate::R<RXIPV6_NOPAY_FRMS_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXIPV6_NOPAY_FRMS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\"Number of IPv6 datagram frames received that did not have a TCP, UDP, or ICMP payload. This includes all IPv6 datagrams with fragmentation or security extension headers \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv6_nopay_frms::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXIPV6_NOPAY_FRMS_SPEC;
impl crate::RegisterSpec for RXIPV6_NOPAY_FRMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv6_nopay_frms::R`](R) reader structure"]
impl crate::Readable for RXIPV6_NOPAY_FRMS_SPEC {}
#[doc = "`reset()` method sets rxipv6_nopay_frms to value 0"]
impl crate::Resettable for RXIPV6_NOPAY_FRMS_SPEC {
    const RESET_VALUE: u32 = 0;
}
