#[doc = "Register `MUX_SWITCH_SQ_CTRL` reader"]
pub struct R(crate::R<MUX_SWITCH_SQ_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MUX_SWITCH_SQ_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MUX_SWITCH_SQ_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MUX_SWITCH_SQ_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MUX_SWITCH_SQ_CTRL` writer"]
pub struct W(crate::W<MUX_SWITCH_SQ_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MUX_SWITCH_SQ_CTRL_SPEC>;
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
impl From<crate::W<MUX_SWITCH_SQ_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MUX_SWITCH_SQ_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MUX_SQ_CTRL_P0` reader - for P0 switches"]
pub struct MUX_SQ_CTRL_P0_R(crate::FieldReader<bool, bool>);
impl MUX_SQ_CTRL_P0_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_SQ_CTRL_P0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_SQ_CTRL_P0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_SQ_CTRL_P0` writer - for P0 switches"]
pub struct MUX_SQ_CTRL_P0_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_SQ_CTRL_P0_W<'a> {
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
#[doc = "Field `MUX_SQ_CTRL_P1` reader - for P1 switches"]
pub struct MUX_SQ_CTRL_P1_R(crate::FieldReader<bool, bool>);
impl MUX_SQ_CTRL_P1_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_SQ_CTRL_P1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_SQ_CTRL_P1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_SQ_CTRL_P1` writer - for P1 switches"]
pub struct MUX_SQ_CTRL_P1_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_SQ_CTRL_P1_W<'a> {
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
#[doc = "Field `MUX_SQ_CTRL_P2` reader - for P2 switches"]
pub struct MUX_SQ_CTRL_P2_R(crate::FieldReader<bool, bool>);
impl MUX_SQ_CTRL_P2_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_SQ_CTRL_P2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_SQ_CTRL_P2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_SQ_CTRL_P2` writer - for P2 switches"]
pub struct MUX_SQ_CTRL_P2_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_SQ_CTRL_P2_W<'a> {
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
#[doc = "Field `MUX_SQ_CTRL_P3` reader - for P3 switches"]
pub struct MUX_SQ_CTRL_P3_R(crate::FieldReader<bool, bool>);
impl MUX_SQ_CTRL_P3_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_SQ_CTRL_P3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_SQ_CTRL_P3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_SQ_CTRL_P3` writer - for P3 switches"]
pub struct MUX_SQ_CTRL_P3_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_SQ_CTRL_P3_W<'a> {
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
#[doc = "Field `MUX_SQ_CTRL_P4` reader - for P4 switches"]
pub struct MUX_SQ_CTRL_P4_R(crate::FieldReader<bool, bool>);
impl MUX_SQ_CTRL_P4_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_SQ_CTRL_P4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_SQ_CTRL_P4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_SQ_CTRL_P4` writer - for P4 switches"]
pub struct MUX_SQ_CTRL_P4_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_SQ_CTRL_P4_W<'a> {
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
#[doc = "Field `MUX_SQ_CTRL_P5` reader - for P5 switches"]
pub struct MUX_SQ_CTRL_P5_R(crate::FieldReader<bool, bool>);
impl MUX_SQ_CTRL_P5_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_SQ_CTRL_P5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_SQ_CTRL_P5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_SQ_CTRL_P5` writer - for P5 switches"]
pub struct MUX_SQ_CTRL_P5_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_SQ_CTRL_P5_W<'a> {
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
#[doc = "Field `MUX_SQ_CTRL_P6` reader - for P6 switches"]
pub struct MUX_SQ_CTRL_P6_R(crate::FieldReader<bool, bool>);
impl MUX_SQ_CTRL_P6_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_SQ_CTRL_P6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_SQ_CTRL_P6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_SQ_CTRL_P6` writer - for P6 switches"]
pub struct MUX_SQ_CTRL_P6_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_SQ_CTRL_P6_W<'a> {
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
#[doc = "Field `MUX_SQ_CTRL_P7` reader - for P7 switches"]
pub struct MUX_SQ_CTRL_P7_R(crate::FieldReader<bool, bool>);
impl MUX_SQ_CTRL_P7_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_SQ_CTRL_P7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_SQ_CTRL_P7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_SQ_CTRL_P7` writer - for P7 switches"]
pub struct MUX_SQ_CTRL_P7_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_SQ_CTRL_P7_W<'a> {
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
#[doc = "Field `MUX_SQ_CTRL_VSSA` reader - for vssa switch"]
pub struct MUX_SQ_CTRL_VSSA_R(crate::FieldReader<bool, bool>);
impl MUX_SQ_CTRL_VSSA_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_SQ_CTRL_VSSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_SQ_CTRL_VSSA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_SQ_CTRL_VSSA` writer - for vssa switch"]
pub struct MUX_SQ_CTRL_VSSA_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_SQ_CTRL_VSSA_W<'a> {
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
#[doc = "Field `MUX_SQ_CTRL_TEMP` reader - for temp switch"]
pub struct MUX_SQ_CTRL_TEMP_R(crate::FieldReader<bool, bool>);
impl MUX_SQ_CTRL_TEMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_SQ_CTRL_TEMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_SQ_CTRL_TEMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_SQ_CTRL_TEMP` writer - for temp switch"]
pub struct MUX_SQ_CTRL_TEMP_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_SQ_CTRL_TEMP_W<'a> {
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
#[doc = "Field `MUX_SQ_CTRL_AMUXBUSA` reader - for amuxbusa switch"]
pub struct MUX_SQ_CTRL_AMUXBUSA_R(crate::FieldReader<bool, bool>);
impl MUX_SQ_CTRL_AMUXBUSA_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_SQ_CTRL_AMUXBUSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_SQ_CTRL_AMUXBUSA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_SQ_CTRL_AMUXBUSA` writer - for amuxbusa switch"]
pub struct MUX_SQ_CTRL_AMUXBUSA_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_SQ_CTRL_AMUXBUSA_W<'a> {
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
#[doc = "Field `MUX_SQ_CTRL_AMUXBUSB` reader - for amuxbusb switches"]
pub struct MUX_SQ_CTRL_AMUXBUSB_R(crate::FieldReader<bool, bool>);
impl MUX_SQ_CTRL_AMUXBUSB_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_SQ_CTRL_AMUXBUSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_SQ_CTRL_AMUXBUSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_SQ_CTRL_AMUXBUSB` writer - for amuxbusb switches"]
pub struct MUX_SQ_CTRL_AMUXBUSB_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_SQ_CTRL_AMUXBUSB_W<'a> {
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
#[doc = "Field `MUX_SQ_CTRL_SARBUS0` reader - for sarbus0 switch"]
pub struct MUX_SQ_CTRL_SARBUS0_R(crate::FieldReader<bool, bool>);
impl MUX_SQ_CTRL_SARBUS0_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_SQ_CTRL_SARBUS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_SQ_CTRL_SARBUS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_SQ_CTRL_SARBUS0` writer - for sarbus0 switch"]
pub struct MUX_SQ_CTRL_SARBUS0_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_SQ_CTRL_SARBUS0_W<'a> {
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
#[doc = "Field `MUX_SQ_CTRL_SARBUS1` reader - for sarbus1 switch"]
pub struct MUX_SQ_CTRL_SARBUS1_R(crate::FieldReader<bool, bool>);
impl MUX_SQ_CTRL_SARBUS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_SQ_CTRL_SARBUS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_SQ_CTRL_SARBUS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_SQ_CTRL_SARBUS1` writer - for sarbus1 switch"]
pub struct MUX_SQ_CTRL_SARBUS1_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_SQ_CTRL_SARBUS1_W<'a> {
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
impl R {
    #[doc = "Bit 0 - for P0 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p0(&self) -> MUX_SQ_CTRL_P0_R {
        MUX_SQ_CTRL_P0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - for P1 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p1(&self) -> MUX_SQ_CTRL_P1_R {
        MUX_SQ_CTRL_P1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - for P2 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p2(&self) -> MUX_SQ_CTRL_P2_R {
        MUX_SQ_CTRL_P2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - for P3 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p3(&self) -> MUX_SQ_CTRL_P3_R {
        MUX_SQ_CTRL_P3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - for P4 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p4(&self) -> MUX_SQ_CTRL_P4_R {
        MUX_SQ_CTRL_P4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - for P5 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p5(&self) -> MUX_SQ_CTRL_P5_R {
        MUX_SQ_CTRL_P5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - for P6 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p6(&self) -> MUX_SQ_CTRL_P6_R {
        MUX_SQ_CTRL_P6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - for P7 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p7(&self) -> MUX_SQ_CTRL_P7_R {
        MUX_SQ_CTRL_P7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - for vssa switch"]
    #[inline(always)]
    pub fn mux_sq_ctrl_vssa(&self) -> MUX_SQ_CTRL_VSSA_R {
        MUX_SQ_CTRL_VSSA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - for temp switch"]
    #[inline(always)]
    pub fn mux_sq_ctrl_temp(&self) -> MUX_SQ_CTRL_TEMP_R {
        MUX_SQ_CTRL_TEMP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - for amuxbusa switch"]
    #[inline(always)]
    pub fn mux_sq_ctrl_amuxbusa(&self) -> MUX_SQ_CTRL_AMUXBUSA_R {
        MUX_SQ_CTRL_AMUXBUSA_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - for amuxbusb switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_amuxbusb(&self) -> MUX_SQ_CTRL_AMUXBUSB_R {
        MUX_SQ_CTRL_AMUXBUSB_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 22 - for sarbus0 switch"]
    #[inline(always)]
    pub fn mux_sq_ctrl_sarbus0(&self) -> MUX_SQ_CTRL_SARBUS0_R {
        MUX_SQ_CTRL_SARBUS0_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - for sarbus1 switch"]
    #[inline(always)]
    pub fn mux_sq_ctrl_sarbus1(&self) -> MUX_SQ_CTRL_SARBUS1_R {
        MUX_SQ_CTRL_SARBUS1_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - for P0 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p0(&mut self) -> MUX_SQ_CTRL_P0_W {
        MUX_SQ_CTRL_P0_W { w: self }
    }
    #[doc = "Bit 1 - for P1 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p1(&mut self) -> MUX_SQ_CTRL_P1_W {
        MUX_SQ_CTRL_P1_W { w: self }
    }
    #[doc = "Bit 2 - for P2 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p2(&mut self) -> MUX_SQ_CTRL_P2_W {
        MUX_SQ_CTRL_P2_W { w: self }
    }
    #[doc = "Bit 3 - for P3 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p3(&mut self) -> MUX_SQ_CTRL_P3_W {
        MUX_SQ_CTRL_P3_W { w: self }
    }
    #[doc = "Bit 4 - for P4 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p4(&mut self) -> MUX_SQ_CTRL_P4_W {
        MUX_SQ_CTRL_P4_W { w: self }
    }
    #[doc = "Bit 5 - for P5 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p5(&mut self) -> MUX_SQ_CTRL_P5_W {
        MUX_SQ_CTRL_P5_W { w: self }
    }
    #[doc = "Bit 6 - for P6 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p6(&mut self) -> MUX_SQ_CTRL_P6_W {
        MUX_SQ_CTRL_P6_W { w: self }
    }
    #[doc = "Bit 7 - for P7 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p7(&mut self) -> MUX_SQ_CTRL_P7_W {
        MUX_SQ_CTRL_P7_W { w: self }
    }
    #[doc = "Bit 16 - for vssa switch"]
    #[inline(always)]
    pub fn mux_sq_ctrl_vssa(&mut self) -> MUX_SQ_CTRL_VSSA_W {
        MUX_SQ_CTRL_VSSA_W { w: self }
    }
    #[doc = "Bit 17 - for temp switch"]
    #[inline(always)]
    pub fn mux_sq_ctrl_temp(&mut self) -> MUX_SQ_CTRL_TEMP_W {
        MUX_SQ_CTRL_TEMP_W { w: self }
    }
    #[doc = "Bit 18 - for amuxbusa switch"]
    #[inline(always)]
    pub fn mux_sq_ctrl_amuxbusa(&mut self) -> MUX_SQ_CTRL_AMUXBUSA_W {
        MUX_SQ_CTRL_AMUXBUSA_W { w: self }
    }
    #[doc = "Bit 19 - for amuxbusb switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_amuxbusb(&mut self) -> MUX_SQ_CTRL_AMUXBUSB_W {
        MUX_SQ_CTRL_AMUXBUSB_W { w: self }
    }
    #[doc = "Bit 22 - for sarbus0 switch"]
    #[inline(always)]
    pub fn mux_sq_ctrl_sarbus0(&mut self) -> MUX_SQ_CTRL_SARBUS0_W {
        MUX_SQ_CTRL_SARBUS0_W { w: self }
    }
    #[doc = "Bit 23 - for sarbus1 switch"]
    #[inline(always)]
    pub fn mux_sq_ctrl_sarbus1(&mut self) -> MUX_SQ_CTRL_SARBUS1_W {
        MUX_SQ_CTRL_SARBUS1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SARMUX switch Sar Sequencer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mux_switch_sq_ctrl](index.html) module"]
pub struct MUX_SWITCH_SQ_CTRL_SPEC;
impl crate::RegisterSpec for MUX_SWITCH_SQ_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mux_switch_sq_ctrl::R](R) reader structure"]
impl crate::Readable for MUX_SWITCH_SQ_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mux_switch_sq_ctrl::W](W) writer structure"]
impl crate::Writable for MUX_SWITCH_SQ_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MUX_SWITCH_SQ_CTRL to value 0"]
impl crate::Resettable for MUX_SWITCH_SQ_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
