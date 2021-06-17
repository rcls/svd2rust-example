#[doc = "Register `INTR_STAT` reader"]
pub struct R(crate::R<INTR_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_STAT` writer"]
pub struct W(crate::W<INTR_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_STAT_SPEC>;
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
impl From<crate::W<INTR_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSM_ENTERED_INTR` reader - On a firmware request to LL to enter into state machine, working on LF clock, LL transitions into Deep Sleep Mode and asserts this interrupt. The interrupt can be cleared by writing one into this location."]
pub struct DSM_ENTERED_INTR_R(crate::FieldReader<bool, bool>);
impl DSM_ENTERED_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSM_ENTERED_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSM_ENTERED_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSM_ENTERED_INTR` writer - On a firmware request to LL to enter into state machine, working on LF clock, LL transitions into Deep Sleep Mode and asserts this interrupt. The interrupt can be cleared by writing one into this location."]
pub struct DSM_ENTERED_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> DSM_ENTERED_INTR_W<'a> {
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
#[doc = "Field `DSM_EXITED_INTR` reader - On a firmware request to LL to exit from Deep Sleep Mode, working on LF clock, LL transitions from Deep Sleep Mode and asserts this interrupt when the Deep Sleep clock gater is turned ON. The interrupt can be cleared by writing one into this location."]
pub struct DSM_EXITED_INTR_R(crate::FieldReader<bool, bool>);
impl DSM_EXITED_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSM_EXITED_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSM_EXITED_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSM_EXITED_INTR` writer - On a firmware request to LL to exit from Deep Sleep Mode, working on LF clock, LL transitions from Deep Sleep Mode and asserts this interrupt when the Deep Sleep clock gater is turned ON. The interrupt can be cleared by writing one into this location."]
pub struct DSM_EXITED_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> DSM_EXITED_INTR_W<'a> {
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
#[doc = "Field `RCBLL_DONE_INTR` reader - RCB transaction Complete"]
pub struct RCBLL_DONE_INTR_R(crate::FieldReader<bool, bool>);
impl RCBLL_DONE_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCBLL_DONE_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCBLL_DONE_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLERD_ACTIVE_INTR` reader - CYBLERD55 is in active mode. RF is active"]
pub struct BLERD_ACTIVE_INTR_R(crate::FieldReader<bool, bool>);
impl BLERD_ACTIVE_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLERD_ACTIVE_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLERD_ACTIVE_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLERD_ACTIVE_INTR` writer - CYBLERD55 is in active mode. RF is active"]
pub struct BLERD_ACTIVE_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> BLERD_ACTIVE_INTR_W<'a> {
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
#[doc = "Field `RCB_INTR` reader - RCB controller Interrupt - Refer to RCB_INTR_STAT register"]
pub struct RCB_INTR_R(crate::FieldReader<bool, bool>);
impl RCB_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCB_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCB_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LL_INTR` reader - LL controller interrupt - Refer to EVENT_INTR register"]
pub struct LL_INTR_R(crate::FieldReader<bool, bool>);
impl LL_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        LL_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LL_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_INTR` reader - GPIO interrupt"]
pub struct GPIO_INTR_R(crate::FieldReader<bool, bool>);
impl GPIO_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_INTR` writer - GPIO interrupt"]
pub struct GPIO_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `EFUSE_INTR` reader - This bit when set by efuse controller logic when the efuse read/write is completed"]
pub struct EFUSE_INTR_R(crate::FieldReader<bool, bool>);
impl EFUSE_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EFUSE_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFUSE_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFUSE_INTR` writer - This bit when set by efuse controller logic when the efuse read/write is completed"]
pub struct EFUSE_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_INTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `XTAL_ON_INTR` reader - enabled crystal stable signal rising edge interrupt. The interrupt can be cleared by writing one into this location."]
pub struct XTAL_ON_INTR_R(crate::FieldReader<bool, bool>);
impl XTAL_ON_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        XTAL_ON_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL_ON_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL_ON_INTR` writer - enabled crystal stable signal rising edge interrupt. The interrupt can be cleared by writing one into this location."]
pub struct XTAL_ON_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_ON_INTR_W<'a> {
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
#[doc = "Field `ENC_INTR` reader - Encryption Interrupt Triggered"]
pub struct ENC_INTR_R(crate::FieldReader<bool, bool>);
impl ENC_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENC_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENC_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HVLDO_LV_DETECT_POS` reader - This interrupt is set on HVLDO LV Detector Rise edge. There is a 1cycle AHB clock glitch filter on the HVLDO LV Detector output"]
pub struct HVLDO_LV_DETECT_POS_R(crate::FieldReader<bool, bool>);
impl HVLDO_LV_DETECT_POS_R {
    pub(crate) fn new(bits: bool) -> Self {
        HVLDO_LV_DETECT_POS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HVLDO_LV_DETECT_POS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HVLDO_LV_DETECT_POS` writer - This interrupt is set on HVLDO LV Detector Rise edge. There is a 1cycle AHB clock glitch filter on the HVLDO LV Detector output"]
pub struct HVLDO_LV_DETECT_POS_W<'a> {
    w: &'a mut W,
}
impl<'a> HVLDO_LV_DETECT_POS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `HVLDO_LV_DETECT_NEG` reader - This interrupt is set on HVLDO LV Detector Fall edge. There is a 1cycle AHB clock glitch filter on the HVLDO LV Detector output"]
pub struct HVLDO_LV_DETECT_NEG_R(crate::FieldReader<bool, bool>);
impl HVLDO_LV_DETECT_NEG_R {
    pub(crate) fn new(bits: bool) -> Self {
        HVLDO_LV_DETECT_NEG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HVLDO_LV_DETECT_NEG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HVLDO_LV_DETECT_NEG` writer - This interrupt is set on HVLDO LV Detector Fall edge. There is a 1cycle AHB clock glitch filter on the HVLDO LV Detector output"]
pub struct HVLDO_LV_DETECT_NEG_W<'a> {
    w: &'a mut W,
}
impl<'a> HVLDO_LV_DETECT_NEG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - On a firmware request to LL to enter into state machine, working on LF clock, LL transitions into Deep Sleep Mode and asserts this interrupt. The interrupt can be cleared by writing one into this location."]
    #[inline(always)]
    pub fn dsm_entered_intr(&self) -> DSM_ENTERED_INTR_R {
        DSM_ENTERED_INTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - On a firmware request to LL to exit from Deep Sleep Mode, working on LF clock, LL transitions from Deep Sleep Mode and asserts this interrupt when the Deep Sleep clock gater is turned ON. The interrupt can be cleared by writing one into this location."]
    #[inline(always)]
    pub fn dsm_exited_intr(&self) -> DSM_EXITED_INTR_R {
        DSM_EXITED_INTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RCB transaction Complete"]
    #[inline(always)]
    pub fn rcbll_done_intr(&self) -> RCBLL_DONE_INTR_R {
        RCBLL_DONE_INTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CYBLERD55 is in active mode. RF is active"]
    #[inline(always)]
    pub fn blerd_active_intr(&self) -> BLERD_ACTIVE_INTR_R {
        BLERD_ACTIVE_INTR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RCB controller Interrupt - Refer to RCB_INTR_STAT register"]
    #[inline(always)]
    pub fn rcb_intr(&self) -> RCB_INTR_R {
        RCB_INTR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LL controller interrupt - Refer to EVENT_INTR register"]
    #[inline(always)]
    pub fn ll_intr(&self) -> LL_INTR_R {
        LL_INTR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIO interrupt"]
    #[inline(always)]
    pub fn gpio_intr(&self) -> GPIO_INTR_R {
        GPIO_INTR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit when set by efuse controller logic when the efuse read/write is completed"]
    #[inline(always)]
    pub fn efuse_intr(&self) -> EFUSE_INTR_R {
        EFUSE_INTR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - enabled crystal stable signal rising edge interrupt. The interrupt can be cleared by writing one into this location."]
    #[inline(always)]
    pub fn xtal_on_intr(&self) -> XTAL_ON_INTR_R {
        XTAL_ON_INTR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Encryption Interrupt Triggered"]
    #[inline(always)]
    pub fn enc_intr(&self) -> ENC_INTR_R {
        ENC_INTR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - This interrupt is set on HVLDO LV Detector Rise edge. There is a 1cycle AHB clock glitch filter on the HVLDO LV Detector output"]
    #[inline(always)]
    pub fn hvldo_lv_detect_pos(&self) -> HVLDO_LV_DETECT_POS_R {
        HVLDO_LV_DETECT_POS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This interrupt is set on HVLDO LV Detector Fall edge. There is a 1cycle AHB clock glitch filter on the HVLDO LV Detector output"]
    #[inline(always)]
    pub fn hvldo_lv_detect_neg(&self) -> HVLDO_LV_DETECT_NEG_R {
        HVLDO_LV_DETECT_NEG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - On a firmware request to LL to enter into state machine, working on LF clock, LL transitions into Deep Sleep Mode and asserts this interrupt. The interrupt can be cleared by writing one into this location."]
    #[inline(always)]
    pub fn dsm_entered_intr(&mut self) -> DSM_ENTERED_INTR_W {
        DSM_ENTERED_INTR_W { w: self }
    }
    #[doc = "Bit 1 - On a firmware request to LL to exit from Deep Sleep Mode, working on LF clock, LL transitions from Deep Sleep Mode and asserts this interrupt when the Deep Sleep clock gater is turned ON. The interrupt can be cleared by writing one into this location."]
    #[inline(always)]
    pub fn dsm_exited_intr(&mut self) -> DSM_EXITED_INTR_W {
        DSM_EXITED_INTR_W { w: self }
    }
    #[doc = "Bit 3 - CYBLERD55 is in active mode. RF is active"]
    #[inline(always)]
    pub fn blerd_active_intr(&mut self) -> BLERD_ACTIVE_INTR_W {
        BLERD_ACTIVE_INTR_W { w: self }
    }
    #[doc = "Bit 6 - GPIO interrupt"]
    #[inline(always)]
    pub fn gpio_intr(&mut self) -> GPIO_INTR_W {
        GPIO_INTR_W { w: self }
    }
    #[doc = "Bit 7 - This bit when set by efuse controller logic when the efuse read/write is completed"]
    #[inline(always)]
    pub fn efuse_intr(&mut self) -> EFUSE_INTR_W {
        EFUSE_INTR_W { w: self }
    }
    #[doc = "Bit 8 - enabled crystal stable signal rising edge interrupt. The interrupt can be cleared by writing one into this location."]
    #[inline(always)]
    pub fn xtal_on_intr(&mut self) -> XTAL_ON_INTR_W {
        XTAL_ON_INTR_W { w: self }
    }
    #[doc = "Bit 10 - This interrupt is set on HVLDO LV Detector Rise edge. There is a 1cycle AHB clock glitch filter on the HVLDO LV Detector output"]
    #[inline(always)]
    pub fn hvldo_lv_detect_pos(&mut self) -> HVLDO_LV_DETECT_POS_W {
        HVLDO_LV_DETECT_POS_W { w: self }
    }
    #[doc = "Bit 11 - This interrupt is set on HVLDO LV Detector Fall edge. There is a 1cycle AHB clock glitch filter on the HVLDO LV Detector output"]
    #[inline(always)]
    pub fn hvldo_lv_detect_neg(&mut self) -> HVLDO_LV_DETECT_NEG_W {
        HVLDO_LV_DETECT_NEG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Link Layer interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_stat](index.html) module"]
pub struct INTR_STAT_SPEC;
impl crate::RegisterSpec for INTR_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_stat::R](R) reader structure"]
impl crate::Readable for INTR_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_stat::W](W) writer structure"]
impl crate::Writable for INTR_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_STAT to value 0"]
impl crate::Resettable for INTR_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
