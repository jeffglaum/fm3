#[doc = "Register `CSV_CTL` reader"]
pub type R = crate::R<CSV_CTL_SPEC>;
#[doc = "Register `CSV_CTL` writer"]
pub type W = crate::W<CSV_CTL_SPEC>;
#[doc = "Field `MCSVE` reader - Main CSV function enable bit"]
pub type MCSVE_R = crate::BitReader;
#[doc = "Field `MCSVE` writer - Main CSV function enable bit"]
pub type MCSVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCSVE` reader - Sub CSV function enable bit"]
pub type SCSVE_R = crate::BitReader;
#[doc = "Field `SCSVE` writer - Sub CSV function enable bit"]
pub type SCSVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCSDE` reader - FCS function enable bit"]
pub type FCSDE_R = crate::BitReader;
#[doc = "Field `FCSDE` writer - FCS function enable bit"]
pub type FCSDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCSRE` reader - FCS reset output enable bit"]
pub type FCSRE_R = crate::BitReader;
#[doc = "Field `FCSRE` writer - FCS reset output enable bit"]
pub type FCSRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCD` reader - FCS count cycle setting bits"]
pub type FCD_R = crate::FieldReader;
#[doc = "Field `FCD` writer - FCS count cycle setting bits"]
pub type FCD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Main CSV function enable bit"]
    #[inline(always)]
    pub fn mcsve(&self) -> MCSVE_R {
        MCSVE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sub CSV function enable bit"]
    #[inline(always)]
    pub fn scsve(&self) -> SCSVE_R {
        SCSVE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - FCS function enable bit"]
    #[inline(always)]
    pub fn fcsde(&self) -> FCSDE_R {
        FCSDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FCS reset output enable bit"]
    #[inline(always)]
    pub fn fcsre(&self) -> FCSRE_R {
        FCSRE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:14 - FCS count cycle setting bits"]
    #[inline(always)]
    pub fn fcd(&self) -> FCD_R {
        FCD_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Main CSV function enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn mcsve(&mut self) -> MCSVE_W<CSV_CTL_SPEC> {
        MCSVE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Sub CSV function enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn scsve(&mut self) -> SCSVE_W<CSV_CTL_SPEC> {
        SCSVE_W::new(self, 1)
    }
    #[doc = "Bit 8 - FCS function enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn fcsde(&mut self) -> FCSDE_W<CSV_CTL_SPEC> {
        FCSDE_W::new(self, 8)
    }
    #[doc = "Bit 9 - FCS reset output enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn fcsre(&mut self) -> FCSRE_W<CSV_CTL_SPEC> {
        FCSRE_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - FCS count cycle setting bits"]
    #[inline(always)]
    #[must_use]
    pub fn fcd(&mut self) -> FCD_W<CSV_CTL_SPEC> {
        FCD_W::new(self, 12)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CSV control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csv_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csv_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSV_CTL_SPEC;
impl crate::RegisterSpec for CSV_CTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csv_ctl::R`](R) reader structure"]
impl crate::Readable for CSV_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csv_ctl::W`](W) writer structure"]
impl crate::Writable for CSV_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CSV_CTL to value 0x7003"]
impl crate::Resettable for CSV_CTL_SPEC {
    const RESET_VALUE: u16 = 0x7003;
}
