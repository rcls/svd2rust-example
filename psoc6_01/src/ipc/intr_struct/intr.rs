#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR` writer"]
pub struct W(crate::W<INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SPEC>;
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
impl From<crate::W<INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RELEASE` reader - These interrupt cause fields are activated (HW sets the field to '1') when a IPC release event is detected. One bit field for each master. SW writes a '1' to these field to clear the interrupt cause."]
pub struct RELEASE_R(crate::FieldReader<u16, u16>);
impl RELEASE_R {
    pub(crate) fn new(bits: u16) -> Self {
        RELEASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RELEASE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RELEASE` writer - These interrupt cause fields are activated (HW sets the field to '1') when a IPC release event is detected. One bit field for each master. SW writes a '1' to these field to clear the interrupt cause."]
pub struct RELEASE_W<'a> {
    w: &'a mut W,
}
impl<'a> RELEASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `NOTIFY` reader - These interrupt cause fields are activated (HW sets the field to '1') when a IPC notification event is detected. One bit field for each master. SW writes a '1' to these field to clear the interrupt cause."]
pub struct NOTIFY_R(crate::FieldReader<u16, u16>);
impl NOTIFY_R {
    pub(crate) fn new(bits: u16) -> Self {
        NOTIFY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOTIFY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOTIFY` writer - These interrupt cause fields are activated (HW sets the field to '1') when a IPC notification event is detected. One bit field for each master. SW writes a '1' to these field to clear the interrupt cause."]
pub struct NOTIFY_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTIFY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - These interrupt cause fields are activated (HW sets the field to '1') when a IPC release event is detected. One bit field for each master. SW writes a '1' to these field to clear the interrupt cause."]
    #[inline(always)]
    pub fn release(&self) -> RELEASE_R {
        RELEASE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - These interrupt cause fields are activated (HW sets the field to '1') when a IPC notification event is detected. One bit field for each master. SW writes a '1' to these field to clear the interrupt cause."]
    #[inline(always)]
    pub fn notify(&self) -> NOTIFY_R {
        NOTIFY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - These interrupt cause fields are activated (HW sets the field to '1') when a IPC release event is detected. One bit field for each master. SW writes a '1' to these field to clear the interrupt cause."]
    #[inline(always)]
    pub fn release(&mut self) -> RELEASE_W {
        RELEASE_W { w: self }
    }
    #[doc = "Bits 16:31 - These interrupt cause fields are activated (HW sets the field to '1') when a IPC notification event is detected. One bit field for each master. SW writes a '1' to these field to clear the interrupt cause."]
    #[inline(always)]
    pub fn notify(&mut self) -> NOTIFY_W {
        NOTIFY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr::W](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
