#[doc = "Register `TR_CMD` reader"]
pub struct R(crate::R<TR_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR_CMD` writer"]
pub struct W(crate::W<TR_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_CMD_SPEC>;
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
impl From<crate::W<TR_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TR_SEL` reader - Specifies the activated trigger when ACTIVATE is '1'. OUT_SEL specifies whether the activated trigger is an input trigger or output trigger to the trigger multiplexer. During activation (ACTIVATE is '1'), SW should not modify this register field. If the specified trigger is not present, the trigger activation has no effect."]
pub struct TR_SEL_R(crate::FieldReader<u8, u8>);
impl TR_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TR_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR_SEL` writer - Specifies the activated trigger when ACTIVATE is '1'. OUT_SEL specifies whether the activated trigger is an input trigger or output trigger to the trigger multiplexer. During activation (ACTIVATE is '1'), SW should not modify this register field. If the specified trigger is not present, the trigger activation has no effect."]
pub struct TR_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TR_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `GROUP_SEL` reader - Specifies the trigger group."]
pub struct GROUP_SEL_R(crate::FieldReader<u8, u8>);
impl GROUP_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        GROUP_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GROUP_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GROUP_SEL` writer - Specifies the trigger group."]
pub struct GROUP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GROUP_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `COUNT` reader - Amount of 'clk_peri' cycles a specific trigger is activated. During activation (ACTIVATE is '1'), HW decrements this field to '0' using a cycle counter. During activation, SW should not modify this register field. A value of 255 is a special case: HW does NOT decrement this field to '0' and trigger activation is under direct control of ACTIVATE when ACTIVATE is '1' the trigger is activated and when ACTIVATE is '0' the trigger is deactivated."]
pub struct COUNT_R(crate::FieldReader<u8, u8>);
impl COUNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNT` writer - Amount of 'clk_peri' cycles a specific trigger is activated. During activation (ACTIVATE is '1'), HW decrements this field to '0' using a cycle counter. During activation, SW should not modify this register field. A value of 255 is a special case: HW does NOT decrement this field to '0' and trigger activation is under direct control of ACTIVATE when ACTIVATE is '1' the trigger is activated and when ACTIVATE is '0' the trigger is deactivated."]
pub struct COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `OUT_SEL` reader - Specifies whether trigger activation is for a specific input or ouput trigger of the trigger multiplexer. Activation of a specific input trigger, will result in activation of all output triggers that have the specific input trigger selected through their TR_OUT_CTL.TR_SEL field. Activation of a specific output trigger, will result in activation of the specified TR_SEL output trigger only. '0': TR_SEL selection and trigger activation is for an input trigger to the trigger multiplexer. '1': TR_SEL selection and trigger activation is for an output trigger from the trigger multiplexer."]
pub struct OUT_SEL_R(crate::FieldReader<bool, bool>);
impl OUT_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_SEL` writer - Specifies whether trigger activation is for a specific input or ouput trigger of the trigger multiplexer. Activation of a specific input trigger, will result in activation of all output triggers that have the specific input trigger selected through their TR_OUT_CTL.TR_SEL field. Activation of a specific output trigger, will result in activation of the specified TR_SEL output trigger only. '0': TR_SEL selection and trigger activation is for an input trigger to the trigger multiplexer. '1': TR_SEL selection and trigger activation is for an output trigger from the trigger multiplexer."]
pub struct OUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_SEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `ACTIVATE` reader - SW sets this field to '1' by to activate (set to '1') a trigger as identified by TR_SEL and OUT_SEL for COUNT cycles. HW sets this field to '0' when the cycle counter is decremented to '0'. Note: a COUNT value of 255 is a special case and trigger activation is under direct control of the ACTIVATE field (the counter is not decremented)."]
pub struct ACTIVATE_R(crate::FieldReader<bool, bool>);
impl ACTIVATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVATE` writer - SW sets this field to '1' by to activate (set to '1') a trigger as identified by TR_SEL and OUT_SEL for COUNT cycles. HW sets this field to '0' when the cycle counter is decremented to '0'. Note: a COUNT value of 255 is a special case and trigger activation is under direct control of the ACTIVATE field (the counter is not decremented)."]
pub struct ACTIVATE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVATE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Specifies the activated trigger when ACTIVATE is '1'. OUT_SEL specifies whether the activated trigger is an input trigger or output trigger to the trigger multiplexer. During activation (ACTIVATE is '1'), SW should not modify this register field. If the specified trigger is not present, the trigger activation has no effect."]
    #[inline(always)]
    pub fn tr_sel(&self) -> TR_SEL_R {
        TR_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Specifies the trigger group."]
    #[inline(always)]
    pub fn group_sel(&self) -> GROUP_SEL_R {
        GROUP_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Amount of 'clk_peri' cycles a specific trigger is activated. During activation (ACTIVATE is '1'), HW decrements this field to '0' using a cycle counter. During activation, SW should not modify this register field. A value of 255 is a special case: HW does NOT decrement this field to '0' and trigger activation is under direct control of ACTIVATE when ACTIVATE is '1' the trigger is activated and when ACTIVATE is '0' the trigger is deactivated."]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 30 - Specifies whether trigger activation is for a specific input or ouput trigger of the trigger multiplexer. Activation of a specific input trigger, will result in activation of all output triggers that have the specific input trigger selected through their TR_OUT_CTL.TR_SEL field. Activation of a specific output trigger, will result in activation of the specified TR_SEL output trigger only. '0': TR_SEL selection and trigger activation is for an input trigger to the trigger multiplexer. '1': TR_SEL selection and trigger activation is for an output trigger from the trigger multiplexer."]
    #[inline(always)]
    pub fn out_sel(&self) -> OUT_SEL_R {
        OUT_SEL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - SW sets this field to '1' by to activate (set to '1') a trigger as identified by TR_SEL and OUT_SEL for COUNT cycles. HW sets this field to '0' when the cycle counter is decremented to '0'. Note: a COUNT value of 255 is a special case and trigger activation is under direct control of the ACTIVATE field (the counter is not decremented)."]
    #[inline(always)]
    pub fn activate(&self) -> ACTIVATE_R {
        ACTIVATE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specifies the activated trigger when ACTIVATE is '1'. OUT_SEL specifies whether the activated trigger is an input trigger or output trigger to the trigger multiplexer. During activation (ACTIVATE is '1'), SW should not modify this register field. If the specified trigger is not present, the trigger activation has no effect."]
    #[inline(always)]
    pub fn tr_sel(&mut self) -> TR_SEL_W {
        TR_SEL_W { w: self }
    }
    #[doc = "Bits 8:11 - Specifies the trigger group."]
    #[inline(always)]
    pub fn group_sel(&mut self) -> GROUP_SEL_W {
        GROUP_SEL_W { w: self }
    }
    #[doc = "Bits 16:23 - Amount of 'clk_peri' cycles a specific trigger is activated. During activation (ACTIVATE is '1'), HW decrements this field to '0' using a cycle counter. During activation, SW should not modify this register field. A value of 255 is a special case: HW does NOT decrement this field to '0' and trigger activation is under direct control of ACTIVATE when ACTIVATE is '1' the trigger is activated and when ACTIVATE is '0' the trigger is deactivated."]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W {
        COUNT_W { w: self }
    }
    #[doc = "Bit 30 - Specifies whether trigger activation is for a specific input or ouput trigger of the trigger multiplexer. Activation of a specific input trigger, will result in activation of all output triggers that have the specific input trigger selected through their TR_OUT_CTL.TR_SEL field. Activation of a specific output trigger, will result in activation of the specified TR_SEL output trigger only. '0': TR_SEL selection and trigger activation is for an input trigger to the trigger multiplexer. '1': TR_SEL selection and trigger activation is for an output trigger from the trigger multiplexer."]
    #[inline(always)]
    pub fn out_sel(&mut self) -> OUT_SEL_W {
        OUT_SEL_W { w: self }
    }
    #[doc = "Bit 31 - SW sets this field to '1' by to activate (set to '1') a trigger as identified by TR_SEL and OUT_SEL for COUNT cycles. HW sets this field to '0' when the cycle counter is decremented to '0'. Note: a COUNT value of 255 is a special case and trigger activation is under direct control of the ACTIVATE field (the counter is not decremented)."]
    #[inline(always)]
    pub fn activate(&mut self) -> ACTIVATE_W {
        ACTIVATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_cmd](index.html) module"]
pub struct TR_CMD_SPEC;
impl crate::RegisterSpec for TR_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_cmd::R](R) reader structure"]
impl crate::Readable for TR_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr_cmd::W](W) writer structure"]
impl crate::Writable for TR_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TR_CMD to value 0"]
impl crate::Resettable for TR_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
