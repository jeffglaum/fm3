#[doc = "Register `EPFR02` reader"]
pub type R = crate::R<EPFR02_SPEC>;
#[doc = "Register `EPFR02` writer"]
pub type W = crate::W<EPFR02_SPEC>;
#[doc = "Field `RTO10E` reader - RTO10E output select bit"]
pub type RTO10E_R = crate::FieldReader;
#[doc = "Field `RTO10E` writer - RTO10E output select bit"]
pub type RTO10E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTO11E` reader - RTO11E output select bit"]
pub type RTO11E_R = crate::FieldReader;
#[doc = "Field `RTO11E` writer - RTO11E output select bit"]
pub type RTO11E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTO12E` reader - RTO12E output select bit"]
pub type RTO12E_R = crate::FieldReader;
#[doc = "Field `RTO12E` writer - RTO12E output select bit"]
pub type RTO12E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTO13E` reader - RTO13E output select bit"]
pub type RTO13E_R = crate::FieldReader;
#[doc = "Field `RTO13E` writer - RTO13E output select bit"]
pub type RTO13E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTO14E` reader - RTO14E output select bit"]
pub type RTO14E_R = crate::FieldReader;
#[doc = "Field `RTO14E` writer - RTO14E output select bit"]
pub type RTO14E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTO15E` reader - RTO15E output select bit"]
pub type RTO15E_R = crate::FieldReader;
#[doc = "Field `RTO15E` writer - RTO15E output select bit"]
pub type RTO15E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DTTI1C` reader - DTTIX1 function select bit"]
pub type DTTI1C_R = crate::BitReader;
#[doc = "Field `DTTI1C` writer - DTTIX1 function select bit"]
pub type DTTI1C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTTI1S` reader - DTTIX1 input select bit"]
pub type DTTI1S_R = crate::FieldReader;
#[doc = "Field `DTTI1S` writer - DTTIX1 input select bit"]
pub type DTTI1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FRCK1S` reader - FRCK1 input select bit"]
pub type FRCK1S_R = crate::FieldReader;
#[doc = "Field `FRCK1S` writer - FRCK1 input select bit"]
pub type FRCK1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC10S` reader - IC10 input select bit"]
pub type IC10S_R = crate::FieldReader;
#[doc = "Field `IC10S` writer - IC10 input select bit"]
pub type IC10S_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IC11S` reader - IC11 input select bit"]
pub type IC11S_R = crate::FieldReader;
#[doc = "Field `IC11S` writer - IC11 input select bit"]
pub type IC11S_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IC12S` reader - IC12 input select bit"]
pub type IC12S_R = crate::FieldReader;
#[doc = "Field `IC12S` writer - IC12 input select bit"]
pub type IC12S_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IC13S` reader - IC13 input select bit"]
pub type IC13S_R = crate::FieldReader;
#[doc = "Field `IC13S` writer - IC13 input select bit"]
pub type IC13S_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - RTO10E output select bit"]
    #[inline(always)]
    pub fn rto10e(&self) -> RTO10E_R {
        RTO10E_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - RTO11E output select bit"]
    #[inline(always)]
    pub fn rto11e(&self) -> RTO11E_R {
        RTO11E_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - RTO12E output select bit"]
    #[inline(always)]
    pub fn rto12e(&self) -> RTO12E_R {
        RTO12E_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - RTO13E output select bit"]
    #[inline(always)]
    pub fn rto13e(&self) -> RTO13E_R {
        RTO13E_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - RTO14E output select bit"]
    #[inline(always)]
    pub fn rto14e(&self) -> RTO14E_R {
        RTO14E_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - RTO15E output select bit"]
    #[inline(always)]
    pub fn rto15e(&self) -> RTO15E_R {
        RTO15E_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - DTTIX1 function select bit"]
    #[inline(always)]
    pub fn dtti1c(&self) -> DTTI1C_R {
        DTTI1C_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - DTTIX1 input select bit"]
    #[inline(always)]
    pub fn dtti1s(&self) -> DTTI1S_R {
        DTTI1S_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - FRCK1 input select bit"]
    #[inline(always)]
    pub fn frck1s(&self) -> FRCK1S_R {
        FRCK1S_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - IC10 input select bit"]
    #[inline(always)]
    pub fn ic10s(&self) -> IC10S_R {
        IC10S_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - IC11 input select bit"]
    #[inline(always)]
    pub fn ic11s(&self) -> IC11S_R {
        IC11S_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:28 - IC12 input select bit"]
    #[inline(always)]
    pub fn ic12s(&self) -> IC12S_R {
        IC12S_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - IC13 input select bit"]
    #[inline(always)]
    pub fn ic13s(&self) -> IC13S_R {
        IC13S_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RTO10E output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn rto10e(&mut self) -> RTO10E_W<EPFR02_SPEC> {
        RTO10E_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - RTO11E output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn rto11e(&mut self) -> RTO11E_W<EPFR02_SPEC> {
        RTO11E_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - RTO12E output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn rto12e(&mut self) -> RTO12E_W<EPFR02_SPEC> {
        RTO12E_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - RTO13E output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn rto13e(&mut self) -> RTO13E_W<EPFR02_SPEC> {
        RTO13E_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - RTO14E output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn rto14e(&mut self) -> RTO14E_W<EPFR02_SPEC> {
        RTO14E_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - RTO15E output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn rto15e(&mut self) -> RTO15E_W<EPFR02_SPEC> {
        RTO15E_W::new(self, 10)
    }
    #[doc = "Bit 12 - DTTIX1 function select bit"]
    #[inline(always)]
    #[must_use]
    pub fn dtti1c(&mut self) -> DTTI1C_W<EPFR02_SPEC> {
        DTTI1C_W::new(self, 12)
    }
    #[doc = "Bits 16:17 - DTTIX1 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn dtti1s(&mut self) -> DTTI1S_W<EPFR02_SPEC> {
        DTTI1S_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - FRCK1 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn frck1s(&mut self) -> FRCK1S_W<EPFR02_SPEC> {
        FRCK1S_W::new(self, 18)
    }
    #[doc = "Bits 20:22 - IC10 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn ic10s(&mut self) -> IC10S_W<EPFR02_SPEC> {
        IC10S_W::new(self, 20)
    }
    #[doc = "Bits 23:25 - IC11 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn ic11s(&mut self) -> IC11S_W<EPFR02_SPEC> {
        IC11S_W::new(self, 23)
    }
    #[doc = "Bits 26:28 - IC12 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn ic12s(&mut self) -> IC12S_W<EPFR02_SPEC> {
        IC12S_W::new(self, 26)
    }
    #[doc = "Bits 29:31 - IC13 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn ic13s(&mut self) -> IC13S_W<EPFR02_SPEC> {
        IC13S_W::new(self, 29)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Extended pin function setting register 02\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr02::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr02::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPFR02_SPEC;
impl crate::RegisterSpec for EPFR02_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epfr02::R`](R) reader structure"]
impl crate::Readable for EPFR02_SPEC {}
#[doc = "`write(|w| ..)` method takes [`epfr02::W`](W) writer structure"]
impl crate::Writable for EPFR02_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EPFR02 to value 0"]
impl crate::Resettable for EPFR02_SPEC {
    const RESET_VALUE: u32 = 0;
}
