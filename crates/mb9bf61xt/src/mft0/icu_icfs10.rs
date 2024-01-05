#[doc = "Register `ICU_ICFS10` reader"]
pub type R = crate::R<ICU_ICFS10_SPEC>;
#[doc = "Register `ICU_ICFS10` writer"]
pub type W = crate::W<ICU_ICFS10_SPEC>;
#[doc = "Field `FSI0` reader - Connects FRT ch.x to ICU ch.(0)"]
pub type FSI0_R = crate::FieldReader;
#[doc = "Field `FSI0` writer - Connects FRT ch.x to ICU ch.(0)"]
pub type FSI0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FSI1` reader - Connects FRT ch.x to ICU ch.(1)"]
pub type FSI1_R = crate::FieldReader;
#[doc = "Field `FSI1` writer - Connects FRT ch.x to ICU ch.(1)"]
pub type FSI1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Connects FRT ch.x to ICU ch.(0)"]
    #[inline(always)]
    pub fn fsi0(&self) -> FSI0_R {
        FSI0_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Connects FRT ch.x to ICU ch.(1)"]
    #[inline(always)]
    pub fn fsi1(&self) -> FSI1_R {
        FSI1_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Connects FRT ch.x to ICU ch.(0)"]
    #[inline(always)]
    #[must_use]
    pub fn fsi0(&mut self) -> FSI0_W<ICU_ICFS10_SPEC> {
        FSI0_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Connects FRT ch.x to ICU ch.(1)"]
    #[inline(always)]
    #[must_use]
    pub fn fsi1(&mut self) -> FSI1_W<ICU_ICFS10_SPEC> {
        FSI1_W::new(self, 4)
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
#[doc = "\"ICU ch.1,0 Connecting FRT Select Register\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icu_icfs10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icu_icfs10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICU_ICFS10_SPEC;
impl crate::RegisterSpec for ICU_ICFS10_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`icu_icfs10::R`](R) reader structure"]
impl crate::Readable for ICU_ICFS10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icu_icfs10::W`](W) writer structure"]
impl crate::Writable for ICU_ICFS10_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ICU_ICFS10 to value 0"]
impl crate::Resettable for ICU_ICFS10_SPEC {
    const RESET_VALUE: u8 = 0;
}
