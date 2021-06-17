#[doc = "Register `NDDRINTST` reader"]
pub struct R(crate::R<NDDRINTST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NDDRINTST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NDDRINTST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NDDRINTST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EPNDDINTST0` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST0_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST0_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST1` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST1_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST2` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST2_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST3` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST3_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST3_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST4` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST4_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST4_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST5` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST5_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST5_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST6` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST6_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST6_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST7` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST7_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST7_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST8` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST8_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST8_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST9` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST9_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST9_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST10` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST10_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST10_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST11` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST11_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST11_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST12` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST12_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST12_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST13` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST13_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST13_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST14` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST14_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST14_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST15` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST15_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST15_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST16` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST16_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST16_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST17` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST17_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST17_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST18` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST18_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST18_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST19` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST19_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST19_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST20` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST20_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST20_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST21` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST21_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST21_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST22` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST22_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST22_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST23` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST23_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST23_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST24` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST24_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST24_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST25` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST25_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST25_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST26` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST26_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST26_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST27` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST27_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST27_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST28` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST28_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST28_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST29` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST29_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST29_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST30` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST30_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST30_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNDDINTST31` reader - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
pub struct EPNDDINTST31_R(crate::FieldReader<bool, bool>);
impl EPNDDINTST31_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPNDDINTST31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNDDINTST31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst0(&self) -> EPNDDINTST0_R {
        EPNDDINTST0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst1(&self) -> EPNDDINTST1_R {
        EPNDDINTST1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst2(&self) -> EPNDDINTST2_R {
        EPNDDINTST2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst3(&self) -> EPNDDINTST3_R {
        EPNDDINTST3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst4(&self) -> EPNDDINTST4_R {
        EPNDDINTST4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst5(&self) -> EPNDDINTST5_R {
        EPNDDINTST5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst6(&self) -> EPNDDINTST6_R {
        EPNDDINTST6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst7(&self) -> EPNDDINTST7_R {
        EPNDDINTST7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst8(&self) -> EPNDDINTST8_R {
        EPNDDINTST8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst9(&self) -> EPNDDINTST9_R {
        EPNDDINTST9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst10(&self) -> EPNDDINTST10_R {
        EPNDDINTST10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst11(&self) -> EPNDDINTST11_R {
        EPNDDINTST11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst12(&self) -> EPNDDINTST12_R {
        EPNDDINTST12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst13(&self) -> EPNDDINTST13_R {
        EPNDDINTST13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst14(&self) -> EPNDDINTST14_R {
        EPNDDINTST14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst15(&self) -> EPNDDINTST15_R {
        EPNDDINTST15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst16(&self) -> EPNDDINTST16_R {
        EPNDDINTST16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst17(&self) -> EPNDDINTST17_R {
        EPNDDINTST17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst18(&self) -> EPNDDINTST18_R {
        EPNDDINTST18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst19(&self) -> EPNDDINTST19_R {
        EPNDDINTST19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst20(&self) -> EPNDDINTST20_R {
        EPNDDINTST20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst21(&self) -> EPNDDINTST21_R {
        EPNDDINTST21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst22(&self) -> EPNDDINTST22_R {
        EPNDDINTST22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst23(&self) -> EPNDDINTST23_R {
        EPNDDINTST23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst24(&self) -> EPNDDINTST24_R {
        EPNDDINTST24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst25(&self) -> EPNDDINTST25_R {
        EPNDDINTST25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst26(&self) -> EPNDDINTST26_R {
        EPNDDINTST26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst27(&self) -> EPNDDINTST27_R {
        EPNDDINTST27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst28(&self) -> EPNDDINTST28_R {
        EPNDDINTST28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst29(&self) -> EPNDDINTST29_R {
        EPNDDINTST29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst30(&self) -> EPNDDINTST30_R {
        EPNDDINTST30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst31(&self) -> EPNDDINTST31_R {
        EPNDDINTST31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "USB New DD Request Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nddrintst](index.html) module"]
pub struct NDDRINTST_SPEC;
impl crate::RegisterSpec for NDDRINTST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nddrintst::R](R) reader structure"]
impl crate::Readable for NDDRINTST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NDDRINTST to value 0"]
impl crate::Resettable for NDDRINTST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
