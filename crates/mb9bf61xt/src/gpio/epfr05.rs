#[doc = "Register `EPFR05` reader"]
pub type R = crate::R<EPFR05_SPEC>;
#[doc = "Register `EPFR05` writer"]
pub type W = crate::W<EPFR05_SPEC>;
#[doc = "Field `TIOA4E` reader - TIOA4 output select bit"]
pub type TIOA4E_R = crate::FieldReader;
#[doc = "Field `TIOA4E` writer - TIOA4 output select bit"]
pub type TIOA4E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOB4S` reader - TIOB4 input select bit"]
pub type TIOB4S_R = crate::FieldReader;
#[doc = "Field `TIOB4S` writer - TIOB4 input select bit"]
pub type TIOB4S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOA5S` reader - TIOA5 input select bit"]
pub type TIOA5S_R = crate::FieldReader;
#[doc = "Field `TIOA5S` writer - TIOA5 input select bit"]
pub type TIOA5S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOA5E` reader - TIOA5E output select bit"]
pub type TIOA5E_R = crate::FieldReader;
#[doc = "Field `TIOA5E` writer - TIOA5E output select bit"]
pub type TIOA5E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOB5S` reader - TIOB5 input select bit"]
pub type TIOB5S_R = crate::FieldReader;
#[doc = "Field `TIOB5S` writer - TIOB5 input select bit"]
pub type TIOB5S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOA6E` reader - TIOA6 output select bit"]
pub type TIOA6E_R = crate::FieldReader;
#[doc = "Field `TIOA6E` writer - TIOA6 output select bit"]
pub type TIOA6E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOB6S` reader - TIOB6 input select bit"]
pub type TIOB6S_R = crate::FieldReader;
#[doc = "Field `TIOB6S` writer - TIOB6 input select bit"]
pub type TIOB6S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOA7S` reader - TIOA7 input select bit"]
pub type TIOA7S_R = crate::FieldReader;
#[doc = "Field `TIOA7S` writer - TIOA7 input select bit"]
pub type TIOA7S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOA7E` reader - TIOA7E output select bit"]
pub type TIOA7E_R = crate::FieldReader;
#[doc = "Field `TIOA7E` writer - TIOA7E output select bit"]
pub type TIOA7E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOB7S` reader - TIOB7 input select Bit"]
pub type TIOB7S_R = crate::FieldReader;
#[doc = "Field `TIOB7S` writer - TIOB7 input select Bit"]
pub type TIOB7S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 2:3 - TIOA4 output select bit"]
    #[inline(always)]
    pub fn tioa4e(&self) -> TIOA4E_R {
        TIOA4E_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - TIOB4 input select bit"]
    #[inline(always)]
    pub fn tiob4s(&self) -> TIOB4S_R {
        TIOB4S_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TIOA5 input select bit"]
    #[inline(always)]
    pub fn tioa5s(&self) -> TIOA5S_R {
        TIOA5S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - TIOA5E output select bit"]
    #[inline(always)]
    pub fn tioa5e(&self) -> TIOA5E_R {
        TIOA5E_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - TIOB5 input select bit"]
    #[inline(always)]
    pub fn tiob5s(&self) -> TIOB5S_R {
        TIOB5S_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 18:19 - TIOA6 output select bit"]
    #[inline(always)]
    pub fn tioa6e(&self) -> TIOA6E_R {
        TIOA6E_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - TIOB6 input select bit"]
    #[inline(always)]
    pub fn tiob6s(&self) -> TIOB6S_R {
        TIOB6S_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - TIOA7 input select bit"]
    #[inline(always)]
    pub fn tioa7s(&self) -> TIOA7S_R {
        TIOA7S_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - TIOA7E output select bit"]
    #[inline(always)]
    pub fn tioa7e(&self) -> TIOA7E_R {
        TIOA7E_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - TIOB7 input select Bit"]
    #[inline(always)]
    pub fn tiob7s(&self) -> TIOB7S_R {
        TIOB7S_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3 - TIOA4 output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn tioa4e(&mut self) -> TIOA4E_W<EPFR05_SPEC> {
        TIOA4E_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - TIOB4 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn tiob4s(&mut self) -> TIOB4S_W<EPFR05_SPEC> {
        TIOB4S_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - TIOA5 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn tioa5s(&mut self) -> TIOA5S_W<EPFR05_SPEC> {
        TIOA5S_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - TIOA5E output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn tioa5e(&mut self) -> TIOA5E_W<EPFR05_SPEC> {
        TIOA5E_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - TIOB5 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn tiob5s(&mut self) -> TIOB5S_W<EPFR05_SPEC> {
        TIOB5S_W::new(self, 12)
    }
    #[doc = "Bits 18:19 - TIOA6 output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn tioa6e(&mut self) -> TIOA6E_W<EPFR05_SPEC> {
        TIOA6E_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - TIOB6 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn tiob6s(&mut self) -> TIOB6S_W<EPFR05_SPEC> {
        TIOB6S_W::new(self, 20)
    }
    #[doc = "Bits 24:25 - TIOA7 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn tioa7s(&mut self) -> TIOA7S_W<EPFR05_SPEC> {
        TIOA7S_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - TIOA7E output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn tioa7e(&mut self) -> TIOA7E_W<EPFR05_SPEC> {
        TIOA7E_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - TIOB7 input select Bit"]
    #[inline(always)]
    #[must_use]
    pub fn tiob7s(&mut self) -> TIOB7S_W<EPFR05_SPEC> {
        TIOB7S_W::new(self, 28)
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
#[doc = "Extended pin function setting register 05\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr05::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr05::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPFR05_SPEC;
impl crate::RegisterSpec for EPFR05_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epfr05::R`](R) reader structure"]
impl crate::Readable for EPFR05_SPEC {}
#[doc = "`write(|w| ..)` method takes [`epfr05::W`](W) writer structure"]
impl crate::Writable for EPFR05_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EPFR05 to value 0"]
impl crate::Resettable for EPFR05_SPEC {
    const RESET_VALUE: u32 = 0;
}
