#[doc = "Register `APBC1_PSR` reader"]
pub type R = crate::R<APBC1_PSR_SPEC>;
#[doc = "Register `APBC1_PSR` writer"]
pub type W = crate::W<APBC1_PSR_SPEC>;
#[doc = "Field `APBC1` reader - APB1 bus clock frequency division ratio setting bit"]
pub type APBC1_R = crate::FieldReader;
#[doc = "Field `APBC1` writer - APB1 bus clock frequency division ratio setting bit"]
pub type APBC1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `APBC1RST` reader - APB1 bus reset control bit"]
pub type APBC1RST_R = crate::BitReader;
#[doc = "Field `APBC1RST` writer - APB1 bus reset control bit"]
pub type APBC1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APBC1EN` reader - APB1 clock enable bit"]
pub type APBC1EN_R = crate::BitReader;
#[doc = "Field `APBC1EN` writer - APB1 clock enable bit"]
pub type APBC1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - APB1 bus clock frequency division ratio setting bit"]
    #[inline(always)]
    pub fn apbc1(&self) -> APBC1_R {
        APBC1_R::new(self.bits & 3)
    }
    #[doc = "Bit 4 - APB1 bus reset control bit"]
    #[inline(always)]
    pub fn apbc1rst(&self) -> APBC1RST_R {
        APBC1RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - APB1 clock enable bit"]
    #[inline(always)]
    pub fn apbc1en(&self) -> APBC1EN_R {
        APBC1EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - APB1 bus clock frequency division ratio setting bit"]
    #[inline(always)]
    #[must_use]
    pub fn apbc1(&mut self) -> APBC1_W<APBC1_PSR_SPEC> {
        APBC1_W::new(self, 0)
    }
    #[doc = "Bit 4 - APB1 bus reset control bit"]
    #[inline(always)]
    #[must_use]
    pub fn apbc1rst(&mut self) -> APBC1RST_W<APBC1_PSR_SPEC> {
        APBC1RST_W::new(self, 4)
    }
    #[doc = "Bit 7 - APB1 clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn apbc1en(&mut self) -> APBC1EN_W<APBC1_PSR_SPEC> {
        APBC1EN_W::new(self, 7)
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
#[doc = "APB1 Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbc1_psr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbc1_psr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APBC1_PSR_SPEC;
impl crate::RegisterSpec for APBC1_PSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`apbc1_psr::R`](R) reader structure"]
impl crate::Readable for APBC1_PSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apbc1_psr::W`](W) writer structure"]
impl crate::Writable for APBC1_PSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets APBC1_PSR to value 0x80"]
impl crate::Resettable for APBC1_PSR_SPEC {
    const RESET_VALUE: u8 = 0x80;
}
