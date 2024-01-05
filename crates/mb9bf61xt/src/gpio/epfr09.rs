#[doc = "Register `EPFR09` reader"]
pub type R = crate::R<EPFR09_SPEC>;
#[doc = "Register `EPFR09` writer"]
pub type W = crate::W<EPFR09_SPEC>;
#[doc = "Field `QAIN0S` reader - QAIN0S input select bit"]
pub type QAIN0S_R = crate::FieldReader;
#[doc = "Field `QAIN0S` writer - QAIN0S input select bit"]
pub type QAIN0S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `QBIN0S` reader - QBIN0S input select bit"]
pub type QBIN0S_R = crate::FieldReader;
#[doc = "Field `QBIN0S` writer - QBIN0S input select bit"]
pub type QBIN0S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `QZIN0S` reader - QZIN0S input select bit"]
pub type QZIN0S_R = crate::FieldReader;
#[doc = "Field `QZIN0S` writer - QZIN0S input select bit"]
pub type QZIN0S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `QAIN1S` reader - QAIN1S input select bit"]
pub type QAIN1S_R = crate::FieldReader;
#[doc = "Field `QAIN1S` writer - QAIN1S input select bit"]
pub type QAIN1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `QBIN1S` reader - QBIN1S input select bit"]
pub type QBIN1S_R = crate::FieldReader;
#[doc = "Field `QBIN1S` writer - QBIN1S input select bit"]
pub type QBIN1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `QZIN1S` reader - QZIN1S input select bit"]
pub type QZIN1S_R = crate::FieldReader;
#[doc = "Field `QZIN1S` writer - QZIN1S input select bit"]
pub type QZIN1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADTRG0S` reader - ADTRG0 input select bit"]
pub type ADTRG0S_R = crate::FieldReader;
#[doc = "Field `ADTRG0S` writer - ADTRG0 input select bit"]
pub type ADTRG0S_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADTRG1S` reader - ADTRG1 input select bit"]
pub type ADTRG1S_R = crate::FieldReader;
#[doc = "Field `ADTRG1S` writer - ADTRG1 input select bit"]
pub type ADTRG1S_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADTRG2S` reader - ADTRG2 input select bit"]
pub type ADTRG2S_R = crate::FieldReader;
#[doc = "Field `ADTRG2S` writer - ADTRG2 input select bit"]
pub type ADTRG2S_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - QAIN0S input select bit"]
    #[inline(always)]
    pub fn qain0s(&self) -> QAIN0S_R {
        QAIN0S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - QBIN0S input select bit"]
    #[inline(always)]
    pub fn qbin0s(&self) -> QBIN0S_R {
        QBIN0S_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - QZIN0S input select bit"]
    #[inline(always)]
    pub fn qzin0s(&self) -> QZIN0S_R {
        QZIN0S_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - QAIN1S input select bit"]
    #[inline(always)]
    pub fn qain1s(&self) -> QAIN1S_R {
        QAIN1S_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - QBIN1S input select bit"]
    #[inline(always)]
    pub fn qbin1s(&self) -> QBIN1S_R {
        QBIN1S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - QZIN1S input select bit"]
    #[inline(always)]
    pub fn qzin1s(&self) -> QZIN1S_R {
        QZIN1S_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - ADTRG0 input select bit"]
    #[inline(always)]
    pub fn adtrg0s(&self) -> ADTRG0S_R {
        ADTRG0S_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - ADTRG1 input select bit"]
    #[inline(always)]
    pub fn adtrg1s(&self) -> ADTRG1S_R {
        ADTRG1S_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - ADTRG2 input select bit"]
    #[inline(always)]
    pub fn adtrg2s(&self) -> ADTRG2S_R {
        ADTRG2S_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - QAIN0S input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn qain0s(&mut self) -> QAIN0S_W<EPFR09_SPEC> {
        QAIN0S_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - QBIN0S input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn qbin0s(&mut self) -> QBIN0S_W<EPFR09_SPEC> {
        QBIN0S_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - QZIN0S input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn qzin0s(&mut self) -> QZIN0S_W<EPFR09_SPEC> {
        QZIN0S_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - QAIN1S input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn qain1s(&mut self) -> QAIN1S_W<EPFR09_SPEC> {
        QAIN1S_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - QBIN1S input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn qbin1s(&mut self) -> QBIN1S_W<EPFR09_SPEC> {
        QBIN1S_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - QZIN1S input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn qzin1s(&mut self) -> QZIN1S_W<EPFR09_SPEC> {
        QZIN1S_W::new(self, 10)
    }
    #[doc = "Bits 12:15 - ADTRG0 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn adtrg0s(&mut self) -> ADTRG0S_W<EPFR09_SPEC> {
        ADTRG0S_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - ADTRG1 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn adtrg1s(&mut self) -> ADTRG1S_W<EPFR09_SPEC> {
        ADTRG1S_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - ADTRG2 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn adtrg2s(&mut self) -> ADTRG2S_W<EPFR09_SPEC> {
        ADTRG2S_W::new(self, 20)
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
#[doc = "Extended pin function setting register 09\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr09::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr09::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPFR09_SPEC;
impl crate::RegisterSpec for EPFR09_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epfr09::R`](R) reader structure"]
impl crate::Readable for EPFR09_SPEC {}
#[doc = "`write(|w| ..)` method takes [`epfr09::W`](W) writer structure"]
impl crate::Writable for EPFR09_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EPFR09 to value 0"]
impl crate::Resettable for EPFR09_SPEC {
    const RESET_VALUE: u32 = 0;
}
