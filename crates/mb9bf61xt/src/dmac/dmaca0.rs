#[doc = "Register `DMACA0` reader"]
pub type R = crate::R<DMACA0_SPEC>;
#[doc = "Register `DMACA0` writer"]
pub type W = crate::W<DMACA0_SPEC>;
#[doc = "Field `TC` reader - Transfer Count"]
pub type TC_R = crate::FieldReader<u16>;
#[doc = "Field `TC` writer - Transfer Count"]
pub type TC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BC` reader - Block Count"]
pub type BC_R = crate::FieldReader;
#[doc = "Field `BC` writer - Block Count"]
pub type BC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IS` reader - Input Select"]
pub type IS_R = crate::FieldReader;
#[doc = "Field `IS` writer - Input Select"]
pub type IS_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ST` reader - Software Trigger"]
pub type ST_R = crate::BitReader;
#[doc = "Field `ST` writer - Software Trigger"]
pub type ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB` reader - Pause bit (individual-channel pause bit)"]
pub type PB_R = crate::BitReader;
#[doc = "Field `PB` writer - Pause bit (individual-channel pause bit)"]
pub type PB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EB` reader - Enable bit (individual-channel operation enable bit)"]
pub type EB_R = crate::BitReader;
#[doc = "Field `EB` writer - Enable bit (individual-channel operation enable bit)"]
pub type EB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Transfer Count"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Block Count"]
    #[inline(always)]
    pub fn bc(&self) -> BC_R {
        BC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 23:28 - Input Select"]
    #[inline(always)]
    pub fn is(&self) -> IS_R {
        IS_R::new(((self.bits >> 23) & 0x3f) as u8)
    }
    #[doc = "Bit 29 - Software Trigger"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Pause bit (individual-channel pause bit)"]
    #[inline(always)]
    pub fn pb(&self) -> PB_R {
        PB_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable bit (individual-channel operation enable bit)"]
    #[inline(always)]
    pub fn eb(&self) -> EB_R {
        EB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transfer Count"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TC_W<DMACA0_SPEC> {
        TC_W::new(self, 0)
    }
    #[doc = "Bits 16:19 - Block Count"]
    #[inline(always)]
    #[must_use]
    pub fn bc(&mut self) -> BC_W<DMACA0_SPEC> {
        BC_W::new(self, 16)
    }
    #[doc = "Bits 23:28 - Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn is(&mut self) -> IS_W<DMACA0_SPEC> {
        IS_W::new(self, 23)
    }
    #[doc = "Bit 29 - Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<DMACA0_SPEC> {
        ST_W::new(self, 29)
    }
    #[doc = "Bit 30 - Pause bit (individual-channel pause bit)"]
    #[inline(always)]
    #[must_use]
    pub fn pb(&mut self) -> PB_W<DMACA0_SPEC> {
        PB_W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable bit (individual-channel operation enable bit)"]
    #[inline(always)]
    #[must_use]
    pub fn eb(&mut self) -> EB_W<DMACA0_SPEC> {
        EB_W::new(self, 31)
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
#[doc = "Configuration A Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaca0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaca0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACA0_SPEC;
impl crate::RegisterSpec for DMACA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaca0::R`](R) reader structure"]
impl crate::Readable for DMACA0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmaca0::W`](W) writer structure"]
impl crate::Writable for DMACA0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACA0 to value 0"]
impl crate::Resettable for DMACA0_SPEC {
    const RESET_VALUE: u32 = 0;
}
