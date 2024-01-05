#[doc = "Register `rxicmp_gd_frms` reader"]
pub type R = crate::R<RXICMP_GD_FRMS_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXICMP_GD_FRMS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of good IP datagrams with a good ICMP payload\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxicmp_gd_frms::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXICMP_GD_FRMS_SPEC;
impl crate::RegisterSpec for RXICMP_GD_FRMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxicmp_gd_frms::R`](R) reader structure"]
impl crate::Readable for RXICMP_GD_FRMS_SPEC {}
#[doc = "`reset()` method sets rxicmp_gd_frms to value 0"]
impl crate::Resettable for RXICMP_GD_FRMS_SPEC {
    const RESET_VALUE: u32 = 0;
}
