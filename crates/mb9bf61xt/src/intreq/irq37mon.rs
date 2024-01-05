#[doc = "Register `IRQ37MON` reader"]
pub type R = crate::R<IRQ37MON_SPEC>;
#[doc = "Field `USB1INT0` reader - Endpoint 0 DRQI interrupt request on the USB ch.1"]
pub type USB1INT0_R = crate::BitReader;
#[doc = "Field `USB1INT1` reader - Endpoint 0 DRQO interrupt request on the USB ch.1"]
pub type USB1INT1_R = crate::BitReader;
#[doc = "Field `USB1INT2` reader - \"Status (SUSP, SOF, BRST, CONF, WKUP) interrupt request on the USB ch.1 \""]
pub type USB1INT2_R = crate::BitReader;
#[doc = "Field `USB1INT3` reader - Status (SPK) interrupt request on the USB ch.1"]
pub type USB1INT3_R = crate::BitReader;
#[doc = "Field `USB1INT4` reader - \"Status (DIRQ, URIRQ, RWKIRQ, CNNIRQ) interrupt request on the USB ch.1 \""]
pub type USB1INT4_R = crate::BitReader;
#[doc = "Field `USB1INT5` reader - \"Status (SOFIRQ, CMPIRO) interrupt request on the USB ch.1 \""]
pub type USB1INT5_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Endpoint 0 DRQI interrupt request on the USB ch.1"]
    #[inline(always)]
    pub fn usb1int0(&self) -> USB1INT0_R {
        USB1INT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint 0 DRQO interrupt request on the USB ch.1"]
    #[inline(always)]
    pub fn usb1int1(&self) -> USB1INT1_R {
        USB1INT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - \"Status (SUSP, SOF, BRST, CONF, WKUP) interrupt request on the USB ch.1 \""]
    #[inline(always)]
    pub fn usb1int2(&self) -> USB1INT2_R {
        USB1INT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status (SPK) interrupt request on the USB ch.1"]
    #[inline(always)]
    pub fn usb1int3(&self) -> USB1INT3_R {
        USB1INT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - \"Status (DIRQ, URIRQ, RWKIRQ, CNNIRQ) interrupt request on the USB ch.1 \""]
    #[inline(always)]
    pub fn usb1int4(&self) -> USB1INT4_R {
        USB1INT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - \"Status (SOFIRQ, CMPIRO) interrupt request on the USB ch.1 \""]
    #[inline(always)]
    pub fn usb1int5(&self) -> USB1INT5_R {
        USB1INT5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "IRQ37 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq37mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ37MON_SPEC;
impl crate::RegisterSpec for IRQ37MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq37mon::R`](R) reader structure"]
impl crate::Readable for IRQ37MON_SPEC {}
#[doc = "`reset()` method sets IRQ37MON to value 0"]
impl crate::Resettable for IRQ37MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
