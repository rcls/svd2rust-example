#[doc = "Register `SYSERRINTST` reader"]
pub struct R(crate::R<SYSERRINTST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSERRINTST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSERRINTST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSERRINTST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EPERRINTST0` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST0_R(crate::FieldReader<bool, bool>);
impl EPERRINTST0_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST1` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST1_R(crate::FieldReader<bool, bool>);
impl EPERRINTST1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST2` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST2_R(crate::FieldReader<bool, bool>);
impl EPERRINTST2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST3` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST3_R(crate::FieldReader<bool, bool>);
impl EPERRINTST3_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST4` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST4_R(crate::FieldReader<bool, bool>);
impl EPERRINTST4_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST5` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST5_R(crate::FieldReader<bool, bool>);
impl EPERRINTST5_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST6` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST6_R(crate::FieldReader<bool, bool>);
impl EPERRINTST6_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST7` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST7_R(crate::FieldReader<bool, bool>);
impl EPERRINTST7_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST8` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST8_R(crate::FieldReader<bool, bool>);
impl EPERRINTST8_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST9` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST9_R(crate::FieldReader<bool, bool>);
impl EPERRINTST9_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST10` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST10_R(crate::FieldReader<bool, bool>);
impl EPERRINTST10_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST11` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST11_R(crate::FieldReader<bool, bool>);
impl EPERRINTST11_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST12` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST12_R(crate::FieldReader<bool, bool>);
impl EPERRINTST12_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST13` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST13_R(crate::FieldReader<bool, bool>);
impl EPERRINTST13_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST14` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST14_R(crate::FieldReader<bool, bool>);
impl EPERRINTST14_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST15` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST15_R(crate::FieldReader<bool, bool>);
impl EPERRINTST15_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST16` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST16_R(crate::FieldReader<bool, bool>);
impl EPERRINTST16_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST17` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST17_R(crate::FieldReader<bool, bool>);
impl EPERRINTST17_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST18` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST18_R(crate::FieldReader<bool, bool>);
impl EPERRINTST18_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST19` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST19_R(crate::FieldReader<bool, bool>);
impl EPERRINTST19_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST20` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST20_R(crate::FieldReader<bool, bool>);
impl EPERRINTST20_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST21` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST21_R(crate::FieldReader<bool, bool>);
impl EPERRINTST21_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST22` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST22_R(crate::FieldReader<bool, bool>);
impl EPERRINTST22_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST23` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST23_R(crate::FieldReader<bool, bool>);
impl EPERRINTST23_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST24` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST24_R(crate::FieldReader<bool, bool>);
impl EPERRINTST24_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST25` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST25_R(crate::FieldReader<bool, bool>);
impl EPERRINTST25_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST26` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST26_R(crate::FieldReader<bool, bool>);
impl EPERRINTST26_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST27` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST27_R(crate::FieldReader<bool, bool>);
impl EPERRINTST27_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST28` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST28_R(crate::FieldReader<bool, bool>);
impl EPERRINTST28_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST29` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST29_R(crate::FieldReader<bool, bool>);
impl EPERRINTST29_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST30` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST30_R(crate::FieldReader<bool, bool>);
impl EPERRINTST30_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPERRINTST31` reader - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
pub struct EPERRINTST31_R(crate::FieldReader<bool, bool>);
impl EPERRINTST31_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPERRINTST31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPERRINTST31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst0(&self) -> EPERRINTST0_R {
        EPERRINTST0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst1(&self) -> EPERRINTST1_R {
        EPERRINTST1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst2(&self) -> EPERRINTST2_R {
        EPERRINTST2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst3(&self) -> EPERRINTST3_R {
        EPERRINTST3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst4(&self) -> EPERRINTST4_R {
        EPERRINTST4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst5(&self) -> EPERRINTST5_R {
        EPERRINTST5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst6(&self) -> EPERRINTST6_R {
        EPERRINTST6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst7(&self) -> EPERRINTST7_R {
        EPERRINTST7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst8(&self) -> EPERRINTST8_R {
        EPERRINTST8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst9(&self) -> EPERRINTST9_R {
        EPERRINTST9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst10(&self) -> EPERRINTST10_R {
        EPERRINTST10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst11(&self) -> EPERRINTST11_R {
        EPERRINTST11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst12(&self) -> EPERRINTST12_R {
        EPERRINTST12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst13(&self) -> EPERRINTST13_R {
        EPERRINTST13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst14(&self) -> EPERRINTST14_R {
        EPERRINTST14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst15(&self) -> EPERRINTST15_R {
        EPERRINTST15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst16(&self) -> EPERRINTST16_R {
        EPERRINTST16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst17(&self) -> EPERRINTST17_R {
        EPERRINTST17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst18(&self) -> EPERRINTST18_R {
        EPERRINTST18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst19(&self) -> EPERRINTST19_R {
        EPERRINTST19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst20(&self) -> EPERRINTST20_R {
        EPERRINTST20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst21(&self) -> EPERRINTST21_R {
        EPERRINTST21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst22(&self) -> EPERRINTST22_R {
        EPERRINTST22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst23(&self) -> EPERRINTST23_R {
        EPERRINTST23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst24(&self) -> EPERRINTST24_R {
        EPERRINTST24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst25(&self) -> EPERRINTST25_R {
        EPERRINTST25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst26(&self) -> EPERRINTST26_R {
        EPERRINTST26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst27(&self) -> EPERRINTST27_R {
        EPERRINTST27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst28(&self) -> EPERRINTST28_R {
        EPERRINTST28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst29(&self) -> EPERRINTST29_R {
        EPERRINTST29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst30(&self) -> EPERRINTST30_R {
        EPERRINTST30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst31(&self) -> EPERRINTST31_R {
        EPERRINTST31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "USB System Error Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syserrintst](index.html) module"]
pub struct SYSERRINTST_SPEC;
impl crate::RegisterSpec for SYSERRINTST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syserrintst::R](R) reader structure"]
impl crate::Readable for SYSERRINTST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSERRINTST to value 0"]
impl crate::Resettable for SYSERRINTST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
