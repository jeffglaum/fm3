#[doc = "Register `PPGC0` reader"]
pub type R = crate::R<PPGC0_SPEC>;
#[doc = "Register `PPGC0` writer"]
pub type W = crate::W<PPGC0_SPEC>;
#[doc = "Field `TTRG` reader - PPG start trigger select bit"]
pub type TTRG_R = crate::BitReader;
#[doc = "Field `TTRG` writer - PPG start trigger select bit"]
pub type TTRG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MD` reader - PPG Operation Mode Set bits"]
pub type MD_R = crate::FieldReader;
#[doc = "Field `MD` writer - PPG Operation Mode Set bits"]
pub type MD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCS` reader - PPG DOWN Counter Operation Clock Select bits"]
pub type PCS_R = crate::FieldReader;
#[doc = "Field `PCS` writer - PPG DOWN Counter Operation Clock Select bits"]
pub type PCS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INTM` reader - Interrupt Mode Select bit"]
pub type INTM_R = crate::BitReader;
#[doc = "Field `INTM` writer - Interrupt Mode Select bit"]
pub type INTM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUF` reader - PPG Counter Underflow bit"]
pub type PUF_R = crate::BitReader;
#[doc = "Field `PUF` writer - PPG Counter Underflow bit"]
pub type PUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIE` reader - PPG Interrupt Enable bit"]
pub type PIE_R = crate::BitReader;
#[doc = "Field `PIE` writer - PPG Interrupt Enable bit"]
pub type PIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PPG start trigger select bit"]
    #[inline(always)]
    pub fn ttrg(&self) -> TTRG_R {
        TTRG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - PPG Operation Mode Set bits"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bits 3:4 - PPG DOWN Counter Operation Clock Select bits"]
    #[inline(always)]
    pub fn pcs(&self) -> PCS_R {
        PCS_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 5 - Interrupt Mode Select bit"]
    #[inline(always)]
    pub fn intm(&self) -> INTM_R {
        INTM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PPG Counter Underflow bit"]
    #[inline(always)]
    pub fn puf(&self) -> PUF_R {
        PUF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PPG Interrupt Enable bit"]
    #[inline(always)]
    pub fn pie(&self) -> PIE_R {
        PIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PPG start trigger select bit"]
    #[inline(always)]
    #[must_use]
    pub fn ttrg(&mut self) -> TTRG_W<PPGC0_SPEC> {
        TTRG_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - PPG Operation Mode Set bits"]
    #[inline(always)]
    #[must_use]
    pub fn md(&mut self) -> MD_W<PPGC0_SPEC> {
        MD_W::new(self, 1)
    }
    #[doc = "Bits 3:4 - PPG DOWN Counter Operation Clock Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn pcs(&mut self) -> PCS_W<PPGC0_SPEC> {
        PCS_W::new(self, 3)
    }
    #[doc = "Bit 5 - Interrupt Mode Select bit"]
    #[inline(always)]
    #[must_use]
    pub fn intm(&mut self) -> INTM_W<PPGC0_SPEC> {
        INTM_W::new(self, 5)
    }
    #[doc = "Bit 6 - PPG Counter Underflow bit"]
    #[inline(always)]
    #[must_use]
    pub fn puf(&mut self) -> PUF_W<PPGC0_SPEC> {
        PUF_W::new(self, 6)
    }
    #[doc = "Bit 7 - PPG Interrupt Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pie(&mut self) -> PIE_W<PPGC0_SPEC> {
        PIE_W::new(self, 7)
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
#[doc = "PPG Operation Mode Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppgc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppgc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PPGC0_SPEC;
impl crate::RegisterSpec for PPGC0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ppgc0::R`](R) reader structure"]
impl crate::Readable for PPGC0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ppgc0::W`](W) writer structure"]
impl crate::Writable for PPGC0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PPGC0 to value 0"]
impl crate::Resettable for PPGC0_SPEC {
    const RESET_VALUE: u8 = 0;
}
