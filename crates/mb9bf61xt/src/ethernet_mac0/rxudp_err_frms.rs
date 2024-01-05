#[doc = "Register `rxudp_err_frms` reader"]
pub type R = crate::R<RXUDP_ERR_FRMS_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXUDP_ERR_FRMS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of good IP datagrams whose UDP payload has a checksum error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxudp_err_frms::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXUDP_ERR_FRMS_SPEC;
impl crate::RegisterSpec for RXUDP_ERR_FRMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxudp_err_frms::R`](R) reader structure"]
impl crate::Readable for RXUDP_ERR_FRMS_SPEC {}
#[doc = "`reset()` method sets rxudp_err_frms to value 0"]
impl crate::Resettable for RXUDP_ERR_FRMS_SPEC {
    const RESET_VALUE: u32 = 0;
}
