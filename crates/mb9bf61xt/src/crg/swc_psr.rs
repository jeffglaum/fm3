#[doc = "Register `SWC_PSR` reader"]
pub type R = crate::R<SWC_PSR_SPEC>;
#[doc = "Register `SWC_PSR` writer"]
pub type W = crate::W<SWC_PSR_SPEC>;
#[doc = "Field `SWDS` reader - Software watchdog clock frequency division ratio setting bit"]
pub type SWDS_R = crate::FieldReader;
#[doc = "Field `SWDS` writer - Software watchdog clock frequency division ratio setting bit"]
pub type SWDS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TESTB` reader - TEST bit"]
pub type TESTB_R = crate::BitReader;
#[doc = "Field `TESTB` writer - TEST bit"]
pub type TESTB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Software watchdog clock frequency division ratio setting bit"]
    #[inline(always)]
    pub fn swds(&self) -> SWDS_R {
        SWDS_R::new(self.bits & 3)
    }
    #[doc = "Bit 7 - TEST bit"]
    #[inline(always)]
    pub fn testb(&self) -> TESTB_R {
        TESTB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Software watchdog clock frequency division ratio setting bit"]
    #[inline(always)]
    #[must_use]
    pub fn swds(&mut self) -> SWDS_W<SWC_PSR_SPEC> {
        SWDS_W::new(self, 0)
    }
    #[doc = "Bit 7 - TEST bit"]
    #[inline(always)]
    #[must_use]
    pub fn testb(&mut self) -> TESTB_W<SWC_PSR_SPEC> {
        TESTB_W::new(self, 7)
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
#[doc = "Software Watchdog Clock Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swc_psr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swc_psr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWC_PSR_SPEC;
impl crate::RegisterSpec for SWC_PSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`swc_psr::R`](R) reader structure"]
impl crate::Readable for SWC_PSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swc_psr::W`](W) writer structure"]
impl crate::Writable for SWC_PSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SWC_PSR to value 0"]
impl crate::Resettable for SWC_PSR_SPEC {
    const RESET_VALUE: u8 = 0;
}
