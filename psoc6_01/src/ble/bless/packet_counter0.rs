#[doc = "Register `PACKET_COUNTER0` reader"]
pub struct R(crate::R<PACKET_COUNTER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PACKET_COUNTER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PACKET_COUNTER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PACKET_COUNTER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PACKET_COUNTER0` writer"]
pub struct W(crate::W<PACKET_COUNTER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PACKET_COUNTER0_SPEC>;
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
impl From<crate::W<PACKET_COUNTER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PACKET_COUNTER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PACKET_COUNTER_LOWER` reader - Lower 32-bits of the packet counter value passed as part of Nonce for the packet to be encrypted."]
pub struct PACKET_COUNTER_LOWER_R(crate::FieldReader<u32, u32>);
impl PACKET_COUNTER_LOWER_R {
    pub(crate) fn new(bits: u32) -> Self {
        PACKET_COUNTER_LOWER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PACKET_COUNTER_LOWER_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PACKET_COUNTER_LOWER` writer - Lower 32-bits of the packet counter value passed as part of Nonce for the packet to be encrypted."]
pub struct PACKET_COUNTER_LOWER_W<'a> {
    w: &'a mut W,
}
impl<'a> PACKET_COUNTER_LOWER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Lower 32-bits of the packet counter value passed as part of Nonce for the packet to be encrypted."]
    #[inline(always)]
    pub fn packet_counter_lower(&self) -> PACKET_COUNTER_LOWER_R {
        PACKET_COUNTER_LOWER_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Lower 32-bits of the packet counter value passed as part of Nonce for the packet to be encrypted."]
    #[inline(always)]
    pub fn packet_counter_lower(&mut self) -> PACKET_COUNTER_LOWER_W {
        PACKET_COUNTER_LOWER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Packet counter 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [packet_counter0](index.html) module"]
pub struct PACKET_COUNTER0_SPEC;
impl crate::RegisterSpec for PACKET_COUNTER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [packet_counter0::R](R) reader structure"]
impl crate::Readable for PACKET_COUNTER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [packet_counter0::W](W) writer structure"]
impl crate::Writable for PACKET_COUNTER0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PACKET_COUNTER0 to value 0"]
impl crate::Resettable for PACKET_COUNTER0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
