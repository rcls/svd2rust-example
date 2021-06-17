#[doc = "Register `TID%s` reader"]
pub struct R(crate::R<TID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TID%s` writer"]
pub struct W(crate::W<TID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TID_SPEC>;
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
impl From<crate::W<TID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID` reader - The 11-bit Identifier to be sent in the next transmit message."]
pub struct ID_R(crate::FieldReader<u16, u16>);
impl ID_R {
    pub(crate) fn new(bits: u16) -> Self {
        ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ID` writer - The 11-bit Identifier to be sent in the next transmit message."]
pub struct ID_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - The 11-bit Identifier to be sent in the next transmit message."]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - The 11-bit Identifier to be sent in the next transmit message."]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Identifier (Tx Buffer)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tid](index.html) module"]
pub struct TID_SPEC;
impl crate::RegisterSpec for TID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tid::R](R) reader structure"]
impl crate::Readable for TID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tid::W](W) writer structure"]
impl crate::Writable for TID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TID%s to value 0"]
impl crate::Resettable for TID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
