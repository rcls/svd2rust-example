#[doc = "Register `SEQ_START` reader"]
pub struct R(crate::R<SEQ_START_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQ_START_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQ_START_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQ_START_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQ_START` writer"]
pub struct W(crate::W<SEQ_START_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQ_START_SPEC>;
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
impl From<crate::W<SEQ_START_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQ_START_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - Start the CSD sequencer. The sequencer will clear this bit when it is done. Depending on the mode the sequencer is done when a sample has been accumulated, when the high speed comparator trips or if the sequencer is aborted. When the ADC is enabled the ADC sequencer will start when the CSD sequencer reaches the Sample_norm state (only with the regular CSD scan mode)."]
pub struct START_R(crate::FieldReader<bool, bool>);
impl START_R {
    pub(crate) fn new(bits: bool) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START` writer - Start the CSD sequencer. The sequencer will clear this bit when it is done. Depending on the mode the sequencer is done when a sample has been accumulated, when the high speed comparator trips or if the sequencer is aborted. When the ADC is enabled the ADC sequencer will start when the CSD sequencer reaches the Sample_norm state (only with the regular CSD scan mode)."]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
#[doc = "Field `SEQ_MODE` reader - 0 = regular CSD scan + optional ADC 1 = coarse initialization, the Sequencer will go to the INIT_COARSE state."]
pub struct SEQ_MODE_R(crate::FieldReader<bool, bool>);
impl SEQ_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEQ_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEQ_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEQ_MODE` writer - 0 = regular CSD scan + optional ADC 1 = coarse initialization, the Sequencer will go to the INIT_COARSE state."]
pub struct SEQ_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQ_MODE_W<'a> {
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
#[doc = "Field `ABORT` reader - When a 1 is written the CSD and ADC sequencers will be aborted (if they are running) and the START bit will be cleared. This bit always read as 0."]
pub struct ABORT_R(crate::FieldReader<bool, bool>);
impl ABORT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABORT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABORT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABORT` writer - When a 1 is written the CSD and ADC sequencers will be aborted (if they are running) and the START bit will be cleared. This bit always read as 0."]
pub struct ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> ABORT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `DSI_START_EN` reader - When this bit is set a positive edge on dsi_start will start the CSD sequencer and if enabled also the ADC sequencer."]
pub struct DSI_START_EN_R(crate::FieldReader<bool, bool>);
impl DSI_START_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSI_START_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSI_START_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSI_START_EN` writer - When this bit is set a positive edge on dsi_start will start the CSD sequencer and if enabled also the ADC sequencer."]
pub struct DSI_START_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSI_START_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `AZ0_SKIP` reader - When set the AutoZero_0 state will be skipped"]
pub struct AZ0_SKIP_R(crate::FieldReader<bool, bool>);
impl AZ0_SKIP_R {
    pub(crate) fn new(bits: bool) -> Self {
        AZ0_SKIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AZ0_SKIP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AZ0_SKIP` writer - When set the AutoZero_0 state will be skipped"]
pub struct AZ0_SKIP_W<'a> {
    w: &'a mut W,
}
impl<'a> AZ0_SKIP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `AZ1_SKIP` reader - When set the AutoZero_1 state will be skipped"]
pub struct AZ1_SKIP_R(crate::FieldReader<bool, bool>);
impl AZ1_SKIP_R {
    pub(crate) fn new(bits: bool) -> Self {
        AZ1_SKIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AZ1_SKIP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AZ1_SKIP` writer - When set the AutoZero_1 state will be skipped"]
pub struct AZ1_SKIP_W<'a> {
    w: &'a mut W,
}
impl<'a> AZ1_SKIP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Start the CSD sequencer. The sequencer will clear this bit when it is done. Depending on the mode the sequencer is done when a sample has been accumulated, when the high speed comparator trips or if the sequencer is aborted. When the ADC is enabled the ADC sequencer will start when the CSD sequencer reaches the Sample_norm state (only with the regular CSD scan mode)."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 0 = regular CSD scan + optional ADC 1 = coarse initialization, the Sequencer will go to the INIT_COARSE state."]
    #[inline(always)]
    pub fn seq_mode(&self) -> SEQ_MODE_R {
        SEQ_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When a 1 is written the CSD and ADC sequencers will be aborted (if they are running) and the START bit will be cleared. This bit always read as 0."]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When this bit is set a positive edge on dsi_start will start the CSD sequencer and if enabled also the ADC sequencer."]
    #[inline(always)]
    pub fn dsi_start_en(&self) -> DSI_START_EN_R {
        DSI_START_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - When set the AutoZero_0 state will be skipped"]
    #[inline(always)]
    pub fn az0_skip(&self) -> AZ0_SKIP_R {
        AZ0_SKIP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - When set the AutoZero_1 state will be skipped"]
    #[inline(always)]
    pub fn az1_skip(&self) -> AZ1_SKIP_R {
        AZ1_SKIP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start the CSD sequencer. The sequencer will clear this bit when it is done. Depending on the mode the sequencer is done when a sample has been accumulated, when the high speed comparator trips or if the sequencer is aborted. When the ADC is enabled the ADC sequencer will start when the CSD sequencer reaches the Sample_norm state (only with the regular CSD scan mode)."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 1 - 0 = regular CSD scan + optional ADC 1 = coarse initialization, the Sequencer will go to the INIT_COARSE state."]
    #[inline(always)]
    pub fn seq_mode(&mut self) -> SEQ_MODE_W {
        SEQ_MODE_W { w: self }
    }
    #[doc = "Bit 3 - When a 1 is written the CSD and ADC sequencers will be aborted (if they are running) and the START bit will be cleared. This bit always read as 0."]
    #[inline(always)]
    pub fn abort(&mut self) -> ABORT_W {
        ABORT_W { w: self }
    }
    #[doc = "Bit 4 - When this bit is set a positive edge on dsi_start will start the CSD sequencer and if enabled also the ADC sequencer."]
    #[inline(always)]
    pub fn dsi_start_en(&mut self) -> DSI_START_EN_W {
        DSI_START_EN_W { w: self }
    }
    #[doc = "Bit 8 - When set the AutoZero_0 state will be skipped"]
    #[inline(always)]
    pub fn az0_skip(&mut self) -> AZ0_SKIP_W {
        AZ0_SKIP_W { w: self }
    }
    #[doc = "Bit 9 - When set the AutoZero_1 state will be skipped"]
    #[inline(always)]
    pub fn az1_skip(&mut self) -> AZ1_SKIP_W {
        AZ1_SKIP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sequencer start\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_start](index.html) module"]
pub struct SEQ_START_SPEC;
impl crate::RegisterSpec for SEQ_START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seq_start::R](R) reader structure"]
impl crate::Readable for SEQ_START_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seq_start::W](W) writer structure"]
impl crate::Writable for SEQ_START_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEQ_START to value 0"]
impl crate::Resettable for SEQ_START_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
