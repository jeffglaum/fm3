#[doc = "Register `TIM0` reader"]
pub type R = crate::R<TIM0_SPEC>;
#[doc = "Register `TIM0` writer"]
pub type W = crate::W<TIM0_SPEC>;
#[doc = "Field `RACC` reader - Read Access Cycle"]
pub type RACC_R = crate::FieldReader;
#[doc = "Field `RACC` writer - Read Access Cycle"]
pub type RACC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RADC` reader - Read Address Setup cycle"]
pub type RADC_R = crate::FieldReader;
#[doc = "Field `RADC` writer - Read Address Setup cycle"]
pub type RADC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FRADC` reader - First Read Address Cycle"]
pub type FRADC_R = crate::FieldReader;
#[doc = "Field `FRADC` writer - First Read Address Cycle"]
pub type FRADC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RIDLC` reader - Read Idle Cycle"]
pub type RIDLC_R = crate::FieldReader;
#[doc = "Field `RIDLC` writer - Read Idle Cycle"]
pub type RIDLC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WACC` reader - Write Access Cycle"]
pub type WACC_R = crate::FieldReader;
#[doc = "Field `WACC` writer - Write Access Cycle"]
pub type WACC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WADC` reader - Write Address Setup cycle"]
pub type WADC_R = crate::FieldReader;
#[doc = "Field `WADC` writer - Write Address Setup cycle"]
pub type WADC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WWEC` reader - Write Enable Cycle"]
pub type WWEC_R = crate::FieldReader;
#[doc = "Field `WWEC` writer - Write Enable Cycle"]
pub type WWEC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WIDLC` reader - Write Idle Cycle"]
pub type WIDLC_R = crate::FieldReader;
#[doc = "Field `WIDLC` writer - Write Idle Cycle"]
pub type WIDLC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Read Access Cycle"]
    #[inline(always)]
    pub fn racc(&self) -> RACC_R {
        RACC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Read Address Setup cycle"]
    #[inline(always)]
    pub fn radc(&self) -> RADC_R {
        RADC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - First Read Address Cycle"]
    #[inline(always)]
    pub fn fradc(&self) -> FRADC_R {
        FRADC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Read Idle Cycle"]
    #[inline(always)]
    pub fn ridlc(&self) -> RIDLC_R {
        RIDLC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Write Access Cycle"]
    #[inline(always)]
    pub fn wacc(&self) -> WACC_R {
        WACC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Write Address Setup cycle"]
    #[inline(always)]
    pub fn wadc(&self) -> WADC_R {
        WADC_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Write Enable Cycle"]
    #[inline(always)]
    pub fn wwec(&self) -> WWEC_R {
        WWEC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Write Idle Cycle"]
    #[inline(always)]
    pub fn widlc(&self) -> WIDLC_R {
        WIDLC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Read Access Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn racc(&mut self) -> RACC_W<TIM0_SPEC> {
        RACC_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Read Address Setup cycle"]
    #[inline(always)]
    #[must_use]
    pub fn radc(&mut self) -> RADC_W<TIM0_SPEC> {
        RADC_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - First Read Address Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn fradc(&mut self) -> FRADC_W<TIM0_SPEC> {
        FRADC_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Read Idle Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn ridlc(&mut self) -> RIDLC_W<TIM0_SPEC> {
        RIDLC_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Write Access Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn wacc(&mut self) -> WACC_W<TIM0_SPEC> {
        WACC_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Write Address Setup cycle"]
    #[inline(always)]
    #[must_use]
    pub fn wadc(&mut self) -> WADC_W<TIM0_SPEC> {
        WADC_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Write Enable Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn wwec(&mut self) -> WWEC_W<TIM0_SPEC> {
        WWEC_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Write Idle Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn widlc(&mut self) -> WIDLC_W<TIM0_SPEC> {
        WIDLC_W::new(self, 28)
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
#[doc = "Timing Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM0_SPEC;
impl crate::RegisterSpec for TIM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim0::R`](R) reader structure"]
impl crate::Readable for TIM0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tim0::W`](W) writer structure"]
impl crate::Writable for TIM0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM0 to value 0x055f_f00f"]
impl crate::Resettable for TIM0_SPEC {
    const RESET_VALUE: u32 = 0x055f_f00f;
}
