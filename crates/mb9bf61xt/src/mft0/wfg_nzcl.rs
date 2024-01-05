#[doc = "Register `WFG_NZCL` reader"]
pub type R = crate::R<WFG_NZCL_SPEC>;
#[doc = "Register `WFG_NZCL` writer"]
pub type W = crate::W<WFG_NZCL_SPEC>;
#[doc = "Field `DTIE` reader - DTIF interrupt enable"]
pub type DTIE_R = crate::BitReader;
#[doc = "Field `DTIE` writer - DTIF interrupt enable"]
pub type DTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NWS` reader - noise-canceling width of the noise-canceller for the DTTIX pin"]
pub type NWS_R = crate::FieldReader;
#[doc = "Field `NWS` writer - noise-canceling width of the noise-canceller for the DTTIX pin"]
pub type NWS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SDTI` writer - Forcibly generates DTIF interrupt"]
pub type SDTI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DTIF interrupt enable"]
    #[inline(always)]
    pub fn dtie(&self) -> DTIE_R {
        DTIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - noise-canceling width of the noise-canceller for the DTTIX pin"]
    #[inline(always)]
    pub fn nws(&self) -> NWS_R {
        NWS_R::new(((self.bits >> 1) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DTIF interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtie(&mut self) -> DTIE_W<WFG_NZCL_SPEC> {
        DTIE_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - noise-canceling width of the noise-canceller for the DTTIX pin"]
    #[inline(always)]
    #[must_use]
    pub fn nws(&mut self) -> NWS_W<WFG_NZCL_SPEC> {
        NWS_W::new(self, 1)
    }
    #[doc = "Bit 4 - Forcibly generates DTIF interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sdti(&mut self) -> SDTI_W<WFG_NZCL_SPEC> {
        SDTI_W::new(self, 4)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "NZCL Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wfg_nzcl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wfg_nzcl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WFG_NZCL_SPEC;
impl crate::RegisterSpec for WFG_NZCL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`wfg_nzcl::R`](R) reader structure"]
impl crate::Readable for WFG_NZCL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wfg_nzcl::W`](W) writer structure"]
impl crate::Writable for WFG_NZCL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets WFG_NZCL to value 0"]
impl crate::Resettable for WFG_NZCL_SPEC {
    const RESET_VALUE: u16 = 0;
}
