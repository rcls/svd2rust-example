#[doc = "Register `MS_CTL` reader"]
pub struct R(crate::R<MS_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MS_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MS_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MS_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MS_CTL` writer"]
pub struct W(crate::W<MS_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MS_CTL_SPEC>;
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
impl From<crate::W<MS_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MS_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PC` reader - N/A"]
pub struct PC_R(crate::FieldReader<u8, u8>);
impl PC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC` writer - N/A"]
pub struct PC_W<'a> {
    w: &'a mut W,
}
impl<'a> PC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `PC_SAVED` reader - Saved protection context. Modifications to this field are constrained by the associated MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\]
fields."]
pub struct PC_SAVED_R(crate::FieldReader<u8, u8>);
impl PC_SAVED_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC_SAVED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC_SAVED_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC_SAVED` writer - Saved protection context. Modifications to this field are constrained by the associated MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\]
fields."]
pub struct PC_SAVED_W<'a> {
    w: &'a mut W,
}
impl<'a> PC_SAVED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Saved protection context. Modifications to this field are constrained by the associated MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\]
fields."]
    #[inline(always)]
    pub fn pc_saved(&self) -> PC_SAVED_R {
        PC_SAVED_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn pc(&mut self) -> PC_W {
        PC_W { w: self }
    }
    #[doc = "Bits 16:19 - Saved protection context. Modifications to this field are constrained by the associated MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\]
fields."]
    #[inline(always)]
    pub fn pc_saved(&mut self) -> PC_SAVED_W {
        PC_SAVED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ms_ctl](index.html) module"]
pub struct MS_CTL_SPEC;
impl crate::RegisterSpec for MS_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ms_ctl::R](R) reader structure"]
impl crate::Readable for MS_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ms_ctl::W](W) writer structure"]
impl crate::Writable for MS_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MS_CTL to value 0"]
impl crate::Resettable for MS_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
