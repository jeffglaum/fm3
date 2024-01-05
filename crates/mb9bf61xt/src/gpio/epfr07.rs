#[doc = "Register `EPFR07` reader"]
pub type R = crate::R<EPFR07_SPEC>;
#[doc = "Register `EPFR07` writer"]
pub type W = crate::W<EPFR07_SPEC>;
#[doc = "Field `SIN0S` reader - SIN0S input select bit"]
pub type SIN0S_R = crate::FieldReader;
#[doc = "Field `SIN0S` writer - SIN0S input select bit"]
pub type SIN0S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SOT0B` reader - SOT0B input/output select bit"]
pub type SOT0B_R = crate::FieldReader;
#[doc = "Field `SOT0B` writer - SOT0B input/output select bit"]
pub type SOT0B_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCK0B` reader - SCK0 input/output select bit"]
pub type SCK0B_R = crate::FieldReader;
#[doc = "Field `SCK0B` writer - SCK0 input/output select bit"]
pub type SCK0B_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SIN1S` reader - SIN1S input select bit"]
pub type SIN1S_R = crate::FieldReader;
#[doc = "Field `SIN1S` writer - SIN1S input select bit"]
pub type SIN1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SOT1B` reader - SCK1B input/output select bit"]
pub type SOT1B_R = crate::FieldReader;
#[doc = "Field `SOT1B` writer - SCK1B input/output select bit"]
pub type SOT1B_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCK1B` reader - SCK1 input/output select bit"]
pub type SCK1B_R = crate::FieldReader;
#[doc = "Field `SCK1B` writer - SCK1 input/output select bit"]
pub type SCK1B_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SIN2S` reader - SIN2S input select bit"]
pub type SIN2S_R = crate::FieldReader;
#[doc = "Field `SIN2S` writer - SIN2S input select bit"]
pub type SIN2S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SOT2B` reader - SOT2B input/output select bit"]
pub type SOT2B_R = crate::FieldReader;
#[doc = "Field `SOT2B` writer - SOT2B input/output select bit"]
pub type SOT2B_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCK2B` reader - SCK2 input/output select bit"]
pub type SCK2B_R = crate::FieldReader;
#[doc = "Field `SCK2B` writer - SCK2 input/output select bit"]
pub type SCK2B_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SIN3S` reader - SIN3S input select bit"]
pub type SIN3S_R = crate::FieldReader;
#[doc = "Field `SIN3S` writer - SIN3S input select bit"]
pub type SIN3S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SOT3B` reader - SOT3B input/output select bit"]
pub type SOT3B_R = crate::FieldReader;
#[doc = "Field `SOT3B` writer - SOT3B input/output select bit"]
pub type SOT3B_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCK3B` reader - SCK3 input/output select bit"]
pub type SCK3B_R = crate::FieldReader;
#[doc = "Field `SCK3B` writer - SCK3 input/output select bit"]
pub type SCK3B_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 4:5 - SIN0S input select bit"]
    #[inline(always)]
    pub fn sin0s(&self) -> SIN0S_R {
        SIN0S_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - SOT0B input/output select bit"]
    #[inline(always)]
    pub fn sot0b(&self) -> SOT0B_R {
        SOT0B_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SCK0 input/output select bit"]
    #[inline(always)]
    pub fn sck0b(&self) -> SCK0B_R {
        SCK0B_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - SIN1S input select bit"]
    #[inline(always)]
    pub fn sin1s(&self) -> SIN1S_R {
        SIN1S_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - SCK1B input/output select bit"]
    #[inline(always)]
    pub fn sot1b(&self) -> SOT1B_R {
        SOT1B_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - SCK1 input/output select bit"]
    #[inline(always)]
    pub fn sck1b(&self) -> SCK1B_R {
        SCK1B_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - SIN2S input select bit"]
    #[inline(always)]
    pub fn sin2s(&self) -> SIN2S_R {
        SIN2S_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - SOT2B input/output select bit"]
    #[inline(always)]
    pub fn sot2b(&self) -> SOT2B_R {
        SOT2B_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - SCK2 input/output select bit"]
    #[inline(always)]
    pub fn sck2b(&self) -> SCK2B_R {
        SCK2B_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - SIN3S input select bit"]
    #[inline(always)]
    pub fn sin3s(&self) -> SIN3S_R {
        SIN3S_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - SOT3B input/output select bit"]
    #[inline(always)]
    pub fn sot3b(&self) -> SOT3B_R {
        SOT3B_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - SCK3 input/output select bit"]
    #[inline(always)]
    pub fn sck3b(&self) -> SCK3B_R {
        SCK3B_R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - SIN0S input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn sin0s(&mut self) -> SIN0S_W<EPFR07_SPEC> {
        SIN0S_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - SOT0B input/output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn sot0b(&mut self) -> SOT0B_W<EPFR07_SPEC> {
        SOT0B_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - SCK0 input/output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn sck0b(&mut self) -> SCK0B_W<EPFR07_SPEC> {
        SCK0B_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - SIN1S input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn sin1s(&mut self) -> SIN1S_W<EPFR07_SPEC> {
        SIN1S_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - SCK1B input/output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn sot1b(&mut self) -> SOT1B_W<EPFR07_SPEC> {
        SOT1B_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - SCK1 input/output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn sck1b(&mut self) -> SCK1B_W<EPFR07_SPEC> {
        SCK1B_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - SIN2S input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn sin2s(&mut self) -> SIN2S_W<EPFR07_SPEC> {
        SIN2S_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - SOT2B input/output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn sot2b(&mut self) -> SOT2B_W<EPFR07_SPEC> {
        SOT2B_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - SCK2 input/output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn sck2b(&mut self) -> SCK2B_W<EPFR07_SPEC> {
        SCK2B_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - SIN3S input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn sin3s(&mut self) -> SIN3S_W<EPFR07_SPEC> {
        SIN3S_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - SOT3B input/output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn sot3b(&mut self) -> SOT3B_W<EPFR07_SPEC> {
        SOT3B_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - SCK3 input/output select bit"]
    #[inline(always)]
    #[must_use]
    pub fn sck3b(&mut self) -> SCK3B_W<EPFR07_SPEC> {
        SCK3B_W::new(self, 26)
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
#[doc = "Extended pin function setting register 07\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr07::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr07::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPFR07_SPEC;
impl crate::RegisterSpec for EPFR07_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epfr07::R`](R) reader structure"]
impl crate::Readable for EPFR07_SPEC {}
#[doc = "`write(|w| ..)` method takes [`epfr07::W`](W) writer structure"]
impl crate::Writable for EPFR07_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EPFR07 to value 0"]
impl crate::Resettable for EPFR07_SPEC {
    const RESET_VALUE: u32 = 0;
}
