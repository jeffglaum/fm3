#[doc = "Register `IRQ30MON` reader"]
pub type R = crate::R<IRQ30MON_SPEC>;
#[doc = "Field `OCU0INT0` reader - Interrupt request on the output compare ch.0 in the MFT unit 0"]
pub type OCU0INT0_R = crate::BitReader;
#[doc = "Field `OCU0INT1` reader - Interrupt request on the output compare ch.1 in the MFT unit 0"]
pub type OCU0INT1_R = crate::BitReader;
#[doc = "Field `OCU0INT2` reader - Interrupt request on the output compare ch.2 in the MFT unit 0"]
pub type OCU0INT2_R = crate::BitReader;
#[doc = "Field `OCU0INT3` reader - Interrupt request on the output compare ch.3 in the MFT unit 0"]
pub type OCU0INT3_R = crate::BitReader;
#[doc = "Field `OCU0INT4` reader - Interrupt request on the output compare ch.4 in the MFT unit 0"]
pub type OCU0INT4_R = crate::BitReader;
#[doc = "Field `OCU0INT5` reader - Interrupt request on the output compare ch.5 in the MFT unit 0"]
pub type OCU0INT5_R = crate::BitReader;
#[doc = "Field `OCU1INT0` reader - Interrupt request on the output compare ch.0 in the MFT unit 1"]
pub type OCU1INT0_R = crate::BitReader;
#[doc = "Field `OCU1INT1` reader - Interrupt request on the output compare ch.1 in the MFT unit 1"]
pub type OCU1INT1_R = crate::BitReader;
#[doc = "Field `OCU1INT2` reader - Interrupt request on the output compare ch.2 in the MFT unit 1"]
pub type OCU1INT2_R = crate::BitReader;
#[doc = "Field `OCU1INT3` reader - Interrupt request on the output compare ch.3 in the MFT unit 1"]
pub type OCU1INT3_R = crate::BitReader;
#[doc = "Field `OCU1INT4` reader - Interrupt request on the output compare ch.4 in the MFT unit 1"]
pub type OCU1INT4_R = crate::BitReader;
#[doc = "Field `OCU1INT5` reader - Interrupt request on the output compare ch.5 in the MFT unit 1"]
pub type OCU1INT5_R = crate::BitReader;
#[doc = "Field `OCU2INT0` reader - Interrupt request on the output compare ch.0 in the MFT unit 2"]
pub type OCU2INT0_R = crate::BitReader;
#[doc = "Field `OCU2INT1` reader - Interrupt request on the output compare ch.1 in the MFT unit 2"]
pub type OCU2INT1_R = crate::BitReader;
#[doc = "Field `OCU2INT2` reader - Interrupt request on the output compare ch.2 in the MFT unit 2"]
pub type OCU2INT2_R = crate::BitReader;
#[doc = "Field `OCU2INT3` reader - Interrupt request on the output compare ch.3 in the MFT unit 2"]
pub type OCU2INT3_R = crate::BitReader;
#[doc = "Field `OCU2INT4` reader - Interrupt request on the output compare ch.4 in the MFT unit 2"]
pub type OCU2INT4_R = crate::BitReader;
#[doc = "Field `OCU2INT5` reader - Interrupt request on the output compare ch.5 in the MFT unit 2"]
pub type OCU2INT5_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt request on the output compare ch.0 in the MFT unit 0"]
    #[inline(always)]
    pub fn ocu0int0(&self) -> OCU0INT0_R {
        OCU0INT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt request on the output compare ch.1 in the MFT unit 0"]
    #[inline(always)]
    pub fn ocu0int1(&self) -> OCU0INT1_R {
        OCU0INT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt request on the output compare ch.2 in the MFT unit 0"]
    #[inline(always)]
    pub fn ocu0int2(&self) -> OCU0INT2_R {
        OCU0INT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt request on the output compare ch.3 in the MFT unit 0"]
    #[inline(always)]
    pub fn ocu0int3(&self) -> OCU0INT3_R {
        OCU0INT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt request on the output compare ch.4 in the MFT unit 0"]
    #[inline(always)]
    pub fn ocu0int4(&self) -> OCU0INT4_R {
        OCU0INT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt request on the output compare ch.5 in the MFT unit 0"]
    #[inline(always)]
    pub fn ocu0int5(&self) -> OCU0INT5_R {
        OCU0INT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt request on the output compare ch.0 in the MFT unit 1"]
    #[inline(always)]
    pub fn ocu1int0(&self) -> OCU1INT0_R {
        OCU1INT0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt request on the output compare ch.1 in the MFT unit 1"]
    #[inline(always)]
    pub fn ocu1int1(&self) -> OCU1INT1_R {
        OCU1INT1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt request on the output compare ch.2 in the MFT unit 1"]
    #[inline(always)]
    pub fn ocu1int2(&self) -> OCU1INT2_R {
        OCU1INT2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt request on the output compare ch.3 in the MFT unit 1"]
    #[inline(always)]
    pub fn ocu1int3(&self) -> OCU1INT3_R {
        OCU1INT3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt request on the output compare ch.4 in the MFT unit 1"]
    #[inline(always)]
    pub fn ocu1int4(&self) -> OCU1INT4_R {
        OCU1INT4_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt request on the output compare ch.5 in the MFT unit 1"]
    #[inline(always)]
    pub fn ocu1int5(&self) -> OCU1INT5_R {
        OCU1INT5_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt request on the output compare ch.0 in the MFT unit 2"]
    #[inline(always)]
    pub fn ocu2int0(&self) -> OCU2INT0_R {
        OCU2INT0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt request on the output compare ch.1 in the MFT unit 2"]
    #[inline(always)]
    pub fn ocu2int1(&self) -> OCU2INT1_R {
        OCU2INT1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt request on the output compare ch.2 in the MFT unit 2"]
    #[inline(always)]
    pub fn ocu2int2(&self) -> OCU2INT2_R {
        OCU2INT2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt request on the output compare ch.3 in the MFT unit 2"]
    #[inline(always)]
    pub fn ocu2int3(&self) -> OCU2INT3_R {
        OCU2INT3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt request on the output compare ch.4 in the MFT unit 2"]
    #[inline(always)]
    pub fn ocu2int4(&self) -> OCU2INT4_R {
        OCU2INT4_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt request on the output compare ch.5 in the MFT unit 2"]
    #[inline(always)]
    pub fn ocu2int5(&self) -> OCU2INT5_R {
        OCU2INT5_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "IRQ30 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq30mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ30MON_SPEC;
impl crate::RegisterSpec for IRQ30MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq30mon::R`](R) reader structure"]
impl crate::Readable for IRQ30MON_SPEC {}
#[doc = "`reset()` method sets IRQ30MON to value 0"]
impl crate::Resettable for IRQ30MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
