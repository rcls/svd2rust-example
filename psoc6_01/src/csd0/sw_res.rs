#[doc = "Register `SW_RES` reader"]
pub struct R(crate::R<SW_RES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_RES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_RES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_RES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SW_RES` writer"]
pub struct W(crate::W<SW_RES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_RES_SPEC>;
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
impl From<crate::W<SW_RES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_RES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Select resistance or low EMI (slow ramp) for the HCAV switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RES_HCAV_A {
    #[doc = "0: Low"]
    LOW = 0,
    #[doc = "1: Medium"]
    MED = 1,
    #[doc = "2: High"]
    HIGH = 2,
    #[doc = "3: Low EMI (slow ramp: 3 switches closed by fixed delay line)"]
    LOWEMI = 3,
}
impl From<RES_HCAV_A> for u8 {
    #[inline(always)]
    fn from(variant: RES_HCAV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RES_HCAV` reader - Select resistance or low EMI (slow ramp) for the HCAV switch"]
pub struct RES_HCAV_R(crate::FieldReader<u8, RES_HCAV_A>);
impl RES_HCAV_R {
    pub(crate) fn new(bits: u8) -> Self {
        RES_HCAV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RES_HCAV_A {
        match self.bits {
            0 => RES_HCAV_A::LOW,
            1 => RES_HCAV_A::MED,
            2 => RES_HCAV_A::HIGH,
            3 => RES_HCAV_A::LOWEMI,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == RES_HCAV_A::LOW
    }
    #[doc = "Checks if the value of the field is `MED`"]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        **self == RES_HCAV_A::MED
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == RES_HCAV_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOWEMI`"]
    #[inline(always)]
    pub fn is_lowemi(&self) -> bool {
        **self == RES_HCAV_A::LOWEMI
    }
}
impl core::ops::Deref for RES_HCAV_R {
    type Target = crate::FieldReader<u8, RES_HCAV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RES_HCAV` writer - Select resistance or low EMI (slow ramp) for the HCAV switch"]
pub struct RES_HCAV_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_HCAV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RES_HCAV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(RES_HCAV_A::LOW)
    }
    #[doc = "Medium"]
    #[inline(always)]
    pub fn med(self) -> &'a mut W {
        self.variant(RES_HCAV_A::MED)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(RES_HCAV_A::HIGH)
    }
    #[doc = "Low EMI (slow ramp: 3 switches closed by fixed delay line)"]
    #[inline(always)]
    pub fn lowemi(self) -> &'a mut W {
        self.variant(RES_HCAV_A::LOWEMI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `RES_HCAG` reader - Select resistance or low EMI for the corresponding switch"]
pub struct RES_HCAG_R(crate::FieldReader<u8, u8>);
impl RES_HCAG_R {
    pub(crate) fn new(bits: u8) -> Self {
        RES_HCAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RES_HCAG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RES_HCAG` writer - Select resistance or low EMI for the corresponding switch"]
pub struct RES_HCAG_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_HCAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `RES_HCBV` reader - Select resistance or low EMI for the corresponding switch"]
pub struct RES_HCBV_R(crate::FieldReader<u8, u8>);
impl RES_HCBV_R {
    pub(crate) fn new(bits: u8) -> Self {
        RES_HCBV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RES_HCBV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RES_HCBV` writer - Select resistance or low EMI for the corresponding switch"]
pub struct RES_HCBV_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_HCBV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `RES_HCBG` reader - Select resistance or low EMI for the corresponding switch"]
pub struct RES_HCBG_R(crate::FieldReader<u8, u8>);
impl RES_HCBG_R {
    pub(crate) fn new(bits: u8) -> Self {
        RES_HCBG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RES_HCBG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RES_HCBG` writer - Select resistance or low EMI for the corresponding switch"]
pub struct RES_HCBG_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_HCBG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Select resistance for the corresponding switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RES_F1PM_A {
    #[doc = "0: Low"]
    LOW = 0,
    #[doc = "1: Medium"]
    MED = 1,
    #[doc = "2: High"]
    HIGH = 2,
    #[doc = "3: N/A"]
    RSVD = 3,
}
impl From<RES_F1PM_A> for u8 {
    #[inline(always)]
    fn from(variant: RES_F1PM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RES_F1PM` reader - Select resistance for the corresponding switch"]
pub struct RES_F1PM_R(crate::FieldReader<u8, RES_F1PM_A>);
impl RES_F1PM_R {
    pub(crate) fn new(bits: u8) -> Self {
        RES_F1PM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RES_F1PM_A {
        match self.bits {
            0 => RES_F1PM_A::LOW,
            1 => RES_F1PM_A::MED,
            2 => RES_F1PM_A::HIGH,
            3 => RES_F1PM_A::RSVD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == RES_F1PM_A::LOW
    }
    #[doc = "Checks if the value of the field is `MED`"]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        **self == RES_F1PM_A::MED
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == RES_F1PM_A::HIGH
    }
    #[doc = "Checks if the value of the field is `RSVD`"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        **self == RES_F1PM_A::RSVD
    }
}
impl core::ops::Deref for RES_F1PM_R {
    type Target = crate::FieldReader<u8, RES_F1PM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RES_F1PM` writer - Select resistance for the corresponding switch"]
pub struct RES_F1PM_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_F1PM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RES_F1PM_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(RES_F1PM_A::LOW)
    }
    #[doc = "Medium"]
    #[inline(always)]
    pub fn med(self) -> &'a mut W {
        self.variant(RES_F1PM_A::MED)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(RES_F1PM_A::HIGH)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut W {
        self.variant(RES_F1PM_A::RSVD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `RES_F2PT` reader - Select resistance for the corresponding switch"]
pub struct RES_F2PT_R(crate::FieldReader<u8, u8>);
impl RES_F2PT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RES_F2PT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RES_F2PT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RES_F2PT` writer - Select resistance for the corresponding switch"]
pub struct RES_F2PT_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_F2PT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Select resistance or low EMI (slow ramp) for the HCAV switch"]
    #[inline(always)]
    pub fn res_hcav(&self) -> RES_HCAV_R {
        RES_HCAV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub fn res_hcag(&self) -> RES_HCAG_R {
        RES_HCAG_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub fn res_hcbv(&self) -> RES_HCBV_R {
        RES_HCBV_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub fn res_hcbg(&self) -> RES_HCBG_R {
        RES_HCBG_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Select resistance for the corresponding switch"]
    #[inline(always)]
    pub fn res_f1pm(&self) -> RES_F1PM_R {
        RES_F1PM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Select resistance for the corresponding switch"]
    #[inline(always)]
    pub fn res_f2pt(&self) -> RES_F2PT_R {
        RES_F2PT_R::new(((self.bits >> 18) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select resistance or low EMI (slow ramp) for the HCAV switch"]
    #[inline(always)]
    pub fn res_hcav(&mut self) -> RES_HCAV_W {
        RES_HCAV_W { w: self }
    }
    #[doc = "Bits 2:3 - Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub fn res_hcag(&mut self) -> RES_HCAG_W {
        RES_HCAG_W { w: self }
    }
    #[doc = "Bits 4:5 - Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub fn res_hcbv(&mut self) -> RES_HCBV_W {
        RES_HCBV_W { w: self }
    }
    #[doc = "Bits 6:7 - Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub fn res_hcbg(&mut self) -> RES_HCBG_W {
        RES_HCBG_W { w: self }
    }
    #[doc = "Bits 16:17 - Select resistance for the corresponding switch"]
    #[inline(always)]
    pub fn res_f1pm(&mut self) -> RES_F1PM_W {
        RES_F1PM_W { w: self }
    }
    #[doc = "Bits 18:19 - Select resistance for the corresponding switch"]
    #[inline(always)]
    pub fn res_f2pt(&mut self) -> RES_F2PT_W {
        RES_F2PT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Switch Resistance configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_res](index.html) module"]
pub struct SW_RES_SPEC;
impl crate::RegisterSpec for SW_RES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sw_res::R](R) reader structure"]
impl crate::Readable for SW_RES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_res::W](W) writer structure"]
impl crate::Writable for SW_RES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SW_RES to value 0"]
impl crate::Resettable for SW_RES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
