#[doc = "Register `CHTBAR` reader"]
pub type R = crate::R<CHTBAR_SPEC>;
#[doc = "Field `HTBAR` reader - Host Transmit Buffer Address Register"]
pub type HTBAR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Transmit Buffer Address Register"]
    #[inline(always)]
    pub fn htbar(&self) -> HTBAR_R {
        HTBAR_R::new(self.bits)
    }
}
#[doc = "Current Host Transmit Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chtbar::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHTBAR_SPEC;
impl crate::RegisterSpec for CHTBAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chtbar::R`](R) reader structure"]
impl crate::Readable for CHTBAR_SPEC {}
#[doc = "`reset()` method sets CHTBAR to value 0"]
impl crate::Resettable for CHTBAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
