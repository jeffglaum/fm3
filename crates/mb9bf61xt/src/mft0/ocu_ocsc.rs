#[doc = "Register `OCU_OCSC` reader"]
pub type R = crate::R<OCU_OCSC_SPEC>;
#[doc = "Register `OCU_OCSC` writer"]
pub type W = crate::W<OCU_OCSC_SPEC>;
#[doc = "Field `MOD0` reader - OCSC.MOD0 and OCSC.MOD1 determine the operation mode of OCU ch.0/ch.1 in combination with OCSB10.CMOD"]
pub type MOD0_R = crate::BitReader;
#[doc = "Field `MOD0` writer - OCSC.MOD0 and OCSC.MOD1 determine the operation mode of OCU ch.0/ch.1 in combination with OCSB10.CMOD"]
pub type MOD0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOD1` reader - OCSC.MOD0 and OCSC.MOD1 determine the operation mode of OCU ch.0/ch.1 in combination with OCSB10.CMOD"]
pub type MOD1_R = crate::BitReader;
#[doc = "Field `MOD1` writer - OCSC.MOD0 and OCSC.MOD1 determine the operation mode of OCU ch.0/ch.1 in combination with OCSB10.CMOD"]
pub type MOD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOD2` reader - OCSC.MOD2 and OCSC.MOD3 determine the operation mode of OCU ch.2/ch.3 in combination with OCSB32.CMOD"]
pub type MOD2_R = crate::BitReader;
#[doc = "Field `MOD2` writer - OCSC.MOD2 and OCSC.MOD3 determine the operation mode of OCU ch.2/ch.3 in combination with OCSB32.CMOD"]
pub type MOD2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOD3` reader - OCSC.MOD2 and OCSC.MOD3 determine the operation mode of OCU ch.2/ch.3 in combination with OCSB32.CMOD"]
pub type MOD3_R = crate::BitReader;
#[doc = "Field `MOD3` writer - OCSC.MOD2 and OCSC.MOD3 determine the operation mode of OCU ch.2/ch.3 in combination with OCSB32.CMOD"]
pub type MOD3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOD4` reader - OCSC.MOD4 and OCSC.MOD5 determine the operation mode of OCU ch.4/ch.5 in combination with OCSB54.CMOD"]
pub type MOD4_R = crate::BitReader;
#[doc = "Field `MOD4` writer - OCSC.MOD4 and OCSC.MOD5 determine the operation mode of OCU ch.4/ch.5 in combination with OCSB54.CMOD"]
pub type MOD4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOD5` reader - OCSC.MOD4 and OCSC.MOD5 determine the operation mode of OCU ch.4/ch.5 in combination with OCSB54.CMOD"]
pub type MOD5_R = crate::BitReader;
#[doc = "Field `MOD5` writer - OCSC.MOD4 and OCSC.MOD5 determine the operation mode of OCU ch.4/ch.5 in combination with OCSB54.CMOD"]
pub type MOD5_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - OCSC.MOD0 and OCSC.MOD1 determine the operation mode of OCU ch.0/ch.1 in combination with OCSB10.CMOD"]
    #[inline(always)]
    pub fn mod0(&self) -> MOD0_R {
        MOD0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OCSC.MOD0 and OCSC.MOD1 determine the operation mode of OCU ch.0/ch.1 in combination with OCSB10.CMOD"]
    #[inline(always)]
    pub fn mod1(&self) -> MOD1_R {
        MOD1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OCSC.MOD2 and OCSC.MOD3 determine the operation mode of OCU ch.2/ch.3 in combination with OCSB32.CMOD"]
    #[inline(always)]
    pub fn mod2(&self) -> MOD2_R {
        MOD2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OCSC.MOD2 and OCSC.MOD3 determine the operation mode of OCU ch.2/ch.3 in combination with OCSB32.CMOD"]
    #[inline(always)]
    pub fn mod3(&self) -> MOD3_R {
        MOD3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - OCSC.MOD4 and OCSC.MOD5 determine the operation mode of OCU ch.4/ch.5 in combination with OCSB54.CMOD"]
    #[inline(always)]
    pub fn mod4(&self) -> MOD4_R {
        MOD4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - OCSC.MOD4 and OCSC.MOD5 determine the operation mode of OCU ch.4/ch.5 in combination with OCSB54.CMOD"]
    #[inline(always)]
    pub fn mod5(&self) -> MOD5_R {
        MOD5_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - OCSC.MOD0 and OCSC.MOD1 determine the operation mode of OCU ch.0/ch.1 in combination with OCSB10.CMOD"]
    #[inline(always)]
    #[must_use]
    pub fn mod0(&mut self) -> MOD0_W<OCU_OCSC_SPEC> {
        MOD0_W::new(self, 8)
    }
    #[doc = "Bit 9 - OCSC.MOD0 and OCSC.MOD1 determine the operation mode of OCU ch.0/ch.1 in combination with OCSB10.CMOD"]
    #[inline(always)]
    #[must_use]
    pub fn mod1(&mut self) -> MOD1_W<OCU_OCSC_SPEC> {
        MOD1_W::new(self, 9)
    }
    #[doc = "Bit 10 - OCSC.MOD2 and OCSC.MOD3 determine the operation mode of OCU ch.2/ch.3 in combination with OCSB32.CMOD"]
    #[inline(always)]
    #[must_use]
    pub fn mod2(&mut self) -> MOD2_W<OCU_OCSC_SPEC> {
        MOD2_W::new(self, 10)
    }
    #[doc = "Bit 11 - OCSC.MOD2 and OCSC.MOD3 determine the operation mode of OCU ch.2/ch.3 in combination with OCSB32.CMOD"]
    #[inline(always)]
    #[must_use]
    pub fn mod3(&mut self) -> MOD3_W<OCU_OCSC_SPEC> {
        MOD3_W::new(self, 11)
    }
    #[doc = "Bit 12 - OCSC.MOD4 and OCSC.MOD5 determine the operation mode of OCU ch.4/ch.5 in combination with OCSB54.CMOD"]
    #[inline(always)]
    #[must_use]
    pub fn mod4(&mut self) -> MOD4_W<OCU_OCSC_SPEC> {
        MOD4_W::new(self, 12)
    }
    #[doc = "Bit 13 - OCSC.MOD4 and OCSC.MOD5 determine the operation mode of OCU ch.4/ch.5 in combination with OCSB54.CMOD"]
    #[inline(always)]
    #[must_use]
    pub fn mod5(&mut self) -> MOD5_W<OCU_OCSC_SPEC> {
        MOD5_W::new(self, 13)
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
#[doc = "OCU Control Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocu_ocsc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocu_ocsc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCU_OCSC_SPEC;
impl crate::RegisterSpec for OCU_OCSC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ocu_ocsc::R`](R) reader structure"]
impl crate::Readable for OCU_OCSC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ocu_ocsc::W`](W) writer structure"]
impl crate::Writable for OCU_OCSC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets OCU_OCSC to value 0"]
impl crate::Resettable for OCU_OCSC_SPEC {
    const RESET_VALUE: u16 = 0;
}
