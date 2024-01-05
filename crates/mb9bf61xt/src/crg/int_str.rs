#[doc = "Register `INT_STR` reader"]
pub type R = crate::R<INT_STR_SPEC>;
#[doc = "Field `MCSI` reader - Main oscillation stabilization completion interrupt status bit"]
pub type MCSI_R = crate::BitReader;
#[doc = "Field `SCSI` reader - Sub oscillation stabilization completion interrupt status bit"]
pub type SCSI_R = crate::BitReader;
#[doc = "Field `PCSI` reader - PLL oscillation stabilization completion interrupt status bit"]
pub type PCSI_R = crate::BitReader;
#[doc = "Field `FCSI` reader - Anomalous frequency detection interrupt status bit"]
pub type FCSI_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Main oscillation stabilization completion interrupt status bit"]
    #[inline(always)]
    pub fn mcsi(&self) -> MCSI_R {
        MCSI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sub oscillation stabilization completion interrupt status bit"]
    #[inline(always)]
    pub fn scsi(&self) -> SCSI_R {
        SCSI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PLL oscillation stabilization completion interrupt status bit"]
    #[inline(always)]
    pub fn pcsi(&self) -> PCSI_R {
        PCSI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Anomalous frequency detection interrupt status bit"]
    #[inline(always)]
    pub fn fcsi(&self) -> FCSI_R {
        FCSI_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_str::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_STR_SPEC;
impl crate::RegisterSpec for INT_STR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`int_str::R`](R) reader structure"]
impl crate::Readable for INT_STR_SPEC {}
#[doc = "`reset()` method sets INT_STR to value 0"]
impl crate::Resettable for INT_STR_SPEC {
    const RESET_VALUE: u8 = 0;
}
