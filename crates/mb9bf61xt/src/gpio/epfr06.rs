#[doc = "Register `EPFR06` reader"]
pub type R = crate::R<EPFR06_SPEC>;
#[doc = "Register `EPFR06` writer"]
pub type W = crate::W<EPFR06_SPEC>;
#[doc = "Field `EINT00S` reader - External interrupt 0 input select bit"]
pub type EINT00S_R = crate::FieldReader;
#[doc = "Field `EINT00S` writer - External interrupt 0 input select bit"]
pub type EINT00S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT01S` reader - External interrupt 1 input select bit"]
pub type EINT01S_R = crate::FieldReader;
#[doc = "Field `EINT01S` writer - External interrupt 1 input select bit"]
pub type EINT01S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT02S` reader - External interrupt 2 input select bit"]
pub type EINT02S_R = crate::FieldReader;
#[doc = "Field `EINT02S` writer - External interrupt 2 input select bit"]
pub type EINT02S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT03S` reader - External interrupt 3 input select bit"]
pub type EINT03S_R = crate::FieldReader;
#[doc = "Field `EINT03S` writer - External interrupt 3 input select bit"]
pub type EINT03S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT04S` reader - External interrupt 4 input select bit"]
pub type EINT04S_R = crate::FieldReader;
#[doc = "Field `EINT04S` writer - External interrupt 4 input select bit"]
pub type EINT04S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT05S` reader - External interrupt 5 input select bit"]
pub type EINT05S_R = crate::FieldReader;
#[doc = "Field `EINT05S` writer - External interrupt 5 input select bit"]
pub type EINT05S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT06S` reader - External interrupt 6 input select bit"]
pub type EINT06S_R = crate::FieldReader;
#[doc = "Field `EINT06S` writer - External interrupt 6 input select bit"]
pub type EINT06S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT07S` reader - External interrupt 7 input select bit"]
pub type EINT07S_R = crate::FieldReader;
#[doc = "Field `EINT07S` writer - External interrupt 7 input select bit"]
pub type EINT07S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT08S` reader - External interrupt 8 input select bit"]
pub type EINT08S_R = crate::FieldReader;
#[doc = "Field `EINT08S` writer - External interrupt 8 input select bit"]
pub type EINT08S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT09S` reader - External interrupt 9 input select bit"]
pub type EINT09S_R = crate::FieldReader;
#[doc = "Field `EINT09S` writer - External interrupt 9 input select bit"]
pub type EINT09S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT10S` reader - External interrupt 10 input select bit"]
pub type EINT10S_R = crate::FieldReader;
#[doc = "Field `EINT10S` writer - External interrupt 10 input select bit"]
pub type EINT10S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT11S` reader - External interrupt 11 input select bit"]
pub type EINT11S_R = crate::FieldReader;
#[doc = "Field `EINT11S` writer - External interrupt 11 input select bit"]
pub type EINT11S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT12S` reader - External interrupt 12 input select bit"]
pub type EINT12S_R = crate::FieldReader;
#[doc = "Field `EINT12S` writer - External interrupt 12 input select bit"]
pub type EINT12S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT13S` reader - External interrupt 13 input select bit"]
pub type EINT13S_R = crate::FieldReader;
#[doc = "Field `EINT13S` writer - External interrupt 13 input select bit"]
pub type EINT13S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT14S` reader - External interrupt 14 input select bit"]
pub type EINT14S_R = crate::FieldReader;
#[doc = "Field `EINT14S` writer - External interrupt 14 input select bit"]
pub type EINT14S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT15S` reader - External interrupt 15 input select bit"]
pub type EINT15S_R = crate::FieldReader;
#[doc = "Field `EINT15S` writer - External interrupt 15 input select bit"]
pub type EINT15S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - External interrupt 0 input select bit"]
    #[inline(always)]
    pub fn eint00s(&self) -> EINT00S_R {
        EINT00S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - External interrupt 1 input select bit"]
    #[inline(always)]
    pub fn eint01s(&self) -> EINT01S_R {
        EINT01S_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - External interrupt 2 input select bit"]
    #[inline(always)]
    pub fn eint02s(&self) -> EINT02S_R {
        EINT02S_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - External interrupt 3 input select bit"]
    #[inline(always)]
    pub fn eint03s(&self) -> EINT03S_R {
        EINT03S_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - External interrupt 4 input select bit"]
    #[inline(always)]
    pub fn eint04s(&self) -> EINT04S_R {
        EINT04S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - External interrupt 5 input select bit"]
    #[inline(always)]
    pub fn eint05s(&self) -> EINT05S_R {
        EINT05S_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - External interrupt 6 input select bit"]
    #[inline(always)]
    pub fn eint06s(&self) -> EINT06S_R {
        EINT06S_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - External interrupt 7 input select bit"]
    #[inline(always)]
    pub fn eint07s(&self) -> EINT07S_R {
        EINT07S_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - External interrupt 8 input select bit"]
    #[inline(always)]
    pub fn eint08s(&self) -> EINT08S_R {
        EINT08S_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - External interrupt 9 input select bit"]
    #[inline(always)]
    pub fn eint09s(&self) -> EINT09S_R {
        EINT09S_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - External interrupt 10 input select bit"]
    #[inline(always)]
    pub fn eint10s(&self) -> EINT10S_R {
        EINT10S_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - External interrupt 11 input select bit"]
    #[inline(always)]
    pub fn eint11s(&self) -> EINT11S_R {
        EINT11S_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - External interrupt 12 input select bit"]
    #[inline(always)]
    pub fn eint12s(&self) -> EINT12S_R {
        EINT12S_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - External interrupt 13 input select bit"]
    #[inline(always)]
    pub fn eint13s(&self) -> EINT13S_R {
        EINT13S_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - External interrupt 14 input select bit"]
    #[inline(always)]
    pub fn eint14s(&self) -> EINT14S_R {
        EINT14S_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - External interrupt 15 input select bit"]
    #[inline(always)]
    pub fn eint15s(&self) -> EINT15S_R {
        EINT15S_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External interrupt 0 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint00s(&mut self) -> EINT00S_W<EPFR06_SPEC> {
        EINT00S_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - External interrupt 1 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint01s(&mut self) -> EINT01S_W<EPFR06_SPEC> {
        EINT01S_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - External interrupt 2 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint02s(&mut self) -> EINT02S_W<EPFR06_SPEC> {
        EINT02S_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - External interrupt 3 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint03s(&mut self) -> EINT03S_W<EPFR06_SPEC> {
        EINT03S_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - External interrupt 4 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint04s(&mut self) -> EINT04S_W<EPFR06_SPEC> {
        EINT04S_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - External interrupt 5 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint05s(&mut self) -> EINT05S_W<EPFR06_SPEC> {
        EINT05S_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - External interrupt 6 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint06s(&mut self) -> EINT06S_W<EPFR06_SPEC> {
        EINT06S_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - External interrupt 7 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint07s(&mut self) -> EINT07S_W<EPFR06_SPEC> {
        EINT07S_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - External interrupt 8 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint08s(&mut self) -> EINT08S_W<EPFR06_SPEC> {
        EINT08S_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - External interrupt 9 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint09s(&mut self) -> EINT09S_W<EPFR06_SPEC> {
        EINT09S_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - External interrupt 10 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint10s(&mut self) -> EINT10S_W<EPFR06_SPEC> {
        EINT10S_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - External interrupt 11 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint11s(&mut self) -> EINT11S_W<EPFR06_SPEC> {
        EINT11S_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - External interrupt 12 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint12s(&mut self) -> EINT12S_W<EPFR06_SPEC> {
        EINT12S_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - External interrupt 13 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint13s(&mut self) -> EINT13S_W<EPFR06_SPEC> {
        EINT13S_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - External interrupt 14 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint14s(&mut self) -> EINT14S_W<EPFR06_SPEC> {
        EINT14S_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - External interrupt 15 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint15s(&mut self) -> EINT15S_W<EPFR06_SPEC> {
        EINT15S_W::new(self, 30)
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
#[doc = "Extended pin function setting register 06\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr06::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr06::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPFR06_SPEC;
impl crate::RegisterSpec for EPFR06_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epfr06::R`](R) reader structure"]
impl crate::Readable for EPFR06_SPEC {}
#[doc = "`write(|w| ..)` method takes [`epfr06::W`](W) writer structure"]
impl crate::Writable for EPFR06_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EPFR06 to value 0"]
impl crate::Resettable for EPFR06_SPEC {
    const RESET_VALUE: u32 = 0;
}
