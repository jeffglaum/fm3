#[doc = "Register `OCU_OCFS10` reader"]
pub type R = crate::R<OCU_OCFS10_SPEC>;
#[doc = "Register `OCU_OCFS10` writer"]
pub type W = crate::W<OCU_OCFS10_SPEC>;
#[doc = "Field `FSO0` reader - Connects FRT ch.x to OCU ch.0"]
pub type FSO0_R = crate::FieldReader;
#[doc = "Field `FSO0` writer - Connects FRT ch.x to OCU ch.0"]
pub type FSO0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FSO1` reader - Connects FRT ch.x to OCU ch.1"]
pub type FSO1_R = crate::FieldReader;
#[doc = "Field `FSO1` writer - Connects FRT ch.x to OCU ch.1"]
pub type FSO1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Connects FRT ch.x to OCU ch.0"]
    #[inline(always)]
    pub fn fso0(&self) -> FSO0_R {
        FSO0_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Connects FRT ch.x to OCU ch.1"]
    #[inline(always)]
    pub fn fso1(&self) -> FSO1_R {
        FSO1_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Connects FRT ch.x to OCU ch.0"]
    #[inline(always)]
    #[must_use]
    pub fn fso0(&mut self) -> FSO0_W<OCU_OCFS10_SPEC> {
        FSO0_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Connects FRT ch.x to OCU ch.1"]
    #[inline(always)]
    #[must_use]
    pub fn fso1(&mut self) -> FSO1_W<OCU_OCFS10_SPEC> {
        FSO1_W::new(self, 4)
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
#[doc = "\"OCU ch.1,0 Connecting FRT Select Register\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocu_ocfs10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocu_ocfs10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCU_OCFS10_SPEC;
impl crate::RegisterSpec for OCU_OCFS10_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ocu_ocfs10::R`](R) reader structure"]
impl crate::Readable for OCU_OCFS10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ocu_ocfs10::W`](W) writer structure"]
impl crate::Writable for OCU_OCFS10_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OCU_OCFS10 to value 0"]
impl crate::Resettable for OCU_OCFS10_SPEC {
    const RESET_VALUE: u8 = 0;
}
