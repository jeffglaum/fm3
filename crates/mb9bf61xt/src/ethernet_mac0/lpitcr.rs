#[doc = "Register `LPITCR` reader"]
pub type R = crate::R<LPITCR_SPEC>;
#[doc = "Register `LPITCR` writer"]
pub type W = crate::W<LPITCR_SPEC>;
#[doc = "Field `TWT` reader - LPI TW TIMER"]
pub type TWT_R = crate::FieldReader<u16>;
#[doc = "Field `TWT` writer - LPI TW TIMER"]
pub type TWT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LIT` reader - LPI LS TIMER"]
pub type LIT_R = crate::FieldReader<u16>;
#[doc = "Field `LIT` writer - LPI LS TIMER"]
pub type LIT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:15 - LPI TW TIMER"]
    #[inline(always)]
    pub fn twt(&self) -> TWT_R {
        TWT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - LPI LS TIMER"]
    #[inline(always)]
    pub fn lit(&self) -> LIT_R {
        LIT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - LPI TW TIMER"]
    #[inline(always)]
    #[must_use]
    pub fn twt(&mut self) -> TWT_W<LPITCR_SPEC> {
        TWT_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - LPI LS TIMER"]
    #[inline(always)]
    #[must_use]
    pub fn lit(&mut self) -> LIT_W<LPITCR_SPEC> {
        LIT_W::new(self, 16)
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
#[doc = "LPI Timers Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpitcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpitcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPITCR_SPEC;
impl crate::RegisterSpec for LPITCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpitcr::R`](R) reader structure"]
impl crate::Readable for LPITCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpitcr::W`](W) writer structure"]
impl crate::Writable for LPITCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPITCR to value 0x03e8_0000"]
impl crate::Resettable for LPITCR_SPEC {
    const RESET_VALUE: u32 = 0x03e8_0000;
}
