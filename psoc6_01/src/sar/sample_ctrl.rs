#[doc = "Register `SAMPLE_CTRL` reader"]
pub struct R(crate::R<SAMPLE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAMPLE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAMPLE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAMPLE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAMPLE_CTRL` writer"]
pub struct W(crate::W<SAMPLE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAMPLE_CTRL_SPEC>;
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
impl From<crate::W<SAMPLE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAMPLE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEFT_ALIGN` reader - Left align data in data\\[15:0\\], default data is right aligned in data\\[11:0\\], with sign extension to 16 bits if the channel is differential."]
pub struct LEFT_ALIGN_R(crate::FieldReader<bool, bool>);
impl LEFT_ALIGN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LEFT_ALIGN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEFT_ALIGN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEFT_ALIGN` writer - Left align data in data\\[15:0\\], default data is right aligned in data\\[11:0\\], with sign extension to 16 bits if the channel is differential."]
pub struct LEFT_ALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEFT_ALIGN_W<'a> {
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
#[doc = "Output data from a single ended conversion as a signed value If AVG_MODE = 1 (Interleaved averaging), then SINGLE_ENDED_SIGNED must be configured identically to DIFFERENTIAL_SIGNED.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SINGLE_ENDED_SIGNED_A {
    #[doc = "0: Default: result data is unsigned (zero extended if needed)"]
    UNSIGNED = 0,
    #[doc = "1: result data is signed (sign extended if needed)"]
    SIGNED = 1,
}
impl From<SINGLE_ENDED_SIGNED_A> for bool {
    #[inline(always)]
    fn from(variant: SINGLE_ENDED_SIGNED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SINGLE_ENDED_SIGNED` reader - Output data from a single ended conversion as a signed value If AVG_MODE = 1 (Interleaved averaging), then SINGLE_ENDED_SIGNED must be configured identically to DIFFERENTIAL_SIGNED."]
pub struct SINGLE_ENDED_SIGNED_R(crate::FieldReader<bool, SINGLE_ENDED_SIGNED_A>);
impl SINGLE_ENDED_SIGNED_R {
    pub(crate) fn new(bits: bool) -> Self {
        SINGLE_ENDED_SIGNED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SINGLE_ENDED_SIGNED_A {
        match self.bits {
            false => SINGLE_ENDED_SIGNED_A::UNSIGNED,
            true => SINGLE_ENDED_SIGNED_A::SIGNED,
        }
    }
    #[doc = "Checks if the value of the field is `UNSIGNED`"]
    #[inline(always)]
    pub fn is_unsigned(&self) -> bool {
        **self == SINGLE_ENDED_SIGNED_A::UNSIGNED
    }
    #[doc = "Checks if the value of the field is `SIGNED`"]
    #[inline(always)]
    pub fn is_signed(&self) -> bool {
        **self == SINGLE_ENDED_SIGNED_A::SIGNED
    }
}
impl core::ops::Deref for SINGLE_ENDED_SIGNED_R {
    type Target = crate::FieldReader<bool, SINGLE_ENDED_SIGNED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SINGLE_ENDED_SIGNED` writer - Output data from a single ended conversion as a signed value If AVG_MODE = 1 (Interleaved averaging), then SINGLE_ENDED_SIGNED must be configured identically to DIFFERENTIAL_SIGNED."]
pub struct SINGLE_ENDED_SIGNED_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLE_ENDED_SIGNED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SINGLE_ENDED_SIGNED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Default: result data is unsigned (zero extended if needed)"]
    #[inline(always)]
    pub fn unsigned(self) -> &'a mut W {
        self.variant(SINGLE_ENDED_SIGNED_A::UNSIGNED)
    }
    #[doc = "result data is signed (sign extended if needed)"]
    #[inline(always)]
    pub fn signed(self) -> &'a mut W {
        self.variant(SINGLE_ENDED_SIGNED_A::SIGNED)
    }
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
#[doc = "Output data from a differential conversion as a signed value when DIFFERENTIAL_EN or NEG_ADDR_EN is set to 1 If AVG_MODE = 1 (Interleaved averaging), then DIFFERENTIAL_SIGNED must be configured identically to SINGLE_ENDED_SIGNED.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIFFERENTIAL_SIGNED_A {
    #[doc = "0: result data is unsigned (zero extended if needed)"]
    UNSIGNED = 0,
    #[doc = "1: Default: result data is signed (sign extended if needed)"]
    SIGNED = 1,
}
impl From<DIFFERENTIAL_SIGNED_A> for bool {
    #[inline(always)]
    fn from(variant: DIFFERENTIAL_SIGNED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIFFERENTIAL_SIGNED` reader - Output data from a differential conversion as a signed value when DIFFERENTIAL_EN or NEG_ADDR_EN is set to 1 If AVG_MODE = 1 (Interleaved averaging), then DIFFERENTIAL_SIGNED must be configured identically to SINGLE_ENDED_SIGNED."]
pub struct DIFFERENTIAL_SIGNED_R(crate::FieldReader<bool, DIFFERENTIAL_SIGNED_A>);
impl DIFFERENTIAL_SIGNED_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIFFERENTIAL_SIGNED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIFFERENTIAL_SIGNED_A {
        match self.bits {
            false => DIFFERENTIAL_SIGNED_A::UNSIGNED,
            true => DIFFERENTIAL_SIGNED_A::SIGNED,
        }
    }
    #[doc = "Checks if the value of the field is `UNSIGNED`"]
    #[inline(always)]
    pub fn is_unsigned(&self) -> bool {
        **self == DIFFERENTIAL_SIGNED_A::UNSIGNED
    }
    #[doc = "Checks if the value of the field is `SIGNED`"]
    #[inline(always)]
    pub fn is_signed(&self) -> bool {
        **self == DIFFERENTIAL_SIGNED_A::SIGNED
    }
}
impl core::ops::Deref for DIFFERENTIAL_SIGNED_R {
    type Target = crate::FieldReader<bool, DIFFERENTIAL_SIGNED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFFERENTIAL_SIGNED` writer - Output data from a differential conversion as a signed value when DIFFERENTIAL_EN or NEG_ADDR_EN is set to 1 If AVG_MODE = 1 (Interleaved averaging), then DIFFERENTIAL_SIGNED must be configured identically to SINGLE_ENDED_SIGNED."]
pub struct DIFFERENTIAL_SIGNED_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFFERENTIAL_SIGNED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFFERENTIAL_SIGNED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "result data is unsigned (zero extended if needed)"]
    #[inline(always)]
    pub fn unsigned(self) -> &'a mut W {
        self.variant(DIFFERENTIAL_SIGNED_A::UNSIGNED)
    }
    #[doc = "Default: result data is signed (sign extended if needed)"]
    #[inline(always)]
    pub fn signed(self) -> &'a mut W {
        self.variant(DIFFERENTIAL_SIGNED_A::SIGNED)
    }
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
#[doc = "Field `AVG_CNT` reader - Averaging Count for channels that have averaging enabled (AVG_EN). A channel will be sampled (1<<(AVG_CNT+1)) = \\[2..256\\]
times. - In ACCUNDUMP mode (1st order accumulate and dump filter) a channel will be sampled back to back, the average result is calculated and stored and then the next enabled channel is sampled. If shifting is not enabled (AVG_SHIFT=0) then the result is forced to shift right so that is fits in 16 bits, so right shift is done by max(0,AVG_CNT-3). - In INTERLEAVED mode one sample is taken per triggered scan, only in the scan where the final averaging count is reached a valid average is calculated and stored in the RESULT register (by definition the same scan for all the channels that have averaging enabled). In all other scans the RESULT register for averaged channels will have an invalid result and the intermediate accumulated value is stored in the 16-bit WORK register. In this mode make sure that the averaging count is low enough to ensure that the intermediate value does not exceed 16-bits otherwise the MSBs will be lost. So for a 12-bit resolution the averaging count should be set to 16 or less (AVG_CNT=<3)."]
pub struct AVG_CNT_R(crate::FieldReader<u8, u8>);
impl AVG_CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        AVG_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVG_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVG_CNT` writer - Averaging Count for channels that have averaging enabled (AVG_EN). A channel will be sampled (1<<(AVG_CNT+1)) = \\[2..256\\]
times. - In ACCUNDUMP mode (1st order accumulate and dump filter) a channel will be sampled back to back, the average result is calculated and stored and then the next enabled channel is sampled. If shifting is not enabled (AVG_SHIFT=0) then the result is forced to shift right so that is fits in 16 bits, so right shift is done by max(0,AVG_CNT-3). - In INTERLEAVED mode one sample is taken per triggered scan, only in the scan where the final averaging count is reached a valid average is calculated and stored in the RESULT register (by definition the same scan for all the channels that have averaging enabled). In all other scans the RESULT register for averaged channels will have an invalid result and the intermediate accumulated value is stored in the 16-bit WORK register. In this mode make sure that the averaging count is low enough to ensure that the intermediate value does not exceed 16-bits otherwise the MSBs will be lost. So for a 12-bit resolution the averaging count should be set to 16 or less (AVG_CNT=<3)."]
pub struct AVG_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> AVG_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `AVG_SHIFT` reader - Averaging shifting: after averaging the result is shifted right to fit in 12 bits."]
pub struct AVG_SHIFT_R(crate::FieldReader<bool, bool>);
impl AVG_SHIFT_R {
    pub(crate) fn new(bits: bool) -> Self {
        AVG_SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVG_SHIFT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVG_SHIFT` writer - Averaging shifting: after averaging the result is shifted right to fit in 12 bits."]
pub struct AVG_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> AVG_SHIFT_W<'a> {
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
#[doc = "Averaging mode, in DSI mode this bit is ignored and only AccuNDump mode is available.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVG_MODE_A {
    #[doc = "0: Accumulate and Dump (1st order accumulate and dump filter): a channel will be sampled back to back and averaged"]
    ACCUNDUMP = 0,
    #[doc = "1: Interleaved: Each scan (trigger) one sample is taken per channel and averaged over several scans."]
    INTERLEAVED = 1,
}
impl From<AVG_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: AVG_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVG_MODE` reader - Averaging mode, in DSI mode this bit is ignored and only AccuNDump mode is available."]
pub struct AVG_MODE_R(crate::FieldReader<bool, AVG_MODE_A>);
impl AVG_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AVG_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVG_MODE_A {
        match self.bits {
            false => AVG_MODE_A::ACCUNDUMP,
            true => AVG_MODE_A::INTERLEAVED,
        }
    }
    #[doc = "Checks if the value of the field is `ACCUNDUMP`"]
    #[inline(always)]
    pub fn is_accundump(&self) -> bool {
        **self == AVG_MODE_A::ACCUNDUMP
    }
    #[doc = "Checks if the value of the field is `INTERLEAVED`"]
    #[inline(always)]
    pub fn is_interleaved(&self) -> bool {
        **self == AVG_MODE_A::INTERLEAVED
    }
}
impl core::ops::Deref for AVG_MODE_R {
    type Target = crate::FieldReader<bool, AVG_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVG_MODE` writer - Averaging mode, in DSI mode this bit is ignored and only AccuNDump mode is available."]
pub struct AVG_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> AVG_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVG_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Accumulate and Dump (1st order accumulate and dump filter): a channel will be sampled back to back and averaged"]
    #[inline(always)]
    pub fn accundump(self) -> &'a mut W {
        self.variant(AVG_MODE_A::ACCUNDUMP)
    }
    #[doc = "Interleaved: Each scan (trigger) one sample is taken per channel and averaged over several scans."]
    #[inline(always)]
    pub fn interleaved(self) -> &'a mut W {
        self.variant(AVG_MODE_A::INTERLEAVED)
    }
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
#[doc = "Field `CONTINUOUS` reader - - 0: Wait for next FW_TRIGGER (one shot) or hardware trigger (e.g. from TPWM for periodic triggering) before scanning enabled channels. - 1: Continuously scan enabled channels, ignore triggers."]
pub struct CONTINUOUS_R(crate::FieldReader<bool, bool>);
impl CONTINUOUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONTINUOUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONTINUOUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONTINUOUS` writer - - 0: Wait for next FW_TRIGGER (one shot) or hardware trigger (e.g. from TPWM for periodic triggering) before scanning enabled channels. - 1: Continuously scan enabled channels, ignore triggers."]
pub struct CONTINUOUS_W<'a> {
    w: &'a mut W,
}
impl<'a> CONTINUOUS_W<'a> {
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
#[doc = "Field `DSI_TRIGGER_EN` reader - - 0: firmware trigger only: disable hardware trigger tr_sar_in. - 1: enable hardware trigger tr_sar_in (e.g. from TCPWM, GPIO or UDB)."]
pub struct DSI_TRIGGER_EN_R(crate::FieldReader<bool, bool>);
impl DSI_TRIGGER_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSI_TRIGGER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSI_TRIGGER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSI_TRIGGER_EN` writer - - 0: firmware trigger only: disable hardware trigger tr_sar_in. - 1: enable hardware trigger tr_sar_in (e.g. from TCPWM, GPIO or UDB)."]
pub struct DSI_TRIGGER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSI_TRIGGER_EN_W<'a> {
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
#[doc = "Field `DSI_TRIGGER_LEVEL` reader - - 0: trigger signal is a pulse input, a positive edge detected on the trigger signal triggers a new scan. - 1: trigger signal is a level input, as long as the trigger signal remains high the SAR will do continuous scans."]
pub struct DSI_TRIGGER_LEVEL_R(crate::FieldReader<bool, bool>);
impl DSI_TRIGGER_LEVEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSI_TRIGGER_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSI_TRIGGER_LEVEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSI_TRIGGER_LEVEL` writer - - 0: trigger signal is a pulse input, a positive edge detected on the trigger signal triggers a new scan. - 1: trigger signal is a level input, as long as the trigger signal remains high the SAR will do continuous scans."]
pub struct DSI_TRIGGER_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DSI_TRIGGER_LEVEL_W<'a> {
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
#[doc = "Field `DSI_SYNC_TRIGGER` reader - - 0: bypass clock domain synchronization of the trigger signal. - 1: synchronize the trigger signal to the SAR clock domain, if needed an edge detect is done in the peripheral clock domain."]
pub struct DSI_SYNC_TRIGGER_R(crate::FieldReader<bool, bool>);
impl DSI_SYNC_TRIGGER_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSI_SYNC_TRIGGER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSI_SYNC_TRIGGER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSI_SYNC_TRIGGER` writer - - 0: bypass clock domain synchronization of the trigger signal. - 1: synchronize the trigger signal to the SAR clock domain, if needed an edge detect is done in the peripheral clock domain."]
pub struct DSI_SYNC_TRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> DSI_SYNC_TRIGGER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Select whether UABs are scheduled or unscheduled. When no UAB is scanned this selection is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UAB_SCAN_MODE_A {
    #[doc = "0: Unscheduled UABs: one or more of the UABs scanned by the SAR is not scheduled, for each channel that scans a UAB the SAR will wait for a positive edge on the trigger output of that UAB. Caveat: in this mode the length of SAR scan can be variable."]
    UNSCHEDULED = 0,
    #[doc = "1: Scheduled UABs: All UABs scanned by the SAR are assumed to be properly scheduled, i.e. their output is assumed to be valid when sampled by the SAR and the SAR does not wait. In this mode the length of the SAR scan is constant. \nThis mode requires that the SAR scans strictly periodically, i.e. the SAR has to either run continuously or has to be triggered by a periodic hardware trigger (TCPWM or UDB timer). It also requires that the end of the UAB valid phase is precisely aligned with the end of the SAR sample period (using UAB.STARTUP_DELAY). Normally this scheduling is done by Creator."]
    SCHEDULED = 1,
}
impl From<UAB_SCAN_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: UAB_SCAN_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UAB_SCAN_MODE` reader - Select whether UABs are scheduled or unscheduled. When no UAB is scanned this selection is ignored."]
pub struct UAB_SCAN_MODE_R(crate::FieldReader<bool, UAB_SCAN_MODE_A>);
impl UAB_SCAN_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UAB_SCAN_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UAB_SCAN_MODE_A {
        match self.bits {
            false => UAB_SCAN_MODE_A::UNSCHEDULED,
            true => UAB_SCAN_MODE_A::SCHEDULED,
        }
    }
    #[doc = "Checks if the value of the field is `UNSCHEDULED`"]
    #[inline(always)]
    pub fn is_unscheduled(&self) -> bool {
        **self == UAB_SCAN_MODE_A::UNSCHEDULED
    }
    #[doc = "Checks if the value of the field is `SCHEDULED`"]
    #[inline(always)]
    pub fn is_scheduled(&self) -> bool {
        **self == UAB_SCAN_MODE_A::SCHEDULED
    }
}
impl core::ops::Deref for UAB_SCAN_MODE_R {
    type Target = crate::FieldReader<bool, UAB_SCAN_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UAB_SCAN_MODE` writer - Select whether UABs are scheduled or unscheduled. When no UAB is scanned this selection is ignored."]
pub struct UAB_SCAN_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> UAB_SCAN_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UAB_SCAN_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Unscheduled UABs: one or more of the UABs scanned by the SAR is not scheduled, for each channel that scans a UAB the SAR will wait for a positive edge on the trigger output of that UAB. Caveat: in this mode the length of SAR scan can be variable."]
    #[inline(always)]
    pub fn unscheduled(self) -> &'a mut W {
        self.variant(UAB_SCAN_MODE_A::UNSCHEDULED)
    }
    #[doc = "Scheduled UABs: All UABs scanned by the SAR are assumed to be properly scheduled, i.e. their output is assumed to be valid when sampled by the SAR and the SAR does not wait. In this mode the length of the SAR scan is constant. This mode requires that the SAR scans strictly periodically, i.e. the SAR has to either run continuously or has to be triggered by a periodic hardware trigger (TCPWM or UDB timer). It also requires that the end of the UAB valid phase is precisely aligned with the end of the SAR sample period (using UAB.STARTUP_DELAY). Normally this scheduling is done by Creator."]
    #[inline(always)]
    pub fn scheduled(self) -> &'a mut W {
        self.variant(UAB_SCAN_MODE_A::SCHEDULED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `REPEAT_INVALID` reader - For unscheduled UAB_SCAN_MODE only, do the following if an invalid sample is received: - 0: use the last known valid sample for that channel and clear the NEWVALUE flag - 1: repeat the conversions until a valid sample is received (caveat: could be never if the UAB valid window is incorrectly schedule w.r.t. SAR sampling)"]
pub struct REPEAT_INVALID_R(crate::FieldReader<bool, bool>);
impl REPEAT_INVALID_R {
    pub(crate) fn new(bits: bool) -> Self {
        REPEAT_INVALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REPEAT_INVALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REPEAT_INVALID` writer - For unscheduled UAB_SCAN_MODE only, do the following if an invalid sample is received: - 0: use the last known valid sample for that channel and clear the NEWVALUE flag - 1: repeat the conversions until a valid sample is received (caveat: could be never if the UAB valid window is incorrectly schedule w.r.t. SAR sampling)"]
pub struct REPEAT_INVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> REPEAT_INVALID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `VALID_SEL` reader - Static UAB Valid select 0=UAB0 half 0 Valid output 1=UAB0 half 1 Valid output 2=UAB1 half 0 Valid output 3=UAB1 half 1 Valid output 4=UAB2 half 0 Valid output 5=UAB2 half 1 Valid output 6=UAB3 half 0 Valid output 7=UAB3 half 1 Valid output"]
pub struct VALID_SEL_R(crate::FieldReader<u8, u8>);
impl VALID_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        VALID_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALID_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VALID_SEL` writer - Static UAB Valid select 0=UAB0 half 0 Valid output 1=UAB0 half 1 Valid output 2=UAB1 half 0 Valid output 3=UAB1 half 1 Valid output 4=UAB2 half 0 Valid output 5=UAB2 half 1 Valid output 6=UAB3 half 0 Valid output 7=UAB3 half 1 Valid output"]
pub struct VALID_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VALID_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `VALID_SEL_EN` reader - Enable static UAB Valid selection (override Hardware)"]
pub struct VALID_SEL_EN_R(crate::FieldReader<bool, bool>);
impl VALID_SEL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VALID_SEL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALID_SEL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VALID_SEL_EN` writer - Enable static UAB Valid selection (override Hardware)"]
pub struct VALID_SEL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VALID_SEL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `VALID_IGNORE` reader - Ignore UAB valid signal, including the dynamic/Hardware from AROUTE and the static Valid selection from the VALID_SEL fields above"]
pub struct VALID_IGNORE_R(crate::FieldReader<bool, bool>);
impl VALID_IGNORE_R {
    pub(crate) fn new(bits: bool) -> Self {
        VALID_IGNORE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALID_IGNORE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VALID_IGNORE` writer - Ignore UAB valid signal, including the dynamic/Hardware from AROUTE and the static Valid selection from the VALID_SEL fields above"]
pub struct VALID_IGNORE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALID_IGNORE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `TRIGGER_OUT_EN` reader - SAR output trigger enable (used for UAB synchronization). To ensure multiple UABs starting at the same trigger it is recommended to use this bit to temporarily disable the trigger output until all those UABs are set to run (UAB.SRAM_CTRL.RUN=1)."]
pub struct TRIGGER_OUT_EN_R(crate::FieldReader<bool, bool>);
impl TRIGGER_OUT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRIGGER_OUT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIGGER_OUT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGGER_OUT_EN` writer - SAR output trigger enable (used for UAB synchronization). To ensure multiple UABs starting at the same trigger it is recommended to use this bit to temporarily disable the trigger output until all those UABs are set to run (UAB.SRAM_CTRL.RUN=1)."]
pub struct TRIGGER_OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGER_OUT_EN_W<'a> {
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
#[doc = "Field `EOS_DSI_OUT_EN` reader - Enable to output EOS_INTR to DSI. When enabled each time EOS_INTR is set by the hardware also a trigger pulse is send on the tr_sar_out signal."]
pub struct EOS_DSI_OUT_EN_R(crate::FieldReader<bool, bool>);
impl EOS_DSI_OUT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOS_DSI_OUT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOS_DSI_OUT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOS_DSI_OUT_EN` writer - Enable to output EOS_INTR to DSI. When enabled each time EOS_INTR is set by the hardware also a trigger pulse is send on the tr_sar_out signal."]
pub struct EOS_DSI_OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EOS_DSI_OUT_EN_W<'a> {
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
    #[doc = "Bit 1 - Left align data in data\\[15:0\\], default data is right aligned in data\\[11:0\\], with sign extension to 16 bits if the channel is differential."]
    #[inline(always)]
    pub fn left_align(&self) -> LEFT_ALIGN_R {
        LEFT_ALIGN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Output data from a single ended conversion as a signed value If AVG_MODE = 1 (Interleaved averaging), then SINGLE_ENDED_SIGNED must be configured identically to DIFFERENTIAL_SIGNED."]
    #[inline(always)]
    pub fn single_ended_signed(&self) -> SINGLE_ENDED_SIGNED_R {
        SINGLE_ENDED_SIGNED_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Output data from a differential conversion as a signed value when DIFFERENTIAL_EN or NEG_ADDR_EN is set to 1 If AVG_MODE = 1 (Interleaved averaging), then DIFFERENTIAL_SIGNED must be configured identically to SINGLE_ENDED_SIGNED."]
    #[inline(always)]
    pub fn differential_signed(&self) -> DIFFERENTIAL_SIGNED_R {
        DIFFERENTIAL_SIGNED_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Averaging Count for channels that have averaging enabled (AVG_EN). A channel will be sampled (1<<(AVG_CNT+1)) = \\[2..256\\]
times. - In ACCUNDUMP mode (1st order accumulate and dump filter) a channel will be sampled back to back, the average result is calculated and stored and then the next enabled channel is sampled. If shifting is not enabled (AVG_SHIFT=0) then the result is forced to shift right so that is fits in 16 bits, so right shift is done by max(0,AVG_CNT-3). - In INTERLEAVED mode one sample is taken per triggered scan, only in the scan where the final averaging count is reached a valid average is calculated and stored in the RESULT register (by definition the same scan for all the channels that have averaging enabled). In all other scans the RESULT register for averaged channels will have an invalid result and the intermediate accumulated value is stored in the 16-bit WORK register. In this mode make sure that the averaging count is low enough to ensure that the intermediate value does not exceed 16-bits otherwise the MSBs will be lost. So for a 12-bit resolution the averaging count should be set to 16 or less (AVG_CNT=<3)."]
    #[inline(always)]
    pub fn avg_cnt(&self) -> AVG_CNT_R {
        AVG_CNT_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Averaging shifting: after averaging the result is shifted right to fit in 12 bits."]
    #[inline(always)]
    pub fn avg_shift(&self) -> AVG_SHIFT_R {
        AVG_SHIFT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Averaging mode, in DSI mode this bit is ignored and only AccuNDump mode is available."]
    #[inline(always)]
    pub fn avg_mode(&self) -> AVG_MODE_R {
        AVG_MODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - - 0: Wait for next FW_TRIGGER (one shot) or hardware trigger (e.g. from TPWM for periodic triggering) before scanning enabled channels. - 1: Continuously scan enabled channels, ignore triggers."]
    #[inline(always)]
    pub fn continuous(&self) -> CONTINUOUS_R {
        CONTINUOUS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - - 0: firmware trigger only: disable hardware trigger tr_sar_in. - 1: enable hardware trigger tr_sar_in (e.g. from TCPWM, GPIO or UDB)."]
    #[inline(always)]
    pub fn dsi_trigger_en(&self) -> DSI_TRIGGER_EN_R {
        DSI_TRIGGER_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - - 0: trigger signal is a pulse input, a positive edge detected on the trigger signal triggers a new scan. - 1: trigger signal is a level input, as long as the trigger signal remains high the SAR will do continuous scans."]
    #[inline(always)]
    pub fn dsi_trigger_level(&self) -> DSI_TRIGGER_LEVEL_R {
        DSI_TRIGGER_LEVEL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - - 0: bypass clock domain synchronization of the trigger signal. - 1: synchronize the trigger signal to the SAR clock domain, if needed an edge detect is done in the peripheral clock domain."]
    #[inline(always)]
    pub fn dsi_sync_trigger(&self) -> DSI_SYNC_TRIGGER_R {
        DSI_SYNC_TRIGGER_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Select whether UABs are scheduled or unscheduled. When no UAB is scanned this selection is ignored."]
    #[inline(always)]
    pub fn uab_scan_mode(&self) -> UAB_SCAN_MODE_R {
        UAB_SCAN_MODE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - For unscheduled UAB_SCAN_MODE only, do the following if an invalid sample is received: - 0: use the last known valid sample for that channel and clear the NEWVALUE flag - 1: repeat the conversions until a valid sample is received (caveat: could be never if the UAB valid window is incorrectly schedule w.r.t. SAR sampling)"]
    #[inline(always)]
    pub fn repeat_invalid(&self) -> REPEAT_INVALID_R {
        REPEAT_INVALID_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Static UAB Valid select 0=UAB0 half 0 Valid output 1=UAB0 half 1 Valid output 2=UAB1 half 0 Valid output 3=UAB1 half 1 Valid output 4=UAB2 half 0 Valid output 5=UAB2 half 1 Valid output 6=UAB3 half 0 Valid output 7=UAB3 half 1 Valid output"]
    #[inline(always)]
    pub fn valid_sel(&self) -> VALID_SEL_R {
        VALID_SEL_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 27 - Enable static UAB Valid selection (override Hardware)"]
    #[inline(always)]
    pub fn valid_sel_en(&self) -> VALID_SEL_EN_R {
        VALID_SEL_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Ignore UAB valid signal, including the dynamic/Hardware from AROUTE and the static Valid selection from the VALID_SEL fields above"]
    #[inline(always)]
    pub fn valid_ignore(&self) -> VALID_IGNORE_R {
        VALID_IGNORE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - SAR output trigger enable (used for UAB synchronization). To ensure multiple UABs starting at the same trigger it is recommended to use this bit to temporarily disable the trigger output until all those UABs are set to run (UAB.SRAM_CTRL.RUN=1)."]
    #[inline(always)]
    pub fn trigger_out_en(&self) -> TRIGGER_OUT_EN_R {
        TRIGGER_OUT_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable to output EOS_INTR to DSI. When enabled each time EOS_INTR is set by the hardware also a trigger pulse is send on the tr_sar_out signal."]
    #[inline(always)]
    pub fn eos_dsi_out_en(&self) -> EOS_DSI_OUT_EN_R {
        EOS_DSI_OUT_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Left align data in data\\[15:0\\], default data is right aligned in data\\[11:0\\], with sign extension to 16 bits if the channel is differential."]
    #[inline(always)]
    pub fn left_align(&mut self) -> LEFT_ALIGN_W {
        LEFT_ALIGN_W { w: self }
    }
    #[doc = "Bit 2 - Output data from a single ended conversion as a signed value If AVG_MODE = 1 (Interleaved averaging), then SINGLE_ENDED_SIGNED must be configured identically to DIFFERENTIAL_SIGNED."]
    #[inline(always)]
    pub fn single_ended_signed(&mut self) -> SINGLE_ENDED_SIGNED_W {
        SINGLE_ENDED_SIGNED_W { w: self }
    }
    #[doc = "Bit 3 - Output data from a differential conversion as a signed value when DIFFERENTIAL_EN or NEG_ADDR_EN is set to 1 If AVG_MODE = 1 (Interleaved averaging), then DIFFERENTIAL_SIGNED must be configured identically to SINGLE_ENDED_SIGNED."]
    #[inline(always)]
    pub fn differential_signed(&mut self) -> DIFFERENTIAL_SIGNED_W {
        DIFFERENTIAL_SIGNED_W { w: self }
    }
    #[doc = "Bits 4:6 - Averaging Count for channels that have averaging enabled (AVG_EN). A channel will be sampled (1<<(AVG_CNT+1)) = \\[2..256\\]
times. - In ACCUNDUMP mode (1st order accumulate and dump filter) a channel will be sampled back to back, the average result is calculated and stored and then the next enabled channel is sampled. If shifting is not enabled (AVG_SHIFT=0) then the result is forced to shift right so that is fits in 16 bits, so right shift is done by max(0,AVG_CNT-3). - In INTERLEAVED mode one sample is taken per triggered scan, only in the scan where the final averaging count is reached a valid average is calculated and stored in the RESULT register (by definition the same scan for all the channels that have averaging enabled). In all other scans the RESULT register for averaged channels will have an invalid result and the intermediate accumulated value is stored in the 16-bit WORK register. In this mode make sure that the averaging count is low enough to ensure that the intermediate value does not exceed 16-bits otherwise the MSBs will be lost. So for a 12-bit resolution the averaging count should be set to 16 or less (AVG_CNT=<3)."]
    #[inline(always)]
    pub fn avg_cnt(&mut self) -> AVG_CNT_W {
        AVG_CNT_W { w: self }
    }
    #[doc = "Bit 7 - Averaging shifting: after averaging the result is shifted right to fit in 12 bits."]
    #[inline(always)]
    pub fn avg_shift(&mut self) -> AVG_SHIFT_W {
        AVG_SHIFT_W { w: self }
    }
    #[doc = "Bit 8 - Averaging mode, in DSI mode this bit is ignored and only AccuNDump mode is available."]
    #[inline(always)]
    pub fn avg_mode(&mut self) -> AVG_MODE_W {
        AVG_MODE_W { w: self }
    }
    #[doc = "Bit 16 - - 0: Wait for next FW_TRIGGER (one shot) or hardware trigger (e.g. from TPWM for periodic triggering) before scanning enabled channels. - 1: Continuously scan enabled channels, ignore triggers."]
    #[inline(always)]
    pub fn continuous(&mut self) -> CONTINUOUS_W {
        CONTINUOUS_W { w: self }
    }
    #[doc = "Bit 17 - - 0: firmware trigger only: disable hardware trigger tr_sar_in. - 1: enable hardware trigger tr_sar_in (e.g. from TCPWM, GPIO or UDB)."]
    #[inline(always)]
    pub fn dsi_trigger_en(&mut self) -> DSI_TRIGGER_EN_W {
        DSI_TRIGGER_EN_W { w: self }
    }
    #[doc = "Bit 18 - - 0: trigger signal is a pulse input, a positive edge detected on the trigger signal triggers a new scan. - 1: trigger signal is a level input, as long as the trigger signal remains high the SAR will do continuous scans."]
    #[inline(always)]
    pub fn dsi_trigger_level(&mut self) -> DSI_TRIGGER_LEVEL_W {
        DSI_TRIGGER_LEVEL_W { w: self }
    }
    #[doc = "Bit 19 - - 0: bypass clock domain synchronization of the trigger signal. - 1: synchronize the trigger signal to the SAR clock domain, if needed an edge detect is done in the peripheral clock domain."]
    #[inline(always)]
    pub fn dsi_sync_trigger(&mut self) -> DSI_SYNC_TRIGGER_W {
        DSI_SYNC_TRIGGER_W { w: self }
    }
    #[doc = "Bit 22 - Select whether UABs are scheduled or unscheduled. When no UAB is scanned this selection is ignored."]
    #[inline(always)]
    pub fn uab_scan_mode(&mut self) -> UAB_SCAN_MODE_W {
        UAB_SCAN_MODE_W { w: self }
    }
    #[doc = "Bit 23 - For unscheduled UAB_SCAN_MODE only, do the following if an invalid sample is received: - 0: use the last known valid sample for that channel and clear the NEWVALUE flag - 1: repeat the conversions until a valid sample is received (caveat: could be never if the UAB valid window is incorrectly schedule w.r.t. SAR sampling)"]
    #[inline(always)]
    pub fn repeat_invalid(&mut self) -> REPEAT_INVALID_W {
        REPEAT_INVALID_W { w: self }
    }
    #[doc = "Bits 24:26 - Static UAB Valid select 0=UAB0 half 0 Valid output 1=UAB0 half 1 Valid output 2=UAB1 half 0 Valid output 3=UAB1 half 1 Valid output 4=UAB2 half 0 Valid output 5=UAB2 half 1 Valid output 6=UAB3 half 0 Valid output 7=UAB3 half 1 Valid output"]
    #[inline(always)]
    pub fn valid_sel(&mut self) -> VALID_SEL_W {
        VALID_SEL_W { w: self }
    }
    #[doc = "Bit 27 - Enable static UAB Valid selection (override Hardware)"]
    #[inline(always)]
    pub fn valid_sel_en(&mut self) -> VALID_SEL_EN_W {
        VALID_SEL_EN_W { w: self }
    }
    #[doc = "Bit 28 - Ignore UAB valid signal, including the dynamic/Hardware from AROUTE and the static Valid selection from the VALID_SEL fields above"]
    #[inline(always)]
    pub fn valid_ignore(&mut self) -> VALID_IGNORE_W {
        VALID_IGNORE_W { w: self }
    }
    #[doc = "Bit 30 - SAR output trigger enable (used for UAB synchronization). To ensure multiple UABs starting at the same trigger it is recommended to use this bit to temporarily disable the trigger output until all those UABs are set to run (UAB.SRAM_CTRL.RUN=1)."]
    #[inline(always)]
    pub fn trigger_out_en(&mut self) -> TRIGGER_OUT_EN_W {
        TRIGGER_OUT_EN_W { w: self }
    }
    #[doc = "Bit 31 - Enable to output EOS_INTR to DSI. When enabled each time EOS_INTR is set by the hardware also a trigger pulse is send on the tr_sar_out signal."]
    #[inline(always)]
    pub fn eos_dsi_out_en(&mut self) -> EOS_DSI_OUT_EN_W {
        EOS_DSI_OUT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sample control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sample_ctrl](index.html) module"]
pub struct SAMPLE_CTRL_SPEC;
impl crate::RegisterSpec for SAMPLE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sample_ctrl::R](R) reader structure"]
impl crate::Readable for SAMPLE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sample_ctrl::W](W) writer structure"]
impl crate::Writable for SAMPLE_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAMPLE_CTRL to value 0x0008_0008"]
impl crate::Resettable for SAMPLE_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0008_0008
    }
}
