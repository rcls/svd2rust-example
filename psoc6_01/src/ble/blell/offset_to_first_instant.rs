#[doc = "Register `OFFSET_TO_FIRST_INSTANT` reader"]
pub struct R(crate::R<OFFSET_TO_FIRST_INSTANT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFFSET_TO_FIRST_INSTANT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFFSET_TO_FIRST_INSTANT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFFSET_TO_FIRST_INSTANT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OFFSET_TO_FIRST_INSTANT` writer"]
pub struct W(crate::W<OFFSET_TO_FIRST_INSTANT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFFSET_TO_FIRST_INSTANT_SPEC>;
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
impl From<crate::W<OFFSET_TO_FIRST_INSTANT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFFSET_TO_FIRST_INSTANT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSET_TO_FIRST_EVENT` reader - The offset w.r.t the internal reference clock at which instant the first event occurs. This register will give flexibility to the firmware to position the con-nection at a desired point with respect to the internal free running clock. It is optional to be updated by firmware. This is not updated in the current firmware."]
pub struct OFFSET_TO_FIRST_EVENT_R(crate::FieldReader<u16, u16>);
impl OFFSET_TO_FIRST_EVENT_R {
    pub(crate) fn new(bits: u16) -> Self {
        OFFSET_TO_FIRST_EVENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSET_TO_FIRST_EVENT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSET_TO_FIRST_EVENT` writer - The offset w.r.t the internal reference clock at which instant the first event occurs. This register will give flexibility to the firmware to position the con-nection at a desired point with respect to the internal free running clock. It is optional to be updated by firmware. This is not updated in the current firmware."]
pub struct OFFSET_TO_FIRST_EVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET_TO_FIRST_EVENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - The offset w.r.t the internal reference clock at which instant the first event occurs. This register will give flexibility to the firmware to position the con-nection at a desired point with respect to the internal free running clock. It is optional to be updated by firmware. This is not updated in the current firmware."]
    #[inline(always)]
    pub fn offset_to_first_event(&self) -> OFFSET_TO_FIRST_EVENT_R {
        OFFSET_TO_FIRST_EVENT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The offset w.r.t the internal reference clock at which instant the first event occurs. This register will give flexibility to the firmware to position the con-nection at a desired point with respect to the internal free running clock. It is optional to be updated by firmware. This is not updated in the current firmware."]
    #[inline(always)]
    pub fn offset_to_first_event(&mut self) -> OFFSET_TO_FIRST_EVENT_W {
        OFFSET_TO_FIRST_EVENT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset to first instant\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [offset_to_first_instant](index.html) module"]
pub struct OFFSET_TO_FIRST_INSTANT_SPEC;
impl crate::RegisterSpec for OFFSET_TO_FIRST_INSTANT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [offset_to_first_instant::R](R) reader structure"]
impl crate::Readable for OFFSET_TO_FIRST_INSTANT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [offset_to_first_instant::W](W) writer structure"]
impl crate::Writable for OFFSET_TO_FIRST_INSTANT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OFFSET_TO_FIRST_INSTANT to value 0x06"]
impl crate::Resettable for OFFSET_TO_FIRST_INSTANT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06
    }
}
