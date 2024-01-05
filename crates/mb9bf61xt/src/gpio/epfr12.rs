#[doc = "Register `EPFR12` reader"]
pub type R = crate::R<EPFR12_SPEC>;
#[doc = "Register `EPFR12` writer"]
pub type W = crate::W<EPFR12_SPEC>;
#[doc = "Field `TIOA8E` reader - TIOA8 Output Select bits"]
pub type TIOA8E_R = crate::FieldReader;
#[doc = "Field `TIOA8E` writer - TIOA8 Output Select bits"]
pub type TIOA8E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOB8S` reader - TIOB8 Input Select bits"]
pub type TIOB8S_R = crate::FieldReader;
#[doc = "Field `TIOB8S` writer - TIOB8 Input Select bits"]
pub type TIOB8S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOA9S` reader - TIOA9 Input Select bits"]
pub type TIOA9S_R = crate::FieldReader;
#[doc = "Field `TIOA9S` writer - TIOA9 Input Select bits"]
pub type TIOA9S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOA9E` reader - TIOA9 Output Select bits"]
pub type TIOA9E_R = crate::FieldReader;
#[doc = "Field `TIOA9E` writer - TIOA9 Output Select bits"]
pub type TIOA9E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOB9S` reader - TIOB9 Input Select bits"]
pub type TIOB9S_R = crate::FieldReader;
#[doc = "Field `TIOB9S` writer - TIOB9 Input Select bits"]
pub type TIOB9S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOA10E` reader - TIOA10 Output Select bits"]
pub type TIOA10E_R = crate::FieldReader;
#[doc = "Field `TIOA10E` writer - TIOA10 Output Select bits"]
pub type TIOA10E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOB10S` reader - TIOB10 Input Select bits"]
pub type TIOB10S_R = crate::FieldReader;
#[doc = "Field `TIOB10S` writer - TIOB10 Input Select bits"]
pub type TIOB10S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOA11S` reader - TIOA11 Input Select bits"]
pub type TIOA11S_R = crate::FieldReader;
#[doc = "Field `TIOA11S` writer - TIOA11 Input Select bits"]
pub type TIOA11S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOA11E` reader - TIOA11 Output Select bits"]
pub type TIOA11E_R = crate::FieldReader;
#[doc = "Field `TIOA11E` writer - TIOA11 Output Select bits"]
pub type TIOA11E_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIOB11S` reader - TIOB11 Input Select bits"]
pub type TIOB11S_R = crate::FieldReader;
#[doc = "Field `TIOB11S` writer - TIOB11 Input Select bits"]
pub type TIOB11S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 2:3 - TIOA8 Output Select bits"]
    #[inline(always)]
    pub fn tioa8e(&self) -> TIOA8E_R {
        TIOA8E_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - TIOB8 Input Select bits"]
    #[inline(always)]
    pub fn tiob8s(&self) -> TIOB8S_R {
        TIOB8S_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TIOA9 Input Select bits"]
    #[inline(always)]
    pub fn tioa9s(&self) -> TIOA9S_R {
        TIOA9S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - TIOA9 Output Select bits"]
    #[inline(always)]
    pub fn tioa9e(&self) -> TIOA9E_R {
        TIOA9E_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - TIOB9 Input Select bits"]
    #[inline(always)]
    pub fn tiob9s(&self) -> TIOB9S_R {
        TIOB9S_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 18:19 - TIOA10 Output Select bits"]
    #[inline(always)]
    pub fn tioa10e(&self) -> TIOA10E_R {
        TIOA10E_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - TIOB10 Input Select bits"]
    #[inline(always)]
    pub fn tiob10s(&self) -> TIOB10S_R {
        TIOB10S_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - TIOA11 Input Select bits"]
    #[inline(always)]
    pub fn tioa11s(&self) -> TIOA11S_R {
        TIOA11S_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - TIOA11 Output Select bits"]
    #[inline(always)]
    pub fn tioa11e(&self) -> TIOA11E_R {
        TIOA11E_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - TIOB11 Input Select bits"]
    #[inline(always)]
    pub fn tiob11s(&self) -> TIOB11S_R {
        TIOB11S_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3 - TIOA8 Output Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn tioa8e(&mut self) -> TIOA8E_W<EPFR12_SPEC> {
        TIOA8E_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - TIOB8 Input Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn tiob8s(&mut self) -> TIOB8S_W<EPFR12_SPEC> {
        TIOB8S_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - TIOA9 Input Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn tioa9s(&mut self) -> TIOA9S_W<EPFR12_SPEC> {
        TIOA9S_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - TIOA9 Output Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn tioa9e(&mut self) -> TIOA9E_W<EPFR12_SPEC> {
        TIOA9E_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - TIOB9 Input Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn tiob9s(&mut self) -> TIOB9S_W<EPFR12_SPEC> {
        TIOB9S_W::new(self, 12)
    }
    #[doc = "Bits 18:19 - TIOA10 Output Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn tioa10e(&mut self) -> TIOA10E_W<EPFR12_SPEC> {
        TIOA10E_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - TIOB10 Input Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn tiob10s(&mut self) -> TIOB10S_W<EPFR12_SPEC> {
        TIOB10S_W::new(self, 20)
    }
    #[doc = "Bits 24:25 - TIOA11 Input Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn tioa11s(&mut self) -> TIOA11S_W<EPFR12_SPEC> {
        TIOA11S_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - TIOA11 Output Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn tioa11e(&mut self) -> TIOA11E_W<EPFR12_SPEC> {
        TIOA11E_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - TIOB11 Input Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn tiob11s(&mut self) -> TIOB11S_W<EPFR12_SPEC> {
        TIOB11S_W::new(self, 28)
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
#[doc = "Extended pin function setting register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPFR12_SPEC;
impl crate::RegisterSpec for EPFR12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epfr12::R`](R) reader structure"]
impl crate::Readable for EPFR12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`epfr12::W`](W) writer structure"]
impl crate::Writable for EPFR12_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EPFR12 to value 0"]
impl crate::Resettable for EPFR12_SPEC {
    const RESET_VALUE: u32 = 0;
}
