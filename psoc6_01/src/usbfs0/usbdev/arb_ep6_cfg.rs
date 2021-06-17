#[doc = "Register `ARB_EP6_CFG` reader"]
pub struct R(crate::R<ARB_EP6_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARB_EP6_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARB_EP6_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARB_EP6_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARB_EP6_CFG` writer"]
pub struct W(crate::W<ARB_EP6_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARB_EP6_CFG_SPEC>;
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
impl From<crate::W<ARB_EP6_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARB_EP6_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_DATA_RDY` reader - Indication that Endpoint Packet Data is Ready in Main memory"]
pub struct IN_DATA_RDY_R(crate::FieldReader<bool, bool>);
impl IN_DATA_RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN_DATA_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_DATA_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_DATA_RDY` writer - Indication that Endpoint Packet Data is Ready in Main memory"]
pub struct IN_DATA_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_DATA_RDY_W<'a> {
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
#[doc = "Field `DMA_REQ` reader - Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
pub struct DMA_REQ_R(crate::FieldReader<bool, bool>);
impl DMA_REQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_REQ` writer - Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
pub struct DMA_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQ_W<'a> {
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
#[doc = "Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_BYPASS_A {
    #[doc = "0: No CRC bypass; CRC bytes will be written to memory and Termin will be generated for the CRC byte/s"]
    CRC_NORMAL = 0,
    #[doc = "1: CRC Bypass Set; CRC bytes will not be written into memory and Termin will be generated for the last data byte/s"]
    CRC_BYPASS = 1,
}
impl From<CRC_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: CRC_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRC_BYPASS` reader - Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
pub struct CRC_BYPASS_R(crate::FieldReader<bool, CRC_BYPASS_A>);
impl CRC_BYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRC_BYPASS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_BYPASS_A {
        match self.bits {
            false => CRC_BYPASS_A::CRC_NORMAL,
            true => CRC_BYPASS_A::CRC_BYPASS,
        }
    }
    #[doc = "Checks if the value of the field is `CRC_NORMAL`"]
    #[inline(always)]
    pub fn is_crc_normal(&self) -> bool {
        **self == CRC_BYPASS_A::CRC_NORMAL
    }
    #[doc = "Checks if the value of the field is `CRC_BYPASS`"]
    #[inline(always)]
    pub fn is_crc_bypass(&self) -> bool {
        **self == CRC_BYPASS_A::CRC_BYPASS
    }
}
impl core::ops::Deref for CRC_BYPASS_R {
    type Target = crate::FieldReader<bool, CRC_BYPASS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_BYPASS` writer - Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
pub struct CRC_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_BYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC_BYPASS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No CRC bypass; CRC bytes will be written to memory and Termin will be generated for the CRC byte/s"]
    #[inline(always)]
    pub fn crc_normal(self) -> &'a mut W {
        self.variant(CRC_BYPASS_A::CRC_NORMAL)
    }
    #[doc = "CRC Bypass Set; CRC bytes will not be written into memory and Termin will be generated for the last data byte/s"]
    #[inline(always)]
    pub fn crc_bypass(self) -> &'a mut W {
        self.variant(CRC_BYPASS_A::CRC_BYPASS)
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
#[doc = "Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_PTR_A {
    #[doc = "0: Do not Reset Pointer; Krypton Backward compatibility mode"]
    RESET_KRYPTON = 0,
    #[doc = "1: Reset Pointer; recommended value for reduction of CPU Configuration Writes."]
    RESET_NORMAL = 1,
}
impl From<RESET_PTR_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_PTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET_PTR` reader - Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
pub struct RESET_PTR_R(crate::FieldReader<bool, RESET_PTR_A>);
impl RESET_PTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESET_PTR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_PTR_A {
        match self.bits {
            false => RESET_PTR_A::RESET_KRYPTON,
            true => RESET_PTR_A::RESET_NORMAL,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_KRYPTON`"]
    #[inline(always)]
    pub fn is_reset_krypton(&self) -> bool {
        **self == RESET_PTR_A::RESET_KRYPTON
    }
    #[doc = "Checks if the value of the field is `RESET_NORMAL`"]
    #[inline(always)]
    pub fn is_reset_normal(&self) -> bool {
        **self == RESET_PTR_A::RESET_NORMAL
    }
}
impl core::ops::Deref for RESET_PTR_R {
    type Target = crate::FieldReader<bool, RESET_PTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET_PTR` writer - Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
pub struct RESET_PTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_PTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESET_PTR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Do not Reset Pointer; Krypton Backward compatibility mode"]
    #[inline(always)]
    pub fn reset_krypton(self) -> &'a mut W {
        self.variant(RESET_PTR_A::RESET_KRYPTON)
    }
    #[doc = "Reset Pointer; recommended value for reduction of CPU Configuration Writes."]
    #[inline(always)]
    pub fn reset_normal(self) -> &'a mut W {
        self.variant(RESET_PTR_A::RESET_NORMAL)
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
impl R {
    #[doc = "Bit 0 - Indication that Endpoint Packet Data is Ready in Main memory"]
    #[inline(always)]
    pub fn in_data_rdy(&self) -> IN_DATA_RDY_R {
        IN_DATA_RDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
    #[inline(always)]
    pub fn dma_req(&self) -> DMA_REQ_R {
        DMA_REQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
    #[inline(always)]
    pub fn crc_bypass(&self) -> CRC_BYPASS_R {
        CRC_BYPASS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
    #[inline(always)]
    pub fn reset_ptr(&self) -> RESET_PTR_R {
        RESET_PTR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indication that Endpoint Packet Data is Ready in Main memory"]
    #[inline(always)]
    pub fn in_data_rdy(&mut self) -> IN_DATA_RDY_W {
        IN_DATA_RDY_W { w: self }
    }
    #[doc = "Bit 1 - Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
    #[inline(always)]
    pub fn dma_req(&mut self) -> DMA_REQ_W {
        DMA_REQ_W { w: self }
    }
    #[doc = "Bit 2 - Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
    #[inline(always)]
    pub fn crc_bypass(&mut self) -> CRC_BYPASS_W {
        CRC_BYPASS_W { w: self }
    }
    #[doc = "Bit 3 - Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
    #[inline(always)]
    pub fn reset_ptr(&mut self) -> RESET_PTR_W {
        RESET_PTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Configuration Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ep6_cfg](index.html) module"]
pub struct ARB_EP6_CFG_SPEC;
impl crate::RegisterSpec for ARB_EP6_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arb_ep6_cfg::R](R) reader structure"]
impl crate::Readable for ARB_EP6_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [arb_ep6_cfg::W](W) writer structure"]
impl crate::Writable for ARB_EP6_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ARB_EP6_CFG to value 0"]
impl crate::Resettable for ARB_EP6_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
