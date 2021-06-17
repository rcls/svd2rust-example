#[doc = "Register `ATT1` reader"]
pub struct R(crate::R<ATT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATT1` writer"]
pub struct W(crate::W<ATT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATT1_SPEC>;
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
impl From<crate::W<ATT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UR` reader - See corresponding field for PPU structure with programmable address. Note that this register is constant '1'; i.e. user read accesses are ALWAYS allowed."]
pub struct UR_R(crate::FieldReader<bool, bool>);
impl UR_R {
    pub(crate) fn new(bits: bool) -> Self {
        UR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UW` reader - See corresponding field for PPU structure with programmable address."]
pub struct UW_R(crate::FieldReader<bool, bool>);
impl UW_R {
    pub(crate) fn new(bits: bool) -> Self {
        UW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UW` writer - See corresponding field for PPU structure with programmable address."]
pub struct UW_W<'a> {
    w: &'a mut W,
}
impl<'a> UW_W<'a> {
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
#[doc = "Field `UX` reader - See corresponding field for PPU structure with programmable address. Note that this register is constant '0'; i.e. user execute accesses are NEVER allowed."]
pub struct UX_R(crate::FieldReader<bool, bool>);
impl UX_R {
    pub(crate) fn new(bits: bool) -> Self {
        UX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR` reader - See corresponding field for PPU structure with programmable address. Note that this register is constant '1'; i.e. privileged read accesses are ALWAYS allowed."]
pub struct PR_R(crate::FieldReader<bool, bool>);
impl PR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PW` reader - See corresponding field for PPU structure with programmable address."]
pub struct PW_R(crate::FieldReader<bool, bool>);
impl PW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PW` writer - See corresponding field for PPU structure with programmable address."]
pub struct PW_W<'a> {
    w: &'a mut W,
}
impl<'a> PW_W<'a> {
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
#[doc = "Field `PX` reader - See corresponding field for PPU structure with programmable address. Note that this register is constant '0'; i.e. privileged execute accesses are NEVER allowed."]
pub struct PX_R(crate::FieldReader<bool, bool>);
impl PX_R {
    pub(crate) fn new(bits: bool) -> Self {
        PX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NS` reader - See corresponding field for PPU structure with programmable address."]
pub struct NS_R(crate::FieldReader<bool, bool>);
impl NS_R {
    pub(crate) fn new(bits: bool) -> Self {
        NS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NS` writer - See corresponding field for PPU structure with programmable address."]
pub struct NS_W<'a> {
    w: &'a mut W,
}
impl<'a> NS_W<'a> {
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
#[doc = "Field `PC_MASK_0` reader - See corresponding field for PPU structure with programmable address."]
pub struct PC_MASK_0_R(crate::FieldReader<bool, bool>);
impl PC_MASK_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PC_MASK_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC_MASK_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC_MASK_15_TO_1` reader - See corresponding field for PPU structure with programmable address."]
pub struct PC_MASK_15_TO_1_R(crate::FieldReader<u16, u16>);
impl PC_MASK_15_TO_1_R {
    pub(crate) fn new(bits: u16) -> Self {
        PC_MASK_15_TO_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC_MASK_15_TO_1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC_MASK_15_TO_1` writer - See corresponding field for PPU structure with programmable address."]
pub struct PC_MASK_15_TO_1_W<'a> {
    w: &'a mut W,
}
impl<'a> PC_MASK_15_TO_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 9)) | ((value as u32 & 0x7fff) << 9);
        self.w
    }
}
#[doc = "Field `REGION_SIZE` reader - See corresponding field for PPU structure with programmable address. '7': 256 B region"]
pub struct REGION_SIZE_R(crate::FieldReader<u8, u8>);
impl REGION_SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        REGION_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGION_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC_MATCH` reader - See corresponding field for PPU structure with programmable address."]
pub struct PC_MATCH_R(crate::FieldReader<bool, bool>);
impl PC_MATCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        PC_MATCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC_MATCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC_MATCH` writer - See corresponding field for PPU structure with programmable address."]
pub struct PC_MATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> PC_MATCH_W<'a> {
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
#[doc = "Field `ENABLED` reader - See corresponding field for PPU structure with programmable address."]
pub struct ENABLED_R(crate::FieldReader<bool, bool>);
impl ENABLED_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLED` writer - See corresponding field for PPU structure with programmable address."]
pub struct ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_W<'a> {
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
    #[doc = "Bit 0 - See corresponding field for PPU structure with programmable address. Note that this register is constant '1'; i.e. user read accesses are ALWAYS allowed."]
    #[inline(always)]
    pub fn ur(&self) -> UR_R {
        UR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn uw(&self) -> UW_R {
        UW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - See corresponding field for PPU structure with programmable address. Note that this register is constant '0'; i.e. user execute accesses are NEVER allowed."]
    #[inline(always)]
    pub fn ux(&self) -> UX_R {
        UX_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - See corresponding field for PPU structure with programmable address. Note that this register is constant '1'; i.e. privileged read accesses are ALWAYS allowed."]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn pw(&self) -> PW_R {
        PW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - See corresponding field for PPU structure with programmable address. Note that this register is constant '0'; i.e. privileged execute accesses are NEVER allowed."]
    #[inline(always)]
    pub fn px(&self) -> PX_R {
        PX_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn pc_mask_0(&self) -> PC_MASK_0_R {
        PC_MASK_0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:23 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn pc_mask_15_to_1(&self) -> PC_MASK_15_TO_1_R {
        PC_MASK_15_TO_1_R::new(((self.bits >> 9) & 0x7fff) as u16)
    }
    #[doc = "Bits 24:28 - See corresponding field for PPU structure with programmable address. '7': 256 B region"]
    #[inline(always)]
    pub fn region_size(&self) -> REGION_SIZE_R {
        REGION_SIZE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn pc_match(&self) -> PC_MATCH_R {
        PC_MATCH_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn uw(&mut self) -> UW_W {
        UW_W { w: self }
    }
    #[doc = "Bit 4 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn pw(&mut self) -> PW_W {
        PW_W { w: self }
    }
    #[doc = "Bit 6 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn ns(&mut self) -> NS_W {
        NS_W { w: self }
    }
    #[doc = "Bits 9:23 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn pc_mask_15_to_1(&mut self) -> PC_MASK_15_TO_1_W {
        PC_MASK_15_TO_1_W { w: self }
    }
    #[doc = "Bit 30 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn pc_match(&mut self) -> PC_MATCH_W {
        PC_MATCH_W { w: self }
    }
    #[doc = "Bit 31 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PPU region attributes 1 (master structure)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [att1](index.html) module"]
pub struct ATT1_SPEC;
impl crate::RegisterSpec for ATT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [att1::R](R) reader structure"]
impl crate::Readable for ATT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [att1::W](W) writer structure"]
impl crate::Writable for ATT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ATT1 to value 0x0700_0109"]
impl crate::Resettable for ATT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0700_0109
    }
}
