#[doc = "Register `STNUR` reader"]
pub type R = crate::R<STNUR_SPEC>;
#[doc = "Register `STNUR` writer"]
pub type W = crate::W<STNUR_SPEC>;
#[doc = "Field `TSSS` reader - Time Stamp Sub-Seconds"]
pub type TSSS_R = crate::FieldReader<u32>;
#[doc = "Field `TSSS` writer - Time Stamp Sub-Seconds"]
pub type TSSS_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `ADDSUB` reader - Add or Subtract Time"]
pub type ADDSUB_R = crate::BitReader;
#[doc = "Field `ADDSUB` writer - Add or Subtract Time"]
pub type ADDSUB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - Time Stamp Sub-Seconds"]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Add or Subtract Time"]
    #[inline(always)]
    pub fn addsub(&self) -> ADDSUB_R {
        ADDSUB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Time Stamp Sub-Seconds"]
    #[inline(always)]
    #[must_use]
    pub fn tsss(&mut self) -> TSSS_W<STNUR_SPEC> {
        TSSS_W::new(self, 0)
    }
    #[doc = "Bit 31 - Add or Subtract Time"]
    #[inline(always)]
    #[must_use]
    pub fn addsub(&mut self) -> ADDSUB_W<STNUR_SPEC> {
        ADDSUB_W::new(self, 31)
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
#[doc = "System Time - Nanoseconds Update Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stnur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stnur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STNUR_SPEC;
impl crate::RegisterSpec for STNUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stnur::R`](R) reader structure"]
impl crate::Readable for STNUR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stnur::W`](W) writer structure"]
impl crate::Writable for STNUR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STNUR to value 0"]
impl crate::Resettable for STNUR_SPEC {
    const RESET_VALUE: u32 = 0;
}
