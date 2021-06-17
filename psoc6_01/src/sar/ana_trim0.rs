#[doc = "Register `ANA_TRIM0` reader"]
pub struct R(crate::R<ANA_TRIM0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANA_TRIM0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANA_TRIM0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANA_TRIM0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANA_TRIM0` writer"]
pub struct W(crate::W<ANA_TRIM0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANA_TRIM0_SPEC>;
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
impl From<crate::W<ANA_TRIM0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANA_TRIM0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAP_TRIM` reader - Attenuation cap trimming"]
pub struct CAP_TRIM_R(crate::FieldReader<u8, u8>);
impl CAP_TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        CAP_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP_TRIM` writer - Attenuation cap trimming"]
pub struct CAP_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `TRIMUNIT` reader - Attenuation cap trimming"]
pub struct TRIMUNIT_R(crate::FieldReader<bool, bool>);
impl TRIMUNIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRIMUNIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIMUNIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIMUNIT` writer - Attenuation cap trimming"]
pub struct TRIMUNIT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMUNIT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Attenuation cap trimming"]
    #[inline(always)]
    pub fn cap_trim(&self) -> CAP_TRIM_R {
        CAP_TRIM_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Attenuation cap trimming"]
    #[inline(always)]
    pub fn trimunit(&self) -> TRIMUNIT_R {
        TRIMUNIT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Attenuation cap trimming"]
    #[inline(always)]
    pub fn cap_trim(&mut self) -> CAP_TRIM_W {
        CAP_TRIM_W { w: self }
    }
    #[doc = "Bit 5 - Attenuation cap trimming"]
    #[inline(always)]
    pub fn trimunit(&mut self) -> TRIMUNIT_W {
        TRIMUNIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog trim register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_trim0](index.html) module"]
pub struct ANA_TRIM0_SPEC;
impl crate::RegisterSpec for ANA_TRIM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ana_trim0::R](R) reader structure"]
impl crate::Readable for ANA_TRIM0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ana_trim0::W](W) writer structure"]
impl crate::Writable for ANA_TRIM0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ANA_TRIM0 to value 0"]
impl crate::Resettable for ANA_TRIM0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
