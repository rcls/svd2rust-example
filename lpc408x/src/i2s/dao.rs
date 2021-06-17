#[doc = "Register `DAO` reader"]
pub struct R(crate::R<DAO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAO` writer"]
pub struct W(crate::W<DAO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAO_SPEC>;
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
impl From<crate::W<DAO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects the number of bytes in data as follows:\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WORDWIDTH_A {
    #[doc = "0: 8-bit data"]
    _8_BIT_DATA = 0,
    #[doc = "1: 16-bit data"]
    _16_BIT_DATA = 1,
    #[doc = "3: 32-bit data"]
    _32_BIT_DATA = 3,
}
impl From<WORDWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: WORDWIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WORDWIDTH` reader - Selects the number of bytes in data as follows:"]
pub struct WORDWIDTH_R(crate::FieldReader<u8, WORDWIDTH_A>);
impl WORDWIDTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        WORDWIDTH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WORDWIDTH_A> {
        match self.bits {
            0 => Some(WORDWIDTH_A::_8_BIT_DATA),
            1 => Some(WORDWIDTH_A::_16_BIT_DATA),
            3 => Some(WORDWIDTH_A::_32_BIT_DATA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_8_BIT_DATA`"]
    #[inline(always)]
    pub fn is_8_bit_data(&self) -> bool {
        **self == WORDWIDTH_A::_8_BIT_DATA
    }
    #[doc = "Checks if the value of the field is `_16_BIT_DATA`"]
    #[inline(always)]
    pub fn is_16_bit_data(&self) -> bool {
        **self == WORDWIDTH_A::_16_BIT_DATA
    }
    #[doc = "Checks if the value of the field is `_32_BIT_DATA`"]
    #[inline(always)]
    pub fn is_32_bit_data(&self) -> bool {
        **self == WORDWIDTH_A::_32_BIT_DATA
    }
}
impl core::ops::Deref for WORDWIDTH_R {
    type Target = crate::FieldReader<u8, WORDWIDTH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WORDWIDTH` writer - Selects the number of bytes in data as follows:"]
pub struct WORDWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> WORDWIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WORDWIDTH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8-bit data"]
    #[inline(always)]
    pub fn _8_bit_data(self) -> &'a mut W {
        self.variant(WORDWIDTH_A::_8_BIT_DATA)
    }
    #[doc = "16-bit data"]
    #[inline(always)]
    pub fn _16_bit_data(self) -> &'a mut W {
        self.variant(WORDWIDTH_A::_16_BIT_DATA)
    }
    #[doc = "32-bit data"]
    #[inline(always)]
    pub fn _32_bit_data(self) -> &'a mut W {
        self.variant(WORDWIDTH_A::_32_BIT_DATA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `MONO` reader - When 1, data is of monaural format. When 0, the data is in stereo format."]
pub struct MONO_R(crate::FieldReader<bool, bool>);
impl MONO_R {
    pub(crate) fn new(bits: bool) -> Self {
        MONO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MONO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONO` writer - When 1, data is of monaural format. When 0, the data is in stereo format."]
pub struct MONO_W<'a> {
    w: &'a mut W,
}
impl<'a> MONO_W<'a> {
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
#[doc = "Field `STOP` reader - When 1, disables accesses on FIFOs, places the transmit channel in mute mode."]
pub struct STOP_R(crate::FieldReader<bool, bool>);
impl STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP` writer - When 1, disables accesses on FIFOs, places the transmit channel in mute mode."]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
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
#[doc = "Field `RESET` reader - When 1, asynchronously resets the transmit channel and FIFO."]
pub struct RESET_R(crate::FieldReader<bool, bool>);
impl RESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET` writer - When 1, asynchronously resets the transmit channel and FIFO."]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
#[doc = "Field `WS_SEL` reader - When 0, the interface is in master mode. When 1, the interface is in slave mode. See Section 34.7.2 for a summary of useful combinations for this bit with TXMODE."]
pub struct WS_SEL_R(crate::FieldReader<bool, bool>);
impl WS_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        WS_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WS_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WS_SEL` writer - When 0, the interface is in master mode. When 1, the interface is in slave mode. See Section 34.7.2 for a summary of useful combinations for this bit with TXMODE."]
pub struct WS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WS_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `WS_HALFPERIOD` reader - Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31."]
pub struct WS_HALFPERIOD_R(crate::FieldReader<u16, u16>);
impl WS_HALFPERIOD_R {
    pub(crate) fn new(bits: u16) -> Self {
        WS_HALFPERIOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WS_HALFPERIOD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WS_HALFPERIOD` writer - Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31."]
pub struct WS_HALFPERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> WS_HALFPERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 6)) | ((value as u32 & 0x01ff) << 6);
        self.w
    }
}
#[doc = "Field `MUTE` reader - When 1, the transmit channel sends only zeroes."]
pub struct MUTE_R(crate::FieldReader<bool, bool>);
impl MUTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUTE` writer - When 1, the transmit channel sends only zeroes."]
pub struct MUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects the number of bytes in data as follows:"]
    #[inline(always)]
    pub fn wordwidth(&self) -> WORDWIDTH_R {
        WORDWIDTH_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - When 1, data is of monaural format. When 0, the data is in stereo format."]
    #[inline(always)]
    pub fn mono(&self) -> MONO_R {
        MONO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When 1, disables accesses on FIFOs, places the transmit channel in mute mode."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When 1, asynchronously resets the transmit channel and FIFO."]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - When 0, the interface is in master mode. When 1, the interface is in slave mode. See Section 34.7.2 for a summary of useful combinations for this bit with TXMODE."]
    #[inline(always)]
    pub fn ws_sel(&self) -> WS_SEL_R {
        WS_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:14 - Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31."]
    #[inline(always)]
    pub fn ws_halfperiod(&self) -> WS_HALFPERIOD_R {
        WS_HALFPERIOD_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bit 15 - When 1, the transmit channel sends only zeroes."]
    #[inline(always)]
    pub fn mute(&self) -> MUTE_R {
        MUTE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects the number of bytes in data as follows:"]
    #[inline(always)]
    pub fn wordwidth(&mut self) -> WORDWIDTH_W {
        WORDWIDTH_W { w: self }
    }
    #[doc = "Bit 2 - When 1, data is of monaural format. When 0, the data is in stereo format."]
    #[inline(always)]
    pub fn mono(&mut self) -> MONO_W {
        MONO_W { w: self }
    }
    #[doc = "Bit 3 - When 1, disables accesses on FIFOs, places the transmit channel in mute mode."]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 4 - When 1, asynchronously resets the transmit channel and FIFO."]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bit 5 - When 0, the interface is in master mode. When 1, the interface is in slave mode. See Section 34.7.2 for a summary of useful combinations for this bit with TXMODE."]
    #[inline(always)]
    pub fn ws_sel(&mut self) -> WS_SEL_W {
        WS_SEL_W { w: self }
    }
    #[doc = "Bits 6:14 - Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31."]
    #[inline(always)]
    pub fn ws_halfperiod(&mut self) -> WS_HALFPERIOD_W {
        WS_HALFPERIOD_W { w: self }
    }
    #[doc = "Bit 15 - When 1, the transmit channel sends only zeroes."]
    #[inline(always)]
    pub fn mute(&mut self) -> MUTE_W {
        MUTE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S Digital Audio Output Register. Contains control bits for the I2S transmit channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dao](index.html) module"]
pub struct DAO_SPEC;
impl crate::RegisterSpec for DAO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dao::R](R) reader structure"]
impl crate::Readable for DAO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dao::W](W) writer structure"]
impl crate::Writable for DAO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAO to value 0x87e1"]
impl crate::Resettable for DAO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x87e1
    }
}
