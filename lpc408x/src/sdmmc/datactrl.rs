#[doc = "Register `DATACTRL` reader"]
pub struct R(crate::R<DATACTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATACTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATACTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATACTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATACTRL` writer"]
pub struct W(crate::W<DATACTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATACTRL_SPEC>;
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
impl From<crate::W<DATACTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATACTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Data transfer enable."]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Data transfer enable."]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
#[doc = "Data transfer direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRECTION_A {
    #[doc = "0: From controller to card."]
    FROM_CONTROLLER_TO_C = 0,
    #[doc = "1: From card to controller."]
    FROM_CARD_TO_CONTROL = 1,
}
impl From<DIRECTION_A> for bool {
    #[inline(always)]
    fn from(variant: DIRECTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRECTION` reader - Data transfer direction"]
pub struct DIRECTION_R(crate::FieldReader<bool, DIRECTION_A>);
impl DIRECTION_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRECTION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRECTION_A {
        match self.bits {
            false => DIRECTION_A::FROM_CONTROLLER_TO_C,
            true => DIRECTION_A::FROM_CARD_TO_CONTROL,
        }
    }
    #[doc = "Checks if the value of the field is `FROM_CONTROLLER_TO_C`"]
    #[inline(always)]
    pub fn is_from_controller_to_c(&self) -> bool {
        **self == DIRECTION_A::FROM_CONTROLLER_TO_C
    }
    #[doc = "Checks if the value of the field is `FROM_CARD_TO_CONTROL`"]
    #[inline(always)]
    pub fn is_from_card_to_control(&self) -> bool {
        **self == DIRECTION_A::FROM_CARD_TO_CONTROL
    }
}
impl core::ops::Deref for DIRECTION_R {
    type Target = crate::FieldReader<bool, DIRECTION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRECTION` writer - Data transfer direction"]
pub struct DIRECTION_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRECTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRECTION_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "From controller to card."]
    #[inline(always)]
    pub fn from_controller_to_c(self) -> &'a mut W {
        self.variant(DIRECTION_A::FROM_CONTROLLER_TO_C)
    }
    #[doc = "From card to controller."]
    #[inline(always)]
    pub fn from_card_to_control(self) -> &'a mut W {
        self.variant(DIRECTION_A::FROM_CARD_TO_CONTROL)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Data transfer mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Block data transfer."]
    BLOCK_DATA_TRANSFER_ = 0,
    #[doc = "1: Stream data transfer."]
    STREAM_DATA_TRANSFER = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Data transfer mode"]
pub struct MODE_R(crate::FieldReader<bool, MODE_A>);
impl MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::BLOCK_DATA_TRANSFER_,
            true => MODE_A::STREAM_DATA_TRANSFER,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCK_DATA_TRANSFER_`"]
    #[inline(always)]
    pub fn is_block_data_transfer_(&self) -> bool {
        **self == MODE_A::BLOCK_DATA_TRANSFER_
    }
    #[doc = "Checks if the value of the field is `STREAM_DATA_TRANSFER`"]
    #[inline(always)]
    pub fn is_stream_data_transfer(&self) -> bool {
        **self == MODE_A::STREAM_DATA_TRANSFER
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<bool, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Data transfer mode"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Block data transfer."]
    #[inline(always)]
    pub fn block_data_transfer_(self) -> &'a mut W {
        self.variant(MODE_A::BLOCK_DATA_TRANSFER_)
    }
    #[doc = "Stream data transfer."]
    #[inline(always)]
    pub fn stream_data_transfer(self) -> &'a mut W {
        self.variant(MODE_A::STREAM_DATA_TRANSFER)
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
#[doc = "Enable DMA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAENABLE_A {
    #[doc = "0: DMA disabled."]
    DMA_DISABLED_ = 0,
    #[doc = "1: DMA enabled."]
    DMA_ENABLED_ = 1,
}
impl From<DMAENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DMAENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAENABLE` reader - Enable DMA"]
pub struct DMAENABLE_R(crate::FieldReader<bool, DMAENABLE_A>);
impl DMAENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAENABLE_A {
        match self.bits {
            false => DMAENABLE_A::DMA_DISABLED_,
            true => DMAENABLE_A::DMA_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_DISABLED_`"]
    #[inline(always)]
    pub fn is_dma_disabled_(&self) -> bool {
        **self == DMAENABLE_A::DMA_DISABLED_
    }
    #[doc = "Checks if the value of the field is `DMA_ENABLED_`"]
    #[inline(always)]
    pub fn is_dma_enabled_(&self) -> bool {
        **self == DMAENABLE_A::DMA_ENABLED_
    }
}
impl core::ops::Deref for DMAENABLE_R {
    type Target = crate::FieldReader<bool, DMAENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAENABLE` writer - Enable DMA"]
pub struct DMAENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA disabled."]
    #[inline(always)]
    pub fn dma_disabled_(self) -> &'a mut W {
        self.variant(DMAENABLE_A::DMA_DISABLED_)
    }
    #[doc = "DMA enabled."]
    #[inline(always)]
    pub fn dma_enabled_(self) -> &'a mut W {
        self.variant(DMAENABLE_A::DMA_ENABLED_)
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
#[doc = "Field `BLOCKSIZE` reader - Data block length"]
pub struct BLOCKSIZE_R(crate::FieldReader<u8, u8>);
impl BLOCKSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        BLOCKSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLOCKSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLOCKSIZE` writer - Data block length"]
pub struct BLOCKSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCKSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Data transfer enable."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data transfer direction"]
    #[inline(always)]
    pub fn direction(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Data transfer mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable DMA"]
    #[inline(always)]
    pub fn dmaenable(&self) -> DMAENABLE_R {
        DMAENABLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Data block length"]
    #[inline(always)]
    pub fn blocksize(&self) -> BLOCKSIZE_R {
        BLOCKSIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Data transfer enable."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Data transfer direction"]
    #[inline(always)]
    pub fn direction(&mut self) -> DIRECTION_W {
        DIRECTION_W { w: self }
    }
    #[doc = "Bit 2 - Data transfer mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 3 - Enable DMA"]
    #[inline(always)]
    pub fn dmaenable(&mut self) -> DMAENABLE_W {
        DMAENABLE_W { w: self }
    }
    #[doc = "Bits 4:7 - Data block length"]
    #[inline(always)]
    pub fn blocksize(&mut self) -> BLOCKSIZE_W {
        BLOCKSIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datactrl](index.html) module"]
pub struct DATACTRL_SPEC;
impl crate::RegisterSpec for DATACTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datactrl::R](R) reader structure"]
impl crate::Readable for DATACTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datactrl::W](W) writer structure"]
impl crate::Writable for DATACTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATACTRL to value 0"]
impl crate::Resettable for DATACTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
