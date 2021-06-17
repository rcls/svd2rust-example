#[doc = "Register `MAXF` reader"]
pub struct R(crate::R<MAXF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAXF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAXF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAXF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAXF` writer"]
pub struct W(crate::W<MAXF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAXF_SPEC>;
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
impl From<crate::W<MAXF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAXF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAXFLEN` reader - MAXIMUM FRAME LENGTH. This field resets to the value 0x0600, which represents a maximum receive frame of 1536 octets. An untagged maximum size Ethernet frame is 1518 octets. A tagged frame adds four octets for a total of 1522 octets. If a shorter maximum length restriction is desired, program this 16-bit field."]
pub struct MAXFLEN_R(crate::FieldReader<u16, u16>);
impl MAXFLEN_R {
    pub(crate) fn new(bits: u16) -> Self {
        MAXFLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAXFLEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAXFLEN` writer - MAXIMUM FRAME LENGTH. This field resets to the value 0x0600, which represents a maximum receive frame of 1536 octets. An untagged maximum size Ethernet frame is 1518 octets. A tagged frame adds four octets for a total of 1522 octets. If a shorter maximum length restriction is desired, program this 16-bit field."]
pub struct MAXFLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXFLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - MAXIMUM FRAME LENGTH. This field resets to the value 0x0600, which represents a maximum receive frame of 1536 octets. An untagged maximum size Ethernet frame is 1518 octets. A tagged frame adds four octets for a total of 1522 octets. If a shorter maximum length restriction is desired, program this 16-bit field."]
    #[inline(always)]
    pub fn maxflen(&self) -> MAXFLEN_R {
        MAXFLEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAXIMUM FRAME LENGTH. This field resets to the value 0x0600, which represents a maximum receive frame of 1536 octets. An untagged maximum size Ethernet frame is 1518 octets. A tagged frame adds four octets for a total of 1522 octets. If a shorter maximum length restriction is desired, program this 16-bit field."]
    #[inline(always)]
    pub fn maxflen(&mut self) -> MAXFLEN_W {
        MAXFLEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Maximum Frame register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maxf](index.html) module"]
pub struct MAXF_SPEC;
impl crate::RegisterSpec for MAXF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maxf::R](R) reader structure"]
impl crate::Readable for MAXF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maxf::W](W) writer structure"]
impl crate::Writable for MAXF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAXF to value 0x0600"]
impl crate::Resettable for MAXF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0600
    }
}
