#[doc = "Register `RECEIVE_TRIG_CTRL` reader"]
pub struct R(crate::R<RECEIVE_TRIG_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RECEIVE_TRIG_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RECEIVE_TRIG_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RECEIVE_TRIG_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RECEIVE_TRIG_CTRL` writer"]
pub struct W(crate::W<RECEIVE_TRIG_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RECEIVE_TRIG_CTRL_SPEC>;
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
impl From<crate::W<RECEIVE_TRIG_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RECEIVE_TRIG_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACC_TRIGGER_THRESHOLD` reader - Access address match threshold value. Number of bits of ac-cess address that should match with the expected access ad-dress to trigger an access code match. Max value : 32 (for 32-bit access address) Lower values may be programmed for bad radios or channels but care must be taken to ensure there are no 'false' matches due to reduced number of bits required to match."]
pub struct ACC_TRIGGER_THRESHOLD_R(crate::FieldReader<u8, u8>);
impl ACC_TRIGGER_THRESHOLD_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACC_TRIGGER_THRESHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACC_TRIGGER_THRESHOLD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACC_TRIGGER_THRESHOLD` writer - Access address match threshold value. Number of bits of ac-cess address that should match with the expected access ad-dress to trigger an access code match. Max value : 32 (for 32-bit access address) Lower values may be programmed for bad radios or channels but care must be taken to ensure there are no 'false' matches due to reduced number of bits required to match."]
pub struct ACC_TRIGGER_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> ACC_TRIGGER_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `ACC_TRIGGER_TIMEOUT` reader - If access address match does not occur then within this time from the start of receive operation, the receive operation times out and stops. An internal counter value of 1usec resolution is continuously compared with the value programmed. Max value :0xFF"]
pub struct ACC_TRIGGER_TIMEOUT_R(crate::FieldReader<u8, u8>);
impl ACC_TRIGGER_TIMEOUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACC_TRIGGER_TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACC_TRIGGER_TIMEOUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACC_TRIGGER_TIMEOUT` writer - If access address match does not occur then within this time from the start of receive operation, the receive operation times out and stops. An internal counter value of 1usec resolution is continuously compared with the value programmed. Max value :0xFF"]
pub struct ACC_TRIGGER_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> ACC_TRIGGER_TIMEOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Access address match threshold value. Number of bits of ac-cess address that should match with the expected access ad-dress to trigger an access code match. Max value : 32 (for 32-bit access address) Lower values may be programmed for bad radios or channels but care must be taken to ensure there are no 'false' matches due to reduced number of bits required to match."]
    #[inline(always)]
    pub fn acc_trigger_threshold(&self) -> ACC_TRIGGER_THRESHOLD_R {
        ACC_TRIGGER_THRESHOLD_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:15 - If access address match does not occur then within this time from the start of receive operation, the receive operation times out and stops. An internal counter value of 1usec resolution is continuously compared with the value programmed. Max value :0xFF"]
    #[inline(always)]
    pub fn acc_trigger_timeout(&self) -> ACC_TRIGGER_TIMEOUT_R {
        ACC_TRIGGER_TIMEOUT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Access address match threshold value. Number of bits of ac-cess address that should match with the expected access ad-dress to trigger an access code match. Max value : 32 (for 32-bit access address) Lower values may be programmed for bad radios or channels but care must be taken to ensure there are no 'false' matches due to reduced number of bits required to match."]
    #[inline(always)]
    pub fn acc_trigger_threshold(&mut self) -> ACC_TRIGGER_THRESHOLD_W {
        ACC_TRIGGER_THRESHOLD_W { w: self }
    }
    #[doc = "Bits 8:15 - If access address match does not occur then within this time from the start of receive operation, the receive operation times out and stops. An internal counter value of 1usec resolution is continuously compared with the value programmed. Max value :0xFF"]
    #[inline(always)]
    pub fn acc_trigger_timeout(&mut self) -> ACC_TRIGGER_TIMEOUT_W {
        ACC_TRIGGER_TIMEOUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive trigger control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [receive_trig_ctrl](index.html) module"]
pub struct RECEIVE_TRIG_CTRL_SPEC;
impl crate::RegisterSpec for RECEIVE_TRIG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [receive_trig_ctrl::R](R) reader structure"]
impl crate::Readable for RECEIVE_TRIG_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [receive_trig_ctrl::W](W) writer structure"]
impl crate::Writable for RECEIVE_TRIG_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RECEIVE_TRIG_CTRL to value 0"]
impl crate::Resettable for RECEIVE_TRIG_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
