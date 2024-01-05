#[doc = "Register `USBEN0` reader"]
pub type R = crate::R<USBEN0_SPEC>;
#[doc = "Register `USBEN0` writer"]
pub type W = crate::W<USBEN0_SPEC>;
#[doc = "Field `USBEN0` reader - USB0 enable bit"]
pub type USBEN0_R = crate::BitReader;
#[doc = "Field `USBEN0` writer - USB0 enable bit"]
pub type USBEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB0 enable bit"]
    #[inline(always)]
    pub fn usben0(&self) -> USBEN0_R {
        USBEN0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB0 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn usben0(&mut self) -> USBEN0_W<USBEN0_SPEC> {
        USBEN0_W::new(self, 0)
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
#[doc = "USB0 Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usben0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usben0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBEN0_SPEC;
impl crate::RegisterSpec for USBEN0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usben0::R`](R) reader structure"]
impl crate::Readable for USBEN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usben0::W`](W) writer structure"]
impl crate::Writable for USBEN0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets USBEN0 to value 0"]
impl crate::Resettable for USBEN0_SPEC {
    const RESET_VALUE: u8 = 0;
}
