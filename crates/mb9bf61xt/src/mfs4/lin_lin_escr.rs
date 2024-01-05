#[doc = "Register `LIN_ESCR` reader"]
pub type R = crate::R<LIN_LIN_ESCR_SPEC>;
#[doc = "Register `LIN_ESCR` writer"]
pub type W = crate::W<LIN_LIN_ESCR_SPEC>;
#[doc = "Field `DEL` reader - LIN Break delimiter length select bits (valid in master mode only)"]
pub type DEL_R = crate::FieldReader;
#[doc = "Field `DEL` writer - LIN Break delimiter length select bits (valid in master mode only)"]
pub type DEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LBL` reader - LIN Break field length select bits (valid in master mode only)"]
pub type LBL_R = crate::FieldReader;
#[doc = "Field `LBL` writer - LIN Break field length select bits (valid in master mode only)"]
pub type LBL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LBIE` reader - LIN Break field detect interrupt enable bit"]
pub type LBIE_R = crate::BitReader;
#[doc = "Field `LBIE` writer - LIN Break field detect interrupt enable bit"]
pub type LBIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESBL` reader - Extended stop bit length select bit"]
pub type ESBL_R = crate::BitReader;
#[doc = "Field `ESBL` writer - Extended stop bit length select bit"]
pub type ESBL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - LIN Break delimiter length select bits (valid in master mode only)"]
    #[inline(always)]
    pub fn del(&self) -> DEL_R {
        DEL_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - LIN Break field length select bits (valid in master mode only)"]
    #[inline(always)]
    pub fn lbl(&self) -> LBL_R {
        LBL_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - LIN Break field detect interrupt enable bit"]
    #[inline(always)]
    pub fn lbie(&self) -> LBIE_R {
        LBIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Extended stop bit length select bit"]
    #[inline(always)]
    pub fn esbl(&self) -> ESBL_R {
        ESBL_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - LIN Break delimiter length select bits (valid in master mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn del(&mut self) -> DEL_W<LIN_LIN_ESCR_SPEC> {
        DEL_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - LIN Break field length select bits (valid in master mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn lbl(&mut self) -> LBL_W<LIN_LIN_ESCR_SPEC> {
        LBL_W::new(self, 2)
    }
    #[doc = "Bit 4 - LIN Break field detect interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn lbie(&mut self) -> LBIE_W<LIN_LIN_ESCR_SPEC> {
        LBIE_W::new(self, 4)
    }
    #[doc = "Bit 6 - Extended stop bit length select bit"]
    #[inline(always)]
    #[must_use]
    pub fn esbl(&mut self) -> ESBL_W<LIN_LIN_ESCR_SPEC> {
        ESBL_W::new(self, 6)
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
#[doc = "Extended Communication Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lin_lin_escr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lin_lin_escr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LIN_LIN_ESCR_SPEC;
impl crate::RegisterSpec for LIN_LIN_ESCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lin_lin_escr::R`](R) reader structure"]
impl crate::Readable for LIN_LIN_ESCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lin_lin_escr::W`](W) writer structure"]
impl crate::Writable for LIN_LIN_ESCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LIN_ESCR to value 0"]
impl crate::Resettable for LIN_LIN_ESCR_SPEC {
    const RESET_VALUE: u8 = 0;
}
