#[doc = "Register `UP_STR` reader"]
pub type R = crate::R<UP_STR_SPEC>;
#[doc = "Field `UPRDY` reader - USB/Ethernet-PLL oscillation stabilization bit"]
pub type UPRDY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - USB/Ethernet-PLL oscillation stabilization bit"]
    #[inline(always)]
    pub fn uprdy(&self) -> UPRDY_R {
        UPRDY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "USB/Ethernet-PLL Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`up_str::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UP_STR_SPEC;
impl crate::RegisterSpec for UP_STR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`up_str::R`](R) reader structure"]
impl crate::Readable for UP_STR_SPEC {}
#[doc = "`reset()` method sets UP_STR to value 0"]
impl crate::Resettable for UP_STR_SPEC {
    const RESET_VALUE: u8 = 0;
}
