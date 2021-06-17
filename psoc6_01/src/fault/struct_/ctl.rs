#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TR_EN` reader - Trigger output enable: '0': Disabled. The trigger output 'tr_fault' is '0'. '1': Enabled. The trigger output 'tr_fault' reflects STATUS.VALID. The trigger can be used to initiate a Datawire transfer of the FAULT data (FAULT_DATA0 through FAULT_DATA3)."]
pub struct TR_EN_R(crate::FieldReader<bool, bool>);
impl TR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR_EN` writer - Trigger output enable: '0': Disabled. The trigger output 'tr_fault' is '0'. '1': Enabled. The trigger output 'tr_fault' reflects STATUS.VALID. The trigger can be used to initiate a Datawire transfer of the FAULT data (FAULT_DATA0 through FAULT_DATA3)."]
pub struct TR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `OUT_EN` reader - IO output signal enable: '0': Disabled. The IO output signal 'fault_out' is '0'. The IO output enable signal 'fault_out_en' is '0'. '1': Enabled. The IO output signal 'fault_out' reflects STATUS.VALID. The IO output enable signal 'fault_out_en' is '1'."]
pub struct OUT_EN_R(crate::FieldReader<bool, bool>);
impl OUT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_EN` writer - IO output signal enable: '0': Disabled. The IO output signal 'fault_out' is '0'. The IO output enable signal 'fault_out_en' is '0'. '1': Enabled. The IO output signal 'fault_out' reflects STATUS.VALID. The IO output enable signal 'fault_out_en' is '1'."]
pub struct OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `RESET_REQ_EN` reader - Reset request enable: '0': Disabled. '1': Enabled. The output reset request signal 'fault_reset_req' reflects STATUS.VALID. This reset causes a warm/soft/core reset. This warm/soft/core reset does not affect the fault logic STATUS, DATA0, ..., DATA3 registers (allowing for post soft reset failure analysis). The 'fault_reset_req' signals of the individual fault report structures are combined (logically OR'd) into a single SRSS 'fault_reset_req' signal."]
pub struct RESET_REQ_EN_R(crate::FieldReader<bool, bool>);
impl RESET_REQ_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESET_REQ_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_REQ_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET_REQ_EN` writer - Reset request enable: '0': Disabled. '1': Enabled. The output reset request signal 'fault_reset_req' reflects STATUS.VALID. This reset causes a warm/soft/core reset. This warm/soft/core reset does not affect the fault logic STATUS, DATA0, ..., DATA3 registers (allowing for post soft reset failure analysis). The 'fault_reset_req' signals of the individual fault report structures are combined (logically OR'd) into a single SRSS 'fault_reset_req' signal."]
pub struct RESET_REQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_REQ_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Trigger output enable: '0': Disabled. The trigger output 'tr_fault' is '0'. '1': Enabled. The trigger output 'tr_fault' reflects STATUS.VALID. The trigger can be used to initiate a Datawire transfer of the FAULT data (FAULT_DATA0 through FAULT_DATA3)."]
    #[inline(always)]
    pub fn tr_en(&self) -> TR_EN_R {
        TR_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IO output signal enable: '0': Disabled. The IO output signal 'fault_out' is '0'. The IO output enable signal 'fault_out_en' is '0'. '1': Enabled. The IO output signal 'fault_out' reflects STATUS.VALID. The IO output enable signal 'fault_out_en' is '1'."]
    #[inline(always)]
    pub fn out_en(&self) -> OUT_EN_R {
        OUT_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reset request enable: '0': Disabled. '1': Enabled. The output reset request signal 'fault_reset_req' reflects STATUS.VALID. This reset causes a warm/soft/core reset. This warm/soft/core reset does not affect the fault logic STATUS, DATA0, ..., DATA3 registers (allowing for post soft reset failure analysis). The 'fault_reset_req' signals of the individual fault report structures are combined (logically OR'd) into a single SRSS 'fault_reset_req' signal."]
    #[inline(always)]
    pub fn reset_req_en(&self) -> RESET_REQ_EN_R {
        RESET_REQ_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger output enable: '0': Disabled. The trigger output 'tr_fault' is '0'. '1': Enabled. The trigger output 'tr_fault' reflects STATUS.VALID. The trigger can be used to initiate a Datawire transfer of the FAULT data (FAULT_DATA0 through FAULT_DATA3)."]
    #[inline(always)]
    pub fn tr_en(&mut self) -> TR_EN_W {
        TR_EN_W { w: self }
    }
    #[doc = "Bit 1 - IO output signal enable: '0': Disabled. The IO output signal 'fault_out' is '0'. The IO output enable signal 'fault_out_en' is '0'. '1': Enabled. The IO output signal 'fault_out' reflects STATUS.VALID. The IO output enable signal 'fault_out_en' is '1'."]
    #[inline(always)]
    pub fn out_en(&mut self) -> OUT_EN_W {
        OUT_EN_W { w: self }
    }
    #[doc = "Bit 2 - Reset request enable: '0': Disabled. '1': Enabled. The output reset request signal 'fault_reset_req' reflects STATUS.VALID. This reset causes a warm/soft/core reset. This warm/soft/core reset does not affect the fault logic STATUS, DATA0, ..., DATA3 registers (allowing for post soft reset failure analysis). The 'fault_reset_req' signals of the individual fault report structures are combined (logically OR'd) into a single SRSS 'fault_reset_req' signal."]
    #[inline(always)]
    pub fn reset_req_en(&mut self) -> RESET_REQ_EN_W {
        RESET_REQ_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fault control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
