#[doc = "Register `CFG_SIO` reader"]
pub struct R(crate::R<CFG_SIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_SIO` writer"]
pub struct W(crate::W<CFG_SIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SIO_SPEC>;
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
impl From<crate::W<CFG_SIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VREG_EN01` reader - Selects the output buffer mode: '0': Unregulated output buffer '1': Regulated output buffer The regulated output mode is selected ONLY if the CFG.DRIVE_MODE bits are set to the strong pull up (Z_1 = '5') mode. If the CFG.DRIVE_MODE bits are set to any other mode the regulated output buffer will be disabled and the standard CMOS output buffer is used."]
pub struct VREG_EN01_R(crate::FieldReader<bool, bool>);
impl VREG_EN01_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREG_EN01_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREG_EN01_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREG_EN01` writer - Selects the output buffer mode: '0': Unregulated output buffer '1': Regulated output buffer The regulated output mode is selected ONLY if the CFG.DRIVE_MODE bits are set to the strong pull up (Z_1 = '5') mode. If the CFG.DRIVE_MODE bits are set to any other mode the regulated output buffer will be disabled and the standard CMOS output buffer is used."]
pub struct VREG_EN01_W<'a> {
    w: &'a mut W,
}
impl<'a> VREG_EN01_W<'a> {
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
#[doc = "Field `IBUF_SEL01` reader - Selects the input buffer mode: 0: Singled ended input buffer 1: Differential input buffer"]
pub struct IBUF_SEL01_R(crate::FieldReader<bool, bool>);
impl IBUF_SEL01_R {
    pub(crate) fn new(bits: bool) -> Self {
        IBUF_SEL01_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBUF_SEL01_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBUF_SEL01` writer - Selects the input buffer mode: 0: Singled ended input buffer 1: Differential input buffer"]
pub struct IBUF_SEL01_W<'a> {
    w: &'a mut W,
}
impl<'a> IBUF_SEL01_W<'a> {
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
#[doc = "Field `VTRIP_SEL01` reader - Selects the input buffer trip-point in single ended input buffer mode (IBUF_SEL = '0'): '0': Input buffer functions as a CMOS input buffer. '1': Input buffer functions as a TTL input buffer. In differential input buffer mode (IBUF_SEL = '1') '0': Trip-point is 0.5*Vddio or 0.5*Voh (depends on VREF_SEL/VOH_SEL) '1': Trip-point is 0.4*Vddio or 1.0*Vref (depends on VREF_SEL)"]
pub struct VTRIP_SEL01_R(crate::FieldReader<bool, bool>);
impl VTRIP_SEL01_R {
    pub(crate) fn new(bits: bool) -> Self {
        VTRIP_SEL01_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTRIP_SEL01_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTRIP_SEL01` writer - Selects the input buffer trip-point in single ended input buffer mode (IBUF_SEL = '0'): '0': Input buffer functions as a CMOS input buffer. '1': Input buffer functions as a TTL input buffer. In differential input buffer mode (IBUF_SEL = '1') '0': Trip-point is 0.5*Vddio or 0.5*Voh (depends on VREF_SEL/VOH_SEL) '1': Trip-point is 0.4*Vddio or 1.0*Vref (depends on VREF_SEL)"]
pub struct VTRIP_SEL01_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL01_W<'a> {
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
#[doc = "Field `VREF_SEL01` reader - Selects reference voltage (Vref) trip-point of the input buffer: '0': Trip-point reference from pin_ref '1': Trip-point reference of SRSS internal reference Vref (1.2 V) '2': Trip-point reference of AMUXBUS_A '3': Trip-point reference of AMUXBUS_B"]
pub struct VREF_SEL01_R(crate::FieldReader<u8, u8>);
impl VREF_SEL01_R {
    pub(crate) fn new(bits: u8) -> Self {
        VREF_SEL01_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREF_SEL01_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREF_SEL01` writer - Selects reference voltage (Vref) trip-point of the input buffer: '0': Trip-point reference from pin_ref '1': Trip-point reference of SRSS internal reference Vref (1.2 V) '2': Trip-point reference of AMUXBUS_A '3': Trip-point reference of AMUXBUS_B"]
pub struct VREF_SEL01_W<'a> {
    w: &'a mut W,
}
impl<'a> VREF_SEL01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `VOH_SEL01` reader - Selects the regulated Voh output level and trip point of the input buffer for a specific SIO pin pair. Voh depends on the selected reference voltage (VREF_SEL). '0': Voh = 1*reference; e.g. reference at 1.2V -> Voh = 1.2V '1': Voh = 1.25*reference; e.g. reference at 1.2V -> Voh = 1.5V '2': Voh = 1.49*reference; e.g. reference at 1.2V -> Voh = ~1.8V '3': Voh = 1.67*reference; e.g. reference at 1.2V -> Voh = 2V '4': Voh = 2.08*reference; e.g. reference at 1.2V -> Voh = 2.5V '5': Voh = 2.5*reference; e.g. reference at 1.2V -> Voh = 3V '6': Voh = 2.78*reference; e.g. reference at 1.2V -> Voh = ~3.3V '7': Voh = 4.16*reference; e.g. reference at 1.2V -> Voh = 5.0V Note: The upper value on Voh is limited to Vddio - 400mV"]
pub struct VOH_SEL01_R(crate::FieldReader<u8, u8>);
impl VOH_SEL01_R {
    pub(crate) fn new(bits: u8) -> Self {
        VOH_SEL01_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VOH_SEL01_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VOH_SEL01` writer - Selects the regulated Voh output level and trip point of the input buffer for a specific SIO pin pair. Voh depends on the selected reference voltage (VREF_SEL). '0': Voh = 1*reference; e.g. reference at 1.2V -> Voh = 1.2V '1': Voh = 1.25*reference; e.g. reference at 1.2V -> Voh = 1.5V '2': Voh = 1.49*reference; e.g. reference at 1.2V -> Voh = ~1.8V '3': Voh = 1.67*reference; e.g. reference at 1.2V -> Voh = 2V '4': Voh = 2.08*reference; e.g. reference at 1.2V -> Voh = 2.5V '5': Voh = 2.5*reference; e.g. reference at 1.2V -> Voh = 3V '6': Voh = 2.78*reference; e.g. reference at 1.2V -> Voh = ~3.3V '7': Voh = 4.16*reference; e.g. reference at 1.2V -> Voh = 5.0V Note: The upper value on Voh is limited to Vddio - 400mV"]
pub struct VOH_SEL01_W<'a> {
    w: &'a mut W,
}
impl<'a> VOH_SEL01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `VREG_EN23` reader - See corresponding definition for IO pins 0 and 1"]
pub struct VREG_EN23_R(crate::FieldReader<bool, bool>);
impl VREG_EN23_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREG_EN23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREG_EN23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREG_EN23` writer - See corresponding definition for IO pins 0 and 1"]
pub struct VREG_EN23_W<'a> {
    w: &'a mut W,
}
impl<'a> VREG_EN23_W<'a> {
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
#[doc = "Field `IBUF_SEL23` reader - See corresponding definition for IO pins 0 and 1"]
pub struct IBUF_SEL23_R(crate::FieldReader<bool, bool>);
impl IBUF_SEL23_R {
    pub(crate) fn new(bits: bool) -> Self {
        IBUF_SEL23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBUF_SEL23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBUF_SEL23` writer - See corresponding definition for IO pins 0 and 1"]
pub struct IBUF_SEL23_W<'a> {
    w: &'a mut W,
}
impl<'a> IBUF_SEL23_W<'a> {
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
#[doc = "Field `VTRIP_SEL23` reader - See corresponding definition for IO pins 0 and 1"]
pub struct VTRIP_SEL23_R(crate::FieldReader<bool, bool>);
impl VTRIP_SEL23_R {
    pub(crate) fn new(bits: bool) -> Self {
        VTRIP_SEL23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTRIP_SEL23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTRIP_SEL23` writer - See corresponding definition for IO pins 0 and 1"]
pub struct VTRIP_SEL23_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL23_W<'a> {
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
#[doc = "Field `VREF_SEL23` reader - See corresponding definition for IO pins 0 and 1"]
pub struct VREF_SEL23_R(crate::FieldReader<u8, u8>);
impl VREF_SEL23_R {
    pub(crate) fn new(bits: u8) -> Self {
        VREF_SEL23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREF_SEL23_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREF_SEL23` writer - See corresponding definition for IO pins 0 and 1"]
pub struct VREF_SEL23_W<'a> {
    w: &'a mut W,
}
impl<'a> VREF_SEL23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "Field `VOH_SEL23` reader - See corresponding definition for IO pins 0 and 1"]
pub struct VOH_SEL23_R(crate::FieldReader<u8, u8>);
impl VOH_SEL23_R {
    pub(crate) fn new(bits: u8) -> Self {
        VOH_SEL23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VOH_SEL23_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VOH_SEL23` writer - See corresponding definition for IO pins 0 and 1"]
pub struct VOH_SEL23_W<'a> {
    w: &'a mut W,
}
impl<'a> VOH_SEL23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | ((value as u32 & 0x07) << 13);
        self.w
    }
}
#[doc = "Field `VREG_EN45` reader - See corresponding definition for IO pins 0 and 1"]
pub struct VREG_EN45_R(crate::FieldReader<bool, bool>);
impl VREG_EN45_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREG_EN45_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREG_EN45_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREG_EN45` writer - See corresponding definition for IO pins 0 and 1"]
pub struct VREG_EN45_W<'a> {
    w: &'a mut W,
}
impl<'a> VREG_EN45_W<'a> {
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
#[doc = "Field `IBUF_SEL45` reader - See corresponding definition for IO pins 0 and 1"]
pub struct IBUF_SEL45_R(crate::FieldReader<bool, bool>);
impl IBUF_SEL45_R {
    pub(crate) fn new(bits: bool) -> Self {
        IBUF_SEL45_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBUF_SEL45_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBUF_SEL45` writer - See corresponding definition for IO pins 0 and 1"]
pub struct IBUF_SEL45_W<'a> {
    w: &'a mut W,
}
impl<'a> IBUF_SEL45_W<'a> {
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
#[doc = "Field `VTRIP_SEL45` reader - See corresponding definition for IO pins 0 and 1"]
pub struct VTRIP_SEL45_R(crate::FieldReader<bool, bool>);
impl VTRIP_SEL45_R {
    pub(crate) fn new(bits: bool) -> Self {
        VTRIP_SEL45_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTRIP_SEL45_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTRIP_SEL45` writer - See corresponding definition for IO pins 0 and 1"]
pub struct VTRIP_SEL45_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL45_W<'a> {
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
#[doc = "Field `VREF_SEL45` reader - See corresponding definition for IO pins 0 and 1"]
pub struct VREF_SEL45_R(crate::FieldReader<u8, u8>);
impl VREF_SEL45_R {
    pub(crate) fn new(bits: u8) -> Self {
        VREF_SEL45_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREF_SEL45_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREF_SEL45` writer - See corresponding definition for IO pins 0 and 1"]
pub struct VREF_SEL45_W<'a> {
    w: &'a mut W,
}
impl<'a> VREF_SEL45_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | ((value as u32 & 0x03) << 19);
        self.w
    }
}
#[doc = "Field `VOH_SEL45` reader - See corresponding definition for IO pins 0 and 1"]
pub struct VOH_SEL45_R(crate::FieldReader<u8, u8>);
impl VOH_SEL45_R {
    pub(crate) fn new(bits: u8) -> Self {
        VOH_SEL45_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VOH_SEL45_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VOH_SEL45` writer - See corresponding definition for IO pins 0 and 1"]
pub struct VOH_SEL45_W<'a> {
    w: &'a mut W,
}
impl<'a> VOH_SEL45_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | ((value as u32 & 0x07) << 21);
        self.w
    }
}
#[doc = "Field `VREG_EN67` reader - See corresponding definition for IO pins 0 and 1"]
pub struct VREG_EN67_R(crate::FieldReader<bool, bool>);
impl VREG_EN67_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREG_EN67_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREG_EN67_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREG_EN67` writer - See corresponding definition for IO pins 0 and 1"]
pub struct VREG_EN67_W<'a> {
    w: &'a mut W,
}
impl<'a> VREG_EN67_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `IBUF_SEL67` reader - See corresponding definition for IO pins 0 and 1"]
pub struct IBUF_SEL67_R(crate::FieldReader<bool, bool>);
impl IBUF_SEL67_R {
    pub(crate) fn new(bits: bool) -> Self {
        IBUF_SEL67_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBUF_SEL67_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBUF_SEL67` writer - See corresponding definition for IO pins 0 and 1"]
pub struct IBUF_SEL67_W<'a> {
    w: &'a mut W,
}
impl<'a> IBUF_SEL67_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `VTRIP_SEL67` reader - See corresponding definition for IO pins 0 and 1"]
pub struct VTRIP_SEL67_R(crate::FieldReader<bool, bool>);
impl VTRIP_SEL67_R {
    pub(crate) fn new(bits: bool) -> Self {
        VTRIP_SEL67_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTRIP_SEL67_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTRIP_SEL67` writer - See corresponding definition for IO pins 0 and 1"]
pub struct VTRIP_SEL67_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL67_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `VREF_SEL67` reader - See corresponding definition for IO pins 0 and 1"]
pub struct VREF_SEL67_R(crate::FieldReader<u8, u8>);
impl VREF_SEL67_R {
    pub(crate) fn new(bits: u8) -> Self {
        VREF_SEL67_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREF_SEL67_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREF_SEL67` writer - See corresponding definition for IO pins 0 and 1"]
pub struct VREF_SEL67_W<'a> {
    w: &'a mut W,
}
impl<'a> VREF_SEL67_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | ((value as u32 & 0x03) << 27);
        self.w
    }
}
#[doc = "Field `VOH_SEL67` reader - See corresponding definition for IO pins 0 and 1"]
pub struct VOH_SEL67_R(crate::FieldReader<u8, u8>);
impl VOH_SEL67_R {
    pub(crate) fn new(bits: u8) -> Self {
        VOH_SEL67_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VOH_SEL67_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VOH_SEL67` writer - See corresponding definition for IO pins 0 and 1"]
pub struct VOH_SEL67_W<'a> {
    w: &'a mut W,
}
impl<'a> VOH_SEL67_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | ((value as u32 & 0x07) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Selects the output buffer mode: '0': Unregulated output buffer '1': Regulated output buffer The regulated output mode is selected ONLY if the CFG.DRIVE_MODE bits are set to the strong pull up (Z_1 = '5') mode. If the CFG.DRIVE_MODE bits are set to any other mode the regulated output buffer will be disabled and the standard CMOS output buffer is used."]
    #[inline(always)]
    pub fn vreg_en01(&self) -> VREG_EN01_R {
        VREG_EN01_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Selects the input buffer mode: 0: Singled ended input buffer 1: Differential input buffer"]
    #[inline(always)]
    pub fn ibuf_sel01(&self) -> IBUF_SEL01_R {
        IBUF_SEL01_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Selects the input buffer trip-point in single ended input buffer mode (IBUF_SEL = '0'): '0': Input buffer functions as a CMOS input buffer. '1': Input buffer functions as a TTL input buffer. In differential input buffer mode (IBUF_SEL = '1') '0': Trip-point is 0.5*Vddio or 0.5*Voh (depends on VREF_SEL/VOH_SEL) '1': Trip-point is 0.4*Vddio or 1.0*Vref (depends on VREF_SEL)"]
    #[inline(always)]
    pub fn vtrip_sel01(&self) -> VTRIP_SEL01_R {
        VTRIP_SEL01_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Selects reference voltage (Vref) trip-point of the input buffer: '0': Trip-point reference from pin_ref '1': Trip-point reference of SRSS internal reference Vref (1.2 V) '2': Trip-point reference of AMUXBUS_A '3': Trip-point reference of AMUXBUS_B"]
    #[inline(always)]
    pub fn vref_sel01(&self) -> VREF_SEL01_R {
        VREF_SEL01_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 5:7 - Selects the regulated Voh output level and trip point of the input buffer for a specific SIO pin pair. Voh depends on the selected reference voltage (VREF_SEL). '0': Voh = 1*reference; e.g. reference at 1.2V -> Voh = 1.2V '1': Voh = 1.25*reference; e.g. reference at 1.2V -> Voh = 1.5V '2': Voh = 1.49*reference; e.g. reference at 1.2V -> Voh = ~1.8V '3': Voh = 1.67*reference; e.g. reference at 1.2V -> Voh = 2V '4': Voh = 2.08*reference; e.g. reference at 1.2V -> Voh = 2.5V '5': Voh = 2.5*reference; e.g. reference at 1.2V -> Voh = 3V '6': Voh = 2.78*reference; e.g. reference at 1.2V -> Voh = ~3.3V '7': Voh = 4.16*reference; e.g. reference at 1.2V -> Voh = 5.0V Note: The upper value on Voh is limited to Vddio - 400mV"]
    #[inline(always)]
    pub fn voh_sel01(&self) -> VOH_SEL01_R {
        VOH_SEL01_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 8 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vreg_en23(&self) -> VREG_EN23_R {
        VREG_EN23_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn ibuf_sel23(&self) -> IBUF_SEL23_R {
        IBUF_SEL23_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vtrip_sel23(&self) -> VTRIP_SEL23_R {
        VTRIP_SEL23_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:12 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vref_sel23(&self) -> VREF_SEL23_R {
        VREF_SEL23_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 13:15 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn voh_sel23(&self) -> VOH_SEL23_R {
        VOH_SEL23_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bit 16 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vreg_en45(&self) -> VREG_EN45_R {
        VREG_EN45_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn ibuf_sel45(&self) -> IBUF_SEL45_R {
        IBUF_SEL45_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vtrip_sel45(&self) -> VTRIP_SEL45_R {
        VTRIP_SEL45_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 19:20 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vref_sel45(&self) -> VREF_SEL45_R {
        VREF_SEL45_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bits 21:23 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn voh_sel45(&self) -> VOH_SEL45_R {
        VOH_SEL45_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bit 24 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vreg_en67(&self) -> VREG_EN67_R {
        VREG_EN67_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn ibuf_sel67(&self) -> IBUF_SEL67_R {
        IBUF_SEL67_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vtrip_sel67(&self) -> VTRIP_SEL67_R {
        VTRIP_SEL67_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 27:28 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vref_sel67(&self) -> VREF_SEL67_R {
        VREF_SEL67_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bits 29:31 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn voh_sel67(&self) -> VOH_SEL67_R {
        VOH_SEL67_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Selects the output buffer mode: '0': Unregulated output buffer '1': Regulated output buffer The regulated output mode is selected ONLY if the CFG.DRIVE_MODE bits are set to the strong pull up (Z_1 = '5') mode. If the CFG.DRIVE_MODE bits are set to any other mode the regulated output buffer will be disabled and the standard CMOS output buffer is used."]
    #[inline(always)]
    pub fn vreg_en01(&mut self) -> VREG_EN01_W {
        VREG_EN01_W { w: self }
    }
    #[doc = "Bit 1 - Selects the input buffer mode: 0: Singled ended input buffer 1: Differential input buffer"]
    #[inline(always)]
    pub fn ibuf_sel01(&mut self) -> IBUF_SEL01_W {
        IBUF_SEL01_W { w: self }
    }
    #[doc = "Bit 2 - Selects the input buffer trip-point in single ended input buffer mode (IBUF_SEL = '0'): '0': Input buffer functions as a CMOS input buffer. '1': Input buffer functions as a TTL input buffer. In differential input buffer mode (IBUF_SEL = '1') '0': Trip-point is 0.5*Vddio or 0.5*Voh (depends on VREF_SEL/VOH_SEL) '1': Trip-point is 0.4*Vddio or 1.0*Vref (depends on VREF_SEL)"]
    #[inline(always)]
    pub fn vtrip_sel01(&mut self) -> VTRIP_SEL01_W {
        VTRIP_SEL01_W { w: self }
    }
    #[doc = "Bits 3:4 - Selects reference voltage (Vref) trip-point of the input buffer: '0': Trip-point reference from pin_ref '1': Trip-point reference of SRSS internal reference Vref (1.2 V) '2': Trip-point reference of AMUXBUS_A '3': Trip-point reference of AMUXBUS_B"]
    #[inline(always)]
    pub fn vref_sel01(&mut self) -> VREF_SEL01_W {
        VREF_SEL01_W { w: self }
    }
    #[doc = "Bits 5:7 - Selects the regulated Voh output level and trip point of the input buffer for a specific SIO pin pair. Voh depends on the selected reference voltage (VREF_SEL). '0': Voh = 1*reference; e.g. reference at 1.2V -> Voh = 1.2V '1': Voh = 1.25*reference; e.g. reference at 1.2V -> Voh = 1.5V '2': Voh = 1.49*reference; e.g. reference at 1.2V -> Voh = ~1.8V '3': Voh = 1.67*reference; e.g. reference at 1.2V -> Voh = 2V '4': Voh = 2.08*reference; e.g. reference at 1.2V -> Voh = 2.5V '5': Voh = 2.5*reference; e.g. reference at 1.2V -> Voh = 3V '6': Voh = 2.78*reference; e.g. reference at 1.2V -> Voh = ~3.3V '7': Voh = 4.16*reference; e.g. reference at 1.2V -> Voh = 5.0V Note: The upper value on Voh is limited to Vddio - 400mV"]
    #[inline(always)]
    pub fn voh_sel01(&mut self) -> VOH_SEL01_W {
        VOH_SEL01_W { w: self }
    }
    #[doc = "Bit 8 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vreg_en23(&mut self) -> VREG_EN23_W {
        VREG_EN23_W { w: self }
    }
    #[doc = "Bit 9 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn ibuf_sel23(&mut self) -> IBUF_SEL23_W {
        IBUF_SEL23_W { w: self }
    }
    #[doc = "Bit 10 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vtrip_sel23(&mut self) -> VTRIP_SEL23_W {
        VTRIP_SEL23_W { w: self }
    }
    #[doc = "Bits 11:12 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vref_sel23(&mut self) -> VREF_SEL23_W {
        VREF_SEL23_W { w: self }
    }
    #[doc = "Bits 13:15 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn voh_sel23(&mut self) -> VOH_SEL23_W {
        VOH_SEL23_W { w: self }
    }
    #[doc = "Bit 16 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vreg_en45(&mut self) -> VREG_EN45_W {
        VREG_EN45_W { w: self }
    }
    #[doc = "Bit 17 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn ibuf_sel45(&mut self) -> IBUF_SEL45_W {
        IBUF_SEL45_W { w: self }
    }
    #[doc = "Bit 18 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vtrip_sel45(&mut self) -> VTRIP_SEL45_W {
        VTRIP_SEL45_W { w: self }
    }
    #[doc = "Bits 19:20 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vref_sel45(&mut self) -> VREF_SEL45_W {
        VREF_SEL45_W { w: self }
    }
    #[doc = "Bits 21:23 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn voh_sel45(&mut self) -> VOH_SEL45_W {
        VOH_SEL45_W { w: self }
    }
    #[doc = "Bit 24 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vreg_en67(&mut self) -> VREG_EN67_W {
        VREG_EN67_W { w: self }
    }
    #[doc = "Bit 25 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn ibuf_sel67(&mut self) -> IBUF_SEL67_W {
        IBUF_SEL67_W { w: self }
    }
    #[doc = "Bit 26 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vtrip_sel67(&mut self) -> VTRIP_SEL67_W {
        VTRIP_SEL67_W { w: self }
    }
    #[doc = "Bits 27:28 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vref_sel67(&mut self) -> VREF_SEL67_W {
        VREF_SEL67_W { w: self }
    }
    #[doc = "Bits 29:31 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn voh_sel67(&mut self) -> VOH_SEL67_W {
        VOH_SEL67_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port SIO configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_sio](index.html) module"]
pub struct CFG_SIO_SPEC;
impl crate::RegisterSpec for CFG_SIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_sio::R](R) reader structure"]
impl crate::Readable for CFG_SIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_sio::W](W) writer structure"]
impl crate::Writable for CFG_SIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG_SIO to value 0"]
impl crate::Resettable for CFG_SIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
