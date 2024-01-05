#[doc = "Register `IRQ27MON` reader"]
pub type R = crate::R<IRQ27MON_SPEC>;
#[doc = "Field `ADCINT0` reader - Priority conversion interrupt request in the corresponding A/D unit 2"]
pub type ADCINT0_R = crate::BitReader;
#[doc = "Field `ADCINT1` reader - Scan conversion interrupt request in the corresponding A/D unit 2"]
pub type ADCINT1_R = crate::BitReader;
#[doc = "Field `ADCINT2` reader - FIFO overrun interrupt request in the corresponding A/D unit 2"]
pub type ADCINT2_R = crate::BitReader;
#[doc = "Field `ADCINT3` reader - Conversion result comparison interrupt request in the corresponding A/D unit 2"]
pub type ADCINT3_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Priority conversion interrupt request in the corresponding A/D unit 2"]
    #[inline(always)]
    pub fn adcint0(&self) -> ADCINT0_R {
        ADCINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan conversion interrupt request in the corresponding A/D unit 2"]
    #[inline(always)]
    pub fn adcint1(&self) -> ADCINT1_R {
        ADCINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO overrun interrupt request in the corresponding A/D unit 2"]
    #[inline(always)]
    pub fn adcint2(&self) -> ADCINT2_R {
        ADCINT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Conversion result comparison interrupt request in the corresponding A/D unit 2"]
    #[inline(always)]
    pub fn adcint3(&self) -> ADCINT3_R {
        ADCINT3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "IRQ27 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq27mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ27MON_SPEC;
impl crate::RegisterSpec for IRQ27MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq27mon::R`](R) reader structure"]
impl crate::Readable for IRQ27MON_SPEC {}
#[doc = "`reset()` method sets IRQ27MON to value 0"]
impl crate::Resettable for IRQ27MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
