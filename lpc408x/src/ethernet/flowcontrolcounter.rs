#[doc = "Register `FLOWCONTROLCOUNTER` reader"]
pub struct R(crate::R<FLOWCONTROLCOUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLOWCONTROLCOUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLOWCONTROLCOUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLOWCONTROLCOUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLOWCONTROLCOUNTER` writer"]
pub struct W(crate::W<FLOWCONTROLCOUNTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLOWCONTROLCOUNTER_SPEC>;
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
impl From<crate::W<FLOWCONTROLCOUNTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLOWCONTROLCOUNTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MC` reader - MirrorCounter. In full duplex mode the MirrorCounter specifies the number of cycles before re-issuing the Pause control frame."]
pub struct MC_R(crate::FieldReader<u16, u16>);
impl MC_R {
    pub(crate) fn new(bits: u16) -> Self {
        MC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MC` writer - MirrorCounter. In full duplex mode the MirrorCounter specifies the number of cycles before re-issuing the Pause control frame."]
pub struct MC_W<'a> {
    w: &'a mut W,
}
impl<'a> MC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `PT` reader - PauseTimer. In full-duplex mode the PauseTimer specifies the value that is inserted into the pause timer field of a pause flow control frame. In half duplex mode the PauseTimer specifies the number of backpressure cycles."]
pub struct PT_R(crate::FieldReader<u16, u16>);
impl PT_R {
    pub(crate) fn new(bits: u16) -> Self {
        PT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PT` writer - PauseTimer. In full-duplex mode the PauseTimer specifies the value that is inserted into the pause timer field of a pause flow control frame. In half duplex mode the PauseTimer specifies the number of backpressure cycles."]
pub struct PT_W<'a> {
    w: &'a mut W,
}
impl<'a> PT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - MirrorCounter. In full duplex mode the MirrorCounter specifies the number of cycles before re-issuing the Pause control frame."]
    #[inline(always)]
    pub fn mc(&self) -> MC_R {
        MC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - PauseTimer. In full-duplex mode the PauseTimer specifies the value that is inserted into the pause timer field of a pause flow control frame. In half duplex mode the PauseTimer specifies the number of backpressure cycles."]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MirrorCounter. In full duplex mode the MirrorCounter specifies the number of cycles before re-issuing the Pause control frame."]
    #[inline(always)]
    pub fn mc(&mut self) -> MC_W {
        MC_W { w: self }
    }
    #[doc = "Bits 16:31 - PauseTimer. In full-duplex mode the PauseTimer specifies the value that is inserted into the pause timer field of a pause flow control frame. In half duplex mode the PauseTimer specifies the number of backpressure cycles."]
    #[inline(always)]
    pub fn pt(&mut self) -> PT_W {
        PT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flow control counter register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flowcontrolcounter](index.html) module"]
pub struct FLOWCONTROLCOUNTER_SPEC;
impl crate::RegisterSpec for FLOWCONTROLCOUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flowcontrolcounter::R](R) reader structure"]
impl crate::Readable for FLOWCONTROLCOUNTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flowcontrolcounter::W](W) writer structure"]
impl crate::Writable for FLOWCONTROLCOUNTER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLOWCONTROLCOUNTER to value 0"]
impl crate::Resettable for FLOWCONTROLCOUNTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
