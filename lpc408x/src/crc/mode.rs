#[doc = "Register `MODE` reader"]
pub struct R(crate::R<MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE` writer"]
pub struct W(crate::W<MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_SPEC>;
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
impl From<crate::W<MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Select CRC polynomial\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CRC_POLY_A {
    #[doc = "0: CRC-CCITT polynomial"]
    CRC_CCITT_POLYNOMIAL = 0,
    #[doc = "1: CRC-16 polynomial"]
    CRC_16_POLYNOMIAL = 1,
    #[doc = "2: CRC-32 polynomial"]
    CRC_32_POLYNOMIAL = 2,
}
impl From<CRC_POLY_A> for u8 {
    #[inline(always)]
    fn from(variant: CRC_POLY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CRC_POLY` reader - Select CRC polynomial"]
pub struct CRC_POLY_R(crate::FieldReader<u8, CRC_POLY_A>);
impl CRC_POLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        CRC_POLY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CRC_POLY_A> {
        match self.bits {
            0 => Some(CRC_POLY_A::CRC_CCITT_POLYNOMIAL),
            1 => Some(CRC_POLY_A::CRC_16_POLYNOMIAL),
            2 => Some(CRC_POLY_A::CRC_32_POLYNOMIAL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CRC_CCITT_POLYNOMIAL`"]
    #[inline(always)]
    pub fn is_crc_ccitt_polynomial(&self) -> bool {
        **self == CRC_POLY_A::CRC_CCITT_POLYNOMIAL
    }
    #[doc = "Checks if the value of the field is `CRC_16_POLYNOMIAL`"]
    #[inline(always)]
    pub fn is_crc_16_polynomial(&self) -> bool {
        **self == CRC_POLY_A::CRC_16_POLYNOMIAL
    }
    #[doc = "Checks if the value of the field is `CRC_32_POLYNOMIAL`"]
    #[inline(always)]
    pub fn is_crc_32_polynomial(&self) -> bool {
        **self == CRC_POLY_A::CRC_32_POLYNOMIAL
    }
}
impl core::ops::Deref for CRC_POLY_R {
    type Target = crate::FieldReader<u8, CRC_POLY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_POLY` writer - Select CRC polynomial"]
pub struct CRC_POLY_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_POLY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC_POLY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CRC-CCITT polynomial"]
    #[inline(always)]
    pub fn crc_ccitt_polynomial(self) -> &'a mut W {
        self.variant(CRC_POLY_A::CRC_CCITT_POLYNOMIAL)
    }
    #[doc = "CRC-16 polynomial"]
    #[inline(always)]
    pub fn crc_16_polynomial(self) -> &'a mut W {
        self.variant(CRC_POLY_A::CRC_16_POLYNOMIAL)
    }
    #[doc = "CRC-32 polynomial"]
    #[inline(always)]
    pub fn crc_32_polynomial(self) -> &'a mut W {
        self.variant(CRC_POLY_A::CRC_32_POLYNOMIAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Select bit order for CRC_WR_DATA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIT_RVS_WR_A {
    #[doc = "0: No bit order reverse for CRC_WR_DATA (per byte)"]
    NO_BIT_ORDER_REVERSE = 0,
    #[doc = "1: Bit order reverse for CRC_WR_DATA (per byte)"]
    BIT_ORDER_REVERSE_FO = 1,
}
impl From<BIT_RVS_WR_A> for bool {
    #[inline(always)]
    fn from(variant: BIT_RVS_WR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT_RVS_WR` reader - Select bit order for CRC_WR_DATA"]
pub struct BIT_RVS_WR_R(crate::FieldReader<bool, BIT_RVS_WR_A>);
impl BIT_RVS_WR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIT_RVS_WR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIT_RVS_WR_A {
        match self.bits {
            false => BIT_RVS_WR_A::NO_BIT_ORDER_REVERSE,
            true => BIT_RVS_WR_A::BIT_ORDER_REVERSE_FO,
        }
    }
    #[doc = "Checks if the value of the field is `NO_BIT_ORDER_REVERSE`"]
    #[inline(always)]
    pub fn is_no_bit_order_reverse(&self) -> bool {
        **self == BIT_RVS_WR_A::NO_BIT_ORDER_REVERSE
    }
    #[doc = "Checks if the value of the field is `BIT_ORDER_REVERSE_FO`"]
    #[inline(always)]
    pub fn is_bit_order_reverse_fo(&self) -> bool {
        **self == BIT_RVS_WR_A::BIT_ORDER_REVERSE_FO
    }
}
impl core::ops::Deref for BIT_RVS_WR_R {
    type Target = crate::FieldReader<bool, BIT_RVS_WR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIT_RVS_WR` writer - Select bit order for CRC_WR_DATA"]
pub struct BIT_RVS_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT_RVS_WR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIT_RVS_WR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No bit order reverse for CRC_WR_DATA (per byte)"]
    #[inline(always)]
    pub fn no_bit_order_reverse(self) -> &'a mut W {
        self.variant(BIT_RVS_WR_A::NO_BIT_ORDER_REVERSE)
    }
    #[doc = "Bit order reverse for CRC_WR_DATA (per byte)"]
    #[inline(always)]
    pub fn bit_order_reverse_fo(self) -> &'a mut W {
        self.variant(BIT_RVS_WR_A::BIT_ORDER_REVERSE_FO)
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
#[doc = "Select one's complement for CRC_WR_DATA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPL_WR_A {
    #[doc = "0: No one's complement for CRC_WR_DATA"]
    NO_ONES_COMPLEMENT_ = 0,
    #[doc = "1: One's complement for CRC_WR_DATA"]
    ONES_COMPLEMENT_FOR = 1,
}
impl From<CMPL_WR_A> for bool {
    #[inline(always)]
    fn from(variant: CMPL_WR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPL_WR` reader - Select one's complement for CRC_WR_DATA"]
pub struct CMPL_WR_R(crate::FieldReader<bool, CMPL_WR_A>);
impl CMPL_WR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPL_WR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPL_WR_A {
        match self.bits {
            false => CMPL_WR_A::NO_ONES_COMPLEMENT_,
            true => CMPL_WR_A::ONES_COMPLEMENT_FOR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ONES_COMPLEMENT_`"]
    #[inline(always)]
    pub fn is_no_ones_complement_(&self) -> bool {
        **self == CMPL_WR_A::NO_ONES_COMPLEMENT_
    }
    #[doc = "Checks if the value of the field is `ONES_COMPLEMENT_FOR`"]
    #[inline(always)]
    pub fn is_ones_complement_for(&self) -> bool {
        **self == CMPL_WR_A::ONES_COMPLEMENT_FOR
    }
}
impl core::ops::Deref for CMPL_WR_R {
    type Target = crate::FieldReader<bool, CMPL_WR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPL_WR` writer - Select one's complement for CRC_WR_DATA"]
pub struct CMPL_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPL_WR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPL_WR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No one's complement for CRC_WR_DATA"]
    #[inline(always)]
    pub fn no_ones_complement_(self) -> &'a mut W {
        self.variant(CMPL_WR_A::NO_ONES_COMPLEMENT_)
    }
    #[doc = "One's complement for CRC_WR_DATA"]
    #[inline(always)]
    pub fn ones_complement_for(self) -> &'a mut W {
        self.variant(CMPL_WR_A::ONES_COMPLEMENT_FOR)
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
#[doc = "Select bit order revers for CRC_SUM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIT_RVS_SUM_A {
    #[doc = "0: No bit order reverse for CRC_SUM"]
    NO_BIT_ORDER_REVERSE = 0,
    #[doc = "1: Bit order reverse for CRC_SUM"]
    BIT_ORDER_REVERSE_FO = 1,
}
impl From<BIT_RVS_SUM_A> for bool {
    #[inline(always)]
    fn from(variant: BIT_RVS_SUM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT_RVS_SUM` reader - Select bit order revers for CRC_SUM"]
pub struct BIT_RVS_SUM_R(crate::FieldReader<bool, BIT_RVS_SUM_A>);
impl BIT_RVS_SUM_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIT_RVS_SUM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIT_RVS_SUM_A {
        match self.bits {
            false => BIT_RVS_SUM_A::NO_BIT_ORDER_REVERSE,
            true => BIT_RVS_SUM_A::BIT_ORDER_REVERSE_FO,
        }
    }
    #[doc = "Checks if the value of the field is `NO_BIT_ORDER_REVERSE`"]
    #[inline(always)]
    pub fn is_no_bit_order_reverse(&self) -> bool {
        **self == BIT_RVS_SUM_A::NO_BIT_ORDER_REVERSE
    }
    #[doc = "Checks if the value of the field is `BIT_ORDER_REVERSE_FO`"]
    #[inline(always)]
    pub fn is_bit_order_reverse_fo(&self) -> bool {
        **self == BIT_RVS_SUM_A::BIT_ORDER_REVERSE_FO
    }
}
impl core::ops::Deref for BIT_RVS_SUM_R {
    type Target = crate::FieldReader<bool, BIT_RVS_SUM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIT_RVS_SUM` writer - Select bit order revers for CRC_SUM"]
pub struct BIT_RVS_SUM_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT_RVS_SUM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIT_RVS_SUM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No bit order reverse for CRC_SUM"]
    #[inline(always)]
    pub fn no_bit_order_reverse(self) -> &'a mut W {
        self.variant(BIT_RVS_SUM_A::NO_BIT_ORDER_REVERSE)
    }
    #[doc = "Bit order reverse for CRC_SUM"]
    #[inline(always)]
    pub fn bit_order_reverse_fo(self) -> &'a mut W {
        self.variant(BIT_RVS_SUM_A::BIT_ORDER_REVERSE_FO)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Select one's complement for CRC_SUM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPL_SUM_A {
    #[doc = "0: No one's complement for CRC_SUM"]
    NO_ONES_COMPLEMENT_ = 0,
    #[doc = "1: One's complement for CRC_SUM"]
    ONES_COMPLEMENT_FOR = 1,
}
impl From<CMPL_SUM_A> for bool {
    #[inline(always)]
    fn from(variant: CMPL_SUM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPL_SUM` reader - Select one's complement for CRC_SUM"]
pub struct CMPL_SUM_R(crate::FieldReader<bool, CMPL_SUM_A>);
impl CMPL_SUM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPL_SUM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPL_SUM_A {
        match self.bits {
            false => CMPL_SUM_A::NO_ONES_COMPLEMENT_,
            true => CMPL_SUM_A::ONES_COMPLEMENT_FOR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ONES_COMPLEMENT_`"]
    #[inline(always)]
    pub fn is_no_ones_complement_(&self) -> bool {
        **self == CMPL_SUM_A::NO_ONES_COMPLEMENT_
    }
    #[doc = "Checks if the value of the field is `ONES_COMPLEMENT_FOR`"]
    #[inline(always)]
    pub fn is_ones_complement_for(&self) -> bool {
        **self == CMPL_SUM_A::ONES_COMPLEMENT_FOR
    }
}
impl core::ops::Deref for CMPL_SUM_R {
    type Target = crate::FieldReader<bool, CMPL_SUM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPL_SUM` writer - Select one's complement for CRC_SUM"]
pub struct CMPL_SUM_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPL_SUM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPL_SUM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No one's complement for CRC_SUM"]
    #[inline(always)]
    pub fn no_ones_complement_(self) -> &'a mut W {
        self.variant(CMPL_SUM_A::NO_ONES_COMPLEMENT_)
    }
    #[doc = "One's complement for CRC_SUM"]
    #[inline(always)]
    pub fn ones_complement_for(self) -> &'a mut W {
        self.variant(CMPL_SUM_A::ONES_COMPLEMENT_FOR)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Select CRC polynomial"]
    #[inline(always)]
    pub fn crc_poly(&self) -> CRC_POLY_R {
        CRC_POLY_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Select bit order for CRC_WR_DATA"]
    #[inline(always)]
    pub fn bit_rvs_wr(&self) -> BIT_RVS_WR_R {
        BIT_RVS_WR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Select one's complement for CRC_WR_DATA"]
    #[inline(always)]
    pub fn cmpl_wr(&self) -> CMPL_WR_R {
        CMPL_WR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Select bit order revers for CRC_SUM"]
    #[inline(always)]
    pub fn bit_rvs_sum(&self) -> BIT_RVS_SUM_R {
        BIT_RVS_SUM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Select one's complement for CRC_SUM"]
    #[inline(always)]
    pub fn cmpl_sum(&self) -> CMPL_SUM_R {
        CMPL_SUM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select CRC polynomial"]
    #[inline(always)]
    pub fn crc_poly(&mut self) -> CRC_POLY_W {
        CRC_POLY_W { w: self }
    }
    #[doc = "Bit 2 - Select bit order for CRC_WR_DATA"]
    #[inline(always)]
    pub fn bit_rvs_wr(&mut self) -> BIT_RVS_WR_W {
        BIT_RVS_WR_W { w: self }
    }
    #[doc = "Bit 3 - Select one's complement for CRC_WR_DATA"]
    #[inline(always)]
    pub fn cmpl_wr(&mut self) -> CMPL_WR_W {
        CMPL_WR_W { w: self }
    }
    #[doc = "Bit 4 - Select bit order revers for CRC_SUM"]
    #[inline(always)]
    pub fn bit_rvs_sum(&mut self) -> BIT_RVS_SUM_W {
        BIT_RVS_SUM_W { w: self }
    }
    #[doc = "Bit 5 - Select one's complement for CRC_SUM"]
    #[inline(always)]
    pub fn cmpl_sum(&mut self) -> CMPL_SUM_W {
        CMPL_SUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](index.html) module"]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode::R](R) reader structure"]
impl crate::Readable for MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode::W](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
