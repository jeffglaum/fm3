#[doc = "Register `EPFR15` reader"]
pub type R = crate::R<EPFR15_SPEC>;
#[doc = "Register `EPFR15` writer"]
pub type W = crate::W<EPFR15_SPEC>;
#[doc = "Field `EINT16S` reader - External interrupt 16 input select bit"]
pub type EINT16S_R = crate::FieldReader;
#[doc = "Field `EINT16S` writer - External interrupt 16 input select bit"]
pub type EINT16S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT17S` reader - External interrupt 17 input select bit"]
pub type EINT17S_R = crate::FieldReader;
#[doc = "Field `EINT17S` writer - External interrupt 17 input select bit"]
pub type EINT17S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT18S` reader - External interrupt 18 input select bit"]
pub type EINT18S_R = crate::FieldReader;
#[doc = "Field `EINT18S` writer - External interrupt 18 input select bit"]
pub type EINT18S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT19S` reader - External interrupt 19 input select bit"]
pub type EINT19S_R = crate::FieldReader;
#[doc = "Field `EINT19S` writer - External interrupt 19 input select bit"]
pub type EINT19S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT20S` reader - External interrupt 20 input select bit"]
pub type EINT20S_R = crate::FieldReader;
#[doc = "Field `EINT20S` writer - External interrupt 20 input select bit"]
pub type EINT20S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT21S` reader - External interrupt 21 input select bit"]
pub type EINT21S_R = crate::FieldReader;
#[doc = "Field `EINT21S` writer - External interrupt 21 input select bit"]
pub type EINT21S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT22S` reader - External interrupt 22 input select bit"]
pub type EINT22S_R = crate::FieldReader;
#[doc = "Field `EINT22S` writer - External interrupt 22 input select bit"]
pub type EINT22S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT23S` reader - External interrupt 23 input select bit"]
pub type EINT23S_R = crate::FieldReader;
#[doc = "Field `EINT23S` writer - External interrupt 23 input select bit"]
pub type EINT23S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT24S` reader - External interrupt 24 input select bit"]
pub type EINT24S_R = crate::FieldReader;
#[doc = "Field `EINT24S` writer - External interrupt 24 input select bit"]
pub type EINT24S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT25S` reader - External interrupt 25 input select bit"]
pub type EINT25S_R = crate::FieldReader;
#[doc = "Field `EINT25S` writer - External interrupt 25 input select bit"]
pub type EINT25S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT26S` reader - External interrupt 26 input select bit"]
pub type EINT26S_R = crate::FieldReader;
#[doc = "Field `EINT26S` writer - External interrupt 26 input select bit"]
pub type EINT26S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT27S` reader - External interrupt 27 input select bit"]
pub type EINT27S_R = crate::FieldReader;
#[doc = "Field `EINT27S` writer - External interrupt 27 input select bit"]
pub type EINT27S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT28S` reader - External interrupt 28 input select bit"]
pub type EINT28S_R = crate::FieldReader;
#[doc = "Field `EINT28S` writer - External interrupt 28 input select bit"]
pub type EINT28S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT29S` reader - External interrupt 29 input select bit"]
pub type EINT29S_R = crate::FieldReader;
#[doc = "Field `EINT29S` writer - External interrupt 29 input select bit"]
pub type EINT29S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT30S` reader - External interrupt 30 input select bit"]
pub type EINT30S_R = crate::FieldReader;
#[doc = "Field `EINT30S` writer - External interrupt 30 input select bit"]
pub type EINT30S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EINT31S` reader - External interrupt 31 input select bit"]
pub type EINT31S_R = crate::FieldReader;
#[doc = "Field `EINT31S` writer - External interrupt 31 input select bit"]
pub type EINT31S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - External interrupt 16 input select bit"]
    #[inline(always)]
    pub fn eint16s(&self) -> EINT16S_R {
        EINT16S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - External interrupt 17 input select bit"]
    #[inline(always)]
    pub fn eint17s(&self) -> EINT17S_R {
        EINT17S_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - External interrupt 18 input select bit"]
    #[inline(always)]
    pub fn eint18s(&self) -> EINT18S_R {
        EINT18S_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - External interrupt 19 input select bit"]
    #[inline(always)]
    pub fn eint19s(&self) -> EINT19S_R {
        EINT19S_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - External interrupt 20 input select bit"]
    #[inline(always)]
    pub fn eint20s(&self) -> EINT20S_R {
        EINT20S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - External interrupt 21 input select bit"]
    #[inline(always)]
    pub fn eint21s(&self) -> EINT21S_R {
        EINT21S_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - External interrupt 22 input select bit"]
    #[inline(always)]
    pub fn eint22s(&self) -> EINT22S_R {
        EINT22S_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - External interrupt 23 input select bit"]
    #[inline(always)]
    pub fn eint23s(&self) -> EINT23S_R {
        EINT23S_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - External interrupt 24 input select bit"]
    #[inline(always)]
    pub fn eint24s(&self) -> EINT24S_R {
        EINT24S_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - External interrupt 25 input select bit"]
    #[inline(always)]
    pub fn eint25s(&self) -> EINT25S_R {
        EINT25S_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - External interrupt 26 input select bit"]
    #[inline(always)]
    pub fn eint26s(&self) -> EINT26S_R {
        EINT26S_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - External interrupt 27 input select bit"]
    #[inline(always)]
    pub fn eint27s(&self) -> EINT27S_R {
        EINT27S_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - External interrupt 28 input select bit"]
    #[inline(always)]
    pub fn eint28s(&self) -> EINT28S_R {
        EINT28S_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - External interrupt 29 input select bit"]
    #[inline(always)]
    pub fn eint29s(&self) -> EINT29S_R {
        EINT29S_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - External interrupt 30 input select bit"]
    #[inline(always)]
    pub fn eint30s(&self) -> EINT30S_R {
        EINT30S_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - External interrupt 31 input select bit"]
    #[inline(always)]
    pub fn eint31s(&self) -> EINT31S_R {
        EINT31S_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External interrupt 16 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint16s(&mut self) -> EINT16S_W<EPFR15_SPEC> {
        EINT16S_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - External interrupt 17 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint17s(&mut self) -> EINT17S_W<EPFR15_SPEC> {
        EINT17S_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - External interrupt 18 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint18s(&mut self) -> EINT18S_W<EPFR15_SPEC> {
        EINT18S_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - External interrupt 19 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint19s(&mut self) -> EINT19S_W<EPFR15_SPEC> {
        EINT19S_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - External interrupt 20 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint20s(&mut self) -> EINT20S_W<EPFR15_SPEC> {
        EINT20S_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - External interrupt 21 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint21s(&mut self) -> EINT21S_W<EPFR15_SPEC> {
        EINT21S_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - External interrupt 22 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint22s(&mut self) -> EINT22S_W<EPFR15_SPEC> {
        EINT22S_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - External interrupt 23 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint23s(&mut self) -> EINT23S_W<EPFR15_SPEC> {
        EINT23S_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - External interrupt 24 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint24s(&mut self) -> EINT24S_W<EPFR15_SPEC> {
        EINT24S_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - External interrupt 25 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint25s(&mut self) -> EINT25S_W<EPFR15_SPEC> {
        EINT25S_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - External interrupt 26 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint26s(&mut self) -> EINT26S_W<EPFR15_SPEC> {
        EINT26S_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - External interrupt 27 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint27s(&mut self) -> EINT27S_W<EPFR15_SPEC> {
        EINT27S_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - External interrupt 28 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint28s(&mut self) -> EINT28S_W<EPFR15_SPEC> {
        EINT28S_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - External interrupt 29 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint29s(&mut self) -> EINT29S_W<EPFR15_SPEC> {
        EINT29S_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - External interrupt 30 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint30s(&mut self) -> EINT30S_W<EPFR15_SPEC> {
        EINT30S_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - External interrupt 31 input select bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint31s(&mut self) -> EINT31S_W<EPFR15_SPEC> {
        EINT31S_W::new(self, 30)
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
#[doc = "Extended pin function setting register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPFR15_SPEC;
impl crate::RegisterSpec for EPFR15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epfr15::R`](R) reader structure"]
impl crate::Readable for EPFR15_SPEC {}
#[doc = "`write(|w| ..)` method takes [`epfr15::W`](W) writer structure"]
impl crate::Writable for EPFR15_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EPFR15 to value 0"]
impl crate::Resettable for EPFR15_SPEC {
    const RESET_VALUE: u32 = 0;
}
