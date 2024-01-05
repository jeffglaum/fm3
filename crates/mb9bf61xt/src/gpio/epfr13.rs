#[doc = "Register `EPFR13` reader"]
pub type R = crate::R<EPFR13_SPEC>;
#[doc = "Register `EPFR13` writer"]
pub type W = crate::W<EPFR13_SPEC>;
#[doc = "Field `TIOA12E` reader - TIOA12 Output Select bits"]
pub type TIOA12E_R = crate::FieldReader;
#[doc = "Field `TIOA12E` writer - TIOA12 Output Select bits"]
pub type TIOA12E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOB12S` reader - TIOB12 Input Select bits"]
pub type TIOB12S_R = crate::FieldReader;
#[doc = "Field `TIOB12S` writer - TIOB12 Input Select bits"]
pub type TIOB12S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOA13S` reader - TIOA13 Input Select bits"]
pub type TIOA13S_R = crate::FieldReader;
#[doc = "Field `TIOA13S` writer - TIOA13 Input Select bits"]
pub type TIOA13S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOA13E` reader - TIOA13 Output Select bits"]
pub type TIOA13E_R = crate::FieldReader;
#[doc = "Field `TIOA13E` writer - TIOA13 Output Select bits"]
pub type TIOA13E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOB13S` reader - TIOB13 Input Select bits"]
pub type TIOB13S_R = crate::FieldReader;
#[doc = "Field `TIOB13S` writer - TIOB13 Input Select bits"]
pub type TIOB13S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOA14E` reader - TIOA14 Output Select bits"]
pub type TIOA14E_R = crate::FieldReader;
#[doc = "Field `TIOA14E` writer - TIOA14 Output Select bits"]
pub type TIOA14E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOB14S` reader - TIOB14 Input Select bits"]
pub type TIOB14S_R = crate::FieldReader;
#[doc = "Field `TIOB14S` writer - TIOB14 Input Select bits"]
pub type TIOB14S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOA15S` reader - TIOA15 Input Select bits"]
pub type TIOA15S_R = crate::FieldReader;
#[doc = "Field `TIOA15S` writer - TIOA15 Input Select bits"]
pub type TIOA15S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOA15E` reader - TIOA15 Output Select bits"]
pub type TIOA15E_R = crate::FieldReader;
#[doc = "Field `TIOA15E` writer - TIOA15 Output Select bits"]
pub type TIOA15E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOB15S` reader - TIOB15 Input Select bits"]
pub type TIOB15S_R = crate::FieldReader;
#[doc = "Field `TIOB15S` writer - TIOB15 Input Select bits"]
pub type TIOB15S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 2:3 - TIOA12 Output Select bits"]
    #[inline(always)]
    pub fn tioa12e(&self) -> TIOA12E_R {
        TIOA12E_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - TIOB12 Input Select bits"]
    #[inline(always)]
    pub fn tiob12s(&self) -> TIOB12S_R {
        TIOB12S_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TIOA13 Input Select bits"]
    #[inline(always)]
    pub fn tioa13s(&self) -> TIOA13S_R {
        TIOA13S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - TIOA13 Output Select bits"]
    #[inline(always)]
    pub fn tioa13e(&self) -> TIOA13E_R {
        TIOA13E_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - TIOB13 Input Select bits"]
    #[inline(always)]
    pub fn tiob13s(&self) -> TIOB13S_R {
        TIOB13S_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 18:19 - TIOA14 Output Select bits"]
    #[inline(always)]
    pub fn tioa14e(&self) -> TIOA14E_R {
        TIOA14E_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - TIOB14 Input Select bits"]
    #[inline(always)]
    pub fn tiob14s(&self) -> TIOB14S_R {
        TIOB14S_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - TIOA15 Input Select bits"]
    #[inline(always)]
    pub fn tioa15s(&self) -> TIOA15S_R {
        TIOA15S_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - TIOA15 Output Select bits"]
    #[inline(always)]
    pub fn tioa15e(&self) -> TIOA15E_R {
        TIOA15E_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - TIOB15 Input Select bits"]
    #[inline(always)]
    pub fn tiob15s(&self) -> TIOB15S_R {
        TIOB15S_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3 - TIOA12 Output Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn tioa12e(&mut self) -> TIOA12E_W<EPFR13_SPEC> {
        TIOA12E_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - TIOB12 Input Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn tiob12s(&mut self) -> TIOB12S_W<EPFR13_SPEC> {
        TIOB12S_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - TIOA13 Input Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn tioa13s(&mut self) -> TIOA13S_W<EPFR13_SPEC> {
        TIOA13S_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - TIOA13 Output Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn tioa13e(&mut self) -> TIOA13E_W<EPFR13_SPEC> {
        TIOA13E_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - TIOB13 Input Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn tiob13s(&mut self) -> TIOB13S_W<EPFR13_SPEC> {
        TIOB13S_W::new(self, 12)
    }
    #[doc = "Bits 18:19 - TIOA14 Output Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn tioa14e(&mut self) -> TIOA14E_W<EPFR13_SPEC> {
        TIOA14E_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - TIOB14 Input Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn tiob14s(&mut self) -> TIOB14S_W<EPFR13_SPEC> {
        TIOB14S_W::new(self, 20)
    }
    #[doc = "Bits 24:25 - TIOA15 Input Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn tioa15s(&mut self) -> TIOA15S_W<EPFR13_SPEC> {
        TIOA15S_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - TIOA15 Output Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn tioa15e(&mut self) -> TIOA15E_W<EPFR13_SPEC> {
        TIOA15E_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - TIOB15 Input Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn tiob15s(&mut self) -> TIOB15S_W<EPFR13_SPEC> {
        TIOB15S_W::new(self, 28)
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
#[doc = "Extended pin function setting register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPFR13_SPEC;
impl crate::RegisterSpec for EPFR13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epfr13::R`](R) reader structure"]
impl crate::Readable for EPFR13_SPEC {}
#[doc = "`write(|w| ..)` method takes [`epfr13::W`](W) writer structure"]
impl crate::Writable for EPFR13_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EPFR13 to value 0"]
impl crate::Resettable for EPFR13_SPEC {
    const RESET_VALUE: u32 = 0;
}
