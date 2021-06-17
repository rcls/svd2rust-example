#[doc = "Register `EEWSTATE` reader"]
pub struct R(crate::R<EEWSTATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEWSTATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEWSTATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEWSTATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEWSTATE` writer"]
pub struct W(crate::W<EEWSTATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEWSTATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<EEWSTATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEWSTATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHASE3` reader - Wait states 3 (minus 1 encoded). The number of system clock periods required to give a minimum time of 15 ns."]
pub struct PHASE3_R(crate::FieldReader<u8, u8>);
impl PHASE3_R {
    pub(crate) fn new(bits: u8) -> Self {
        PHASE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHASE3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHASE3` writer - Wait states 3 (minus 1 encoded). The number of system clock periods required to give a minimum time of 15 ns."]
pub struct PHASE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `PHASE2` reader - Wait states 2 (minus 1 encoded). The number of system clock periods required to give a minimum time of 55 ns."]
pub struct PHASE2_R(crate::FieldReader<u8, u8>);
impl PHASE2_R {
    pub(crate) fn new(bits: u8) -> Self {
        PHASE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHASE2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHASE2` writer - Wait states 2 (minus 1 encoded). The number of system clock periods required to give a minimum time of 55 ns."]
pub struct PHASE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `PHASE1` reader - Wait states 1 (minus 1 encoded). The number of system clock periods required to give a minimum time of 35 ns."]
pub struct PHASE1_R(crate::FieldReader<u8, u8>);
impl PHASE1_R {
    pub(crate) fn new(bits: u8) -> Self {
        PHASE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHASE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHASE1` writer - Wait states 1 (minus 1 encoded). The number of system clock periods required to give a minimum time of 35 ns."]
pub struct PHASE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Wait states 3 (minus 1 encoded). The number of system clock periods required to give a minimum time of 15 ns."]
    #[inline(always)]
    pub fn phase3(&self) -> PHASE3_R {
        PHASE3_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Wait states 2 (minus 1 encoded). The number of system clock periods required to give a minimum time of 55 ns."]
    #[inline(always)]
    pub fn phase2(&self) -> PHASE2_R {
        PHASE2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Wait states 1 (minus 1 encoded). The number of system clock periods required to give a minimum time of 35 ns."]
    #[inline(always)]
    pub fn phase1(&self) -> PHASE1_R {
        PHASE1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wait states 3 (minus 1 encoded). The number of system clock periods required to give a minimum time of 15 ns."]
    #[inline(always)]
    pub fn phase3(&mut self) -> PHASE3_W {
        PHASE3_W { w: self }
    }
    #[doc = "Bits 8:15 - Wait states 2 (minus 1 encoded). The number of system clock periods required to give a minimum time of 55 ns."]
    #[inline(always)]
    pub fn phase2(&mut self) -> PHASE2_W {
        PHASE2_W { w: self }
    }
    #[doc = "Bits 16:23 - Wait states 1 (minus 1 encoded). The number of system clock periods required to give a minimum time of 35 ns."]
    #[inline(always)]
    pub fn phase1(&mut self) -> PHASE1_W {
        PHASE1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM wait state register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eewstate](index.html) module"]
pub struct EEWSTATE_SPEC;
impl crate::RegisterSpec for EEWSTATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eewstate::R](R) reader structure"]
impl crate::Readable for EEWSTATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eewstate::W](W) writer structure"]
impl crate::Writable for EEWSTATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EEWSTATE to value 0"]
impl crate::Resettable for EEWSTATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
