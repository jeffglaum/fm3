#[doc = "Register `IRQ03MON` reader"]
pub type R = crate::R<IRQ03MON_SPEC>;
#[doc = "Field `WAVE0INT0` reader - DTIF (motor emergency stop) interrupt request in MFT unit 0"]
pub type WAVE0INT0_R = crate::BitReader;
#[doc = "Field `WAVE0INT1` reader - WFG timer 10 interrupt request in MFT unit 0"]
pub type WAVE0INT1_R = crate::BitReader;
#[doc = "Field `WAVE0INT2` reader - WFG timer 32 interrupt request in MFT unit 0"]
pub type WAVE0INT2_R = crate::BitReader;
#[doc = "Field `WAVE0INT3` reader - WFG timer 54 interrupt request in MFT unit 0"]
pub type WAVE0INT3_R = crate::BitReader;
#[doc = "Field `WAVE1INT0` reader - DTIF (motor emergency stop) interrupt request in MFT unit 1"]
pub type WAVE1INT0_R = crate::BitReader;
#[doc = "Field `WAVE1INT1` reader - WFG timer 10 interrupt request in MFT unit 1"]
pub type WAVE1INT1_R = crate::BitReader;
#[doc = "Field `WAVE1INT2` reader - WFG timer 32 interrupt request in MFT unit 1"]
pub type WAVE1INT2_R = crate::BitReader;
#[doc = "Field `WAVE1INT3` reader - WFG timer 54 interrupt request in MFT unit 1"]
pub type WAVE1INT3_R = crate::BitReader;
#[doc = "Field `WAVE2INT0` reader - DTIF (motor emergency stop) interrupt request in MFT unit 2"]
pub type WAVE2INT0_R = crate::BitReader;
#[doc = "Field `WAVE2INT1` reader - WFG timer 10 interrupt request in MFT unit 2"]
pub type WAVE2INT1_R = crate::BitReader;
#[doc = "Field `WAVE2INT2` reader - WFG timer 32 interrupt request in MFT unit 2"]
pub type WAVE2INT2_R = crate::BitReader;
#[doc = "Field `WAVE2INT3` reader - WFG timer 54 interrupt request in MFT unit 2"]
pub type WAVE2INT3_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DTIF (motor emergency stop) interrupt request in MFT unit 0"]
    #[inline(always)]
    pub fn wave0int0(&self) -> WAVE0INT0_R {
        WAVE0INT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WFG timer 10 interrupt request in MFT unit 0"]
    #[inline(always)]
    pub fn wave0int1(&self) -> WAVE0INT1_R {
        WAVE0INT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WFG timer 32 interrupt request in MFT unit 0"]
    #[inline(always)]
    pub fn wave0int2(&self) -> WAVE0INT2_R {
        WAVE0INT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WFG timer 54 interrupt request in MFT unit 0"]
    #[inline(always)]
    pub fn wave0int3(&self) -> WAVE0INT3_R {
        WAVE0INT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DTIF (motor emergency stop) interrupt request in MFT unit 1"]
    #[inline(always)]
    pub fn wave1int0(&self) -> WAVE1INT0_R {
        WAVE1INT0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - WFG timer 10 interrupt request in MFT unit 1"]
    #[inline(always)]
    pub fn wave1int1(&self) -> WAVE1INT1_R {
        WAVE1INT1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - WFG timer 32 interrupt request in MFT unit 1"]
    #[inline(always)]
    pub fn wave1int2(&self) -> WAVE1INT2_R {
        WAVE1INT2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - WFG timer 54 interrupt request in MFT unit 1"]
    #[inline(always)]
    pub fn wave1int3(&self) -> WAVE1INT3_R {
        WAVE1INT3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DTIF (motor emergency stop) interrupt request in MFT unit 2"]
    #[inline(always)]
    pub fn wave2int0(&self) -> WAVE2INT0_R {
        WAVE2INT0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WFG timer 10 interrupt request in MFT unit 2"]
    #[inline(always)]
    pub fn wave2int1(&self) -> WAVE2INT1_R {
        WAVE2INT1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - WFG timer 32 interrupt request in MFT unit 2"]
    #[inline(always)]
    pub fn wave2int2(&self) -> WAVE2INT2_R {
        WAVE2INT2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WFG timer 54 interrupt request in MFT unit 2"]
    #[inline(always)]
    pub fn wave2int3(&self) -> WAVE2INT3_R {
        WAVE2INT3_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "IRQ03 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq03mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ03MON_SPEC;
impl crate::RegisterSpec for IRQ03MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq03mon::R`](R) reader structure"]
impl crate::Readable for IRQ03MON_SPEC {}
#[doc = "`reset()` method sets IRQ03MON to value 0"]
impl crate::Resettable for IRQ03MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
