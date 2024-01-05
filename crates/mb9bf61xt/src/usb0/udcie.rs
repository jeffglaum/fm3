#[doc = "Register `UDCIE` reader"]
pub type R = crate::R<UDCIE_SPEC>;
#[doc = "Register `UDCIE` writer"]
pub type W = crate::W<UDCIE_SPEC>;
#[doc = "Field `CONFIE` reader - Configuration Interrupt Enable bit"]
pub type CONFIE_R = crate::BitReader;
#[doc = "Field `CONFIE` writer - Configuration Interrupt Enable bit"]
pub type CONFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONFN` reader - Configuration Number Indication bit"]
pub type CONFN_R = crate::BitReader;
#[doc = "Field `WKUPIE` reader - Wake-up Interrupt Enable bit"]
pub type WKUPIE_R = crate::BitReader;
#[doc = "Field `WKUPIE` writer - Wake-up Interrupt Enable bit"]
pub type WKUPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRSTIE` reader - Bus Reset Enable bit"]
pub type BRSTIE_R = crate::BitReader;
#[doc = "Field `BRSTIE` writer - Bus Reset Enable bit"]
pub type BRSTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFIE` reader - SOF Reception Interrupt Enable bit"]
pub type SOFIE_R = crate::BitReader;
#[doc = "Field `SOFIE` writer - SOF Reception Interrupt Enable bit"]
pub type SOFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPIE` reader - Suspend Interrupt Enable bit"]
pub type SUSPIE_R = crate::BitReader;
#[doc = "Field `SUSPIE` writer - Suspend Interrupt Enable bit"]
pub type SUSPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configuration Interrupt Enable bit"]
    #[inline(always)]
    pub fn confie(&self) -> CONFIE_R {
        CONFIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configuration Number Indication bit"]
    #[inline(always)]
    pub fn confn(&self) -> CONFN_R {
        CONFN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake-up Interrupt Enable bit"]
    #[inline(always)]
    pub fn wkupie(&self) -> WKUPIE_R {
        WKUPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bus Reset Enable bit"]
    #[inline(always)]
    pub fn brstie(&self) -> BRSTIE_R {
        BRSTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SOF Reception Interrupt Enable bit"]
    #[inline(always)]
    pub fn sofie(&self) -> SOFIE_R {
        SOFIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Suspend Interrupt Enable bit"]
    #[inline(always)]
    pub fn suspie(&self) -> SUSPIE_R {
        SUSPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configuration Interrupt Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn confie(&mut self) -> CONFIE_W<UDCIE_SPEC> {
        CONFIE_W::new(self, 0)
    }
    #[doc = "Bit 2 - Wake-up Interrupt Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn wkupie(&mut self) -> WKUPIE_W<UDCIE_SPEC> {
        WKUPIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bus Reset Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn brstie(&mut self) -> BRSTIE_W<UDCIE_SPEC> {
        BRSTIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - SOF Reception Interrupt Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn sofie(&mut self) -> SOFIE_W<UDCIE_SPEC> {
        SOFIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Suspend Interrupt Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn suspie(&mut self) -> SUSPIE_W<UDCIE_SPEC> {
        SUSPIE_W::new(self, 5)
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
#[doc = "UDC Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udcie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udcie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UDCIE_SPEC;
impl crate::RegisterSpec for UDCIE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`udcie::R`](R) reader structure"]
impl crate::Readable for UDCIE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`udcie::W`](W) writer structure"]
impl crate::Writable for UDCIE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UDCIE to value 0"]
impl crate::Resettable for UDCIE_SPEC {
    const RESET_VALUE: u8 = 0;
}
