#[doc = "Register `IRQ32MON` reader"]
pub type R = crate::R<IRQ32MON_SPEC>;
#[doc = "Field `MAC0SBD` reader - SBD interrupt request of Ethernet MAC ch.0"]
pub type MAC0SBD_R = crate::BitReader;
#[doc = "Field `MAC0PMI` reader - PMI interrupt request of Ethernet MAC ch.0"]
pub type MAC0PMI_R = crate::BitReader;
#[doc = "Field `MAC0LPI` reader - LPI interrupt request of Ethernet MAC ch.0"]
pub type MAC0LPI_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - SBD interrupt request of Ethernet MAC ch.0"]
    #[inline(always)]
    pub fn mac0sbd(&self) -> MAC0SBD_R {
        MAC0SBD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PMI interrupt request of Ethernet MAC ch.0"]
    #[inline(always)]
    pub fn mac0pmi(&self) -> MAC0PMI_R {
        MAC0PMI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LPI interrupt request of Ethernet MAC ch.0"]
    #[inline(always)]
    pub fn mac0lpi(&self) -> MAC0LPI_R {
        MAC0LPI_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "IRQ32 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq32mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ32MON_SPEC;
impl crate::RegisterSpec for IRQ32MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq32mon::R`](R) reader structure"]
impl crate::Readable for IRQ32MON_SPEC {}
#[doc = "`reset()` method sets IRQ32MON to value 0"]
impl crate::Resettable for IRQ32MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
