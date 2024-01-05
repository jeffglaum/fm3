#[doc = "Register `EPFR04` reader"]
pub type R = crate::R<EPFR04_SPEC>;
#[doc = "Register `EPFR04` writer"]
pub type W = crate::W<EPFR04_SPEC>;
#[doc = "Field `TIOA0E` reader - TIOA0 output select bit"]
pub type TIOA0E_R = crate::FieldReader;
#[doc = "Field `TIOA0E` writer - TIOA0 output select bit"]
pub type TIOA0E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOB0S` reader - TIOB0 input select bit"]
pub type TIOB0S_R = crate::FieldReader;
#[doc = "Field `TIOB0S` writer - TIOB0 input select bit"]
pub type TIOB0S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOA1S` reader - TIOA1 input select bit"]
pub type TIOA1S_R = crate::FieldReader;
#[doc = "Field `TIOA1S` writer - TIOA1 input select bit"]
pub type TIOA1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOA1E` reader - TIOA1E output select bit"]
pub type TIOA1E_R = crate::FieldReader;
#[doc = "Field `TIOA1E` writer - TIOA1E output select bit"]
pub type TIOA1E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOB1S` reader - TIOB1 input select bit"]
pub type TIOB1S_R = crate::FieldReader;
#[doc = "Field `TIOB1S` writer - TIOB1 input select bit"]
pub type TIOB1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOA2E` reader - TIOA2 output select bit"]
pub type TIOA2E_R = crate::FieldReader;
#[doc = "Field `TIOA2E` writer - TIOA2 output select bit"]
pub type TIOA2E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOB2S` reader - TIOB2 input select bit"]
pub type TIOB2S_R = crate::FieldReader;
#[doc = "Field `TIOB2S` writer - TIOB2 input select bit"]
pub type TIOB2S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOA3S` reader - TIOA3 input select bit"]
pub type TIOA3S_R = crate::FieldReader;
#[doc = "Field `TIOA3S` writer - TIOA3 input select bit"]
pub type TIOA3S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOA3E` reader - TIOA3E output select bit"]
pub type TIOA3E_R = crate::FieldReader;
#[doc = "Field `TIOA3E` writer - TIOA3E output select bit"]
pub type TIOA3E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOB3S` reader - TIOB3 input select bit"]
pub type TIOB3S_R = crate::FieldReader;
#[doc = "Field `TIOB3S` writer - TIOB3 input select bit"]
pub type TIOB3S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 2:3 - TIOA0 output select bit"]
    #[inline(always)]
    pub fn tioa0e(&self) -> TIOA0E_R {
        TIOA0E_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - TIOB0 input select bit"]
    #[inline(always)]
    pub fn tiob0s(&self) -> TIOB0S_R {
        TIOB0S_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TIOA1 input select bit"]
    #[inline(always)]
    pub fn tioa1s(&self) -> TIOA1S_R {
        TIOA1S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - TIOA1E output select bit"]
    #[inline(always)]
    pub fn tioa1e(&self) -> TIOA1E_R {
        TIOA1E_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - TIOB1 input select bit"]
    #[inline(always)]
    pub fn tiob1s(&self) -> TIOB1S_R {
        TIOB1S_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 18:19 - TIOA2 output select bit"]
    #[inline(always)]
    pub fn tioa2e(&self) -> TIOA2E_R {
        TIOA2E_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - TIOB2 input select bit"]
    #[inline(always)]
    pub fn tiob2s(&self) -> TIOB2S_R {
        TIOB2S_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - TIOA3 input select bit"]
    #[inline(always)]
    pub fn tioa3s(&self) -> TIOA3S_R {
        TIOA3S_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - TIOA3E output select bit"]
    #[inline(always)]
    pub fn tioa3e(&self) -> TIOA3E_R {
        TIOA3E_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - TIOB3 input select bit"]
    #[inline(always)]
    pub fn tiob3s(&self) -> TIOB3S_R {
        TIOB3S_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3 - TIOA0 output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn tioa0e(&mut self) -> TIOA0E_W<EPFR04_SPEC> {
        TIOA0E_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - TIOB0 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn tiob0s(&mut self) -> TIOB0S_W<EPFR04_SPEC> {
        TIOB0S_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - TIOA1 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn tioa1s(&mut self) -> TIOA1S_W<EPFR04_SPEC> {
        TIOA1S_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - TIOA1E output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn tioa1e(&mut self) -> TIOA1E_W<EPFR04_SPEC> {
        TIOA1E_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - TIOB1 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn tiob1s(&mut self) -> TIOB1S_W<EPFR04_SPEC> {
        TIOB1S_W::new(self, 12)
    }
    #[doc = "Bits 18:19 - TIOA2 output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn tioa2e(&mut self) -> TIOA2E_W<EPFR04_SPEC> {
        TIOA2E_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - TIOB2 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn tiob2s(&mut self) -> TIOB2S_W<EPFR04_SPEC> {
        TIOB2S_W::new(self, 20)
    }
    #[doc = "Bits 24:25 - TIOA3 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn tioa3s(&mut self) -> TIOA3S_W<EPFR04_SPEC> {
        TIOA3S_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - TIOA3E output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn tioa3e(&mut self) -> TIOA3E_W<EPFR04_SPEC> {
        TIOA3E_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - TIOB3 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn tiob3s(&mut self) -> TIOB3S_W<EPFR04_SPEC> {
        TIOB3S_W::new(self, 28)
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
#[doc = "Extended pin function setting register 04\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr04::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr04::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPFR04_SPEC;
impl crate::RegisterSpec for EPFR04_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epfr04::R`](R) reader structure"]
impl crate::Readable for EPFR04_SPEC {}
#[doc = "`write(|w| ..)` method takes [`epfr04::W`](W) writer structure"]
impl crate::Writable for EPFR04_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EPFR04 to value 0"]
impl crate::Resettable for EPFR04_SPEC {
    const RESET_VALUE: u32 = 0;
}
