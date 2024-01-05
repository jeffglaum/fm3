#[doc = "Register `APBC2_PSR` reader"]
pub type R = crate::R<APBC2_PSR_SPEC>;
#[doc = "Register `APBC2_PSR` writer"]
pub type W = crate::W<APBC2_PSR_SPEC>;
#[doc = "Field `APBC2` reader - APB2 bus clock frequency division ratio setting bit"]
pub type APBC2_R = crate::FieldReader;
#[doc = "Field `APBC2` writer - APB2 bus clock frequency division ratio setting bit"]
pub type APBC2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `APBC2RST` reader - APB2 bus reset control bit"]
pub type APBC2RST_R = crate::BitReader;
#[doc = "Field `APBC2RST` writer - APB2 bus reset control bit"]
pub type APBC2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APBC2EN` reader - APB2 clock enable bit"]
pub type APBC2EN_R = crate::BitReader;
#[doc = "Field `APBC2EN` writer - APB2 clock enable bit"]
pub type APBC2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - APB2 bus clock frequency division ratio setting bit"]
    #[inline(always)]
    pub fn apbc2(&self) -> APBC2_R {
        APBC2_R::new(self.bits & 3)
    }
    #[doc = "Bit 4 - APB2 bus reset control bit"]
    #[inline(always)]
    pub fn apbc2rst(&self) -> APBC2RST_R {
        APBC2RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - APB2 clock enable bit"]
    #[inline(always)]
    pub fn apbc2en(&self) -> APBC2EN_R {
        APBC2EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - APB2 bus clock frequency division ratio setting bit"]
    #[inline(always)]
    #[must_use]
    pub fn apbc2(&mut self) -> APBC2_W<APBC2_PSR_SPEC> {
        APBC2_W::new(self, 0)
    }
    #[doc = "Bit 4 - APB2 bus reset control bit"]
    #[inline(always)]
    #[must_use]
    pub fn apbc2rst(&mut self) -> APBC2RST_W<APBC2_PSR_SPEC> {
        APBC2RST_W::new(self, 4)
    }
    #[doc = "Bit 7 - APB2 clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn apbc2en(&mut self) -> APBC2EN_W<APBC2_PSR_SPEC> {
        APBC2EN_W::new(self, 7)
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
#[doc = "APB2 Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbc2_psr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbc2_psr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APBC2_PSR_SPEC;
impl crate::RegisterSpec for APBC2_PSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`apbc2_psr::R`](R) reader structure"]
impl crate::Readable for APBC2_PSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apbc2_psr::W`](W) writer structure"]
impl crate::Writable for APBC2_PSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets APBC2_PSR to value 0x80"]
impl crate::Resettable for APBC2_PSR_SPEC {
    const RESET_VALUE: u8 = 0x80;
}
