#[doc = "Register `POC_REG__TIM_CONTROL` reader"]
pub struct R(crate::R<POC_REG__TIM_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POC_REG__TIM_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POC_REG__TIM_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POC_REG__TIM_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POC_REG__TIM_CONTROL` writer"]
pub struct W(crate::W<POC_REG__TIM_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POC_REG__TIM_CONTROL_SPEC>;
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
impl From<crate::W<POC_REG__TIM_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POC_REG__TIM_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BB_CLK_FREQ_MINUS_1` reader - LLH clock configuration. The clock frequency of the clock input to this design is configured in this register. This is used to derive a 1MHz clock."]
pub struct BB_CLK_FREQ_MINUS_1_R(crate::FieldReader<u8, u8>);
impl BB_CLK_FREQ_MINUS_1_R {
    pub(crate) fn new(bits: u8) -> Self {
        BB_CLK_FREQ_MINUS_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BB_CLK_FREQ_MINUS_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BB_CLK_FREQ_MINUS_1` writer - LLH clock configuration. The clock frequency of the clock input to this design is configured in this register. This is used to derive a 1MHz clock."]
pub struct BB_CLK_FREQ_MINUS_1_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_CLK_FREQ_MINUS_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | ((value as u32 & 0x1f) << 3);
        self.w
    }
}
#[doc = "Field `START_SLOT_OFFSET` reader - LLH clock configuration. The start of slot signal is offset by this value. If value is 0, the start of slot signal is generated at the 625us. The offset value is in terms of us."]
pub struct START_SLOT_OFFSET_R(crate::FieldReader<u8, u8>);
impl START_SLOT_OFFSET_R {
    pub(crate) fn new(bits: u8) -> Self {
        START_SLOT_OFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_SLOT_OFFSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START_SLOT_OFFSET` writer - LLH clock configuration. The start of slot signal is offset by this value. If value is 0, the start of slot signal is generated at the 625us. The offset value is in terms of us."]
pub struct START_SLOT_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> START_SLOT_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - LLH clock configuration. The clock frequency of the clock input to this design is configured in this register. This is used to derive a 1MHz clock."]
    #[inline(always)]
    pub fn bb_clk_freq_minus_1(&self) -> BB_CLK_FREQ_MINUS_1_R {
        BB_CLK_FREQ_MINUS_1_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - LLH clock configuration. The start of slot signal is offset by this value. If value is 0, the start of slot signal is generated at the 625us. The offset value is in terms of us."]
    #[inline(always)]
    pub fn start_slot_offset(&self) -> START_SLOT_OFFSET_R {
        START_SLOT_OFFSET_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - LLH clock configuration. The clock frequency of the clock input to this design is configured in this register. This is used to derive a 1MHz clock."]
    #[inline(always)]
    pub fn bb_clk_freq_minus_1(&mut self) -> BB_CLK_FREQ_MINUS_1_W {
        BB_CLK_FREQ_MINUS_1_W { w: self }
    }
    #[doc = "Bits 8:11 - LLH clock configuration. The start of slot signal is offset by this value. If value is 0, the start of slot signal is generated at the 625us. The offset value is in terms of us."]
    #[inline(always)]
    pub fn start_slot_offset(&mut self) -> START_SLOT_OFFSET_W {
        START_SLOT_OFFSET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BLE Time Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [poc_reg__tim_control](index.html) module"]
pub struct POC_REG__TIM_CONTROL_SPEC;
impl crate::RegisterSpec for POC_REG__TIM_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [poc_reg__tim_control::R](R) reader structure"]
impl crate::Readable for POC_REG__TIM_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [poc_reg__tim_control::W](W) writer structure"]
impl crate::Writable for POC_REG__TIM_CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POC_REG__TIM_CONTROL to value 0"]
impl crate::Resettable for POC_REG__TIM_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
