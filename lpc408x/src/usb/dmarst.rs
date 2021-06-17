#[doc = "Register `DMARST` reader"]
pub struct R(crate::R<DMARST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMARST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMARST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMARST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EPRST0` reader - Control endpoint OUT (DMA cannot be enabled for this endpoint and EP0 bit must be 0)."]
pub struct EPRST0_R(crate::FieldReader<bool, bool>);
impl EPRST0_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST1` reader - Control endpoint IN (DMA cannot be enabled for this endpoint and EP1 bit must be 0)."]
pub struct EPRST1_R(crate::FieldReader<bool, bool>);
impl EPRST1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST2` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST2_R(crate::FieldReader<bool, bool>);
impl EPRST2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST3` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST3_R(crate::FieldReader<bool, bool>);
impl EPRST3_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST4` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST4_R(crate::FieldReader<bool, bool>);
impl EPRST4_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST5` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST5_R(crate::FieldReader<bool, bool>);
impl EPRST5_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST6` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST6_R(crate::FieldReader<bool, bool>);
impl EPRST6_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST7` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST7_R(crate::FieldReader<bool, bool>);
impl EPRST7_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST8` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST8_R(crate::FieldReader<bool, bool>);
impl EPRST8_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST9` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST9_R(crate::FieldReader<bool, bool>);
impl EPRST9_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST10` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST10_R(crate::FieldReader<bool, bool>);
impl EPRST10_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST11` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST11_R(crate::FieldReader<bool, bool>);
impl EPRST11_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST12` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST12_R(crate::FieldReader<bool, bool>);
impl EPRST12_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST13` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST13_R(crate::FieldReader<bool, bool>);
impl EPRST13_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST14` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST14_R(crate::FieldReader<bool, bool>);
impl EPRST14_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST15` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST15_R(crate::FieldReader<bool, bool>);
impl EPRST15_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST16` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST16_R(crate::FieldReader<bool, bool>);
impl EPRST16_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST17` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST17_R(crate::FieldReader<bool, bool>);
impl EPRST17_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST18` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST18_R(crate::FieldReader<bool, bool>);
impl EPRST18_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST19` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST19_R(crate::FieldReader<bool, bool>);
impl EPRST19_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST20` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST20_R(crate::FieldReader<bool, bool>);
impl EPRST20_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST21` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST21_R(crate::FieldReader<bool, bool>);
impl EPRST21_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST22` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST22_R(crate::FieldReader<bool, bool>);
impl EPRST22_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST23` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST23_R(crate::FieldReader<bool, bool>);
impl EPRST23_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST24` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST24_R(crate::FieldReader<bool, bool>);
impl EPRST24_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST25` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST25_R(crate::FieldReader<bool, bool>);
impl EPRST25_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST26` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST26_R(crate::FieldReader<bool, bool>);
impl EPRST26_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST27` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST27_R(crate::FieldReader<bool, bool>);
impl EPRST27_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST28` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST28_R(crate::FieldReader<bool, bool>);
impl EPRST28_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST29` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST29_R(crate::FieldReader<bool, bool>);
impl EPRST29_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST30` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST30_R(crate::FieldReader<bool, bool>);
impl EPRST30_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST31` reader - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
pub struct EPRST31_R(crate::FieldReader<bool, bool>);
impl EPRST31_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRST31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Control endpoint OUT (DMA cannot be enabled for this endpoint and EP0 bit must be 0)."]
    #[inline(always)]
    pub fn eprst0(&self) -> EPRST0_R {
        EPRST0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Control endpoint IN (DMA cannot be enabled for this endpoint and EP1 bit must be 0)."]
    #[inline(always)]
    pub fn eprst1(&self) -> EPRST1_R {
        EPRST1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst2(&self) -> EPRST2_R {
        EPRST2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst3(&self) -> EPRST3_R {
        EPRST3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst4(&self) -> EPRST4_R {
        EPRST4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst5(&self) -> EPRST5_R {
        EPRST5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst6(&self) -> EPRST6_R {
        EPRST6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst7(&self) -> EPRST7_R {
        EPRST7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst8(&self) -> EPRST8_R {
        EPRST8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst9(&self) -> EPRST9_R {
        EPRST9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst10(&self) -> EPRST10_R {
        EPRST10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst11(&self) -> EPRST11_R {
        EPRST11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst12(&self) -> EPRST12_R {
        EPRST12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst13(&self) -> EPRST13_R {
        EPRST13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst14(&self) -> EPRST14_R {
        EPRST14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst15(&self) -> EPRST15_R {
        EPRST15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst16(&self) -> EPRST16_R {
        EPRST16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst17(&self) -> EPRST17_R {
        EPRST17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst18(&self) -> EPRST18_R {
        EPRST18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst19(&self) -> EPRST19_R {
        EPRST19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst20(&self) -> EPRST20_R {
        EPRST20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst21(&self) -> EPRST21_R {
        EPRST21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst22(&self) -> EPRST22_R {
        EPRST22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst23(&self) -> EPRST23_R {
        EPRST23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst24(&self) -> EPRST24_R {
        EPRST24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst25(&self) -> EPRST25_R {
        EPRST25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst26(&self) -> EPRST26_R {
        EPRST26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst27(&self) -> EPRST27_R {
        EPRST27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst28(&self) -> EPRST28_R {
        EPRST28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst29(&self) -> EPRST29_R {
        EPRST29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst30(&self) -> EPRST30_R {
        EPRST30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst31(&self) -> EPRST31_R {
        EPRST31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "USB DMA Request Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmarst](index.html) module"]
pub struct DMARST_SPEC;
impl crate::RegisterSpec for DMARST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmarst::R](R) reader structure"]
impl crate::Readable for DMARST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMARST to value 0"]
impl crate::Resettable for DMARST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
