#[doc = "Register `IRQ06MON` reader"]
pub type R = crate::R<IRQ06MON_SPEC>;
#[doc = "Field `TIMINT1` reader - Dual timer 1 interrupt request"]
pub type TIMINT1_R = crate::BitReader;
#[doc = "Field `TIMINT2` reader - Dual timer 2 interrupt request"]
pub type TIMINT2_R = crate::BitReader;
#[doc = "Field `QUD0INT0` reader - PC match interrupt request on QPRC ch.0"]
pub type QUD0INT0_R = crate::BitReader;
#[doc = "Field `QUD0INT1` reader - PC and RC match interrupt request on QPRC ch.0"]
pub type QUD0INT1_R = crate::BitReader;
#[doc = "Field `QUD0INT2` reader - Overflow/underflow/zero index interrupt request on QPRC ch.0"]
pub type QUD0INT2_R = crate::BitReader;
#[doc = "Field `QUD0INT3` reader - PC count invert interrupt request on QPRC ch.0"]
pub type QUD0INT3_R = crate::BitReader;
#[doc = "Field `QUD0INT4` reader - Interrupt request detected RC out of range on QPRC ch.0"]
pub type QUD0INT4_R = crate::BitReader;
#[doc = "Field `QUD0INT5` reader - PC match and RC match interrupt request on QPRC ch.0"]
pub type QUD0INT5_R = crate::BitReader;
#[doc = "Field `QUD1INT0` reader - PC match interrupt request on QPRC ch.1"]
pub type QUD1INT0_R = crate::BitReader;
#[doc = "Field `QUD1INT1` reader - PC and RC match interrupt request on QPRC ch.1"]
pub type QUD1INT1_R = crate::BitReader;
#[doc = "Field `QUD1INT2` reader - Overflow/underflow/zero index interrupt request on QPRC ch.1"]
pub type QUD1INT2_R = crate::BitReader;
#[doc = "Field `QUD1INT3` reader - PC count invert interrupt request on QPRC ch.1"]
pub type QUD1INT3_R = crate::BitReader;
#[doc = "Field `QUD1INT4` reader - Interrupt request detected RC out of range on QPRC ch.1"]
pub type QUD1INT4_R = crate::BitReader;
#[doc = "Field `QUD1INT5` reader - PC match and RC match interrupt request on QPRC ch.1"]
pub type QUD1INT5_R = crate::BitReader;
#[doc = "Field `QUD2INT0` reader - PC match interrupt request on QPRC ch.2"]
pub type QUD2INT0_R = crate::BitReader;
#[doc = "Field `QUD2INT1` reader - PC and RC match interrupt request on QPRC ch.2"]
pub type QUD2INT1_R = crate::BitReader;
#[doc = "Field `QUD2INT2` reader - Overflow/underflow/zero index interrupt request on QPRC ch.2"]
pub type QUD2INT2_R = crate::BitReader;
#[doc = "Field `QUD2INT3` reader - PC count invert interrupt request on QPRC ch.2"]
pub type QUD2INT3_R = crate::BitReader;
#[doc = "Field `QUD2INT4` reader - Interrupt request detected RC out of range on QPRC ch.2"]
pub type QUD2INT4_R = crate::BitReader;
#[doc = "Field `QUD2INT5` reader - PC match and RC match interrupt request on QPRC ch.2"]
pub type QUD2INT5_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Dual timer 1 interrupt request"]
    #[inline(always)]
    pub fn timint1(&self) -> TIMINT1_R {
        TIMINT1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Dual timer 2 interrupt request"]
    #[inline(always)]
    pub fn timint2(&self) -> TIMINT2_R {
        TIMINT2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PC match interrupt request on QPRC ch.0"]
    #[inline(always)]
    pub fn qud0int0(&self) -> QUD0INT0_R {
        QUD0INT0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PC and RC match interrupt request on QPRC ch.0"]
    #[inline(always)]
    pub fn qud0int1(&self) -> QUD0INT1_R {
        QUD0INT1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overflow/underflow/zero index interrupt request on QPRC ch.0"]
    #[inline(always)]
    pub fn qud0int2(&self) -> QUD0INT2_R {
        QUD0INT2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PC count invert interrupt request on QPRC ch.0"]
    #[inline(always)]
    pub fn qud0int3(&self) -> QUD0INT3_R {
        QUD0INT3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt request detected RC out of range on QPRC ch.0"]
    #[inline(always)]
    pub fn qud0int4(&self) -> QUD0INT4_R {
        QUD0INT4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PC match and RC match interrupt request on QPRC ch.0"]
    #[inline(always)]
    pub fn qud0int5(&self) -> QUD0INT5_R {
        QUD0INT5_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PC match interrupt request on QPRC ch.1"]
    #[inline(always)]
    pub fn qud1int0(&self) -> QUD1INT0_R {
        QUD1INT0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PC and RC match interrupt request on QPRC ch.1"]
    #[inline(always)]
    pub fn qud1int1(&self) -> QUD1INT1_R {
        QUD1INT1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Overflow/underflow/zero index interrupt request on QPRC ch.1"]
    #[inline(always)]
    pub fn qud1int2(&self) -> QUD1INT2_R {
        QUD1INT2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PC count invert interrupt request on QPRC ch.1"]
    #[inline(always)]
    pub fn qud1int3(&self) -> QUD1INT3_R {
        QUD1INT3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt request detected RC out of range on QPRC ch.1"]
    #[inline(always)]
    pub fn qud1int4(&self) -> QUD1INT4_R {
        QUD1INT4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PC match and RC match interrupt request on QPRC ch.1"]
    #[inline(always)]
    pub fn qud1int5(&self) -> QUD1INT5_R {
        QUD1INT5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PC match interrupt request on QPRC ch.2"]
    #[inline(always)]
    pub fn qud2int0(&self) -> QUD2INT0_R {
        QUD2INT0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PC and RC match interrupt request on QPRC ch.2"]
    #[inline(always)]
    pub fn qud2int1(&self) -> QUD2INT1_R {
        QUD2INT1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Overflow/underflow/zero index interrupt request on QPRC ch.2"]
    #[inline(always)]
    pub fn qud2int2(&self) -> QUD2INT2_R {
        QUD2INT2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PC count invert interrupt request on QPRC ch.2"]
    #[inline(always)]
    pub fn qud2int3(&self) -> QUD2INT3_R {
        QUD2INT3_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt request detected RC out of range on QPRC ch.2"]
    #[inline(always)]
    pub fn qud2int4(&self) -> QUD2INT4_R {
        QUD2INT4_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PC match and RC match interrupt request on QPRC ch.2"]
    #[inline(always)]
    pub fn qud2int5(&self) -> QUD2INT5_R {
        QUD2INT5_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "IRQ06 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq06mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ06MON_SPEC;
impl crate::RegisterSpec for IRQ06MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq06mon::R`](R) reader structure"]
impl crate::Readable for IRQ06MON_SPEC {}
#[doc = "`reset()` method sets IRQ06MON to value 0"]
impl crate::Resettable for IRQ06MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
