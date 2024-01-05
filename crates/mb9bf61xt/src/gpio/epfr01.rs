#[doc = "Register `EPFR01` reader"]
pub type R = crate::R<EPFR01_SPEC>;
#[doc = "Register `EPFR01` writer"]
pub type W = crate::W<EPFR01_SPEC>;
#[doc = "Field `RTO00E` reader - RTO00E output select bit"]
pub type RTO00E_R = crate::FieldReader;
#[doc = "Field `RTO00E` writer - RTO00E output select bit"]
pub type RTO00E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTO01E` reader - RTO01E output select bit"]
pub type RTO01E_R = crate::FieldReader;
#[doc = "Field `RTO01E` writer - RTO01E output select bit"]
pub type RTO01E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTO02E` reader - RTO02E output select bit"]
pub type RTO02E_R = crate::FieldReader;
#[doc = "Field `RTO02E` writer - RTO02E output select bit"]
pub type RTO02E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTO03E` reader - RTO03E output select bit"]
pub type RTO03E_R = crate::FieldReader;
#[doc = "Field `RTO03E` writer - RTO03E output select bit"]
pub type RTO03E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTO04E` reader - RTO04E output select bit"]
pub type RTO04E_R = crate::FieldReader;
#[doc = "Field `RTO04E` writer - RTO04E output select bit"]
pub type RTO04E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTO05E` reader - RTO05E output select bit"]
pub type RTO05E_R = crate::FieldReader;
#[doc = "Field `RTO05E` writer - RTO05E output select bit"]
pub type RTO05E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DTTI0C` reader - DTTIX0 function select bit"]
pub type DTTI0C_R = crate::BitReader;
#[doc = "Field `DTTI0C` writer - DTTIX0 function select bit"]
pub type DTTI0C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTTI0S` reader - DTTIX0 input select bit"]
pub type DTTI0S_R = crate::FieldReader;
#[doc = "Field `DTTI0S` writer - DTTIX0 input select bit"]
pub type DTTI0S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FRCK0S` reader - FRCK0 input select bit"]
pub type FRCK0S_R = crate::FieldReader;
#[doc = "Field `FRCK0S` writer - FRCK0 input select bit"]
pub type FRCK0S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC00S` reader - IC00 input select bit"]
pub type IC00S_R = crate::FieldReader;
#[doc = "Field `IC00S` writer - IC00 input select bit"]
pub type IC00S_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IC01S` reader - IC01 input select bit"]
pub type IC01S_R = crate::FieldReader;
#[doc = "Field `IC01S` writer - IC01 input select bit"]
pub type IC01S_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IC02S` reader - IC02 input select bit"]
pub type IC02S_R = crate::FieldReader;
#[doc = "Field `IC02S` writer - IC02 input select bit"]
pub type IC02S_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IC03S` reader - IC03 input select bit"]
pub type IC03S_R = crate::FieldReader;
#[doc = "Field `IC03S` writer - IC03 input select bit"]
pub type IC03S_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - RTO00E output select bit"]
    #[inline(always)]
    pub fn rto00e(&self) -> RTO00E_R {
        RTO00E_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - RTO01E output select bit"]
    #[inline(always)]
    pub fn rto01e(&self) -> RTO01E_R {
        RTO01E_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - RTO02E output select bit"]
    #[inline(always)]
    pub fn rto02e(&self) -> RTO02E_R {
        RTO02E_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - RTO03E output select bit"]
    #[inline(always)]
    pub fn rto03e(&self) -> RTO03E_R {
        RTO03E_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - RTO04E output select bit"]
    #[inline(always)]
    pub fn rto04e(&self) -> RTO04E_R {
        RTO04E_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - RTO05E output select bit"]
    #[inline(always)]
    pub fn rto05e(&self) -> RTO05E_R {
        RTO05E_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - DTTIX0 function select bit"]
    #[inline(always)]
    pub fn dtti0c(&self) -> DTTI0C_R {
        DTTI0C_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - DTTIX0 input select bit"]
    #[inline(always)]
    pub fn dtti0s(&self) -> DTTI0S_R {
        DTTI0S_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - FRCK0 input select bit"]
    #[inline(always)]
    pub fn frck0s(&self) -> FRCK0S_R {
        FRCK0S_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - IC00 input select bit"]
    #[inline(always)]
    pub fn ic00s(&self) -> IC00S_R {
        IC00S_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - IC01 input select bit"]
    #[inline(always)]
    pub fn ic01s(&self) -> IC01S_R {
        IC01S_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:28 - IC02 input select bit"]
    #[inline(always)]
    pub fn ic02s(&self) -> IC02S_R {
        IC02S_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - IC03 input select bit"]
    #[inline(always)]
    pub fn ic03s(&self) -> IC03S_R {
        IC03S_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RTO00E output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn rto00e(&mut self) -> RTO00E_W<EPFR01_SPEC> {
        RTO00E_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - RTO01E output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn rto01e(&mut self) -> RTO01E_W<EPFR01_SPEC> {
        RTO01E_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - RTO02E output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn rto02e(&mut self) -> RTO02E_W<EPFR01_SPEC> {
        RTO02E_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - RTO03E output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn rto03e(&mut self) -> RTO03E_W<EPFR01_SPEC> {
        RTO03E_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - RTO04E output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn rto04e(&mut self) -> RTO04E_W<EPFR01_SPEC> {
        RTO04E_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - RTO05E output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn rto05e(&mut self) -> RTO05E_W<EPFR01_SPEC> {
        RTO05E_W::new(self, 10)
    }
    #[doc = "Bit 12 - DTTIX0 function select bit"]
    #[inline(always)]
    #[must_use]
    pub fn dtti0c(&mut self) -> DTTI0C_W<EPFR01_SPEC> {
        DTTI0C_W::new(self, 12)
    }
    #[doc = "Bits 16:17 - DTTIX0 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn dtti0s(&mut self) -> DTTI0S_W<EPFR01_SPEC> {
        DTTI0S_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - FRCK0 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn frck0s(&mut self) -> FRCK0S_W<EPFR01_SPEC> {
        FRCK0S_W::new(self, 18)
    }
    #[doc = "Bits 20:22 - IC00 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn ic00s(&mut self) -> IC00S_W<EPFR01_SPEC> {
        IC00S_W::new(self, 20)
    }
    #[doc = "Bits 23:25 - IC01 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn ic01s(&mut self) -> IC01S_W<EPFR01_SPEC> {
        IC01S_W::new(self, 23)
    }
    #[doc = "Bits 26:28 - IC02 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn ic02s(&mut self) -> IC02S_W<EPFR01_SPEC> {
        IC02S_W::new(self, 26)
    }
    #[doc = "Bits 29:31 - IC03 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn ic03s(&mut self) -> IC03S_W<EPFR01_SPEC> {
        IC03S_W::new(self, 29)
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
#[doc = "Extended pin function setting register 01\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPFR01_SPEC;
impl crate::RegisterSpec for EPFR01_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epfr01::R`](R) reader structure"]
impl crate::Readable for EPFR01_SPEC {}
#[doc = "`write(|w| ..)` method takes [`epfr01::W`](W) writer structure"]
impl crate::Writable for EPFR01_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EPFR01 to value 0"]
impl crate::Resettable for EPFR01_SPEC {
    const RESET_VALUE: u32 = 0;
}
