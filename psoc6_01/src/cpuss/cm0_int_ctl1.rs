#[doc = "Register `CM0_INT_CTL1` reader"]
pub struct R(crate::R<CM0_INT_CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM0_INT_CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM0_INT_CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM0_INT_CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM0_INT_CTL1` writer"]
pub struct W(crate::W<CM0_INT_CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM0_INT_CTL1_SPEC>;
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
impl From<crate::W<CM0_INT_CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM0_INT_CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MUX0_SEL` reader - System interrupt select for CPU interrupt source 4."]
pub struct MUX0_SEL_R(crate::FieldReader<u8, u8>);
impl MUX0_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        MUX0_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX0_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX0_SEL` writer - System interrupt select for CPU interrupt source 4."]
pub struct MUX0_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX0_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `MUX1_SEL` reader - System interrupt select for CPU interrupt source 5."]
pub struct MUX1_SEL_R(crate::FieldReader<u8, u8>);
impl MUX1_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        MUX1_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX1_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX1_SEL` writer - System interrupt select for CPU interrupt source 5."]
pub struct MUX1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX1_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `MUX2_SEL` reader - System interrupt select for CPU interrupt source 6."]
pub struct MUX2_SEL_R(crate::FieldReader<u8, u8>);
impl MUX2_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        MUX2_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX2_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX2_SEL` writer - System interrupt select for CPU interrupt source 6."]
pub struct MUX2_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX2_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `MUX3_SEL` reader - System interrupt select for CPU interrupt source 7."]
pub struct MUX3_SEL_R(crate::FieldReader<u8, u8>);
impl MUX3_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        MUX3_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX3_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX3_SEL` writer - System interrupt select for CPU interrupt source 7."]
pub struct MUX3_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX3_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - System interrupt select for CPU interrupt source 4."]
    #[inline(always)]
    pub fn mux0_sel(&self) -> MUX0_SEL_R {
        MUX0_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - System interrupt select for CPU interrupt source 5."]
    #[inline(always)]
    pub fn mux1_sel(&self) -> MUX1_SEL_R {
        MUX1_SEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - System interrupt select for CPU interrupt source 6."]
    #[inline(always)]
    pub fn mux2_sel(&self) -> MUX2_SEL_R {
        MUX2_SEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - System interrupt select for CPU interrupt source 7."]
    #[inline(always)]
    pub fn mux3_sel(&self) -> MUX3_SEL_R {
        MUX3_SEL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - System interrupt select for CPU interrupt source 4."]
    #[inline(always)]
    pub fn mux0_sel(&mut self) -> MUX0_SEL_W {
        MUX0_SEL_W { w: self }
    }
    #[doc = "Bits 8:15 - System interrupt select for CPU interrupt source 5."]
    #[inline(always)]
    pub fn mux1_sel(&mut self) -> MUX1_SEL_W {
        MUX1_SEL_W { w: self }
    }
    #[doc = "Bits 16:23 - System interrupt select for CPU interrupt source 6."]
    #[inline(always)]
    pub fn mux2_sel(&mut self) -> MUX2_SEL_W {
        MUX2_SEL_W { w: self }
    }
    #[doc = "Bits 24:31 - System interrupt select for CPU interrupt source 7."]
    #[inline(always)]
    pub fn mux3_sel(&mut self) -> MUX3_SEL_W {
        MUX3_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CM0+ interrupt control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_int_ctl1](index.html) module"]
pub struct CM0_INT_CTL1_SPEC;
impl crate::RegisterSpec for CM0_INT_CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm0_int_ctl1::R](R) reader structure"]
impl crate::Readable for CM0_INT_CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm0_int_ctl1::W](W) writer structure"]
impl crate::Writable for CM0_INT_CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CM0_INT_CTL1 to value 0xf0f0_f0f0"]
impl crate::Resettable for CM0_INT_CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf0f0_f0f0
    }
}
