#[doc = "Register `IRQ35MON` reader"]
pub type R = crate::R<IRQ35MON_SPEC>;
#[doc = "Field `USB0INT0` reader - Endpoint 0 DRQI interrupt request on the USB ch.0"]
pub type USB0INT0_R = crate::BitReader;
#[doc = "Field `USB0INT1` reader - Endpoint 0 DRQO interrupt request on the USB ch.0"]
pub type USB0INT1_R = crate::BitReader;
#[doc = "Field `USB0INT2` reader - \"Status (SUSP, SOF, BRST, CONF, WKUP) interrupt request on the USB ch.0 \""]
pub type USB0INT2_R = crate::BitReader;
#[doc = "Field `USB0INT3` reader - Status (SPK) interrupt request on the USB ch.0"]
pub type USB0INT3_R = crate::BitReader;
#[doc = "Field `USB0INT4` reader - \"Status (DIRQ, URIRQ, RWKIRQ, CNNIRQ) interrupt request on the USB ch.0 \""]
pub type USB0INT4_R = crate::BitReader;
#[doc = "Field `USB0INT5` reader - \"Status (SOFIRQ, CMPIRO) interrupt request on the USB ch.0 \""]
pub type USB0INT5_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Endpoint 0 DRQI interrupt request on the USB ch.0"]
    #[inline(always)]
    pub fn usb0int0(&self) -> USB0INT0_R {
        USB0INT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint 0 DRQO interrupt request on the USB ch.0"]
    #[inline(always)]
    pub fn usb0int1(&self) -> USB0INT1_R {
        USB0INT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - \"Status (SUSP, SOF, BRST, CONF, WKUP) interrupt request on the USB ch.0 \""]
    #[inline(always)]
    pub fn usb0int2(&self) -> USB0INT2_R {
        USB0INT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status (SPK) interrupt request on the USB ch.0"]
    #[inline(always)]
    pub fn usb0int3(&self) -> USB0INT3_R {
        USB0INT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - \"Status (DIRQ, URIRQ, RWKIRQ, CNNIRQ) interrupt request on the USB ch.0 \""]
    #[inline(always)]
    pub fn usb0int4(&self) -> USB0INT4_R {
        USB0INT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - \"Status (SOFIRQ, CMPIRO) interrupt request on the USB ch.0 \""]
    #[inline(always)]
    pub fn usb0int5(&self) -> USB0INT5_R {
        USB0INT5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "IRQ35 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq35mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ35MON_SPEC;
impl crate::RegisterSpec for IRQ35MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq35mon::R`](R) reader structure"]
impl crate::Readable for IRQ35MON_SPEC {}
#[doc = "`reset()` method sets IRQ35MON to value 0"]
impl crate::Resettable for IRQ35MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
