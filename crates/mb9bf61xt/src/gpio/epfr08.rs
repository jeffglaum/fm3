#[doc = "Register `EPFR08` reader"]
pub type R = crate::R<EPFR08_SPEC>;
#[doc = "Register `EPFR08` writer"]
pub type W = crate::W<EPFR08_SPEC>;
#[doc = "Field `RTS4E` reader - RTS4E output select bit"]
pub type RTS4E_R = crate::FieldReader;
#[doc = "Field `RTS4E` writer - RTS4E output select bit"]
pub type RTS4E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTS4S` reader - CTS4S input select bit"]
pub type CTS4S_R = crate::FieldReader;
#[doc = "Field `CTS4S` writer - CTS4S input select bit"]
pub type CTS4S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SIN4S` reader - SIN4S input select bit"]
pub type SIN4S_R = crate::FieldReader;
#[doc = "Field `SIN4S` writer - SIN4S input select bit"]
pub type SIN4S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SOT4B` reader - SOT4B input/output select bit"]
pub type SOT4B_R = crate::FieldReader;
#[doc = "Field `SOT4B` writer - SOT4B input/output select bit"]
pub type SOT4B_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCK4B` reader - SCK4 input/output select bit"]
pub type SCK4B_R = crate::FieldReader;
#[doc = "Field `SCK4B` writer - SCK4 input/output select bit"]
pub type SCK4B_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SIN5S` reader - SIN5S input select bit"]
pub type SIN5S_R = crate::FieldReader;
#[doc = "Field `SIN5S` writer - SIN5S input select bit"]
pub type SIN5S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SOT5B` reader - SOT5B input/output select bit"]
pub type SOT5B_R = crate::FieldReader;
#[doc = "Field `SOT5B` writer - SOT5B input/output select bit"]
pub type SOT5B_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCK5B` reader - SCK5 input/output select bit"]
pub type SCK5B_R = crate::FieldReader;
#[doc = "Field `SCK5B` writer - SCK5 input/output select bit"]
pub type SCK5B_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SIN6S` reader - SIN6S input select bit"]
pub type SIN6S_R = crate::FieldReader;
#[doc = "Field `SIN6S` writer - SIN6S input select bit"]
pub type SIN6S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SOT6B` reader - SOT6B input/output select bit"]
pub type SOT6B_R = crate::FieldReader;
#[doc = "Field `SOT6B` writer - SOT6B input/output select bit"]
pub type SOT6B_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCK6B` reader - SCK6 input/output select bit"]
pub type SCK6B_R = crate::FieldReader;
#[doc = "Field `SCK6B` writer - SCK6 input/output select bit"]
pub type SCK6B_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SIN7S` reader - SIN7S input select bit"]
pub type SIN7S_R = crate::FieldReader;
#[doc = "Field `SIN7S` writer - SIN7S input select bit"]
pub type SIN7S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SOT7B` reader - SOT7B input/output select bit"]
pub type SOT7B_R = crate::FieldReader;
#[doc = "Field `SOT7B` writer - SOT7B input/output select bit"]
pub type SOT7B_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCK7B` reader - SCK7 input/output select bit"]
pub type SCK7B_R = crate::FieldReader;
#[doc = "Field `SCK7B` writer - SCK7 input/output select bit"]
pub type SCK7B_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - RTS4E output select bit"]
    #[inline(always)]
    pub fn rts4e(&self) -> RTS4E_R {
        RTS4E_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - CTS4S input select bit"]
    #[inline(always)]
    pub fn cts4s(&self) -> CTS4S_R {
        CTS4S_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - SIN4S input select bit"]
    #[inline(always)]
    pub fn sin4s(&self) -> SIN4S_R {
        SIN4S_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - SOT4B input/output select bit"]
    #[inline(always)]
    pub fn sot4b(&self) -> SOT4B_R {
        SOT4B_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SCK4 input/output select bit"]
    #[inline(always)]
    pub fn sck4b(&self) -> SCK4B_R {
        SCK4B_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - SIN5S input select bit"]
    #[inline(always)]
    pub fn sin5s(&self) -> SIN5S_R {
        SIN5S_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - SOT5B input/output select bit"]
    #[inline(always)]
    pub fn sot5b(&self) -> SOT5B_R {
        SOT5B_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - SCK5 input/output select bit"]
    #[inline(always)]
    pub fn sck5b(&self) -> SCK5B_R {
        SCK5B_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - SIN6S input select bit"]
    #[inline(always)]
    pub fn sin6s(&self) -> SIN6S_R {
        SIN6S_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - SOT6B input/output select bit"]
    #[inline(always)]
    pub fn sot6b(&self) -> SOT6B_R {
        SOT6B_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - SCK6 input/output select bit"]
    #[inline(always)]
    pub fn sck6b(&self) -> SCK6B_R {
        SCK6B_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - SIN7S input select bit"]
    #[inline(always)]
    pub fn sin7s(&self) -> SIN7S_R {
        SIN7S_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - SOT7B input/output select bit"]
    #[inline(always)]
    pub fn sot7b(&self) -> SOT7B_R {
        SOT7B_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - SCK7 input/output select bit"]
    #[inline(always)]
    pub fn sck7b(&self) -> SCK7B_R {
        SCK7B_R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RTS4E output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn rts4e(&mut self) -> RTS4E_W<EPFR08_SPEC> {
        RTS4E_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - CTS4S input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn cts4s(&mut self) -> CTS4S_W<EPFR08_SPEC> {
        CTS4S_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - SIN4S input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn sin4s(&mut self) -> SIN4S_W<EPFR08_SPEC> {
        SIN4S_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - SOT4B input/output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn sot4b(&mut self) -> SOT4B_W<EPFR08_SPEC> {
        SOT4B_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - SCK4 input/output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn sck4b(&mut self) -> SCK4B_W<EPFR08_SPEC> {
        SCK4B_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - SIN5S input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn sin5s(&mut self) -> SIN5S_W<EPFR08_SPEC> {
        SIN5S_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - SOT5B input/output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn sot5b(&mut self) -> SOT5B_W<EPFR08_SPEC> {
        SOT5B_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - SCK5 input/output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn sck5b(&mut self) -> SCK5B_W<EPFR08_SPEC> {
        SCK5B_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - SIN6S input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn sin6s(&mut self) -> SIN6S_W<EPFR08_SPEC> {
        SIN6S_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - SOT6B input/output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn sot6b(&mut self) -> SOT6B_W<EPFR08_SPEC> {
        SOT6B_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - SCK6 input/output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn sck6b(&mut self) -> SCK6B_W<EPFR08_SPEC> {
        SCK6B_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - SIN7S input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn sin7s(&mut self) -> SIN7S_W<EPFR08_SPEC> {
        SIN7S_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - SOT7B input/output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn sot7b(&mut self) -> SOT7B_W<EPFR08_SPEC> {
        SOT7B_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - SCK7 input/output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn sck7b(&mut self) -> SCK7B_W<EPFR08_SPEC> {
        SCK7B_W::new(self, 26)
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
#[doc = "Extended pin function setting register 08\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr08::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr08::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPFR08_SPEC;
impl crate::RegisterSpec for EPFR08_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epfr08::R`](R) reader structure"]
impl crate::Readable for EPFR08_SPEC {}
#[doc = "`write(|w| ..)` method takes [`epfr08::W`](W) writer structure"]
impl crate::Writable for EPFR08_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EPFR08 to value 0"]
impl crate::Resettable for EPFR08_SPEC {
    const RESET_VALUE: u32 = 0;
}
