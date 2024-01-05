#[doc = "Register `VTR` reader"]
pub type R = crate::R<VTR_SPEC>;
#[doc = "Register `VTR` writer"]
pub type W = crate::W<VTR_SPEC>;
#[doc = "Field `VL` reader - VLAN Tag Identifier"]
pub type VL_R = crate::FieldReader<u16>;
#[doc = "Field `VL` writer - VLAN Tag Identifier"]
pub type VL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ETV` reader - Enable 12-Bit VLAN Tag Comparison"]
pub type ETV_R = crate::BitReader;
#[doc = "Field `ETV` writer - Enable 12-Bit VLAN Tag Comparison"]
pub type ETV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - VLAN Tag Identifier"]
    #[inline(always)]
    pub fn vl(&self) -> VL_R {
        VL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison"]
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Tag Identifier"]
    #[inline(always)]
    #[must_use]
    pub fn vl(&mut self) -> VL_W<VTR_SPEC> {
        VL_W::new(self, 0)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison"]
    #[inline(always)]
    #[must_use]
    pub fn etv(&mut self) -> ETV_W<VTR_SPEC> {
        ETV_W::new(self, 16)
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
#[doc = "VLAN TAG Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VTR_SPEC;
impl crate::RegisterSpec for VTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vtr::R`](R) reader structure"]
impl crate::Readable for VTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vtr::W`](W) writer structure"]
impl crate::Writable for VTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VTR to value 0"]
impl crate::Resettable for VTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
