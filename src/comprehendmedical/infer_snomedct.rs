use aws_sdk_comprehendmedical::operation::infer_snomedct::InferSnomedctOutput;
use aws_sdk_comprehendmedical::types::{
    Characters, SnomedctAttribute, SnomedctAttributeType, SnomedctConcept, SnomedctDetails,
    SnomedctEntity, SnomedctEntityCategory, SnomedctEntityType, SnomedctRelationshipType,
    SnomedctTrait, SnomedctTraitName,
};
use serde::{Deserialize, Serialize};

use crate::comprehendmedical::primitives::sealed_enum_unknown::UnknownVariantValue_;

#[derive(Serialize, Deserialize, Default)]
pub struct InferSnomedctOutput_ {
    pub entities: Vec<SnomedctEntity_>,
    pub pagination_token: Option<String>,
    pub model_version: Option<String>,
    pub snomedct_details: Option<SnomedctDetails_>,
    pub characters: Option<Characters_>,
}

impl From<InferSnomedctOutput> for InferSnomedctOutput_ {
    fn from(value: InferSnomedctOutput) -> Self {
        InferSnomedctOutput_ {
            entities: value
                .entities
                .into_iter()
                .map(|entity| entity.into())
                .collect(),
            pagination_token: value.pagination_token,
            model_version: value.model_version,
            snomedct_details: value.snomedct_details.map(|details| details.into()),
            characters: value.characters.map(|characters| characters.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct SnomedctEntity_ {
    pub id: Option<i32>,
    pub text: Option<String>,
    pub category: Option<SnomedctEntityCategory_>,
    #[serde(rename = "type")]
    pub type_: Option<SnomedctEntityType_>,
    pub score: Option<f32>,
    pub begin_offset: Option<i32>,
    pub end_offset: Option<i32>,
    pub attributes: Option<Vec<SnomedctAttribute_>>,
    pub traits: Option<Vec<SnomedctTrait_>>,
    pub snomedct_concepts: Option<Vec<SnomedctConcept_>>,
}

impl From<SnomedctEntity> for SnomedctEntity_ {
    fn from(value: SnomedctEntity) -> Self {
        SnomedctEntity_ {
            id: value.id,
            text: value.text,
            category: value.category.map(|category| category.into()),
            type_: value.r#type.map(|type_| type_.into()),
            score: value.score,
            begin_offset: value.begin_offset,
            end_offset: value.end_offset,
            attributes: value.attributes.map(|attributes| {
                attributes
                    .into_iter()
                    .map(|attribute| attribute.into())
                    .collect()
            }),
            traits: value
                .traits
                .map(|traits| traits.into_iter().map(|trait_| trait_.into()).collect()),
            snomedct_concepts: value
                .snomedct_concepts
                .map(|concepts| concepts.into_iter().map(|concept| concept.into()).collect()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum SnomedctEntityCategory_ {
    Anatomy,
    MedicalCondition,
    TestTreatmentProcedure,
    Unknown(UnknownVariantValue_),
}

impl Default for SnomedctEntityCategory_ {
    fn default() -> Self {
        SnomedctEntityCategory_::Unknown(UnknownVariantValue_("Unknown".to_string()))
    }
}

impl From<SnomedctEntityCategory> for SnomedctEntityCategory_ {
    fn from(value: SnomedctEntityCategory) -> Self {
        match value {
            SnomedctEntityCategory::Anatomy => SnomedctEntityCategory_::Anatomy,
            SnomedctEntityCategory::MedicalCondition => SnomedctEntityCategory_::MedicalCondition,
            SnomedctEntityCategory::TestTreatmentProcedure => {
                SnomedctEntityCategory_::TestTreatmentProcedure
            }
            other if other.as_str() == "NewFeature" => {
                SnomedctEntityCategory_::Unknown(UnknownVariantValue_(other.as_str().to_string()))
            }
            _ => SnomedctEntityCategory_::Unknown(UnknownVariantValue_("Unknown".to_string())),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum SnomedctEntityType_ {
    DxName,
    ProcedureName,
    TestName,
    TreatmentName,
    Unknown(UnknownVariantValue_),
}

impl Default for SnomedctEntityType_ {
    fn default() -> Self {
        SnomedctEntityType_::Unknown(UnknownVariantValue_("Unknown".to_string()))
    }
}

impl From<SnomedctEntityType> for SnomedctEntityType_ {
    fn from(value: SnomedctEntityType) -> Self {
        match value {
            SnomedctEntityType::DxName => SnomedctEntityType_::DxName,
            SnomedctEntityType::ProcedureName => SnomedctEntityType_::ProcedureName,
            SnomedctEntityType::TestName => SnomedctEntityType_::TestName,
            SnomedctEntityType::TreatmentName => SnomedctEntityType_::TreatmentName,
            other if other.as_str() == "NewFeature" => {
                SnomedctEntityType_::Unknown(UnknownVariantValue_(other.as_str().to_string()))
            }
            _ => SnomedctEntityType_::Unknown(UnknownVariantValue_("Unknown".to_string())),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct SnomedctAttribute_ {
    pub category: Option<SnomedctEntityCategory_>,
    #[serde(rename = "type")]
    pub type_: Option<SnomedctAttributeType_>,
    pub score: Option<f32>,
    pub relationship_score: Option<f32>,
    pub relationship_type: Option<SnomedctRelationshipType_>,
    pub id: Option<i32>,
    pub begin_offset: Option<i32>,
    pub end_offset: Option<i32>,
    pub text: Option<String>,
    pub traits: Option<Vec<SnomedctTrait_>>,
    pub snomedct_concepts: Option<Vec<SnomedctConcept_>>,
}

impl From<SnomedctAttribute> for SnomedctAttribute_ {
    fn from(value: SnomedctAttribute) -> Self {
        SnomedctAttribute_ {
            category: value.category.map(|category| category.into()),
            type_: value.r#type.map(|type_| type_.into()),
            score: value.score,
            relationship_score: value.relationship_score,
            relationship_type: value
                .relationship_type
                .map(|relationship_type| relationship_type.into()),
            id: value.id,
            begin_offset: value.begin_offset,
            end_offset: value.end_offset,
            text: value.text,
            traits: value
                .traits
                .map(|traits| traits.into_iter().map(|trait_| trait_.into()).collect()),
            snomedct_concepts: value
                .snomedct_concepts
                .map(|concepts| concepts.into_iter().map(|concept| concept.into()).collect()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SnomedctTrait_ {
    pub name: Option<SnomedctTraitName_>,
    pub score: Option<f32>,
}

impl From<SnomedctTrait> for SnomedctTrait_ {
    fn from(value: SnomedctTrait) -> Self {
        SnomedctTrait_ {
            name: value.name.map(|name| name.into()),
            score: value.score,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct SnomedctConcept_ {
    pub description: Option<String>,
    pub code: Option<String>,
    pub score: Option<f32>,
}

impl From<SnomedctConcept> for SnomedctConcept_ {
    fn from(value: SnomedctConcept) -> Self {
        SnomedctConcept_ {
            description: value.description,
            code: value.code,
            score: value.score,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct SnomedctDetails_ {
    pub edition: Option<String>,
    pub language: Option<String>,
    pub version_date: Option<String>,
}

impl From<SnomedctDetails> for SnomedctDetails_ {
    fn from(value: SnomedctDetails) -> Self {
        SnomedctDetails_ {
            edition: value.edition,
            language: value.language,
            version_date: value.version_date,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Characters_ {
    pub original_text_characters: Option<i32>,
}

impl From<Characters> for Characters_ {
    fn from(value: Characters) -> Self {
        Characters_ {
            original_text_characters: value.original_text_characters,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum SnomedctAttributeType_ {
    Acuity,
    Direction,
    Quality,
    SystemOrganSite,
    TestUnit,
    TestValue,
    Unknown(UnknownVariantValue_),
}

impl Default for SnomedctAttributeType_ {
    fn default() -> Self {
        SnomedctAttributeType_::Unknown(UnknownVariantValue_("Unknown".to_string()))
    }
}

impl From<SnomedctAttributeType> for SnomedctAttributeType_ {
    fn from(value: SnomedctAttributeType) -> Self {
        match value {
            SnomedctAttributeType::Acuity => SnomedctAttributeType_::Acuity,
            SnomedctAttributeType::Direction => SnomedctAttributeType_::Direction,
            SnomedctAttributeType::Quality => SnomedctAttributeType_::Quality,
            SnomedctAttributeType::SystemOrganSite => SnomedctAttributeType_::SystemOrganSite,
            SnomedctAttributeType::TestUnit => SnomedctAttributeType_::TestUnit,
            SnomedctAttributeType::TestValue => SnomedctAttributeType_::TestValue,
            other if other.as_str() == "NewFeature" => {
                SnomedctAttributeType_::Unknown(UnknownVariantValue_(other.as_str().to_string()))
            }
            _ => SnomedctAttributeType_::Unknown(UnknownVariantValue_("Unknown".to_string())),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum SnomedctRelationshipType_ {
    Acuity,
    Direction,
    Quality,
    SystemOrganSite,
    TestUnit,
    TestUnits,
    TestValue,
    Unknown(UnknownVariantValue_),
}

impl Default for SnomedctRelationshipType_ {
    fn default() -> Self {
        SnomedctRelationshipType_::Unknown(UnknownVariantValue_("Unknown".to_string()))
    }
}

impl From<SnomedctRelationshipType> for SnomedctRelationshipType_ {
    fn from(value: SnomedctRelationshipType) -> Self {
        match value {
            SnomedctRelationshipType::Acuity => SnomedctRelationshipType_::Acuity,
            SnomedctRelationshipType::Direction => SnomedctRelationshipType_::Direction,
            SnomedctRelationshipType::Quality => SnomedctRelationshipType_::Quality,
            SnomedctRelationshipType::SystemOrganSite => SnomedctRelationshipType_::SystemOrganSite,
            SnomedctRelationshipType::TestUnit => SnomedctRelationshipType_::TestUnit,
            SnomedctRelationshipType::TestUnits => SnomedctRelationshipType_::TestUnits,
            SnomedctRelationshipType::TestValue => SnomedctRelationshipType_::TestValue,
            other if other.as_str() == "NewFeature" => {
                SnomedctRelationshipType_::Unknown(UnknownVariantValue_(other.as_str().to_string()))
            }
            _ => SnomedctRelationshipType_::Unknown(UnknownVariantValue_("Unknown".to_string())),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum SnomedctTraitName_ {
    Diagnosis,
    Future,
    Hypothetical,
    LowConfidence,
    Negation,
    PastHistory,
    PertainsToFamily,
    Sign,
    Symptom,
    Unknown(UnknownVariantValue_),
}

impl Default for SnomedctTraitName_ {
    fn default() -> Self {
        SnomedctTraitName_::Unknown(UnknownVariantValue_("Unknown".to_string()))
    }
}

impl From<SnomedctTraitName> for SnomedctTraitName_ {
    fn from(value: SnomedctTraitName) -> Self {
        match value {
            SnomedctTraitName::Diagnosis => SnomedctTraitName_::Diagnosis,
            SnomedctTraitName::Future => SnomedctTraitName_::Future,
            SnomedctTraitName::Hypothetical => SnomedctTraitName_::Hypothetical,
            SnomedctTraitName::LowConfidence => SnomedctTraitName_::LowConfidence,
            SnomedctTraitName::Negation => SnomedctTraitName_::Negation,
            SnomedctTraitName::PastHistory => SnomedctTraitName_::PastHistory,
            SnomedctTraitName::PertainsToFamily => SnomedctTraitName_::PertainsToFamily,
            SnomedctTraitName::Sign => SnomedctTraitName_::Sign,
            SnomedctTraitName::Symptom => SnomedctTraitName_::Symptom,
            other if other.as_str() == "NewFeature" => {
                SnomedctTraitName_::Unknown(UnknownVariantValue_(other.as_str().to_string()))
            }
            _ => SnomedctTraitName_::Unknown(UnknownVariantValue_("Unknown".to_string())),
        }
    }
}
