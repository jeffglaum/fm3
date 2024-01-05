#[doc = "Register `IRQ36MON` reader"]
pub type R = crate::R<IRQ36MON_SPEC>;
#[doc = "Field `USB1INT0` reader - Endpoint 1 DRQ interrupt request on the USB ch.1"]
pub type USB1INT0_R = crate::BitReader;
#[doc = "Field `USB1INT1` reader - Endpoint 2 DRQ interrupt request on the USB ch.1"]
pub type USB1INT1_R = crate::BitReader;
#[doc = "Field `USB1INT2` reader - Endpoint 3 DRQ interrupt request on the USB ch.1"]
pub type USB1INT2_R = crate::BitReader;
#[doc = "Field `USB1INT3` reader - Endpoint 4 DRQ interrupt request on the USB ch.1"]
pub type USB1INT3_R = crate::BitReader;
#[doc = "Field `USB1INT4` reader - Endpoint 5 DRQ interrupt request on the USB ch.1"]
pub type USB1INT4_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Endpoint 1 DRQ interrupt request on the USB ch.1"]
    #[inline(always)]
    pub fn usb1int0(&self) -> USB1INT0_R {
        USB1INT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint 2 DRQ interrupt request on the USB ch.1"]
    #[inline(always)]
    pub fn usb1int1(&self) -> USB1INT1_R {
        USB1INT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint 3 DRQ interrupt request on the USB ch.1"]
    #[inline(always)]
    pub fn usb1int2(&self) -> USB1INT2_R {
        USB1INT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint 4 DRQ interrupt request on the USB ch.1"]
    #[inline(always)]
    pub fn usb1int3(&self) -> USB1INT3_R {
        USB1INT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint 5 DRQ interrupt request on the USB ch.1"]
    #[inline(always)]
    pub fn usb1int4(&self) -> USB1INT4_R {
        USB1INT4_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "IRQ36 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq36mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ36MON_SPEC;
impl crate::RegisterSpec for IRQ36MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq36mon::R`](R) reader structure"]
impl crate::Readable for IRQ36MON_SPEC {}
#[doc = "`reset()` method sets IRQ36MON to value 0"]
impl crate::Resettable for IRQ36MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
