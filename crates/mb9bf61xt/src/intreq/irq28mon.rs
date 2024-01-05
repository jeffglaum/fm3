#[doc = "Register `IRQ28MON` reader"]
pub type R = crate::R<IRQ28MON_SPEC>;
#[doc = "Field `FRT0INT0` reader - Peak value detection interrupt request on the free run timer ch.0 in the MFT unit 0"]
pub type FRT0INT0_R = crate::BitReader;
#[doc = "Field `FRT0INT1` reader - Peak value detection interrupt request on the free run timer ch.1 in the MFT unit 0"]
pub type FRT0INT1_R = crate::BitReader;
#[doc = "Field `FRT0INT2` reader - Peak value detection interrupt request on the free run timer ch.2 in the MFT unit 0"]
pub type FRT0INT2_R = crate::BitReader;
#[doc = "Field `FRT0INT3` reader - Zero detection interrupt request on the free run timer ch.0 in the MFT unit 0"]
pub type FRT0INT3_R = crate::BitReader;
#[doc = "Field `FRT0INT4` reader - Zero detection interrupt request on the free run timer ch.1 in the MFT unit 0"]
pub type FRT0INT4_R = crate::BitReader;
#[doc = "Field `FRT0INT5` reader - Zero detection interrupt request on the free run timer ch.2 in the MFT unit 0"]
pub type FRT0INT5_R = crate::BitReader;
#[doc = "Field `FRT1INT0` reader - Peak value detection interrupt request on the free run timer ch.0 in the MFT unit 1"]
pub type FRT1INT0_R = crate::BitReader;
#[doc = "Field `FRT1INT1` reader - Peak value detection interrupt request on the free run timer ch.1 in the MFT unit 1"]
pub type FRT1INT1_R = crate::BitReader;
#[doc = "Field `FRT1INT2` reader - Peak value detection interrupt request on the free run timer ch.2 in the MFT unit 1"]
pub type FRT1INT2_R = crate::BitReader;
#[doc = "Field `FRT1INT3` reader - Zero detection interrupt request on the free run timer ch.0 in the MFT unit 1"]
pub type FRT1INT3_R = crate::BitReader;
#[doc = "Field `FRT1INT4` reader - Zero detection interrupt request on the free run timer ch.1 in the MFT unit 1"]
pub type FRT1INT4_R = crate::BitReader;
#[doc = "Field `FRT1INT5` reader - Zero detection interrupt request on the free run timer ch.2 in the MFT unit 1"]
pub type FRT1INT5_R = crate::BitReader;
#[doc = "Field `FRT2INT0` reader - Peak value detection interrupt request on the free run timer ch.0 in the MFT unit 2"]
pub type FRT2INT0_R = crate::BitReader;
#[doc = "Field `FRT2INT1` reader - Peak value detection interrupt request on the free run timer ch.1 in the MFT unit 2"]
pub type FRT2INT1_R = crate::BitReader;
#[doc = "Field `FRT2INT2` reader - Peak value detection interrupt request on the free run timer ch.2 in the MFT unit 2"]
pub type FRT2INT2_R = crate::BitReader;
#[doc = "Field `FRT2INT3` reader - Zero detection interrupt request on the free run timer ch.0 in the MFT unit 2"]
pub type FRT2INT3_R = crate::BitReader;
#[doc = "Field `FRT2INT4` reader - Zero detection interrupt request on the free run timer ch.1 in the MFT unit 2"]
pub type FRT2INT4_R = crate::BitReader;
#[doc = "Field `FRT2INT5` reader - Zero detection interrupt request on the free run timer ch.2 in the MFT unit 2"]
pub type FRT2INT5_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Peak value detection interrupt request on the free run timer ch.0 in the MFT unit 0"]
    #[inline(always)]
    pub fn frt0int0(&self) -> FRT0INT0_R {
        FRT0INT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Peak value detection interrupt request on the free run timer ch.1 in the MFT unit 0"]
    #[inline(always)]
    pub fn frt0int1(&self) -> FRT0INT1_R {
        FRT0INT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Peak value detection interrupt request on the free run timer ch.2 in the MFT unit 0"]
    #[inline(always)]
    pub fn frt0int2(&self) -> FRT0INT2_R {
        FRT0INT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Zero detection interrupt request on the free run timer ch.0 in the MFT unit 0"]
    #[inline(always)]
    pub fn frt0int3(&self) -> FRT0INT3_R {
        FRT0INT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Zero detection interrupt request on the free run timer ch.1 in the MFT unit 0"]
    #[inline(always)]
    pub fn frt0int4(&self) -> FRT0INT4_R {
        FRT0INT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Zero detection interrupt request on the free run timer ch.2 in the MFT unit 0"]
    #[inline(always)]
    pub fn frt0int5(&self) -> FRT0INT5_R {
        FRT0INT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peak value detection interrupt request on the free run timer ch.0 in the MFT unit 1"]
    #[inline(always)]
    pub fn frt1int0(&self) -> FRT1INT0_R {
        FRT1INT0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Peak value detection interrupt request on the free run timer ch.1 in the MFT unit 1"]
    #[inline(always)]
    pub fn frt1int1(&self) -> FRT1INT1_R {
        FRT1INT1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Peak value detection interrupt request on the free run timer ch.2 in the MFT unit 1"]
    #[inline(always)]
    pub fn frt1int2(&self) -> FRT1INT2_R {
        FRT1INT2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Zero detection interrupt request on the free run timer ch.0 in the MFT unit 1"]
    #[inline(always)]
    pub fn frt1int3(&self) -> FRT1INT3_R {
        FRT1INT3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Zero detection interrupt request on the free run timer ch.1 in the MFT unit 1"]
    #[inline(always)]
    pub fn frt1int4(&self) -> FRT1INT4_R {
        FRT1INT4_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Zero detection interrupt request on the free run timer ch.2 in the MFT unit 1"]
    #[inline(always)]
    pub fn frt1int5(&self) -> FRT1INT5_R {
        FRT1INT5_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Peak value detection interrupt request on the free run timer ch.0 in the MFT unit 2"]
    #[inline(always)]
    pub fn frt2int0(&self) -> FRT2INT0_R {
        FRT2INT0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Peak value detection interrupt request on the free run timer ch.1 in the MFT unit 2"]
    #[inline(always)]
    pub fn frt2int1(&self) -> FRT2INT1_R {
        FRT2INT1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Peak value detection interrupt request on the free run timer ch.2 in the MFT unit 2"]
    #[inline(always)]
    pub fn frt2int2(&self) -> FRT2INT2_R {
        FRT2INT2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Zero detection interrupt request on the free run timer ch.0 in the MFT unit 2"]
    #[inline(always)]
    pub fn frt2int3(&self) -> FRT2INT3_R {
        FRT2INT3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Zero detection interrupt request on the free run timer ch.1 in the MFT unit 2"]
    #[inline(always)]
    pub fn frt2int4(&self) -> FRT2INT4_R {
        FRT2INT4_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Zero detection interrupt request on the free run timer ch.2 in the MFT unit 2"]
    #[inline(always)]
    pub fn frt2int5(&self) -> FRT2INT5_R {
        FRT2INT5_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "IRQ28 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq28mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ28MON_SPEC;
impl crate::RegisterSpec for IRQ28MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq28mon::R`](R) reader structure"]
impl crate::Readable for IRQ28MON_SPEC {}
#[doc = "`reset()` method sets IRQ28MON to value 0"]
impl crate::Resettable for IRQ28MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
