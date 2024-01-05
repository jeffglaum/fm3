#[doc = "Register `ADST0` reader"]
pub type R = crate::R<ADST0_SPEC>;
#[doc = "Register `ADST0` writer"]
pub type W = crate::W<ADST0_SPEC>;
#[doc = "Field `ST0` reader - Sampling time setting bits"]
pub type ST0_R = crate::FieldReader;
#[doc = "Field `ST0` writer - Sampling time setting bits"]
pub type ST0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `STX0` reader - Sampling time N times setting bits"]
pub type STX0_R = crate::FieldReader;
#[doc = "Field `STX0` writer - Sampling time N times setting bits"]
pub type STX0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:4 - Sampling time setting bits"]
    #[inline(always)]
    pub fn st0(&self) -> ST0_R {
        ST0_R::new(self.bits & 0x1f)
    }
    #[doc = "Bits 5:7 - Sampling time N times setting bits"]
    #[inline(always)]
    pub fn stx0(&self) -> STX0_R {
        STX0_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bits 0:4 - Sampling time setting bits"]
    #[inline(always)]
    #[must_use]
    pub fn st0(&mut self) -> ST0_W<ADST0_SPEC> {
        ST0_W::new(self, 0)
    }
    #[doc = "Bits 5:7 - Sampling time N times setting bits"]
    #[inline(always)]
    #[must_use]
    pub fn stx0(&mut self) -> STX0_W<ADST0_SPEC> {
        STX0_W::new(self, 5)
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
#[doc = "Sampling Time Setup Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adst0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adst0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADST0_SPEC;
impl crate::RegisterSpec for ADST0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adst0::R`](R) reader structure"]
impl crate::Readable for ADST0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adst0::W`](W) writer structure"]
impl crate::Writable for ADST0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADST0 to value 0x10"]
impl crate::Resettable for ADST0_SPEC {
    const RESET_VALUE: u8 = 0x10;
}
