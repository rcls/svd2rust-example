#[doc = "Register `HOST_LVL1_SEL` reader"]
pub struct R(crate::R<HOST_LVL1_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_LVL1_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_LVL1_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_LVL1_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_LVL1_SEL` writer"]
pub struct W(crate::W<HOST_LVL1_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_LVL1_SEL_SPEC>;
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
impl From<crate::W<HOST_LVL1_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_LVL1_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "These bits assign SOFIRQ interrupt flag to selected interrupt signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SOFIRQ_SEL_A {
    #[doc = "0: High priority interrupt"]
    HI = 0,
    #[doc = "1: Medium priority interrupt"]
    MED = 1,
    #[doc = "2: Low priority interrupt"]
    LO = 2,
    #[doc = "3: N/A"]
    RSVD = 3,
}
impl From<SOFIRQ_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SOFIRQ_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SOFIRQ_SEL` reader - These bits assign SOFIRQ interrupt flag to selected interrupt signals."]
pub struct SOFIRQ_SEL_R(crate::FieldReader<u8, SOFIRQ_SEL_A>);
impl SOFIRQ_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SOFIRQ_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFIRQ_SEL_A {
        match self.bits {
            0 => SOFIRQ_SEL_A::HI,
            1 => SOFIRQ_SEL_A::MED,
            2 => SOFIRQ_SEL_A::LO,
            3 => SOFIRQ_SEL_A::RSVD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HI`"]
    #[inline(always)]
    pub fn is_hi(&self) -> bool {
        **self == SOFIRQ_SEL_A::HI
    }
    #[doc = "Checks if the value of the field is `MED`"]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        **self == SOFIRQ_SEL_A::MED
    }
    #[doc = "Checks if the value of the field is `LO`"]
    #[inline(always)]
    pub fn is_lo(&self) -> bool {
        **self == SOFIRQ_SEL_A::LO
    }
    #[doc = "Checks if the value of the field is `RSVD`"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        **self == SOFIRQ_SEL_A::RSVD
    }
}
impl core::ops::Deref for SOFIRQ_SEL_R {
    type Target = crate::FieldReader<u8, SOFIRQ_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFIRQ_SEL` writer - These bits assign SOFIRQ interrupt flag to selected interrupt signals."]
pub struct SOFIRQ_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFIRQ_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOFIRQ_SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "High priority interrupt"]
    #[inline(always)]
    pub fn hi(self) -> &'a mut W {
        self.variant(SOFIRQ_SEL_A::HI)
    }
    #[doc = "Medium priority interrupt"]
    #[inline(always)]
    pub fn med(self) -> &'a mut W {
        self.variant(SOFIRQ_SEL_A::MED)
    }
    #[doc = "Low priority interrupt"]
    #[inline(always)]
    pub fn lo(self) -> &'a mut W {
        self.variant(SOFIRQ_SEL_A::LO)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut W {
        self.variant(SOFIRQ_SEL_A::RSVD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `DIRQ_SEL` reader - These bits assign DIRQ interrupt flag to selected interrupt signals."]
pub struct DIRQ_SEL_R(crate::FieldReader<u8, u8>);
impl DIRQ_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIRQ_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRQ_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRQ_SEL` writer - These bits assign DIRQ interrupt flag to selected interrupt signals."]
pub struct DIRQ_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRQ_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `CNNIRQ_SEL` reader - These bits assign CNNIRQ interrupt flag to selected interrupt signals."]
pub struct CNNIRQ_SEL_R(crate::FieldReader<u8, u8>);
impl CNNIRQ_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CNNIRQ_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNNIRQ_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNNIRQ_SEL` writer - These bits assign CNNIRQ interrupt flag to selected interrupt signals."]
pub struct CNNIRQ_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CNNIRQ_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `CMPIRQ_SEL` reader - These bits assign URIRQ interrupt flag to selected interrupt signals."]
pub struct CMPIRQ_SEL_R(crate::FieldReader<u8, u8>);
impl CMPIRQ_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPIRQ_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPIRQ_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPIRQ_SEL` writer - These bits assign URIRQ interrupt flag to selected interrupt signals."]
pub struct CMPIRQ_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPIRQ_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `URIRQ_SEL` reader - These bits assign URIRQ interrupt flag to selected interrupt signals."]
pub struct URIRQ_SEL_R(crate::FieldReader<u8, u8>);
impl URIRQ_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        URIRQ_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URIRQ_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `URIRQ_SEL` writer - These bits assign URIRQ interrupt flag to selected interrupt signals."]
pub struct URIRQ_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> URIRQ_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `RWKIRQ_SEL` reader - These bits assign RWKIRQ interrupt flag to selected interrupt signals."]
pub struct RWKIRQ_SEL_R(crate::FieldReader<u8, u8>);
impl RWKIRQ_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RWKIRQ_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWKIRQ_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWKIRQ_SEL` writer - These bits assign RWKIRQ interrupt flag to selected interrupt signals."]
pub struct RWKIRQ_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RWKIRQ_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `RSVD_13_12` reader - N/A"]
pub struct RSVD_13_12_R(crate::FieldReader<u8, u8>);
impl RSVD_13_12_R {
    pub(crate) fn new(bits: u8) -> Self {
        RSVD_13_12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSVD_13_12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSVD_13_12` writer - N/A"]
pub struct RSVD_13_12_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_13_12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `TCAN_SEL` reader - These bits assign TCAN interrupt flag to selected interrupt signals."]
pub struct TCAN_SEL_R(crate::FieldReader<u8, u8>);
impl TCAN_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TCAN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCAN_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCAN_SEL` writer - These bits assign TCAN interrupt flag to selected interrupt signals."]
pub struct TCAN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TCAN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - These bits assign SOFIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn sofirq_sel(&self) -> SOFIRQ_SEL_R {
        SOFIRQ_SEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - These bits assign DIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn dirq_sel(&self) -> DIRQ_SEL_R {
        DIRQ_SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - These bits assign CNNIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn cnnirq_sel(&self) -> CNNIRQ_SEL_R {
        CNNIRQ_SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - These bits assign URIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn cmpirq_sel(&self) -> CMPIRQ_SEL_R {
        CMPIRQ_SEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - These bits assign URIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn urirq_sel(&self) -> URIRQ_SEL_R {
        URIRQ_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - These bits assign RWKIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn rwkirq_sel(&self) -> RWKIRQ_SEL_R {
        RWKIRQ_SEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - N/A"]
    #[inline(always)]
    pub fn rsvd_13_12(&self) -> RSVD_13_12_R {
        RSVD_13_12_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - These bits assign TCAN interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn tcan_sel(&self) -> TCAN_SEL_R {
        TCAN_SEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - These bits assign SOFIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn sofirq_sel(&mut self) -> SOFIRQ_SEL_W {
        SOFIRQ_SEL_W { w: self }
    }
    #[doc = "Bits 2:3 - These bits assign DIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn dirq_sel(&mut self) -> DIRQ_SEL_W {
        DIRQ_SEL_W { w: self }
    }
    #[doc = "Bits 4:5 - These bits assign CNNIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn cnnirq_sel(&mut self) -> CNNIRQ_SEL_W {
        CNNIRQ_SEL_W { w: self }
    }
    #[doc = "Bits 6:7 - These bits assign URIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn cmpirq_sel(&mut self) -> CMPIRQ_SEL_W {
        CMPIRQ_SEL_W { w: self }
    }
    #[doc = "Bits 8:9 - These bits assign URIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn urirq_sel(&mut self) -> URIRQ_SEL_W {
        URIRQ_SEL_W { w: self }
    }
    #[doc = "Bits 10:11 - These bits assign RWKIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn rwkirq_sel(&mut self) -> RWKIRQ_SEL_W {
        RWKIRQ_SEL_W { w: self }
    }
    #[doc = "Bits 12:13 - N/A"]
    #[inline(always)]
    pub fn rsvd_13_12(&mut self) -> RSVD_13_12_W {
        RSVD_13_12_W { w: self }
    }
    #[doc = "Bits 14:15 - These bits assign TCAN interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn tcan_sel(&mut self) -> TCAN_SEL_W {
        TCAN_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Interrupt Level 1 Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_lvl1_sel](index.html) module"]
pub struct HOST_LVL1_SEL_SPEC;
impl crate::RegisterSpec for HOST_LVL1_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_lvl1_sel::R](R) reader structure"]
impl crate::Readable for HOST_LVL1_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_lvl1_sel::W](W) writer structure"]
impl crate::Writable for HOST_LVL1_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_LVL1_SEL to value 0"]
impl crate::Resettable for HOST_LVL1_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
