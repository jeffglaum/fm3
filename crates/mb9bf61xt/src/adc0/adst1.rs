#[doc = "Register `ADST1` reader"]
pub type R = crate::R<ADST1_SPEC>;
#[doc = "Register `ADST1` writer"]
pub type W = crate::W<ADST1_SPEC>;
#[doc = "Field `ST1` reader - Sampling time setting bits"]
pub type ST1_R = crate::FieldReader;
#[doc = "Field `ST1` writer - Sampling time setting bits"]
pub type ST1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `STX1` reader - Sampling time N times setting bits"]
pub type STX1_R = crate::FieldReader;
#[doc = "Field `STX1` writer - Sampling time N times setting bits"]
pub type STX1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:4 - Sampling time setting bits"]
    #[inline(always)]
    pub fn st1(&self) -> ST1_R {
        ST1_R::new(self.bits & 0x1f)
    }
    #[doc = "Bits 5:7 - Sampling time N times setting bits"]
    #[inline(always)]
    pub fn stx1(&self) -> STX1_R {
        STX1_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bits 0:4 - Sampling time setting bits"]
    #[inline(always)]
    #[must_use]
    pub fn st1(&mut self) -> ST1_W<ADST1_SPEC> {
        ST1_W::new(self, 0)
    }
    #[doc = "Bits 5:7 - Sampling time N times setting bits"]
    #[inline(always)]
    #[must_use]
    pub fn stx1(&mut self) -> STX1_W<ADST1_SPEC> {
        STX1_W::new(self, 5)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Sampling Time Setup Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adst1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adst1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADST1_SPEC;
impl crate::RegisterSpec for ADST1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adst1::R`](R) reader structure"]
impl crate::Readable for ADST1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adst1::W`](W) writer structure"]
impl crate::Writable for ADST1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADST1 to value 0x10"]
impl crate::Resettable for ADST1_SPEC {
    const RESET_VALUE: u8 = 0x10;
}
