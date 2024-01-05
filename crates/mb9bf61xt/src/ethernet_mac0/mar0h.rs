#[doc = "Register `MAR0H` reader"]
pub type R = crate::R<MAR0H_SPEC>;
#[doc = "Register `MAR0H` writer"]
pub type W = crate::W<MAR0H_SPEC>;
#[doc = "Field `A32` reader - AD\\[32\\]"]
pub type A32_R = crate::BitReader;
#[doc = "Field `A32` writer - AD\\[32\\]"]
pub type A32_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A33` reader - AD\\[33\\]"]
pub type A33_R = crate::BitReader;
#[doc = "Field `A33` writer - AD\\[33\\]"]
pub type A33_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A34` reader - AD\\[34\\]"]
pub type A34_R = crate::BitReader;
#[doc = "Field `A34` writer - AD\\[34\\]"]
pub type A34_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A35` reader - AD\\[35\\]"]
pub type A35_R = crate::BitReader;
#[doc = "Field `A35` writer - AD\\[35\\]"]
pub type A35_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A36` reader - AD\\[36\\]"]
pub type A36_R = crate::BitReader;
#[doc = "Field `A36` writer - AD\\[36\\]"]
pub type A36_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A37` reader - AD\\[37\\]"]
pub type A37_R = crate::BitReader;
#[doc = "Field `A37` writer - AD\\[37\\]"]
pub type A37_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A38` reader - AD\\[38\\]"]
pub type A38_R = crate::BitReader;
#[doc = "Field `A38` writer - AD\\[38\\]"]
pub type A38_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A39` reader - AD\\[39\\]"]
pub type A39_R = crate::BitReader;
#[doc = "Field `A39` writer - AD\\[39\\]"]
pub type A39_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A40` reader - AD\\[40\\]"]
pub type A40_R = crate::BitReader;
#[doc = "Field `A40` writer - AD\\[40\\]"]
pub type A40_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A41` reader - AD\\[41\\]"]
pub type A41_R = crate::BitReader;
#[doc = "Field `A41` writer - AD\\[41\\]"]
pub type A41_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A42` reader - AD\\[42\\]"]
pub type A42_R = crate::BitReader;
#[doc = "Field `A42` writer - AD\\[42\\]"]
pub type A42_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A43` reader - AD\\[43\\]"]
pub type A43_R = crate::BitReader;
#[doc = "Field `A43` writer - AD\\[43\\]"]
pub type A43_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A44` reader - AD\\[44\\]"]
pub type A44_R = crate::BitReader;
#[doc = "Field `A44` writer - AD\\[44\\]"]
pub type A44_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A45` reader - AD\\[45\\]"]
pub type A45_R = crate::BitReader;
#[doc = "Field `A45` writer - AD\\[45\\]"]
pub type A45_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A46` reader - AD\\[46\\]"]
pub type A46_R = crate::BitReader;
#[doc = "Field `A46` writer - AD\\[46\\]"]
pub type A46_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A47` reader - AD\\[47\\]"]
pub type A47_R = crate::BitReader;
#[doc = "Field `A47` writer - AD\\[47\\]"]
pub type A47_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MO` reader - Must be one"]
pub type MO_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - AD\\[32\\]"]
    #[inline(always)]
    pub fn a32(&self) -> A32_R {
        A32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AD\\[33\\]"]
    #[inline(always)]
    pub fn a33(&self) -> A33_R {
        A33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AD\\[34\\]"]
    #[inline(always)]
    pub fn a34(&self) -> A34_R {
        A34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AD\\[35\\]"]
    #[inline(always)]
    pub fn a35(&self) -> A35_R {
        A35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AD\\[36\\]"]
    #[inline(always)]
    pub fn a36(&self) -> A36_R {
        A36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AD\\[37\\]"]
    #[inline(always)]
    pub fn a37(&self) -> A37_R {
        A37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AD\\[38\\]"]
    #[inline(always)]
    pub fn a38(&self) -> A38_R {
        A38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AD\\[39\\]"]
    #[inline(always)]
    pub fn a39(&self) -> A39_R {
        A39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AD\\[40\\]"]
    #[inline(always)]
    pub fn a40(&self) -> A40_R {
        A40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AD\\[41\\]"]
    #[inline(always)]
    pub fn a41(&self) -> A41_R {
        A41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AD\\[42\\]"]
    #[inline(always)]
    pub fn a42(&self) -> A42_R {
        A42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - AD\\[43\\]"]
    #[inline(always)]
    pub fn a43(&self) -> A43_R {
        A43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AD\\[44\\]"]
    #[inline(always)]
    pub fn a44(&self) -> A44_R {
        A44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AD\\[45\\]"]
    #[inline(always)]
    pub fn a45(&self) -> A45_R {
        A45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AD\\[46\\]"]
    #[inline(always)]
    pub fn a46(&self) -> A46_R {
        A46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AD\\[47\\]"]
    #[inline(always)]
    pub fn a47(&self) -> A47_R {
        A47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - Must be one"]
    #[inline(always)]
    pub fn mo(&self) -> MO_R {
        MO_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AD\\[32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn a32(&mut self) -> A32_W<MAR0H_SPEC> {
        A32_W::new(self, 0)
    }
    #[doc = "Bit 1 - AD\\[33\\]"]
    #[inline(always)]
    #[must_use]
    pub fn a33(&mut self) -> A33_W<MAR0H_SPEC> {
        A33_W::new(self, 1)
    }
    #[doc = "Bit 2 - AD\\[34\\]"]
    #[inline(always)]
    #[must_use]
    pub fn a34(&mut self) -> A34_W<MAR0H_SPEC> {
        A34_W::new(self, 2)
    }
    #[doc = "Bit 3 - AD\\[35\\]"]
    #[inline(always)]
    #[must_use]
    pub fn a35(&mut self) -> A35_W<MAR0H_SPEC> {
        A35_W::new(self, 3)
    }
    #[doc = "Bit 4 - AD\\[36\\]"]
    #[inline(always)]
    #[must_use]
    pub fn a36(&mut self) -> A36_W<MAR0H_SPEC> {
        A36_W::new(self, 4)
    }
    #[doc = "Bit 5 - AD\\[37\\]"]
    #[inline(always)]
    #[must_use]
    pub fn a37(&mut self) -> A37_W<MAR0H_SPEC> {
        A37_W::new(self, 5)
    }
    #[doc = "Bit 6 - AD\\[38\\]"]
    #[inline(always)]
    #[must_use]
    pub fn a38(&mut self) -> A38_W<MAR0H_SPEC> {
        A38_W::new(self, 6)
    }
    #[doc = "Bit 7 - AD\\[39\\]"]
    #[inline(always)]
    #[must_use]
    pub fn a39(&mut self) -> A39_W<MAR0H_SPEC> {
        A39_W::new(self, 7)
    }
    #[doc = "Bit 8 - AD\\[40\\]"]
    #[inline(always)]
    #[must_use]
    pub fn a40(&mut self) -> A40_W<MAR0H_SPEC> {
        A40_W::new(self, 8)
    }
    #[doc = "Bit 9 - AD\\[41\\]"]
    #[inline(always)]
    #[must_use]
    pub fn a41(&mut self) -> A41_W<MAR0H_SPEC> {
        A41_W::new(self, 9)
    }
    #[doc = "Bit 10 - AD\\[42\\]"]
    #[inline(always)]
    #[must_use]
    pub fn a42(&mut self) -> A42_W<MAR0H_SPEC> {
        A42_W::new(self, 10)
    }
    #[doc = "Bit 11 - AD\\[43\\]"]
    #[inline(always)]
    #[must_use]
    pub fn a43(&mut self) -> A43_W<MAR0H_SPEC> {
        A43_W::new(self, 11)
    }
    #[doc = "Bit 12 - AD\\[44\\]"]
    #[inline(always)]
    #[must_use]
    pub fn a44(&mut self) -> A44_W<MAR0H_SPEC> {
        A44_W::new(self, 12)
    }
    #[doc = "Bit 13 - AD\\[45\\]"]
    #[inline(always)]
    #[must_use]
    pub fn a45(&mut self) -> A45_W<MAR0H_SPEC> {
        A45_W::new(self, 13)
    }
    #[doc = "Bit 14 - AD\\[46\\]"]
    #[inline(always)]
    #[must_use]
    pub fn a46(&mut self) -> A46_W<MAR0H_SPEC> {
        A46_W::new(self, 14)
    }
    #[doc = "Bit 15 - AD\\[47\\]"]
    #[inline(always)]
    #[must_use]
    pub fn a47(&mut self) -> A47_W<MAR0H_SPEC> {
        A47_W::new(self, 15)
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
#[doc = "MAC Address0 Register (High)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mar0h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mar0h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAR0H_SPEC;
impl crate::RegisterSpec for MAR0H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mar0h::R`](R) reader structure"]
impl crate::Readable for MAR0H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mar0h::W`](W) writer structure"]
impl crate::Writable for MAR0H_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAR0H to value 0x8000_ffff"]
impl crate::Resettable for MAR0H_SPEC {
    const RESET_VALUE: u32 = 0x8000_ffff;
}
