#[doc = "Register `TEST_CTL` reader"]
pub struct R(crate::R<TEST_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEST_CTL` writer"]
pub struct W(crate::W<TEST_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST_CTL_SPEC>;
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
impl From<crate::W<TEST_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEST_MODE` reader - Test mode control: '0'-'31': TBD"]
pub struct TEST_MODE_R(crate::FieldReader<u8, u8>);
impl TEST_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TEST_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEST_MODE` writer - Test mode control: '0'-'31': TBD"]
pub struct TEST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `PN_CTL` reader - Positive/negative margin mode control: '0': negative margin control '1': positive margin control"]
pub struct PN_CTL_R(crate::FieldReader<bool, bool>);
impl PN_CTL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PN_CTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PN_CTL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PN_CTL` writer - Positive/negative margin mode control: '0': negative margin control '1': positive margin control"]
pub struct PN_CTL_W<'a> {
    w: &'a mut W,
}
impl<'a> PN_CTL_W<'a> {
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
#[doc = "Field `TM_PE` reader - PUMP_EN override: Pump Enable =PUMP_EN | PE_TM"]
pub struct TM_PE_R(crate::FieldReader<bool, bool>);
impl TM_PE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TM_PE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TM_PE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TM_PE` writer - PUMP_EN override: Pump Enable =PUMP_EN | PE_TM"]
pub struct TM_PE_W<'a> {
    w: &'a mut W,
}
impl<'a> TM_PE_W<'a> {
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
#[doc = "Field `TM_DISPOS` reader - Test mode positive pump disable"]
pub struct TM_DISPOS_R(crate::FieldReader<bool, bool>);
impl TM_DISPOS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TM_DISPOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TM_DISPOS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TM_DISPOS` writer - Test mode positive pump disable"]
pub struct TM_DISPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> TM_DISPOS_W<'a> {
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
#[doc = "Field `TM_DISNEG` reader - Test mode negative pump disable"]
pub struct TM_DISNEG_R(crate::FieldReader<bool, bool>);
impl TM_DISNEG_R {
    pub(crate) fn new(bits: bool) -> Self {
        TM_DISNEG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TM_DISNEG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TM_DISNEG` writer - Test mode negative pump disable"]
pub struct TM_DISNEG_W<'a> {
    w: &'a mut W,
}
impl<'a> TM_DISNEG_W<'a> {
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
#[doc = "Field `EN_CLK_MON` reader - 1: enables the oscillator output monitor"]
pub struct EN_CLK_MON_R(crate::FieldReader<bool, bool>);
impl EN_CLK_MON_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_CLK_MON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_CLK_MON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_CLK_MON` writer - 1: enables the oscillator output monitor"]
pub struct EN_CLK_MON_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_CLK_MON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `CSL_DEBUG` reader - Engineering Debug Register"]
pub struct CSL_DEBUG_R(crate::FieldReader<bool, bool>);
impl CSL_DEBUG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSL_DEBUG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSL_DEBUG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSL_DEBUG` writer - Engineering Debug Register"]
pub struct CSL_DEBUG_W<'a> {
    w: &'a mut W,
}
impl<'a> CSL_DEBUG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `ENABLE_OSC` reader - 0': the oscillator enable logic has control over the internal oscillator '1': forces oscillator enable HI"]
pub struct ENABLE_OSC_R(crate::FieldReader<bool, bool>);
impl ENABLE_OSC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_OSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_OSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE_OSC` writer - 0': the oscillator enable logic has control over the internal oscillator '1': forces oscillator enable HI"]
pub struct ENABLE_OSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_OSC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `UNSCRAMBLE_WA` reader - See BSN-242 memo '0': normal '1': disables the Word Address scrambling"]
pub struct UNSCRAMBLE_WA_R(crate::FieldReader<bool, bool>);
impl UNSCRAMBLE_WA_R {
    pub(crate) fn new(bits: bool) -> Self {
        UNSCRAMBLE_WA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNSCRAMBLE_WA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNSCRAMBLE_WA` writer - See BSN-242 memo '0': normal '1': disables the Word Address scrambling"]
pub struct UNSCRAMBLE_WA_W<'a> {
    w: &'a mut W,
}
impl<'a> UNSCRAMBLE_WA_W<'a> {
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
    #[doc = "Bits 0:4 - Test mode control: '0'-'31': TBD"]
    #[inline(always)]
    pub fn test_mode(&self) -> TEST_MODE_R {
        TEST_MODE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Positive/negative margin mode control: '0': negative margin control '1': positive margin control"]
    #[inline(always)]
    pub fn pn_ctl(&self) -> PN_CTL_R {
        PN_CTL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PUMP_EN override: Pump Enable =PUMP_EN | PE_TM"]
    #[inline(always)]
    pub fn tm_pe(&self) -> TM_PE_R {
        TM_PE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Test mode positive pump disable"]
    #[inline(always)]
    pub fn tm_dispos(&self) -> TM_DISPOS_R {
        TM_DISPOS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Test mode negative pump disable"]
    #[inline(always)]
    pub fn tm_disneg(&self) -> TM_DISNEG_R {
        TM_DISNEG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 1: enables the oscillator output monitor"]
    #[inline(always)]
    pub fn en_clk_mon(&self) -> EN_CLK_MON_R {
        EN_CLK_MON_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Engineering Debug Register"]
    #[inline(always)]
    pub fn csl_debug(&self) -> CSL_DEBUG_R {
        CSL_DEBUG_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 0': the oscillator enable logic has control over the internal oscillator '1': forces oscillator enable HI"]
    #[inline(always)]
    pub fn enable_osc(&self) -> ENABLE_OSC_R {
        ENABLE_OSC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 31 - See BSN-242 memo '0': normal '1': disables the Word Address scrambling"]
    #[inline(always)]
    pub fn unscramble_wa(&self) -> UNSCRAMBLE_WA_R {
        UNSCRAMBLE_WA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Test mode control: '0'-'31': TBD"]
    #[inline(always)]
    pub fn test_mode(&mut self) -> TEST_MODE_W {
        TEST_MODE_W { w: self }
    }
    #[doc = "Bit 8 - Positive/negative margin mode control: '0': negative margin control '1': positive margin control"]
    #[inline(always)]
    pub fn pn_ctl(&mut self) -> PN_CTL_W {
        PN_CTL_W { w: self }
    }
    #[doc = "Bit 9 - PUMP_EN override: Pump Enable =PUMP_EN | PE_TM"]
    #[inline(always)]
    pub fn tm_pe(&mut self) -> TM_PE_W {
        TM_PE_W { w: self }
    }
    #[doc = "Bit 10 - Test mode positive pump disable"]
    #[inline(always)]
    pub fn tm_dispos(&mut self) -> TM_DISPOS_W {
        TM_DISPOS_W { w: self }
    }
    #[doc = "Bit 11 - Test mode negative pump disable"]
    #[inline(always)]
    pub fn tm_disneg(&mut self) -> TM_DISNEG_W {
        TM_DISNEG_W { w: self }
    }
    #[doc = "Bit 16 - 1: enables the oscillator output monitor"]
    #[inline(always)]
    pub fn en_clk_mon(&mut self) -> EN_CLK_MON_W {
        EN_CLK_MON_W { w: self }
    }
    #[doc = "Bit 17 - Engineering Debug Register"]
    #[inline(always)]
    pub fn csl_debug(&mut self) -> CSL_DEBUG_W {
        CSL_DEBUG_W { w: self }
    }
    #[doc = "Bit 18 - 0': the oscillator enable logic has control over the internal oscillator '1': forces oscillator enable HI"]
    #[inline(always)]
    pub fn enable_osc(&mut self) -> ENABLE_OSC_W {
        ENABLE_OSC_W { w: self }
    }
    #[doc = "Bit 31 - See BSN-242 memo '0': normal '1': disables the Word Address scrambling"]
    #[inline(always)]
    pub fn unscramble_wa(&mut self) -> UNSCRAMBLE_WA_W {
        UNSCRAMBLE_WA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test mode control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test_ctl](index.html) module"]
pub struct TEST_CTL_SPEC;
impl crate::RegisterSpec for TEST_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [test_ctl::R](R) reader structure"]
impl crate::Readable for TEST_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [test_ctl::W](W) writer structure"]
impl crate::Writable for TEST_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TEST_CTL to value 0"]
impl crate::Resettable for TEST_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
