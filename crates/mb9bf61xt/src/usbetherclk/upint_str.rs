#[doc = "Register `UPINT_STR` reader"]
pub type R = crate::R<UPINT_STR_SPEC>;
#[doc = "Field `UPCSI` reader - USB/Ethernet-PLL interrupt source status bit"]
pub type UPCSI_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - USB/Ethernet-PLL interrupt source status bit"]
    #[inline(always)]
    pub fn upcsi(&self) -> UPCSI_R {
        UPCSI_R::new((self.bits & 1) != 0)
    }
}
#[doc = "USB/Ethernet-PLL Interrupt Source Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`upint_str::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UPINT_STR_SPEC;
impl crate::RegisterSpec for UPINT_STR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`upint_str::R`](R) reader structure"]
impl crate::Readable for UPINT_STR_SPEC {}
#[doc = "`reset()` method sets UPINT_STR to value 0"]
impl crate::Resettable for UPINT_STR_SPEC {
    const RESET_VALUE: u8 = 0;
}
