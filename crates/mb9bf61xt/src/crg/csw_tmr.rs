#[doc = "Register `CSW_TMR` reader"]
pub type R = crate::R<CSW_TMR_SPEC>;
#[doc = "Register `CSW_TMR` writer"]
pub type W = crate::W<CSW_TMR_SPEC>;
#[doc = "Field `MOWT` reader - Main clock stabilization wait time setup bit"]
pub type MOWT_R = crate::FieldReader;
#[doc = "Field `MOWT` writer - Main clock stabilization wait time setup bit"]
pub type MOWT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SOWT` reader - Sub clock stabilization wait time setup bit"]
pub type SOWT_R = crate::FieldReader;
#[doc = "Field `SOWT` writer - Sub clock stabilization wait time setup bit"]
pub type SOWT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - Main clock stabilization wait time setup bit"]
    #[inline(always)]
    pub fn mowt(&self) -> MOWT_R {
        MOWT_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - Sub clock stabilization wait time setup bit"]
    #[inline(always)]
    pub fn sowt(&self) -> SOWT_R {
        SOWT_R::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:3 - Main clock stabilization wait time setup bit"]
    #[inline(always)]
    #[must_use]
    pub fn mowt(&mut self) -> MOWT_W<CSW_TMR_SPEC> {
        MOWT_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Sub clock stabilization wait time setup bit"]
    #[inline(always)]
    #[must_use]
    pub fn sowt(&mut self) -> SOWT_W<CSW_TMR_SPEC> {
        SOWT_W::new(self, 4)
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
#[doc = "Clock Stabilization Wait Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csw_tmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csw_tmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSW_TMR_SPEC;
impl crate::RegisterSpec for CSW_TMR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csw_tmr::R`](R) reader structure"]
impl crate::Readable for CSW_TMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csw_tmr::W`](W) writer structure"]
impl crate::Writable for CSW_TMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CSW_TMR to value 0"]
impl crate::Resettable for CSW_TMR_SPEC {
    const RESET_VALUE: u8 = 0;
}
