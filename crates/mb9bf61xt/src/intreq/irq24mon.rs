#[doc = "Register `IRQ24MON` reader"]
pub type R = crate::R<IRQ24MON_SPEC>;
#[doc = "Field `MOSCINT` reader - Stabilization wait completion interrupt request for main clock oscillation"]
pub type MOSCINT_R = crate::BitReader;
#[doc = "Field `SOSCINT` reader - Stabilization wait completion interrupt request for sub-clock oscillation"]
pub type SOSCINT_R = crate::BitReader;
#[doc = "Field `MPLLINT` reader - Stabilization wait completion interrupt request for main PLL oscillation"]
pub type MPLLINT_R = crate::BitReader;
#[doc = "Field `UPLLINT` reader - Stabilization wait completion interrupt request for USB or USB/Ethernet PLL oscillation."]
pub type UPLLINT_R = crate::BitReader;
#[doc = "Field `WCINT` reader - Watch counter interrupt request"]
pub type WCINT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Stabilization wait completion interrupt request for main clock oscillation"]
    #[inline(always)]
    pub fn moscint(&self) -> MOSCINT_R {
        MOSCINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stabilization wait completion interrupt request for sub-clock oscillation"]
    #[inline(always)]
    pub fn soscint(&self) -> SOSCINT_R {
        SOSCINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stabilization wait completion interrupt request for main PLL oscillation"]
    #[inline(always)]
    pub fn mpllint(&self) -> MPLLINT_R {
        MPLLINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stabilization wait completion interrupt request for USB or USB/Ethernet PLL oscillation."]
    #[inline(always)]
    pub fn upllint(&self) -> UPLLINT_R {
        UPLLINT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Watch counter interrupt request"]
    #[inline(always)]
    pub fn wcint(&self) -> WCINT_R {
        WCINT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "IRQ24 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq24mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ24MON_SPEC;
impl crate::RegisterSpec for IRQ24MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq24mon::R`](R) reader structure"]
impl crate::Readable for IRQ24MON_SPEC {}
#[doc = "`reset()` method sets IRQ24MON to value 0"]
impl crate::Resettable for IRQ24MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
