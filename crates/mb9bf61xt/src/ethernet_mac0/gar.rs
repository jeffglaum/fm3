#[doc = "Register `GAR` reader"]
pub type R = crate::R<GAR_SPEC>;
#[doc = "Register `GAR` writer"]
pub type W = crate::W<GAR_SPEC>;
#[doc = "Field `GB` reader - GMII/MII Busy"]
pub type GB_R = crate::BitReader;
#[doc = "Field `GB` writer - GMII/MII Busy"]
pub type GB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GW` reader - GMII/MII Write"]
pub type GW_R = crate::BitReader;
#[doc = "Field `GW` writer - GMII/MII Write"]
pub type GW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR` reader - Application Clock Range"]
pub type CR_R = crate::FieldReader;
#[doc = "Field `CR` writer - Application Clock Range"]
pub type CR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GR` reader - GMII Register"]
pub type GR_R = crate::FieldReader;
#[doc = "Field `GR` writer - GMII Register"]
pub type GR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PA` reader - Physical Layer Address"]
pub type PA_R = crate::FieldReader;
#[doc = "Field `PA` writer - Physical Layer Address"]
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - GMII/MII Busy"]
    #[inline(always)]
    pub fn gb(&self) -> GB_R {
        GB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GMII/MII Write"]
    #[inline(always)]
    pub fn gw(&self) -> GW_R {
        GW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - Application Clock Range"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:10 - GMII Register"]
    #[inline(always)]
    pub fn gr(&self) -> GR_R {
        GR_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Physical Layer Address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - GMII/MII Busy"]
    #[inline(always)]
    #[must_use]
    pub fn gb(&mut self) -> GB_W<GAR_SPEC> {
        GB_W::new(self, 0)
    }
    #[doc = "Bit 1 - GMII/MII Write"]
    #[inline(always)]
    #[must_use]
    pub fn gw(&mut self) -> GW_W<GAR_SPEC> {
        GW_W::new(self, 1)
    }
    #[doc = "Bits 2:5 - Application Clock Range"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<GAR_SPEC> {
        CR_W::new(self, 2)
    }
    #[doc = "Bits 6:10 - GMII Register"]
    #[inline(always)]
    #[must_use]
    pub fn gr(&mut self) -> GR_W<GAR_SPEC> {
        GR_W::new(self, 6)
    }
    #[doc = "Bits 11:15 - Physical Layer Address"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<GAR_SPEC> {
        PA_W::new(self, 11)
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
#[doc = "GMII/MII Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GAR_SPEC;
impl crate::RegisterSpec for GAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gar::R`](R) reader structure"]
impl crate::Readable for GAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gar::W`](W) writer structure"]
impl crate::Writable for GAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GAR to value 0"]
impl crate::Resettable for GAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
