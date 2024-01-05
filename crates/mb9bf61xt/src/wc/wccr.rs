#[doc = "Register `WCCR` reader"]
pub type R = crate::R<WCCR_SPEC>;
#[doc = "Register `WCCR` writer"]
pub type W = crate::W<WCCR_SPEC>;
#[doc = "Field `WCIF` reader - Interrupt request flag bit"]
pub type WCIF_R = crate::BitReader;
#[doc = "Field `WCIF` writer - Interrupt request flag bit"]
pub type WCIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCIE` reader - Interrupt request enable bit"]
pub type WCIE_R = crate::BitReader;
#[doc = "Field `WCIE` writer - Interrupt request enable bit"]
pub type WCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS` reader - Count clock select bits"]
pub type CS_R = crate::FieldReader;
#[doc = "Field `CS` writer - Count clock select bits"]
pub type CS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WCOP` reader - Watch counter operating state flag"]
pub type WCOP_R = crate::BitReader;
#[doc = "Field `WCEN` reader - Watch counter operation enable bit"]
pub type WCEN_R = crate::BitReader;
#[doc = "Field `WCEN` writer - Watch counter operation enable bit"]
pub type WCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt request flag bit"]
    #[inline(always)]
    pub fn wcif(&self) -> WCIF_R {
        WCIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt request enable bit"]
    #[inline(always)]
    pub fn wcie(&self) -> WCIE_R {
        WCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Count clock select bits"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 6 - Watch counter operating state flag"]
    #[inline(always)]
    pub fn wcop(&self) -> WCOP_R {
        WCOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Watch counter operation enable bit"]
    #[inline(always)]
    pub fn wcen(&self) -> WCEN_R {
        WCEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt request flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn wcif(&mut self) -> WCIF_W<WCCR_SPEC> {
        WCIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt request enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn wcie(&mut self) -> WCIE_W<WCCR_SPEC> {
        WCIE_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Count clock select bits"]
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CS_W<WCCR_SPEC> {
        CS_W::new(self, 2)
    }
    #[doc = "Bit 7 - Watch counter operation enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn wcen(&mut self) -> WCEN_W<WCCR_SPEC> {
        WCEN_W::new(self, 7)
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
#[doc = "Watch Counter Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WCCR_SPEC;
impl crate::RegisterSpec for WCCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wccr::R`](R) reader structure"]
impl crate::Readable for WCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wccr::W`](W) writer structure"]
impl crate::Writable for WCCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WCCR to value 0"]
impl crate::Resettable for WCCR_SPEC {
    const RESET_VALUE: u8 = 0;
}
