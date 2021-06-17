#[doc = "Register `RED_CTL45` reader"]
pub struct R(crate::R<RED_CTL45_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RED_CTL45_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RED_CTL45_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RED_CTL45_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RED_CTL45` writer"]
pub struct W(crate::W<RED_CTL45_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RED_CTL45_SPEC>;
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
impl From<crate::W<RED_CTL45_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RED_CTL45_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DNU_45_1` reader - Not Used"]
pub struct DNU_45_1_R(crate::FieldReader<bool, bool>);
impl DNU_45_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DNU_45_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DNU_45_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DNU_45_1` writer - Not Used"]
pub struct DNU_45_1_W<'a> {
    w: &'a mut W,
}
impl<'a> DNU_45_1_W<'a> {
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
#[doc = "Field `REG_ACT_HV` reader - Forces the VBST regulator in active mode all the time"]
pub struct REG_ACT_HV_R(crate::FieldReader<bool, bool>);
impl REG_ACT_HV_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_ACT_HV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_ACT_HV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG_ACT_HV` writer - Forces the VBST regulator in active mode all the time"]
pub struct REG_ACT_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_ACT_HV_W<'a> {
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
#[doc = "Field `DNU_45_3` reader - Not Used"]
pub struct DNU_45_3_R(crate::FieldReader<bool, bool>);
impl DNU_45_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        DNU_45_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DNU_45_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DNU_45_3` writer - Not Used"]
pub struct DNU_45_3_W<'a> {
    w: &'a mut W,
}
impl<'a> DNU_45_3_W<'a> {
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
#[doc = "Field `FDIV_TRIM_HV_0` reader - '2b00' F = 1MHz see fdiv_trim_hv<1> value as well '2b01' F = 0.5MHz '2b10' F = 2MHz '2b11' F = 4Mhz"]
pub struct FDIV_TRIM_HV_0_R(crate::FieldReader<bool, bool>);
impl FDIV_TRIM_HV_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FDIV_TRIM_HV_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDIV_TRIM_HV_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDIV_TRIM_HV_0` writer - '2b00' F = 1MHz see fdiv_trim_hv<1> value as well '2b01' F = 0.5MHz '2b10' F = 2MHz '2b11' F = 4Mhz"]
pub struct FDIV_TRIM_HV_0_W<'a> {
    w: &'a mut W,
}
impl<'a> FDIV_TRIM_HV_0_W<'a> {
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
#[doc = "Field `DNU_45_5` reader - Not Used"]
pub struct DNU_45_5_R(crate::FieldReader<bool, bool>);
impl DNU_45_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        DNU_45_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DNU_45_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DNU_45_5` writer - Not Used"]
pub struct DNU_45_5_W<'a> {
    w: &'a mut W,
}
impl<'a> DNU_45_5_W<'a> {
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
#[doc = "Field `FDIV_TRIM_HV_1` reader - '2b00' F = 1MHz see fdiv_trim_hv<0> value as well '2b01' F = 0.5MHz '2b10' F = 2MHz '2b11' F = 4Mhz"]
pub struct FDIV_TRIM_HV_1_R(crate::FieldReader<bool, bool>);
impl FDIV_TRIM_HV_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FDIV_TRIM_HV_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDIV_TRIM_HV_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDIV_TRIM_HV_1` writer - '2b00' F = 1MHz see fdiv_trim_hv<0> value as well '2b01' F = 0.5MHz '2b10' F = 2MHz '2b11' F = 4Mhz"]
pub struct FDIV_TRIM_HV_1_W<'a> {
    w: &'a mut W,
}
impl<'a> FDIV_TRIM_HV_1_W<'a> {
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
#[doc = "Field `DNU_45_6` reader - Not Used"]
pub struct DNU_45_6_R(crate::FieldReader<bool, bool>);
impl DNU_45_6_R {
    pub(crate) fn new(bits: bool) -> Self {
        DNU_45_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DNU_45_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DNU_45_6` writer - Not Used"]
pub struct DNU_45_6_W<'a> {
    w: &'a mut W,
}
impl<'a> DNU_45_6_W<'a> {
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
#[doc = "Field `VLIM_TRIM_HV_0` reader - '2b00' V2 = 650mV see vlim_trim_hv<1> value as well '2b01' V2 = 600mV '2b10' V2 = 750mV '2b11' V2 = 700mV"]
pub struct VLIM_TRIM_HV_0_R(crate::FieldReader<bool, bool>);
impl VLIM_TRIM_HV_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        VLIM_TRIM_HV_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VLIM_TRIM_HV_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLIM_TRIM_HV_0` writer - '2b00' V2 = 650mV see vlim_trim_hv<1> value as well '2b01' V2 = 600mV '2b10' V2 = 750mV '2b11' V2 = 700mV"]
pub struct VLIM_TRIM_HV_0_W<'a> {
    w: &'a mut W,
}
impl<'a> VLIM_TRIM_HV_0_W<'a> {
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
#[doc = "Field `DNU_45_8` reader - Not Used"]
pub struct DNU_45_8_R(crate::FieldReader<bool, bool>);
impl DNU_45_8_R {
    pub(crate) fn new(bits: bool) -> Self {
        DNU_45_8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DNU_45_8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DNU_45_8` writer - Not Used"]
pub struct DNU_45_8_W<'a> {
    w: &'a mut W,
}
impl<'a> DNU_45_8_W<'a> {
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
#[doc = "Field `DNU_45_23_16` reader - Not Used"]
pub struct DNU_45_23_16_R(crate::FieldReader<u8, u8>);
impl DNU_45_23_16_R {
    pub(crate) fn new(bits: u8) -> Self {
        DNU_45_23_16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DNU_45_23_16_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DNU_45_23_16` writer - Not Used"]
pub struct DNU_45_23_16_W<'a> {
    w: &'a mut W,
}
impl<'a> DNU_45_23_16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_1(&self) -> DNU_45_1_R {
        DNU_45_1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Forces the VBST regulator in active mode all the time"]
    #[inline(always)]
    pub fn reg_act_hv(&self) -> REG_ACT_HV_R {
        REG_ACT_HV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_3(&self) -> DNU_45_3_R {
        DNU_45_3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - '2b00' F = 1MHz see fdiv_trim_hv<1> value as well '2b01' F = 0.5MHz '2b10' F = 2MHz '2b11' F = 4Mhz"]
    #[inline(always)]
    pub fn fdiv_trim_hv_0(&self) -> FDIV_TRIM_HV_0_R {
        FDIV_TRIM_HV_0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_5(&self) -> DNU_45_5_R {
        DNU_45_5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - '2b00' F = 1MHz see fdiv_trim_hv<0> value as well '2b01' F = 0.5MHz '2b10' F = 2MHz '2b11' F = 4Mhz"]
    #[inline(always)]
    pub fn fdiv_trim_hv_1(&self) -> FDIV_TRIM_HV_1_R {
        FDIV_TRIM_HV_1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_6(&self) -> DNU_45_6_R {
        DNU_45_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - '2b00' V2 = 650mV see vlim_trim_hv<1> value as well '2b01' V2 = 600mV '2b10' V2 = 750mV '2b11' V2 = 700mV"]
    #[inline(always)]
    pub fn vlim_trim_hv_0(&self) -> VLIM_TRIM_HV_0_R {
        VLIM_TRIM_HV_0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_8(&self) -> DNU_45_8_R {
        DNU_45_8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_23_16(&self) -> DNU_45_23_16_R {
        DNU_45_23_16_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_1(&mut self) -> DNU_45_1_W {
        DNU_45_1_W { w: self }
    }
    #[doc = "Bit 1 - Forces the VBST regulator in active mode all the time"]
    #[inline(always)]
    pub fn reg_act_hv(&mut self) -> REG_ACT_HV_W {
        REG_ACT_HV_W { w: self }
    }
    #[doc = "Bit 2 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_3(&mut self) -> DNU_45_3_W {
        DNU_45_3_W { w: self }
    }
    #[doc = "Bit 3 - '2b00' F = 1MHz see fdiv_trim_hv<1> value as well '2b01' F = 0.5MHz '2b10' F = 2MHz '2b11' F = 4Mhz"]
    #[inline(always)]
    pub fn fdiv_trim_hv_0(&mut self) -> FDIV_TRIM_HV_0_W {
        FDIV_TRIM_HV_0_W { w: self }
    }
    #[doc = "Bit 4 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_5(&mut self) -> DNU_45_5_W {
        DNU_45_5_W { w: self }
    }
    #[doc = "Bit 5 - '2b00' F = 1MHz see fdiv_trim_hv<0> value as well '2b01' F = 0.5MHz '2b10' F = 2MHz '2b11' F = 4Mhz"]
    #[inline(always)]
    pub fn fdiv_trim_hv_1(&mut self) -> FDIV_TRIM_HV_1_W {
        FDIV_TRIM_HV_1_W { w: self }
    }
    #[doc = "Bit 6 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_6(&mut self) -> DNU_45_6_W {
        DNU_45_6_W { w: self }
    }
    #[doc = "Bit 7 - '2b00' V2 = 650mV see vlim_trim_hv<1> value as well '2b01' V2 = 600mV '2b10' V2 = 750mV '2b11' V2 = 700mV"]
    #[inline(always)]
    pub fn vlim_trim_hv_0(&mut self) -> VLIM_TRIM_HV_0_W {
        VLIM_TRIM_HV_0_W { w: self }
    }
    #[doc = "Bit 8 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_8(&mut self) -> DNU_45_8_W {
        DNU_45_8_W { w: self }
    }
    #[doc = "Bits 16:23 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_23_16(&mut self) -> DNU_45_23_16_W {
        DNU_45_23_16_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Redundancy Controll normal sectors 4,5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [red_ctl45](index.html) module"]
pub struct RED_CTL45_SPEC;
impl crate::RegisterSpec for RED_CTL45_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [red_ctl45::R](R) reader structure"]
impl crate::Readable for RED_CTL45_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [red_ctl45::W](W) writer structure"]
impl crate::Writable for RED_CTL45_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RED_CTL45 to value 0"]
impl crate::Resettable for RED_CTL45_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
