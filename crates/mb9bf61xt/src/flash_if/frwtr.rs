#[doc = "Register `FRWTR` reader"]
pub type R = crate::R<FRWTR_SPEC>;
#[doc = "Register `FRWTR` writer"]
pub type W = crate::W<FRWTR_SPEC>;
#[doc = "Field `RWT` reader - Read Wait Cycle"]
pub type RWT_R = crate::FieldReader;
#[doc = "Field `RWT` writer - Read Wait Cycle"]
pub type RWT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Read Wait Cycle"]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Read Wait Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn rwt(&mut self) -> RWT_W<FRWTR_SPEC> {
        RWT_W::new(self, 0)
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
#[doc = "Flash Read Wait Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frwtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frwtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRWTR_SPEC;
impl crate::RegisterSpec for FRWTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frwtr::R`](R) reader structure"]
impl crate::Readable for FRWTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`frwtr::W`](W) writer structure"]
impl crate::Writable for FRWTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRWTR to value 0x02"]
impl crate::Resettable for FRWTR_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
