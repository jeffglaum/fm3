#[doc = "Register `CSIO_SMR` reader"]
pub type R = crate::R<CSIO_CSIO_SMR_SPEC>;
#[doc = "Register `CSIO_SMR` writer"]
pub type W = crate::W<CSIO_CSIO_SMR_SPEC>;
#[doc = "Field `SOE` reader - Serial data output enable bit"]
pub type SOE_R = crate::BitReader;
#[doc = "Field `SOE` writer - Serial data output enable bit"]
pub type SOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCKE` reader - Master mode serial clock output enable bit"]
pub type SCKE_R = crate::BitReader;
#[doc = "Field `SCKE` writer - Master mode serial clock output enable bit"]
pub type SCKE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDS` reader - Transfer direction select bit"]
pub type BDS_R = crate::BitReader;
#[doc = "Field `BDS` writer - Transfer direction select bit"]
pub type BDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCINV` reader - Serial clock invert bit"]
pub type SCINV_R = crate::BitReader;
#[doc = "Field `SCINV` writer - Serial clock invert bit"]
pub type SCINV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUCR` reader - Wake-up control bit"]
pub type WUCR_R = crate::BitReader;
#[doc = "Field `WUCR` writer - Wake-up control bit"]
pub type WUCR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MD` reader - Operation mode set bits"]
pub type MD_R = crate::FieldReader;
#[doc = "Field `MD` writer - Operation mode set bits"]
pub type MD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Serial data output enable bit"]
    #[inline(always)]
    pub fn soe(&self) -> SOE_R {
        SOE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master mode serial clock output enable bit"]
    #[inline(always)]
    pub fn scke(&self) -> SCKE_R {
        SCKE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer direction select bit"]
    #[inline(always)]
    pub fn bds(&self) -> BDS_R {
        BDS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Serial clock invert bit"]
    #[inline(always)]
    pub fn scinv(&self) -> SCINV_R {
        SCINV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake-up control bit"]
    #[inline(always)]
    pub fn wucr(&self) -> WUCR_R {
        WUCR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Operation mode set bits"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bit 0 - Serial data output enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn soe(&mut self) -> SOE_W<CSIO_CSIO_SMR_SPEC> {
        SOE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Master mode serial clock output enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn scke(&mut self) -> SCKE_W<CSIO_CSIO_SMR_SPEC> {
        SCKE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transfer direction select bit"]
    #[inline(always)]
    #[must_use]
    pub fn bds(&mut self) -> BDS_W<CSIO_CSIO_SMR_SPEC> {
        BDS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Serial clock invert bit"]
    #[inline(always)]
    #[must_use]
    pub fn scinv(&mut self) -> SCINV_W<CSIO_CSIO_SMR_SPEC> {
        SCINV_W::new(self, 3)
    }
    #[doc = "Bit 4 - Wake-up control bit"]
    #[inline(always)]
    #[must_use]
    pub fn wucr(&mut self) -> WUCR_W<CSIO_CSIO_SMR_SPEC> {
        WUCR_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - Operation mode set bits"]
    #[inline(always)]
    #[must_use]
    pub fn md(&mut self) -> MD_W<CSIO_CSIO_SMR_SPEC> {
        MD_W::new(self, 5)
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
#[doc = "Serial Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csio_csio_smr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csio_csio_smr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIO_CSIO_SMR_SPEC;
impl crate::RegisterSpec for CSIO_CSIO_SMR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csio_csio_smr::R`](R) reader structure"]
impl crate::Readable for CSIO_CSIO_SMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csio_csio_smr::W`](W) writer structure"]
impl crate::Writable for CSIO_CSIO_SMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CSIO_SMR to value 0"]
impl crate::Resettable for CSIO_CSIO_SMR_SPEC {
    const RESET_VALUE: u8 = 0;
}
