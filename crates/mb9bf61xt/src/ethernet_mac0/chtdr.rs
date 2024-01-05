#[doc = "Register `CHTDR` reader"]
pub type R = crate::R<CHTDR_SPEC>;
#[doc = "Field `HTDAP` reader - Host Transmit Descriptor Address Pointer"]
pub type HTDAP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Transmit Descriptor Address Pointer"]
    #[inline(always)]
    pub fn htdap(&self) -> HTDAP_R {
        HTDAP_R::new(self.bits)
    }
}
#[doc = "Current Host Transmit Descriptor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chtdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHTDR_SPEC;
impl crate::RegisterSpec for CHTDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chtdr::R`](R) reader structure"]
impl crate::Readable for CHTDR_SPEC {}
#[doc = "`reset()` method sets CHTDR to value 0"]
impl crate::Resettable for CHTDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
