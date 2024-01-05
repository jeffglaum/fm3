#[doc = "Register `CSIO_ESCR` reader"]
pub type R = crate::R<CSIO_CSIO_ESCR_SPEC>;
#[doc = "Register `CSIO_ESCR` writer"]
pub type W = crate::W<CSIO_CSIO_ESCR_SPEC>;
#[doc = "Field `L` reader - Data length select bits"]
pub type L_R = crate::FieldReader;
#[doc = "Field `L` writer - Data length select bits"]
pub type L_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WT` reader - Data transmit/received wait select bits"]
pub type WT_R = crate::FieldReader;
#[doc = "Field `WT` writer - Data transmit/received wait select bits"]
pub type WT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SOP` reader - Serial output pin set bit"]
pub type SOP_R = crate::BitReader;
#[doc = "Field `SOP` writer - Serial output pin set bit"]
pub type SOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Data length select bits"]
    #[inline(always)]
    pub fn l(&self) -> L_R {
        L_R::new(self.bits & 7)
    }
    #[doc = "Bits 3:4 - Data transmit/received wait select bits"]
    #[inline(always)]
    pub fn wt(&self) -> WT_R {
        WT_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 7 - Serial output pin set bit"]
    #[inline(always)]
    pub fn sop(&self) -> SOP_R {
        SOP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Data length select bits"]
    #[inline(always)]
    #[must_use]
    pub fn l(&mut self) -> L_W<CSIO_CSIO_ESCR_SPEC> {
        L_W::new(self, 0)
    }
    #[doc = "Bits 3:4 - Data transmit/received wait select bits"]
    #[inline(always)]
    #[must_use]
    pub fn wt(&mut self) -> WT_W<CSIO_CSIO_ESCR_SPEC> {
        WT_W::new(self, 3)
    }
    #[doc = "Bit 7 - Serial output pin set bit"]
    #[inline(always)]
    #[must_use]
    pub fn sop(&mut self) -> SOP_W<CSIO_CSIO_ESCR_SPEC> {
        SOP_W::new(self, 7)
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
#[doc = "Extended Communication Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csio_csio_escr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csio_csio_escr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIO_CSIO_ESCR_SPEC;
impl crate::RegisterSpec for CSIO_CSIO_ESCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csio_csio_escr::R`](R) reader structure"]
impl crate::Readable for CSIO_CSIO_ESCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csio_csio_escr::W`](W) writer structure"]
impl crate::Writable for CSIO_CSIO_ESCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CSIO_ESCR to value 0"]
impl crate::Resettable for CSIO_CSIO_ESCR_SPEC {
    const RESET_VALUE: u8 = 0;
}
