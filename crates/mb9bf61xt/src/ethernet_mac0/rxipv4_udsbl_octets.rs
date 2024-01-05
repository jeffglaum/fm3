#[doc = "Register `rxipv4_udsbl_octets` reader"]
pub type R = crate::R<RXIPV4_UDSBL_OCTETS_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXIPV4_UDSBL_OCTETS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of bytes received in a UDP segment that had the UDP checksum disabled. This counter does not count IP Header bytes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_udsbl_octets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXIPV4_UDSBL_OCTETS_SPEC;
impl crate::RegisterSpec for RXIPV4_UDSBL_OCTETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv4_udsbl_octets::R`](R) reader structure"]
impl crate::Readable for RXIPV4_UDSBL_OCTETS_SPEC {}
#[doc = "`reset()` method sets rxipv4_udsbl_octets to value 0"]
impl crate::Resettable for RXIPV4_UDSBL_OCTETS_SPEC {
    const RESET_VALUE: u32 = 0;
}
