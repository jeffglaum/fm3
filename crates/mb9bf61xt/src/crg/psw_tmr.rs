#[doc = "Register `PSW_TMR` reader"]
pub type R = crate::R<PSW_TMR_SPEC>;
#[doc = "Register `PSW_TMR` writer"]
pub type W = crate::W<PSW_TMR_SPEC>;
#[doc = "Field `POWT` reader - PLL clock stabilization wait time setup bit"]
pub type POWT_R = crate::FieldReader;
#[doc = "Field `POWT` writer - PLL clock stabilization wait time setup bit"]
pub type POWT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PINC` reader - PLL input clock select bit"]
pub type PINC_R = crate::BitReader;
#[doc = "Field `PINC` writer - PLL input clock select bit"]
pub type PINC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - PLL clock stabilization wait time setup bit"]
    #[inline(always)]
    pub fn powt(&self) -> POWT_R {
        POWT_R::new(self.bits & 7)
    }
    #[doc = "Bit 4 - PLL input clock select bit"]
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - PLL clock stabilization wait time setup bit"]
    #[inline(always)]
    #[must_use]
    pub fn powt(&mut self) -> POWT_W<PSW_TMR_SPEC> {
        POWT_W::new(self, 0)
    }
    #[doc = "Bit 4 - PLL input clock select bit"]
    #[inline(always)]
    #[must_use]
    pub fn pinc(&mut self) -> PINC_W<PSW_TMR_SPEC> {
        PINC_W::new(self, 4)
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
#[doc = "PLL Clock Stabilization Wait Time Setup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psw_tmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psw_tmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSW_TMR_SPEC;
impl crate::RegisterSpec for PSW_TMR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`psw_tmr::R`](R) reader structure"]
impl crate::Readable for PSW_TMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psw_tmr::W`](W) writer structure"]
impl crate::Writable for PSW_TMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PSW_TMR to value 0"]
impl crate::Resettable for PSW_TMR_SPEC {
    const RESET_VALUE: u8 = 0;
}
