#[doc = "Register `EPFR03` reader"]
pub type R = crate::R<EPFR03_SPEC>;
#[doc = "Register `EPFR03` writer"]
pub type W = crate::W<EPFR03_SPEC>;
#[doc = "Field `RTO20E` reader - RTO20E output select bit"]
pub type RTO20E_R = crate::FieldReader;
#[doc = "Field `RTO20E` writer - RTO20E output select bit"]
pub type RTO20E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTO21E` reader - RTO21E output select bit"]
pub type RTO21E_R = crate::FieldReader;
#[doc = "Field `RTO21E` writer - RTO21E output select bit"]
pub type RTO21E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTO22E` reader - RTO22E output select bit"]
pub type RTO22E_R = crate::FieldReader;
#[doc = "Field `RTO22E` writer - RTO22E output select bit"]
pub type RTO22E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTO23E` reader - RTO23E output select bit"]
pub type RTO23E_R = crate::FieldReader;
#[doc = "Field `RTO23E` writer - RTO23E output select bit"]
pub type RTO23E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTO24E` reader - RTO24E output select bit"]
pub type RTO24E_R = crate::FieldReader;
#[doc = "Field `RTO24E` writer - RTO24E output select bit"]
pub type RTO24E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTO25E` reader - RTO25E output select bit"]
pub type RTO25E_R = crate::FieldReader;
#[doc = "Field `RTO25E` writer - RTO25E output select bit"]
pub type RTO25E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DTTI2C` reader - DTTIX2 function select bit"]
pub type DTTI2C_R = crate::BitReader;
#[doc = "Field `DTTI2C` writer - DTTIX2 function select bit"]
pub type DTTI2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTTI2S` reader - DTTIX2 input select bit"]
pub type DTTI2S_R = crate::FieldReader;
#[doc = "Field `DTTI2S` writer - DTTIX2 input select bit"]
pub type DTTI2S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FRCK2S` reader - FRCK2 input select bit"]
pub type FRCK2S_R = crate::FieldReader;
#[doc = "Field `FRCK2S` writer - FRCK2 input select bit"]
pub type FRCK2S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC20S` reader - IC20 input select bit"]
pub type IC20S_R = crate::FieldReader;
#[doc = "Field `IC20S` writer - IC20 input select bit"]
pub type IC20S_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IC21S` reader - IC21 input select bit"]
pub type IC21S_R = crate::FieldReader;
#[doc = "Field `IC21S` writer - IC21 input select bit"]
pub type IC21S_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IC22S` reader - IC22 input select bit"]
pub type IC22S_R = crate::FieldReader;
#[doc = "Field `IC22S` writer - IC22 input select bit"]
pub type IC22S_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IC23S` reader - IC23 input select bit"]
pub type IC23S_R = crate::FieldReader;
#[doc = "Field `IC23S` writer - IC23 input select bit"]
pub type IC23S_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - RTO20E output select bit"]
    #[inline(always)]
    pub fn rto20e(&self) -> RTO20E_R {
        RTO20E_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - RTO21E output select bit"]
    #[inline(always)]
    pub fn rto21e(&self) -> RTO21E_R {
        RTO21E_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - RTO22E output select bit"]
    #[inline(always)]
    pub fn rto22e(&self) -> RTO22E_R {
        RTO22E_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - RTO23E output select bit"]
    #[inline(always)]
    pub fn rto23e(&self) -> RTO23E_R {
        RTO23E_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - RTO24E output select bit"]
    #[inline(always)]
    pub fn rto24e(&self) -> RTO24E_R {
        RTO24E_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - RTO25E output select bit"]
    #[inline(always)]
    pub fn rto25e(&self) -> RTO25E_R {
        RTO25E_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - DTTIX2 function select bit"]
    #[inline(always)]
    pub fn dtti2c(&self) -> DTTI2C_R {
        DTTI2C_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - DTTIX2 input select bit"]
    #[inline(always)]
    pub fn dtti2s(&self) -> DTTI2S_R {
        DTTI2S_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - FRCK2 input select bit"]
    #[inline(always)]
    pub fn frck2s(&self) -> FRCK2S_R {
        FRCK2S_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - IC20 input select bit"]
    #[inline(always)]
    pub fn ic20s(&self) -> IC20S_R {
        IC20S_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - IC21 input select bit"]
    #[inline(always)]
    pub fn ic21s(&self) -> IC21S_R {
        IC21S_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:28 - IC22 input select bit"]
    #[inline(always)]
    pub fn ic22s(&self) -> IC22S_R {
        IC22S_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - IC23 input select bit"]
    #[inline(always)]
    pub fn ic23s(&self) -> IC23S_R {
        IC23S_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RTO20E output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn rto20e(&mut self) -> RTO20E_W<EPFR03_SPEC> {
        RTO20E_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - RTO21E output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn rto21e(&mut self) -> RTO21E_W<EPFR03_SPEC> {
        RTO21E_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - RTO22E output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn rto22e(&mut self) -> RTO22E_W<EPFR03_SPEC> {
        RTO22E_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - RTO23E output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn rto23e(&mut self) -> RTO23E_W<EPFR03_SPEC> {
        RTO23E_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - RTO24E output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn rto24e(&mut self) -> RTO24E_W<EPFR03_SPEC> {
        RTO24E_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - RTO25E output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn rto25e(&mut self) -> RTO25E_W<EPFR03_SPEC> {
        RTO25E_W::new(self, 10)
    }
    #[doc = "Bit 12 - DTTIX2 function select bit"]
    #[inline(always)]
    #[must_use]
    pub fn dtti2c(&mut self) -> DTTI2C_W<EPFR03_SPEC> {
        DTTI2C_W::new(self, 12)
    }
    #[doc = "Bits 16:17 - DTTIX2 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn dtti2s(&mut self) -> DTTI2S_W<EPFR03_SPEC> {
        DTTI2S_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - FRCK2 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn frck2s(&mut self) -> FRCK2S_W<EPFR03_SPEC> {
        FRCK2S_W::new(self, 18)
    }
    #[doc = "Bits 20:22 - IC20 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn ic20s(&mut self) -> IC20S_W<EPFR03_SPEC> {
        IC20S_W::new(self, 20)
    }
    #[doc = "Bits 23:25 - IC21 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn ic21s(&mut self) -> IC21S_W<EPFR03_SPEC> {
        IC21S_W::new(self, 23)
    }
    #[doc = "Bits 26:28 - IC22 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn ic22s(&mut self) -> IC22S_W<EPFR03_SPEC> {
        IC22S_W::new(self, 26)
    }
    #[doc = "Bits 29:31 - IC23 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn ic23s(&mut self) -> IC23S_W<EPFR03_SPEC> {
        IC23S_W::new(self, 29)
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
#[doc = "Extended pin function setting register 03\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr03::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr03::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPFR03_SPEC;
impl crate::RegisterSpec for EPFR03_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epfr03::R`](R) reader structure"]
impl crate::Readable for EPFR03_SPEC {}
#[doc = "`write(|w| ..)` method takes [`epfr03::W`](W) writer structure"]
impl crate::Writable for EPFR03_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EPFR03 to value 0"]
impl crate::Resettable for EPFR03_SPEC {
    const RESET_VALUE: u32 = 0;
}
