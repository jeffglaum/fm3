#[doc = "Register `IRQ33MON` reader"]
pub type R = crate::R<IRQ33MON_SPEC>;
#[doc = "Field `MAC1SBD` reader - SBD interrupt request of Ethernet MAC ch.1"]
pub type MAC1SBD_R = crate::BitReader;
#[doc = "Field `MAC1PMI` reader - PMI interrupt request of Ethernet MAC ch.1"]
pub type MAC1PMI_R = crate::BitReader;
#[doc = "Field `MAC1LPI` reader - LPI interrupt request of Ethernet MAC ch.1"]
pub type MAC1LPI_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - SBD interrupt request of Ethernet MAC ch.1"]
    #[inline(always)]
    pub fn mac1sbd(&self) -> MAC1SBD_R {
        MAC1SBD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PMI interrupt request of Ethernet MAC ch.1"]
    #[inline(always)]
    pub fn mac1pmi(&self) -> MAC1PMI_R {
        MAC1PMI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LPI interrupt request of Ethernet MAC ch.1"]
    #[inline(always)]
    pub fn mac1lpi(&self) -> MAC1LPI_R {
        MAC1LPI_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "IRQ33 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq33mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ33MON_SPEC;
impl crate::RegisterSpec for IRQ33MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq33mon::R`](R) reader structure"]
impl crate::Readable for IRQ33MON_SPEC {}
#[doc = "`reset()` method sets IRQ33MON to value 0"]
impl crate::Resettable for IRQ33MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
