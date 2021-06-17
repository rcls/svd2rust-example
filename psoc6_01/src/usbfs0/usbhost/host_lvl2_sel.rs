#[doc = "Register `HOST_LVL2_SEL` reader"]
pub struct R(crate::R<HOST_LVL2_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_LVL2_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_LVL2_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_LVL2_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_LVL2_SEL` writer"]
pub struct W(crate::W<HOST_LVL2_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_LVL2_SEL_SPEC>;
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
impl From<crate::W<HOST_LVL2_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_LVL2_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "These bits assign EP1_DRQ interrupt flag to selected interrupt signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EP1_DRQ_SEL_A {
    #[doc = "0: High priority interrupt"]
    HI = 0,
    #[doc = "1: Medium priority interrupt"]
    MED = 1,
    #[doc = "2: Low priority interrupt"]
    LO = 2,
    #[doc = "3: N/A"]
    RSVD = 3,
}
impl From<EP1_DRQ_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EP1_DRQ_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EP1_DRQ_SEL` reader - These bits assign EP1_DRQ interrupt flag to selected interrupt signals."]
pub struct EP1_DRQ_SEL_R(crate::FieldReader<u8, EP1_DRQ_SEL_A>);
impl EP1_DRQ_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EP1_DRQ_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP1_DRQ_SEL_A {
        match self.bits {
            0 => EP1_DRQ_SEL_A::HI,
            1 => EP1_DRQ_SEL_A::MED,
            2 => EP1_DRQ_SEL_A::LO,
            3 => EP1_DRQ_SEL_A::RSVD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HI`"]
    #[inline(always)]
    pub fn is_hi(&self) -> bool {
        **self == EP1_DRQ_SEL_A::HI
    }
    #[doc = "Checks if the value of the field is `MED`"]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        **self == EP1_DRQ_SEL_A::MED
    }
    #[doc = "Checks if the value of the field is `LO`"]
    #[inline(always)]
    pub fn is_lo(&self) -> bool {
        **self == EP1_DRQ_SEL_A::LO
    }
    #[doc = "Checks if the value of the field is `RSVD`"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        **self == EP1_DRQ_SEL_A::RSVD
    }
}
impl core::ops::Deref for EP1_DRQ_SEL_R {
    type Target = crate::FieldReader<u8, EP1_DRQ_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP1_DRQ_SEL` writer - These bits assign EP1_DRQ interrupt flag to selected interrupt signals."]
pub struct EP1_DRQ_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1_DRQ_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EP1_DRQ_SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "High priority interrupt"]
    #[inline(always)]
    pub fn hi(self) -> &'a mut W {
        self.variant(EP1_DRQ_SEL_A::HI)
    }
    #[doc = "Medium priority interrupt"]
    #[inline(always)]
    pub fn med(self) -> &'a mut W {
        self.variant(EP1_DRQ_SEL_A::MED)
    }
    #[doc = "Low priority interrupt"]
    #[inline(always)]
    pub fn lo(self) -> &'a mut W {
        self.variant(EP1_DRQ_SEL_A::LO)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut W {
        self.variant(EP1_DRQ_SEL_A::RSVD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `EP1_SPK_SEL` reader - These bits assign EP1_SPK interrupt flag to selected interrupt signals."]
pub struct EP1_SPK_SEL_R(crate::FieldReader<u8, u8>);
impl EP1_SPK_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EP1_SPK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP1_SPK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP1_SPK_SEL` writer - These bits assign EP1_SPK interrupt flag to selected interrupt signals."]
pub struct EP1_SPK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1_SPK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `EP2_DRQ_SEL` reader - These bits assign EP2_DRQ interrupt flag to selected interrupt signals."]
pub struct EP2_DRQ_SEL_R(crate::FieldReader<u8, u8>);
impl EP2_DRQ_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EP2_DRQ_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP2_DRQ_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP2_DRQ_SEL` writer - These bits assign EP2_DRQ interrupt flag to selected interrupt signals."]
pub struct EP2_DRQ_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2_DRQ_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `EP2_SPK_SEL` reader - These bits assign EP2_SPK interrupt flag to selected interrupt signals."]
pub struct EP2_SPK_SEL_R(crate::FieldReader<u8, u8>);
impl EP2_SPK_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EP2_SPK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP2_SPK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP2_SPK_SEL` writer - These bits assign EP2_SPK interrupt flag to selected interrupt signals."]
pub struct EP2_SPK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2_SPK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:5 - These bits assign EP1_DRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn ep1_drq_sel(&self) -> EP1_DRQ_SEL_R {
        EP1_DRQ_SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - These bits assign EP1_SPK interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn ep1_spk_sel(&self) -> EP1_SPK_SEL_R {
        EP1_SPK_SEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - These bits assign EP2_DRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn ep2_drq_sel(&self) -> EP2_DRQ_SEL_R {
        EP2_DRQ_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - These bits assign EP2_SPK interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn ep2_spk_sel(&self) -> EP2_SPK_SEL_R {
        EP2_SPK_SEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - These bits assign EP1_DRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn ep1_drq_sel(&mut self) -> EP1_DRQ_SEL_W {
        EP1_DRQ_SEL_W { w: self }
    }
    #[doc = "Bits 6:7 - These bits assign EP1_SPK interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn ep1_spk_sel(&mut self) -> EP1_SPK_SEL_W {
        EP1_SPK_SEL_W { w: self }
    }
    #[doc = "Bits 8:9 - These bits assign EP2_DRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn ep2_drq_sel(&mut self) -> EP2_DRQ_SEL_W {
        EP2_DRQ_SEL_W { w: self }
    }
    #[doc = "Bits 10:11 - These bits assign EP2_SPK interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn ep2_spk_sel(&mut self) -> EP2_SPK_SEL_W {
        EP2_SPK_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Interrupt Level 2 Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_lvl2_sel](index.html) module"]
pub struct HOST_LVL2_SEL_SPEC;
impl crate::RegisterSpec for HOST_LVL2_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_lvl2_sel::R](R) reader structure"]
impl crate::Readable for HOST_LVL2_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_lvl2_sel::W](W) writer structure"]
impl crate::Writable for HOST_LVL2_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_LVL2_SEL to value 0"]
impl crate::Resettable for HOST_LVL2_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
