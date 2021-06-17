#[doc = "Register `EPINTST` reader"]
pub struct R(crate::R<EPINTST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPINTST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPINTST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPINTST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EPST0` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST0_R(crate::FieldReader<bool, bool>);
impl EPST0_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST1` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST1_R(crate::FieldReader<bool, bool>);
impl EPST1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST2` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST2_R(crate::FieldReader<bool, bool>);
impl EPST2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST3` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST3_R(crate::FieldReader<bool, bool>);
impl EPST3_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST4` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST4_R(crate::FieldReader<bool, bool>);
impl EPST4_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST5` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST5_R(crate::FieldReader<bool, bool>);
impl EPST5_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST6` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST6_R(crate::FieldReader<bool, bool>);
impl EPST6_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST7` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST7_R(crate::FieldReader<bool, bool>);
impl EPST7_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST8` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST8_R(crate::FieldReader<bool, bool>);
impl EPST8_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST9` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST9_R(crate::FieldReader<bool, bool>);
impl EPST9_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST10` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST10_R(crate::FieldReader<bool, bool>);
impl EPST10_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST11` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST11_R(crate::FieldReader<bool, bool>);
impl EPST11_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST12` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST12_R(crate::FieldReader<bool, bool>);
impl EPST12_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST13` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST13_R(crate::FieldReader<bool, bool>);
impl EPST13_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST14` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST14_R(crate::FieldReader<bool, bool>);
impl EPST14_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST15` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST15_R(crate::FieldReader<bool, bool>);
impl EPST15_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST16` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST16_R(crate::FieldReader<bool, bool>);
impl EPST16_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST17` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST17_R(crate::FieldReader<bool, bool>);
impl EPST17_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST18` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST18_R(crate::FieldReader<bool, bool>);
impl EPST18_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST19` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST19_R(crate::FieldReader<bool, bool>);
impl EPST19_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST20` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST20_R(crate::FieldReader<bool, bool>);
impl EPST20_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST21` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST21_R(crate::FieldReader<bool, bool>);
impl EPST21_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST22` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST22_R(crate::FieldReader<bool, bool>);
impl EPST22_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST23` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST23_R(crate::FieldReader<bool, bool>);
impl EPST23_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST24` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST24_R(crate::FieldReader<bool, bool>);
impl EPST24_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST25` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST25_R(crate::FieldReader<bool, bool>);
impl EPST25_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST26` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST26_R(crate::FieldReader<bool, bool>);
impl EPST26_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST27` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST27_R(crate::FieldReader<bool, bool>);
impl EPST27_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST28` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST28_R(crate::FieldReader<bool, bool>);
impl EPST28_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST29` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST29_R(crate::FieldReader<bool, bool>);
impl EPST29_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST30` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST30_R(crate::FieldReader<bool, bool>);
impl EPST30_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPST31` reader - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
pub struct EPST31_R(crate::FieldReader<bool, bool>);
impl EPST31_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPST31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPST31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst0(&self) -> EPST0_R {
        EPST0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst1(&self) -> EPST1_R {
        EPST1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst2(&self) -> EPST2_R {
        EPST2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst3(&self) -> EPST3_R {
        EPST3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst4(&self) -> EPST4_R {
        EPST4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst5(&self) -> EPST5_R {
        EPST5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst6(&self) -> EPST6_R {
        EPST6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst7(&self) -> EPST7_R {
        EPST7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst8(&self) -> EPST8_R {
        EPST8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst9(&self) -> EPST9_R {
        EPST9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst10(&self) -> EPST10_R {
        EPST10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst11(&self) -> EPST11_R {
        EPST11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst12(&self) -> EPST12_R {
        EPST12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst13(&self) -> EPST13_R {
        EPST13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst14(&self) -> EPST14_R {
        EPST14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst15(&self) -> EPST15_R {
        EPST15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst16(&self) -> EPST16_R {
        EPST16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst17(&self) -> EPST17_R {
        EPST17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst18(&self) -> EPST18_R {
        EPST18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst19(&self) -> EPST19_R {
        EPST19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst20(&self) -> EPST20_R {
        EPST20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst21(&self) -> EPST21_R {
        EPST21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst22(&self) -> EPST22_R {
        EPST22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst23(&self) -> EPST23_R {
        EPST23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst24(&self) -> EPST24_R {
        EPST24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst25(&self) -> EPST25_R {
        EPST25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst26(&self) -> EPST26_R {
        EPST26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst27(&self) -> EPST27_R {
        EPST27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst28(&self) -> EPST28_R {
        EPST28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst29(&self) -> EPST29_R {
        EPST29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst30(&self) -> EPST30_R {
        EPST30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst31(&self) -> EPST31_R {
        EPST31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "USB Endpoint Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epintst](index.html) module"]
pub struct EPINTST_SPEC;
impl crate::RegisterSpec for EPINTST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epintst::R](R) reader structure"]
impl crate::Readable for EPINTST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EPINTST to value 0"]
impl crate::Resettable for EPINTST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
