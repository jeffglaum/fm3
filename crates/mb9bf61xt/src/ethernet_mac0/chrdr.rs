#[doc = "Register `CHRDR` reader"]
pub type R = crate::R<CHRDR_SPEC>;
#[doc = "Field `HRDAP` reader - Host Receive Descriptor Address Pointer"]
pub type HRDAP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Descriptor Address Pointer"]
    #[inline(always)]
    pub fn hrdap(&self) -> HRDAP_R {
        HRDAP_R::new(self.bits)
    }
}
#[doc = "Current Host Receive Descriptor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chrdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHRDR_SPEC;
impl crate::RegisterSpec for CHRDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chrdr::R`](R) reader structure"]
impl crate::Readable for CHRDR_SPEC {}
#[doc = "`reset()` method sets CHRDR to value 0"]
impl crate::Resettable for CHRDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
