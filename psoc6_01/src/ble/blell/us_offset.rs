#[doc = "Register `US_OFFSET` reader"]
pub struct R(crate::R<US_OFFSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_OFFSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_OFFSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_OFFSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_OFFSET` writer"]
pub struct W(crate::W<US_OFFSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_OFFSET_SPEC>;
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
impl From<crate::W<US_OFFSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_OFFSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `US_OFFSET_SLOT_BOUNDARY` reader - Micro Second Offset from the Slot Bounday at which the connection programmed in NEXT_CONN has to be serviced. This register along with NI_TIMER has to be programmed 1.25ms before the connection event. The granularity is 1us"]
pub struct US_OFFSET_SLOT_BOUNDARY_R(crate::FieldReader<u16, u16>);
impl US_OFFSET_SLOT_BOUNDARY_R {
    pub(crate) fn new(bits: u16) -> Self {
        US_OFFSET_SLOT_BOUNDARY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for US_OFFSET_SLOT_BOUNDARY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `US_OFFSET_SLOT_BOUNDARY` writer - Micro Second Offset from the Slot Bounday at which the connection programmed in NEXT_CONN has to be serviced. This register along with NI_TIMER has to be programmed 1.25ms before the connection event. The granularity is 1us"]
pub struct US_OFFSET_SLOT_BOUNDARY_W<'a> {
    w: &'a mut W,
}
impl<'a> US_OFFSET_SLOT_BOUNDARY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Micro Second Offset from the Slot Bounday at which the connection programmed in NEXT_CONN has to be serviced. This register along with NI_TIMER has to be programmed 1.25ms before the connection event. The granularity is 1us"]
    #[inline(always)]
    pub fn us_offset_slot_boundary(&self) -> US_OFFSET_SLOT_BOUNDARY_R {
        US_OFFSET_SLOT_BOUNDARY_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Micro Second Offset from the Slot Bounday at which the connection programmed in NEXT_CONN has to be serviced. This register along with NI_TIMER has to be programmed 1.25ms before the connection event. The granularity is 1us"]
    #[inline(always)]
    pub fn us_offset_slot_boundary(&mut self) -> US_OFFSET_SLOT_BOUNDARY_W {
        US_OFFSET_SLOT_BOUNDARY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Micro-second Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_offset](index.html) module"]
pub struct US_OFFSET_SPEC;
impl crate::RegisterSpec for US_OFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_offset::R](R) reader structure"]
impl crate::Readable for US_OFFSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_offset::W](W) writer structure"]
impl crate::Writable for US_OFFSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets US_OFFSET to value 0"]
impl crate::Resettable for US_OFFSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
