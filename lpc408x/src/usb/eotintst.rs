#[doc = "Register `EOTINTST` reader"]
pub struct R(crate::R<EOTINTST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EOTINTST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EOTINTST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EOTINTST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EPTXINTST0` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST0_R(crate::FieldReader<bool, bool>);
impl EPTXINTST0_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST1` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST1_R(crate::FieldReader<bool, bool>);
impl EPTXINTST1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST2` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST2_R(crate::FieldReader<bool, bool>);
impl EPTXINTST2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST3` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST3_R(crate::FieldReader<bool, bool>);
impl EPTXINTST3_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST4` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST4_R(crate::FieldReader<bool, bool>);
impl EPTXINTST4_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST5` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST5_R(crate::FieldReader<bool, bool>);
impl EPTXINTST5_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST6` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST6_R(crate::FieldReader<bool, bool>);
impl EPTXINTST6_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST7` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST7_R(crate::FieldReader<bool, bool>);
impl EPTXINTST7_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST8` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST8_R(crate::FieldReader<bool, bool>);
impl EPTXINTST8_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST9` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST9_R(crate::FieldReader<bool, bool>);
impl EPTXINTST9_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST10` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST10_R(crate::FieldReader<bool, bool>);
impl EPTXINTST10_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST11` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST11_R(crate::FieldReader<bool, bool>);
impl EPTXINTST11_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST12` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST12_R(crate::FieldReader<bool, bool>);
impl EPTXINTST12_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST13` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST13_R(crate::FieldReader<bool, bool>);
impl EPTXINTST13_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST14` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST14_R(crate::FieldReader<bool, bool>);
impl EPTXINTST14_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST15` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST15_R(crate::FieldReader<bool, bool>);
impl EPTXINTST15_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST16` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST16_R(crate::FieldReader<bool, bool>);
impl EPTXINTST16_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST17` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST17_R(crate::FieldReader<bool, bool>);
impl EPTXINTST17_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST18` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST18_R(crate::FieldReader<bool, bool>);
impl EPTXINTST18_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST19` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST19_R(crate::FieldReader<bool, bool>);
impl EPTXINTST19_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST20` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST20_R(crate::FieldReader<bool, bool>);
impl EPTXINTST20_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST21` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST21_R(crate::FieldReader<bool, bool>);
impl EPTXINTST21_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST22` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST22_R(crate::FieldReader<bool, bool>);
impl EPTXINTST22_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST23` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST23_R(crate::FieldReader<bool, bool>);
impl EPTXINTST23_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST24` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST24_R(crate::FieldReader<bool, bool>);
impl EPTXINTST24_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST25` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST25_R(crate::FieldReader<bool, bool>);
impl EPTXINTST25_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST26` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST26_R(crate::FieldReader<bool, bool>);
impl EPTXINTST26_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST27` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST27_R(crate::FieldReader<bool, bool>);
impl EPTXINTST27_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST28` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST28_R(crate::FieldReader<bool, bool>);
impl EPTXINTST28_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST29` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST29_R(crate::FieldReader<bool, bool>);
impl EPTXINTST29_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST30` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST30_R(crate::FieldReader<bool, bool>);
impl EPTXINTST30_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXINTST31` reader - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
pub struct EPTXINTST31_R(crate::FieldReader<bool, bool>);
impl EPTXINTST31_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXINTST31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXINTST31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst0(&self) -> EPTXINTST0_R {
        EPTXINTST0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst1(&self) -> EPTXINTST1_R {
        EPTXINTST1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst2(&self) -> EPTXINTST2_R {
        EPTXINTST2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst3(&self) -> EPTXINTST3_R {
        EPTXINTST3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst4(&self) -> EPTXINTST4_R {
        EPTXINTST4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst5(&self) -> EPTXINTST5_R {
        EPTXINTST5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst6(&self) -> EPTXINTST6_R {
        EPTXINTST6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst7(&self) -> EPTXINTST7_R {
        EPTXINTST7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst8(&self) -> EPTXINTST8_R {
        EPTXINTST8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst9(&self) -> EPTXINTST9_R {
        EPTXINTST9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst10(&self) -> EPTXINTST10_R {
        EPTXINTST10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst11(&self) -> EPTXINTST11_R {
        EPTXINTST11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst12(&self) -> EPTXINTST12_R {
        EPTXINTST12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst13(&self) -> EPTXINTST13_R {
        EPTXINTST13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst14(&self) -> EPTXINTST14_R {
        EPTXINTST14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst15(&self) -> EPTXINTST15_R {
        EPTXINTST15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst16(&self) -> EPTXINTST16_R {
        EPTXINTST16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst17(&self) -> EPTXINTST17_R {
        EPTXINTST17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst18(&self) -> EPTXINTST18_R {
        EPTXINTST18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst19(&self) -> EPTXINTST19_R {
        EPTXINTST19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst20(&self) -> EPTXINTST20_R {
        EPTXINTST20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst21(&self) -> EPTXINTST21_R {
        EPTXINTST21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst22(&self) -> EPTXINTST22_R {
        EPTXINTST22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst23(&self) -> EPTXINTST23_R {
        EPTXINTST23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst24(&self) -> EPTXINTST24_R {
        EPTXINTST24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst25(&self) -> EPTXINTST25_R {
        EPTXINTST25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst26(&self) -> EPTXINTST26_R {
        EPTXINTST26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst27(&self) -> EPTXINTST27_R {
        EPTXINTST27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst28(&self) -> EPTXINTST28_R {
        EPTXINTST28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst29(&self) -> EPTXINTST29_R {
        EPTXINTST29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst30(&self) -> EPTXINTST30_R {
        EPTXINTST30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst31(&self) -> EPTXINTST31_R {
        EPTXINTST31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "USB End of Transfer Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eotintst](index.html) module"]
pub struct EOTINTST_SPEC;
impl crate::RegisterSpec for EOTINTST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eotintst::R](R) reader structure"]
impl crate::Readable for EOTINTST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EOTINTST to value 0"]
impl crate::Resettable for EOTINTST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
