#[doc = "Register `PPSCR` reader"]
pub type R = crate::R<PPSCR_SPEC>;
#[doc = "Register `PPSCR` writer"]
pub type W = crate::W<PPSCR_SPEC>;
#[doc = "Field `PPSCTRL` reader - Controls the frequency of the PPS output"]
pub type PPSCTRL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Controls the frequency of the PPS output"]
    #[inline(always)]
    pub fn ppsctrl(&self) -> PPSCTRL_R {
        PPSCTRL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
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
#[doc = "PPS Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PPSCR_SPEC;
impl crate::RegisterSpec for PPSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppscr::R`](R) reader structure"]
impl crate::Readable for PPSCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ppscr::W`](W) writer structure"]
impl crate::Writable for PPSCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PPSCR to value 0"]
impl crate::Resettable for PPSCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
