#[doc = "Register `CHRBAR` reader"]
pub type R = crate::R<CHRBAR_SPEC>;
#[doc = "Field `HRBAR` reader - Host Receive Buffer Address Register"]
pub type HRBAR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Buffer Address Register"]
    #[inline(always)]
    pub fn hrbar(&self) -> HRBAR_R {
        HRBAR_R::new(self.bits)
    }
}
#[doc = "Current Host Receive Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chrbar::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHRBAR_SPEC;
impl crate::RegisterSpec for CHRBAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chrbar::R`](R) reader structure"]
impl crate::Readable for CHRBAR_SPEC {}
#[doc = "`reset()` method sets CHRBAR to value 0"]
impl crate::Resettable for CHRBAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
