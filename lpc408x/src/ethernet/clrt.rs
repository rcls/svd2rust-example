#[doc = "Register `CLRT` reader"]
pub struct R(crate::R<CLRT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLRT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLRT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLRT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLRT` writer"]
pub struct W(crate::W<CLRT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLRT_SPEC>;
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
impl From<crate::W<CLRT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLRT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RETRANSMAX` reader - RETRANSMISSION MAXIMUM.This is a programmable field specifying the number of retransmission attempts following a collision before aborting the packet due to excessive collisions. The Standard specifies the attemptLimit to be 0xF (15d). See IEEE 802.3/4.2.3.2.5."]
pub struct RETRANSMAX_R(crate::FieldReader<u8, u8>);
impl RETRANSMAX_R {
    pub(crate) fn new(bits: u8) -> Self {
        RETRANSMAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RETRANSMAX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RETRANSMAX` writer - RETRANSMISSION MAXIMUM.This is a programmable field specifying the number of retransmission attempts following a collision before aborting the packet due to excessive collisions. The Standard specifies the attemptLimit to be 0xF (15d). See IEEE 802.3/4.2.3.2.5."]
pub struct RETRANSMAX_W<'a> {
    w: &'a mut W,
}
impl<'a> RETRANSMAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `COLLWIN` reader - COLLISION WINDOW. This is a programmable field representing the slot time or collision window during which collisions occur in properly configured networks. The default value of 0x37 (55d) represents a 56 byte window following the preamble and SFD."]
pub struct COLLWIN_R(crate::FieldReader<u8, u8>);
impl COLLWIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        COLLWIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COLLWIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COLLWIN` writer - COLLISION WINDOW. This is a programmable field representing the slot time or collision window during which collisions occur in properly configured networks. The default value of 0x37 (55d) represents a 56 byte window following the preamble and SFD."]
pub struct COLLWIN_W<'a> {
    w: &'a mut W,
}
impl<'a> COLLWIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - RETRANSMISSION MAXIMUM.This is a programmable field specifying the number of retransmission attempts following a collision before aborting the packet due to excessive collisions. The Standard specifies the attemptLimit to be 0xF (15d). See IEEE 802.3/4.2.3.2.5."]
    #[inline(always)]
    pub fn retransmax(&self) -> RETRANSMAX_R {
        RETRANSMAX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - COLLISION WINDOW. This is a programmable field representing the slot time or collision window during which collisions occur in properly configured networks. The default value of 0x37 (55d) represents a 56 byte window following the preamble and SFD."]
    #[inline(always)]
    pub fn collwin(&self) -> COLLWIN_R {
        COLLWIN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RETRANSMISSION MAXIMUM.This is a programmable field specifying the number of retransmission attempts following a collision before aborting the packet due to excessive collisions. The Standard specifies the attemptLimit to be 0xF (15d). See IEEE 802.3/4.2.3.2.5."]
    #[inline(always)]
    pub fn retransmax(&mut self) -> RETRANSMAX_W {
        RETRANSMAX_W { w: self }
    }
    #[doc = "Bits 8:13 - COLLISION WINDOW. This is a programmable field representing the slot time or collision window during which collisions occur in properly configured networks. The default value of 0x37 (55d) represents a 56 byte window following the preamble and SFD."]
    #[inline(always)]
    pub fn collwin(&mut self) -> COLLWIN_W {
        COLLWIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Collision window / Retry register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clrt](index.html) module"]
pub struct CLRT_SPEC;
impl crate::RegisterSpec for CLRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clrt::R](R) reader structure"]
impl crate::Readable for CLRT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clrt::W](W) writer structure"]
impl crate::Writable for CLRT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLRT to value 0x370f"]
impl crate::Resettable for CLRT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x370f
    }
}
