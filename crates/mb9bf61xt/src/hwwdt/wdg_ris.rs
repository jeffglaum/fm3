#[doc = "Register `WDG_RIS` reader"]
pub type R = crate::R<WDG_RIS_SPEC>;
#[doc = "Field `RIS` reader - Hardware watchdog interrupt status bit"]
pub type RIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Hardware watchdog interrupt status bit"]
    #[inline(always)]
    pub fn ris(&self) -> RIS_R {
        RIS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Hardware Watchdog Timer Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdg_ris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDG_RIS_SPEC;
impl crate::RegisterSpec for WDG_RIS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wdg_ris::R`](R) reader structure"]
impl crate::Readable for WDG_RIS_SPEC {}
#[doc = "`reset()` method sets WDG_RIS to value 0xff"]
impl crate::Resettable for WDG_RIS_SPEC {
    const RESET_VALUE: u8 = 0xff;
}
