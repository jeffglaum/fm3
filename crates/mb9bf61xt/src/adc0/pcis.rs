#[doc = "Register `PCIS` reader"]
pub type R = crate::R<PCIS_SPEC>;
#[doc = "Register `PCIS` writer"]
pub type W = crate::W<PCIS_SPEC>;
#[doc = "Field `P1A` reader - Priority level 1 analog input selection"]
pub type P1A_R = crate::FieldReader;
#[doc = "Field `P1A` writer - Priority level 1 analog input selection"]
pub type P1A_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `P2A` reader - Priority level 2 analog input selection"]
pub type P2A_R = crate::FieldReader;
#[doc = "Field `P2A` writer - Priority level 2 analog input selection"]
pub type P2A_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:2 - Priority level 1 analog input selection"]
    #[inline(always)]
    pub fn p1a(&self) -> P1A_R {
        P1A_R::new(self.bits & 7)
    }
    #[doc = "Bits 3:7 - Priority level 2 analog input selection"]
    #[inline(always)]
    pub fn p2a(&self) -> P2A_R {
        P2A_R::new((self.bits >> 3) & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:2 - Priority level 1 analog input selection"]
    #[inline(always)]
    #[must_use]
    pub fn p1a(&mut self) -> P1A_W<PCIS_SPEC> {
        P1A_W::new(self, 0)
    }
    #[doc = "Bits 3:7 - Priority level 2 analog input selection"]
    #[inline(always)]
    #[must_use]
    pub fn p2a(&mut self) -> P2A_W<PCIS_SPEC> {
        P2A_W::new(self, 3)
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
#[doc = "Priority Conversion Input Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCIS_SPEC;
impl crate::RegisterSpec for PCIS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pcis::R`](R) reader structure"]
impl crate::Readable for PCIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcis::W`](W) writer structure"]
impl crate::Writable for PCIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PCIS to value 0"]
impl crate::Resettable for PCIS_SPEC {
    const RESET_VALUE: u8 = 0;
}
