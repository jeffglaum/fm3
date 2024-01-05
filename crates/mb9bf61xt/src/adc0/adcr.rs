#[doc = "Register `ADCR` reader"]
pub type R = crate::R<ADCR_SPEC>;
#[doc = "Register `ADCR` writer"]
pub type W = crate::W<ADCR_SPEC>;
#[doc = "Field `OVRIE` reader - FIFO overrun interrupt enable bit"]
pub type OVRIE_R = crate::BitReader;
#[doc = "Field `OVRIE` writer - FIFO overrun interrupt enable bit"]
pub type OVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPIE` reader - Conversion result comparison interrupt enable bit"]
pub type CMPIE_R = crate::BitReader;
#[doc = "Field `CMPIE` writer - Conversion result comparison interrupt enable bit"]
pub type CMPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCIE` reader - Priority conversion interrupt enable bit"]
pub type PCIE_R = crate::BitReader;
#[doc = "Field `PCIE` writer - Priority conversion interrupt enable bit"]
pub type PCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCIE` reader - Scan conversion interrupt enable bit"]
pub type SCIE_R = crate::BitReader;
#[doc = "Field `SCIE` writer - Scan conversion interrupt enable bit"]
pub type SCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPIF` reader - Conversion result comparison interrupt request bit"]
pub type CMPIF_R = crate::BitReader;
#[doc = "Field `CMPIF` writer - Conversion result comparison interrupt request bit"]
pub type CMPIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCIF` reader - Priority conversion interrupt request bit"]
pub type PCIF_R = crate::BitReader;
#[doc = "Field `PCIF` writer - Priority conversion interrupt request bit"]
pub type PCIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCIF` reader - Scan conversion interrupt request bit"]
pub type SCIF_R = crate::BitReader;
#[doc = "Field `SCIF` writer - Scan conversion interrupt request bit"]
pub type SCIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FIFO overrun interrupt enable bit"]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Conversion result comparison interrupt enable bit"]
    #[inline(always)]
    pub fn cmpie(&self) -> CMPIE_R {
        CMPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Priority conversion interrupt enable bit"]
    #[inline(always)]
    pub fn pcie(&self) -> PCIE_R {
        PCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Scan conversion interrupt enable bit"]
    #[inline(always)]
    pub fn scie(&self) -> SCIE_R {
        SCIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Conversion result comparison interrupt request bit"]
    #[inline(always)]
    pub fn cmpif(&self) -> CMPIF_R {
        CMPIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Priority conversion interrupt request bit"]
    #[inline(always)]
    pub fn pcif(&self) -> PCIF_R {
        PCIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Scan conversion interrupt request bit"]
    #[inline(always)]
    pub fn scif(&self) -> SCIF_R {
        SCIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO overrun interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OVRIE_W<ADCR_SPEC> {
        OVRIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Conversion result comparison interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpie(&mut self) -> CMPIE_W<ADCR_SPEC> {
        CMPIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Priority conversion interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pcie(&mut self) -> PCIE_W<ADCR_SPEC> {
        PCIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Scan conversion interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn scie(&mut self) -> SCIE_W<ADCR_SPEC> {
        SCIE_W::new(self, 3)
    }
    #[doc = "Bit 5 - Conversion result comparison interrupt request bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpif(&mut self) -> CMPIF_W<ADCR_SPEC> {
        CMPIF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Priority conversion interrupt request bit"]
    #[inline(always)]
    #[must_use]
    pub fn pcif(&mut self) -> PCIF_W<ADCR_SPEC> {
        PCIF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Scan conversion interrupt request bit"]
    #[inline(always)]
    #[must_use]
    pub fn scif(&mut self) -> SCIF_W<ADCR_SPEC> {
        SCIF_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "A/D Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCR_SPEC;
impl crate::RegisterSpec for ADCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adcr::R`](R) reader structure"]
impl crate::Readable for ADCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adcr::W`](W) writer structure"]
impl crate::Writable for ADCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADCR to value 0"]
impl crate::Resettable for ADCR_SPEC {
    const RESET_VALUE: u8 = 0;
}
