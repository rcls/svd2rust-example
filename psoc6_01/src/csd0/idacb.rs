#[doc = "Register `IDACB` reader"]
pub struct R(crate::R<IDACB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDACB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDACB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDACB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDACB` writer"]
pub struct W(crate::W<IDACB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDACB_SPEC>;
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
impl From<crate::W<IDACB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDACB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL` reader - Current value setting for this IDAC (7 bits)."]
pub struct VAL_R(crate::FieldReader<u8, u8>);
impl VAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL` writer - Current value setting for this IDAC (7 bits)."]
pub struct VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Polarity is dynamic, this bit does not influence the logic in the SoftIP, it only goes to the HardIP.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL_DYN_A {
    #[doc = "0: Static polarity. Polarity is expected to be stable, so to save power this avoids the shunting of the unused polarity, at the expense of response time."]
    STATIC = 0,
    #[doc = "1: Dynamic polarity. Polarity is expected to change frequently (e.g. invert after every csd_sense phase), so to improve response time this keeps the shunt of the unused polarity on at the expense of power."]
    DYNAMIC = 1,
}
impl From<POL_DYN_A> for bool {
    #[inline(always)]
    fn from(variant: POL_DYN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL_DYN` reader - Polarity is dynamic, this bit does not influence the logic in the SoftIP, it only goes to the HardIP."]
pub struct POL_DYN_R(crate::FieldReader<bool, POL_DYN_A>);
impl POL_DYN_R {
    pub(crate) fn new(bits: bool) -> Self {
        POL_DYN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_DYN_A {
        match self.bits {
            false => POL_DYN_A::STATIC,
            true => POL_DYN_A::DYNAMIC,
        }
    }
    #[doc = "Checks if the value of the field is `STATIC`"]
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        **self == POL_DYN_A::STATIC
    }
    #[doc = "Checks if the value of the field is `DYNAMIC`"]
    #[inline(always)]
    pub fn is_dynamic(&self) -> bool {
        **self == POL_DYN_A::DYNAMIC
    }
}
impl core::ops::Deref for POL_DYN_R {
    type Target = crate::FieldReader<bool, POL_DYN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_DYN` writer - Polarity is dynamic, this bit does not influence the logic in the SoftIP, it only goes to the HardIP."]
pub struct POL_DYN_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_DYN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL_DYN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Static polarity. Polarity is expected to be stable, so to save power this avoids the shunting of the unused polarity, at the expense of response time."]
    #[inline(always)]
    pub fn static_(self) -> &'a mut W {
        self.variant(POL_DYN_A::STATIC)
    }
    #[doc = "Dynamic polarity. Polarity is expected to change frequently (e.g. invert after every csd_sense phase), so to improve response time this keeps the shunt of the unused polarity on at the expense of power."]
    #[inline(always)]
    pub fn dynamic(self) -> &'a mut W {
        self.variant(POL_DYN_A::DYNAMIC)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Selects the polarity of the IDAC (sensing operation). Normally the actual polarity depends on this bit, optionally mixed with DSI (see DSI_CTRL_EN) and if LEG1_EN==1 and LEG1_MODE==CSD also mixed with the CSD configuration and operation. In mutual cap mode however (see config.mutual_cap) the polarity of the IDAC is controlled by csd_sense. If LEG3_EN=1 (the other two legs must be off) then the ADC sequencer controls the IDACB polarity, optionally mixed with DSI.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum POLARITY_A {
    #[doc = "0: Normal: switch between Vssa and Cmod.  For non-CSD application, IDAC will source current."]
    VSSA_SRC = 0,
    #[doc = "1: Inverted: switch between Vdda and Cmod. For non-CSD application, IDAC will sink current."]
    VDDA_SNK = 1,
    #[doc = "2: The polarity of the IDAC will follow the csd_sense signal (POL_DYN bit should be set too). The intended usage is for CSX using a single IDAC."]
    SENSE = 2,
    #[doc = "3: The polarity of the IDAC will follow the inverted csd_sense signal (POL_DYN bit should be set too). The intended usage is for CSX using a single IDAC."]
    SENSE_INV = 3,
}
impl From<POLARITY_A> for u8 {
    #[inline(always)]
    fn from(variant: POLARITY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `POLARITY` reader - Selects the polarity of the IDAC (sensing operation). Normally the actual polarity depends on this bit, optionally mixed with DSI (see DSI_CTRL_EN) and if LEG1_EN==1 and LEG1_MODE==CSD also mixed with the CSD configuration and operation. In mutual cap mode however (see config.mutual_cap) the polarity of the IDAC is controlled by csd_sense. If LEG3_EN=1 (the other two legs must be off) then the ADC sequencer controls the IDACB polarity, optionally mixed with DSI."]
pub struct POLARITY_R(crate::FieldReader<u8, POLARITY_A>);
impl POLARITY_R {
    pub(crate) fn new(bits: u8) -> Self {
        POLARITY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLARITY_A {
        match self.bits {
            0 => POLARITY_A::VSSA_SRC,
            1 => POLARITY_A::VDDA_SNK,
            2 => POLARITY_A::SENSE,
            3 => POLARITY_A::SENSE_INV,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VSSA_SRC`"]
    #[inline(always)]
    pub fn is_vssa_src(&self) -> bool {
        **self == POLARITY_A::VSSA_SRC
    }
    #[doc = "Checks if the value of the field is `VDDA_SNK`"]
    #[inline(always)]
    pub fn is_vdda_snk(&self) -> bool {
        **self == POLARITY_A::VDDA_SNK
    }
    #[doc = "Checks if the value of the field is `SENSE`"]
    #[inline(always)]
    pub fn is_sense(&self) -> bool {
        **self == POLARITY_A::SENSE
    }
    #[doc = "Checks if the value of the field is `SENSE_INV`"]
    #[inline(always)]
    pub fn is_sense_inv(&self) -> bool {
        **self == POLARITY_A::SENSE_INV
    }
}
impl core::ops::Deref for POLARITY_R {
    type Target = crate::FieldReader<u8, POLARITY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POLARITY` writer - Selects the polarity of the IDAC (sensing operation). Normally the actual polarity depends on this bit, optionally mixed with DSI (see DSI_CTRL_EN) and if LEG1_EN==1 and LEG1_MODE==CSD also mixed with the CSD configuration and operation. In mutual cap mode however (see config.mutual_cap) the polarity of the IDAC is controlled by csd_sense. If LEG3_EN=1 (the other two legs must be off) then the ADC sequencer controls the IDACB polarity, optionally mixed with DSI."]
pub struct POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> POLARITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POLARITY_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Normal: switch between Vssa and Cmod. For non-CSD application, IDAC will source current."]
    #[inline(always)]
    pub fn vssa_src(self) -> &'a mut W {
        self.variant(POLARITY_A::VSSA_SRC)
    }
    #[doc = "Inverted: switch between Vdda and Cmod. For non-CSD application, IDAC will sink current."]
    #[inline(always)]
    pub fn vdda_snk(self) -> &'a mut W {
        self.variant(POLARITY_A::VDDA_SNK)
    }
    #[doc = "The polarity of the IDAC will follow the csd_sense signal (POL_DYN bit should be set too). The intended usage is for CSX using a single IDAC."]
    #[inline(always)]
    pub fn sense(self) -> &'a mut W {
        self.variant(POLARITY_A::SENSE)
    }
    #[doc = "The polarity of the IDAC will follow the inverted csd_sense signal (POL_DYN bit should be set too). The intended usage is for CSX using a single IDAC."]
    #[inline(always)]
    pub fn sense_inv(self) -> &'a mut W {
        self.variant(POLARITY_A::SENSE_INV)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "same as corresponding IDACA Balancing mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BAL_MODE_A {
    #[doc = "0: same as corresponding IDACA Balancing mode"]
    FULL = 0,
    #[doc = "1: same as corresponding IDACA Balancing mode"]
    PHI1 = 1,
    #[doc = "2: same as corresponding IDACA Balancing mode"]
    PHI2 = 2,
    #[doc = "3: same as corresponding IDACA Balancing mode"]
    PHI1_2 = 3,
}
impl From<BAL_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: BAL_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BAL_MODE` reader - same as corresponding IDACA Balancing mode"]
pub struct BAL_MODE_R(crate::FieldReader<u8, BAL_MODE_A>);
impl BAL_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        BAL_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BAL_MODE_A {
        match self.bits {
            0 => BAL_MODE_A::FULL,
            1 => BAL_MODE_A::PHI1,
            2 => BAL_MODE_A::PHI2,
            3 => BAL_MODE_A::PHI1_2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        **self == BAL_MODE_A::FULL
    }
    #[doc = "Checks if the value of the field is `PHI1`"]
    #[inline(always)]
    pub fn is_phi1(&self) -> bool {
        **self == BAL_MODE_A::PHI1
    }
    #[doc = "Checks if the value of the field is `PHI2`"]
    #[inline(always)]
    pub fn is_phi2(&self) -> bool {
        **self == BAL_MODE_A::PHI2
    }
    #[doc = "Checks if the value of the field is `PHI1_2`"]
    #[inline(always)]
    pub fn is_phi1_2(&self) -> bool {
        **self == BAL_MODE_A::PHI1_2
    }
}
impl core::ops::Deref for BAL_MODE_R {
    type Target = crate::FieldReader<u8, BAL_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BAL_MODE` writer - same as corresponding IDACA Balancing mode"]
pub struct BAL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BAL_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BAL_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "same as corresponding IDACA Balancing mode"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(BAL_MODE_A::FULL)
    }
    #[doc = "same as corresponding IDACA Balancing mode"]
    #[inline(always)]
    pub fn phi1(self) -> &'a mut W {
        self.variant(BAL_MODE_A::PHI1)
    }
    #[doc = "same as corresponding IDACA Balancing mode"]
    #[inline(always)]
    pub fn phi2(self) -> &'a mut W {
        self.variant(BAL_MODE_A::PHI2)
    }
    #[doc = "same as corresponding IDACA Balancing mode"]
    #[inline(always)]
    pub fn phi1_2(self) -> &'a mut W {
        self.variant(BAL_MODE_A::PHI1_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Controls the usage mode of LEG1 and the Polarity bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEG1_MODE_A {
    #[doc = "0: same as corresponding IDACA.LEG1_MODE"]
    GP_STATIC = 0,
    #[doc = "1: same as corresponding IDACA.LEG1_MODE"]
    GP = 1,
    #[doc = "2: same as corresponding IDACA.LEG1_MODE"]
    CSD_STATIC = 2,
    #[doc = "3: same as corresponding IDACA.LEG1_MODE"]
    CSD = 3,
}
impl From<LEG1_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LEG1_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LEG1_MODE` reader - Controls the usage mode of LEG1 and the Polarity bit"]
pub struct LEG1_MODE_R(crate::FieldReader<u8, LEG1_MODE_A>);
impl LEG1_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        LEG1_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEG1_MODE_A {
        match self.bits {
            0 => LEG1_MODE_A::GP_STATIC,
            1 => LEG1_MODE_A::GP,
            2 => LEG1_MODE_A::CSD_STATIC,
            3 => LEG1_MODE_A::CSD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GP_STATIC`"]
    #[inline(always)]
    pub fn is_gp_static(&self) -> bool {
        **self == LEG1_MODE_A::GP_STATIC
    }
    #[doc = "Checks if the value of the field is `GP`"]
    #[inline(always)]
    pub fn is_gp(&self) -> bool {
        **self == LEG1_MODE_A::GP
    }
    #[doc = "Checks if the value of the field is `CSD_STATIC`"]
    #[inline(always)]
    pub fn is_csd_static(&self) -> bool {
        **self == LEG1_MODE_A::CSD_STATIC
    }
    #[doc = "Checks if the value of the field is `CSD`"]
    #[inline(always)]
    pub fn is_csd(&self) -> bool {
        **self == LEG1_MODE_A::CSD
    }
}
impl core::ops::Deref for LEG1_MODE_R {
    type Target = crate::FieldReader<u8, LEG1_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEG1_MODE` writer - Controls the usage mode of LEG1 and the Polarity bit"]
pub struct LEG1_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LEG1_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEG1_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "same as corresponding IDACA.LEG1_MODE"]
    #[inline(always)]
    pub fn gp_static(self) -> &'a mut W {
        self.variant(LEG1_MODE_A::GP_STATIC)
    }
    #[doc = "same as corresponding IDACA.LEG1_MODE"]
    #[inline(always)]
    pub fn gp(self) -> &'a mut W {
        self.variant(LEG1_MODE_A::GP)
    }
    #[doc = "same as corresponding IDACA.LEG1_MODE"]
    #[inline(always)]
    pub fn csd_static(self) -> &'a mut W {
        self.variant(LEG1_MODE_A::CSD_STATIC)
    }
    #[doc = "same as corresponding IDACA.LEG1_MODE"]
    #[inline(always)]
    pub fn csd(self) -> &'a mut W {
        self.variant(LEG1_MODE_A::CSD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Controls the usage mode of LEG2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEG2_MODE_A {
    #[doc = "0: same as corresponding IDACA.LEG2_MODE"]
    GP_STATIC = 0,
    #[doc = "1: same as corresponding IDACA.LEG2_MODE"]
    GP = 1,
    #[doc = "2: same as corresponding IDACA.LEG2_MODE"]
    CSD_STATIC = 2,
    #[doc = "3: same as corresponding IDACA.LEG2_MODE"]
    CSD = 3,
}
impl From<LEG2_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LEG2_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LEG2_MODE` reader - Controls the usage mode of LEG2"]
pub struct LEG2_MODE_R(crate::FieldReader<u8, LEG2_MODE_A>);
impl LEG2_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        LEG2_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEG2_MODE_A {
        match self.bits {
            0 => LEG2_MODE_A::GP_STATIC,
            1 => LEG2_MODE_A::GP,
            2 => LEG2_MODE_A::CSD_STATIC,
            3 => LEG2_MODE_A::CSD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GP_STATIC`"]
    #[inline(always)]
    pub fn is_gp_static(&self) -> bool {
        **self == LEG2_MODE_A::GP_STATIC
    }
    #[doc = "Checks if the value of the field is `GP`"]
    #[inline(always)]
    pub fn is_gp(&self) -> bool {
        **self == LEG2_MODE_A::GP
    }
    #[doc = "Checks if the value of the field is `CSD_STATIC`"]
    #[inline(always)]
    pub fn is_csd_static(&self) -> bool {
        **self == LEG2_MODE_A::CSD_STATIC
    }
    #[doc = "Checks if the value of the field is `CSD`"]
    #[inline(always)]
    pub fn is_csd(&self) -> bool {
        **self == LEG2_MODE_A::CSD
    }
}
impl core::ops::Deref for LEG2_MODE_R {
    type Target = crate::FieldReader<u8, LEG2_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEG2_MODE` writer - Controls the usage mode of LEG2"]
pub struct LEG2_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LEG2_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEG2_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "same as corresponding IDACA.LEG2_MODE"]
    #[inline(always)]
    pub fn gp_static(self) -> &'a mut W {
        self.variant(LEG2_MODE_A::GP_STATIC)
    }
    #[doc = "same as corresponding IDACA.LEG2_MODE"]
    #[inline(always)]
    pub fn gp(self) -> &'a mut W {
        self.variant(LEG2_MODE_A::GP)
    }
    #[doc = "same as corresponding IDACA.LEG2_MODE"]
    #[inline(always)]
    pub fn csd_static(self) -> &'a mut W {
        self.variant(LEG2_MODE_A::CSD_STATIC)
    }
    #[doc = "same as corresponding IDACA.LEG2_MODE"]
    #[inline(always)]
    pub fn csd(self) -> &'a mut W {
        self.variant(LEG2_MODE_A::CSD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `DSI_CTRL_EN` reader - Mix DSI inputs with MMIO controls or not (before getting mixed with CSD controls if enabled) 0: no DSI control IDACB_POLARITY = IDACB.POLARITY IDACB_LEG1_EN = IDACB.LEG1_EN IDACB_LEG2_EN = IDACB.LEG2_EN IDACB_LEG3_EN = IDACB.LEG3_EN 1: Mix MMIO with DSI control IDACB_POLARITY = IDACB.POLARITY EXOR dsi_idacb_pol IDACB_LEG1_EN = IDACB.LEG1_EN AND dsi_idacb_leg1_en IDACB_LEG2_EN = IDACB.LEG2_EN AND dsi_idacb_leg2_en IDACB_LEG3_EN = IDACB.LEG3_EN AND dsi_idacb_leg3_en"]
pub struct DSI_CTRL_EN_R(crate::FieldReader<bool, bool>);
impl DSI_CTRL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSI_CTRL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSI_CTRL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSI_CTRL_EN` writer - Mix DSI inputs with MMIO controls or not (before getting mixed with CSD controls if enabled) 0: no DSI control IDACB_POLARITY = IDACB.POLARITY IDACB_LEG1_EN = IDACB.LEG1_EN IDACB_LEG2_EN = IDACB.LEG2_EN IDACB_LEG3_EN = IDACB.LEG3_EN 1: Mix MMIO with DSI control IDACB_POLARITY = IDACB.POLARITY EXOR dsi_idacb_pol IDACB_LEG1_EN = IDACB.LEG1_EN AND dsi_idacb_leg1_en IDACB_LEG2_EN = IDACB.LEG2_EN AND dsi_idacb_leg2_en IDACB_LEG3_EN = IDACB.LEG3_EN AND dsi_idacb_leg3_en"]
pub struct DSI_CTRL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSI_CTRL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "IDAC multiplier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RANGE_A {
    #[doc = "0: 1 LSB =   37.5 nA"]
    IDAC_LO = 0,
    #[doc = "1: 1 LSB =  300 nA"]
    IDAC_MED = 1,
    #[doc = "2: 1 LSB = 2400 nA"]
    IDAC_HI = 2,
}
impl From<RANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: RANGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RANGE` reader - IDAC multiplier"]
pub struct RANGE_R(crate::FieldReader<u8, RANGE_A>);
impl RANGE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RANGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RANGE_A> {
        match self.bits {
            0 => Some(RANGE_A::IDAC_LO),
            1 => Some(RANGE_A::IDAC_MED),
            2 => Some(RANGE_A::IDAC_HI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IDAC_LO`"]
    #[inline(always)]
    pub fn is_idac_lo(&self) -> bool {
        **self == RANGE_A::IDAC_LO
    }
    #[doc = "Checks if the value of the field is `IDAC_MED`"]
    #[inline(always)]
    pub fn is_idac_med(&self) -> bool {
        **self == RANGE_A::IDAC_MED
    }
    #[doc = "Checks if the value of the field is `IDAC_HI`"]
    #[inline(always)]
    pub fn is_idac_hi(&self) -> bool {
        **self == RANGE_A::IDAC_HI
    }
}
impl core::ops::Deref for RANGE_R {
    type Target = crate::FieldReader<u8, RANGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RANGE` writer - IDAC multiplier"]
pub struct RANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> RANGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RANGE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 LSB = 37.5 nA"]
    #[inline(always)]
    pub fn idac_lo(self) -> &'a mut W {
        self.variant(RANGE_A::IDAC_LO)
    }
    #[doc = "1 LSB = 300 nA"]
    #[inline(always)]
    pub fn idac_med(self) -> &'a mut W {
        self.variant(RANGE_A::IDAC_MED)
    }
    #[doc = "1 LSB = 2400 nA"]
    #[inline(always)]
    pub fn idac_hi(self) -> &'a mut W {
        self.variant(RANGE_A::IDAC_HI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `LEG1_EN` reader - output enable for leg 1 to CSDBUSB or CSDBUSA"]
pub struct LEG1_EN_R(crate::FieldReader<bool, bool>);
impl LEG1_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LEG1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEG1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEG1_EN` writer - output enable for leg 1 to CSDBUSB or CSDBUSA"]
pub struct LEG1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEG1_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `LEG2_EN` reader - output enable for leg 2 to CSDBUSB or CSDBUSA"]
pub struct LEG2_EN_R(crate::FieldReader<bool, bool>);
impl LEG2_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LEG2_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEG2_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEG2_EN` writer - output enable for leg 2 to CSDBUSB or CSDBUSA"]
pub struct LEG2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEG2_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `LEG3_EN` reader - output enable for leg3 to CSDBUSC, only allowed when RANGE = IDAC_LO. When this bit is set both other legs should be off. Note that leg3 can only be used for ADC mode, not GP mode. Which means that leg3 can only be on when the ADC Sequencer is in the ADC_measure or Calib_measure state. In those states leg3 is controlled by the ADC configuration and the HSCMP output. In addition this leg3 enable bit can optionally be mixed with DSI (see DSI_CTRL_EN). When LEG3_EN=1 also the IDACB polarity is controlled by the ADC sequencer."]
pub struct LEG3_EN_R(crate::FieldReader<bool, bool>);
impl LEG3_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LEG3_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEG3_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEG3_EN` writer - output enable for leg3 to CSDBUSC, only allowed when RANGE = IDAC_LO. When this bit is set both other legs should be off. Note that leg3 can only be used for ADC mode, not GP mode. Which means that leg3 can only be on when the ADC Sequencer is in the ADC_measure or Calib_measure state. In those states leg3 is controlled by the ADC configuration and the HSCMP output. In addition this leg3 enable bit can optionally be mixed with DSI (see DSI_CTRL_EN). When LEG3_EN=1 also the IDACB polarity is controlled by the ADC sequencer."]
pub struct LEG3_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEG3_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Current value setting for this IDAC (7 bits)."]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Polarity is dynamic, this bit does not influence the logic in the SoftIP, it only goes to the HardIP."]
    #[inline(always)]
    pub fn pol_dyn(&self) -> POL_DYN_R {
        POL_DYN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Selects the polarity of the IDAC (sensing operation). Normally the actual polarity depends on this bit, optionally mixed with DSI (see DSI_CTRL_EN) and if LEG1_EN==1 and LEG1_MODE==CSD also mixed with the CSD configuration and operation. In mutual cap mode however (see config.mutual_cap) the polarity of the IDAC is controlled by csd_sense. If LEG3_EN=1 (the other two legs must be off) then the ADC sequencer controls the IDACB polarity, optionally mixed with DSI."]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - same as corresponding IDACA Balancing mode"]
    #[inline(always)]
    pub fn bal_mode(&self) -> BAL_MODE_R {
        BAL_MODE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Controls the usage mode of LEG1 and the Polarity bit"]
    #[inline(always)]
    pub fn leg1_mode(&self) -> LEG1_MODE_R {
        LEG1_MODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Controls the usage mode of LEG2"]
    #[inline(always)]
    pub fn leg2_mode(&self) -> LEG2_MODE_R {
        LEG2_MODE_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 21 - Mix DSI inputs with MMIO controls or not (before getting mixed with CSD controls if enabled) 0: no DSI control IDACB_POLARITY = IDACB.POLARITY IDACB_LEG1_EN = IDACB.LEG1_EN IDACB_LEG2_EN = IDACB.LEG2_EN IDACB_LEG3_EN = IDACB.LEG3_EN 1: Mix MMIO with DSI control IDACB_POLARITY = IDACB.POLARITY EXOR dsi_idacb_pol IDACB_LEG1_EN = IDACB.LEG1_EN AND dsi_idacb_leg1_en IDACB_LEG2_EN = IDACB.LEG2_EN AND dsi_idacb_leg2_en IDACB_LEG3_EN = IDACB.LEG3_EN AND dsi_idacb_leg3_en"]
    #[inline(always)]
    pub fn dsi_ctrl_en(&self) -> DSI_CTRL_EN_R {
        DSI_CTRL_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - IDAC multiplier"]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 24 - output enable for leg 1 to CSDBUSB or CSDBUSA"]
    #[inline(always)]
    pub fn leg1_en(&self) -> LEG1_EN_R {
        LEG1_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - output enable for leg 2 to CSDBUSB or CSDBUSA"]
    #[inline(always)]
    pub fn leg2_en(&self) -> LEG2_EN_R {
        LEG2_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - output enable for leg3 to CSDBUSC, only allowed when RANGE = IDAC_LO. When this bit is set both other legs should be off. Note that leg3 can only be used for ADC mode, not GP mode. Which means that leg3 can only be on when the ADC Sequencer is in the ADC_measure or Calib_measure state. In those states leg3 is controlled by the ADC configuration and the HSCMP output. In addition this leg3 enable bit can optionally be mixed with DSI (see DSI_CTRL_EN). When LEG3_EN=1 also the IDACB polarity is controlled by the ADC sequencer."]
    #[inline(always)]
    pub fn leg3_en(&self) -> LEG3_EN_R {
        LEG3_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Current value setting for this IDAC (7 bits)."]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W {
        VAL_W { w: self }
    }
    #[doc = "Bit 7 - Polarity is dynamic, this bit does not influence the logic in the SoftIP, it only goes to the HardIP."]
    #[inline(always)]
    pub fn pol_dyn(&mut self) -> POL_DYN_W {
        POL_DYN_W { w: self }
    }
    #[doc = "Bits 8:9 - Selects the polarity of the IDAC (sensing operation). Normally the actual polarity depends on this bit, optionally mixed with DSI (see DSI_CTRL_EN) and if LEG1_EN==1 and LEG1_MODE==CSD also mixed with the CSD configuration and operation. In mutual cap mode however (see config.mutual_cap) the polarity of the IDAC is controlled by csd_sense. If LEG3_EN=1 (the other two legs must be off) then the ADC sequencer controls the IDACB polarity, optionally mixed with DSI."]
    #[inline(always)]
    pub fn polarity(&mut self) -> POLARITY_W {
        POLARITY_W { w: self }
    }
    #[doc = "Bits 10:11 - same as corresponding IDACA Balancing mode"]
    #[inline(always)]
    pub fn bal_mode(&mut self) -> BAL_MODE_W {
        BAL_MODE_W { w: self }
    }
    #[doc = "Bits 16:17 - Controls the usage mode of LEG1 and the Polarity bit"]
    #[inline(always)]
    pub fn leg1_mode(&mut self) -> LEG1_MODE_W {
        LEG1_MODE_W { w: self }
    }
    #[doc = "Bits 18:19 - Controls the usage mode of LEG2"]
    #[inline(always)]
    pub fn leg2_mode(&mut self) -> LEG2_MODE_W {
        LEG2_MODE_W { w: self }
    }
    #[doc = "Bit 21 - Mix DSI inputs with MMIO controls or not (before getting mixed with CSD controls if enabled) 0: no DSI control IDACB_POLARITY = IDACB.POLARITY IDACB_LEG1_EN = IDACB.LEG1_EN IDACB_LEG2_EN = IDACB.LEG2_EN IDACB_LEG3_EN = IDACB.LEG3_EN 1: Mix MMIO with DSI control IDACB_POLARITY = IDACB.POLARITY EXOR dsi_idacb_pol IDACB_LEG1_EN = IDACB.LEG1_EN AND dsi_idacb_leg1_en IDACB_LEG2_EN = IDACB.LEG2_EN AND dsi_idacb_leg2_en IDACB_LEG3_EN = IDACB.LEG3_EN AND dsi_idacb_leg3_en"]
    #[inline(always)]
    pub fn dsi_ctrl_en(&mut self) -> DSI_CTRL_EN_W {
        DSI_CTRL_EN_W { w: self }
    }
    #[doc = "Bits 22:23 - IDAC multiplier"]
    #[inline(always)]
    pub fn range(&mut self) -> RANGE_W {
        RANGE_W { w: self }
    }
    #[doc = "Bit 24 - output enable for leg 1 to CSDBUSB or CSDBUSA"]
    #[inline(always)]
    pub fn leg1_en(&mut self) -> LEG1_EN_W {
        LEG1_EN_W { w: self }
    }
    #[doc = "Bit 25 - output enable for leg 2 to CSDBUSB or CSDBUSA"]
    #[inline(always)]
    pub fn leg2_en(&mut self) -> LEG2_EN_W {
        LEG2_EN_W { w: self }
    }
    #[doc = "Bit 26 - output enable for leg3 to CSDBUSC, only allowed when RANGE = IDAC_LO. When this bit is set both other legs should be off. Note that leg3 can only be used for ADC mode, not GP mode. Which means that leg3 can only be on when the ADC Sequencer is in the ADC_measure or Calib_measure state. In those states leg3 is controlled by the ADC configuration and the HSCMP output. In addition this leg3 enable bit can optionally be mixed with DSI (see DSI_CTRL_EN). When LEG3_EN=1 also the IDACB polarity is controlled by the ADC sequencer."]
    #[inline(always)]
    pub fn leg3_en(&mut self) -> LEG3_EN_W {
        LEG3_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IDACB Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idacb](index.html) module"]
pub struct IDACB_SPEC;
impl crate::RegisterSpec for IDACB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idacb::R](R) reader structure"]
impl crate::Readable for IDACB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idacb::W](W) writer structure"]
impl crate::Writable for IDACB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IDACB to value 0"]
impl crate::Resettable for IDACB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
