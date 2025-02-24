use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A common base type fault for all Ovf related faults.
/// 
/// The structure of OvfFault is as listed.
/// - OvfFault
///   - OvfInvalidPackage
///     - OvfXmlFormat
///     - OvfWrongNamespace
///     - OvfElement
///       - OvfElementInvalidValue
///       - OvfUnexpectedElement
///       - OvfDuplicateElement
///       - OvfMissingElement
///       - OvfMissingElementNormalBoundary
///       - OvfDuplicatedElementBoundary
///     - OvfAttribute
///       - OvfMissingAttribute
///       - OvfInvalidValue
///         - OvfInvalidValueFormatMalformed
///         - OvfInvalidValueConfiguration
///         - OvfInvalidValueReference
///         - OvfInvalidValueEmpty
///     - OvfProperty
///       - OvfPropertyType
///       - OvfPropertyValue
///       - OvfPropertyNetwork
///       - OvfPropertyQualifier
///       - OvfPropertyQualifierWarning
///   - OvfConstraint
///     - OvfDiskOrderConstraint
///     - OvfHostResourceConstraint
///   - OvfUnsupportedPackage
///     - OvfNoHostNic
///     - OvfInvalidVmName
///     - OvfUnsupportedAttribute
///       - OvfUnsupportedAttributeValue
///     - OvfUnsupportedElement
///       - OvfUnsupportedElementValue
///       - OvfUnsupportedSection
///       - OvfNoSpaceOnController
///     - OvfUnsupportedType
///     - OvfUnsupportedSubType
///     - OvfHardwareCheck
///     - OvfNoSupportedHardwareFamily
///   - OvfExport
///     - OvfExportFailed
///     - OvfHardwareExport
///       - OvfUnsupportedDeviceExport
///       - OvfUnknownDeviceBacking
///       - OvfConnectedDevice
///         - OvfConnectedDeviceISO
///       - OvfUnableToExportDisk
///     - OvfPropertyExport
///     - OvfPropertyNetworkExport
///     - OvfDuplicatedPropertyIdExport
///   - OvfImport (these are typically returned as warnings)
///     - OvfImportFailed
///     - OvfHardwareCheck
///     - OvfMissingHardware
///     - OvfCpuCompatibility
///     - OvfCpuCompatibilityCheckNotSupported
///     - OvfUnsupportedDiskProvisioning
///     - OvfDuplicatedPropertyIdImport
///     - OvfNetworkMappingNotSupported
///   - OvfSystemFault
///     - OvfDiskMappingNotFound
///     - OvfHostValueNotParsed
///     - OvfInternalError
///     - OvfUnsupportedDeviceBackingOption
///     - OvfUnsupportedDeviceBackingInfo
///     - OvfToXmlUnsupportedElement
///     - OvfUnknownDevice
///     - OvfUnknownEntity
///   - OvfConsumerCallbackFault
///     - OvfConsumerFault
///     - OvfConsumerCommunicationError
///     - OvfConsumerInvalidSection
///     - OvfConsumerUndeclaredSection
///     - OvfConsumerUndefinedPrefix
///       
/// All messages go into the vimlocale
pub trait OvfFaultTrait : super::vim_fault_trait::VimFaultTrait {
}
impl<'s> serde::Serialize for dyn OvfFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn OvfFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(OvfFaultVisitor)
            }
        }

struct OvfFaultVisitor;

impl<'de> de::Visitor<'de> for OvfFaultVisitor {
    type Value = Box<dyn OvfFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid OvfFaultTrait JSON object with a _typeName field")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let deserializer = de::value::MapAccessDeserializer::new(&mut map);
        let any: VimAny = de::Deserialize::deserialize(deserializer)?;
        match any {
            VimAny::Object(obj) => Ok(CastFrom::from_box(obj)
                .map_err(|_| de::Error::custom("Internal error converting to trait type"))?),
            VimAny::Value(value) => Err(de::Error::custom(format!(
                "expected object not wrapped value: {:?}",
                value))),
        }
    }
}

impl OvfFaultTrait for OvfFault {
}
impl OvfFaultTrait for OvfConsumerCallbackFault {
}
impl OvfFaultTrait for OvfConsumerCommunicationError {
}
impl OvfFaultTrait for OvfConsumerFault {
}
impl OvfFaultTrait for OvfConsumerInvalidSection {
}
impl OvfFaultTrait for OvfConsumerUndeclaredSection {
}
impl OvfFaultTrait for OvfConsumerUndefinedPrefix {
}
impl OvfFaultTrait for OvfExport {
}
impl OvfFaultTrait for ConnectedIso {
}
impl OvfFaultTrait for OvfDuplicatedPropertyIdExport {
}
impl OvfFaultTrait for OvfDuplicatedPropertyIdImport {
}
impl OvfFaultTrait for OvfExportFailed {
}
impl OvfFaultTrait for OvfHardwareExport {
}
impl OvfFaultTrait for OvfConnectedDevice {
}
impl OvfFaultTrait for OvfConnectedDeviceFloppy {
}
impl OvfFaultTrait for OvfConnectedDeviceIso {
}
impl OvfFaultTrait for OvfUnableToExportDisk {
}
impl OvfFaultTrait for OvfUnknownDeviceBacking {
}
impl OvfFaultTrait for OvfUnsupportedDeviceExport {
}
impl OvfFaultTrait for OvfPropertyExport {
}
impl OvfFaultTrait for OvfPropertyNetworkExport {
}
impl OvfFaultTrait for OvfImport {
}
impl OvfFaultTrait for OvfCpuCompatibility {
}
impl OvfFaultTrait for OvfCpuCompatibilityCheckNotSupported {
}
impl OvfFaultTrait for OvfHardwareCheck {
}
impl OvfFaultTrait for OvfImportFailed {
}
impl OvfFaultTrait for OvfMappedOsId {
}
impl OvfFaultTrait for OvfMissingHardware {
}
impl OvfFaultTrait for OvfNetworkMappingNotSupported {
}
impl OvfFaultTrait for OvfUnsupportedDiskProvisioning {
}
impl OvfFaultTrait for OvfInvalidPackage {
}
impl OvfFaultTrait for OvfAttribute {
}
impl OvfFaultTrait for OvfInvalidValue {
}
impl OvfFaultTrait for OvfInvalidValueConfiguration {
}
impl OvfFaultTrait for OvfInvalidValueEmpty {
}
impl OvfFaultTrait for OvfInvalidValueFormatMalformed {
}
impl OvfFaultTrait for OvfInvalidValueReference {
}
impl OvfFaultTrait for OvfMissingAttribute {
}
impl OvfFaultTrait for OvfConstraint {
}
impl OvfFaultTrait for OvfDiskOrderConstraint {
}
impl OvfFaultTrait for OvfHostResourceConstraint {
}
impl OvfFaultTrait for OvfElement {
}
impl OvfFaultTrait for OvfDuplicateElement {
}
impl OvfFaultTrait for OvfDuplicatedElementBoundary {
}
impl OvfFaultTrait for OvfElementInvalidValue {
}
impl OvfFaultTrait for OvfMissingElement {
}
impl OvfFaultTrait for OvfMissingElementNormalBoundary {
}
impl OvfFaultTrait for OvfUnexpectedElement {
}
impl OvfFaultTrait for OvfWrongElement {
}
impl OvfFaultTrait for OvfProperty {
}
impl OvfFaultTrait for OvfPropertyNetwork {
}
impl OvfFaultTrait for OvfPropertyQualifier {
}
impl OvfFaultTrait for OvfPropertyQualifierDuplicate {
}
impl OvfFaultTrait for OvfPropertyQualifierIgnored {
}
impl OvfFaultTrait for OvfPropertyType {
}
impl OvfFaultTrait for OvfPropertyValue {
}
impl OvfFaultTrait for OvfWrongNamespace {
}
impl OvfFaultTrait for OvfXmlFormat {
}
impl OvfFaultTrait for OvfSystemFault {
}
impl OvfFaultTrait for OvfDiskMappingNotFound {
}
impl OvfFaultTrait for OvfHostValueNotParsed {
}
impl OvfFaultTrait for OvfInternalError {
}
impl OvfFaultTrait for OvfToXmlUnsupportedElement {
}
impl OvfFaultTrait for OvfUnknownDevice {
}
impl OvfFaultTrait for OvfUnknownEntity {
}
impl OvfFaultTrait for OvfUnsupportedDeviceBackingInfo {
}
impl OvfFaultTrait for OvfUnsupportedDeviceBackingOption {
}
impl OvfFaultTrait for OvfUnsupportedPackage {
}
impl OvfFaultTrait for OvfInvalidVmName {
}
impl OvfFaultTrait for OvfNoHostNic {
}
impl OvfFaultTrait for OvfNoSupportedHardwareFamily {
}
impl OvfFaultTrait for OvfUnsupportedAttribute {
}
impl OvfFaultTrait for OvfUnsupportedAttributeValue {
}
impl OvfFaultTrait for OvfUnsupportedElement {
}
impl OvfFaultTrait for OvfNoSpaceOnController {
}
impl OvfFaultTrait for OvfUnsupportedElementValue {
}
impl OvfFaultTrait for OvfUnsupportedSection {
}
impl OvfFaultTrait for OvfUnsupportedSubType {
}
impl OvfFaultTrait for OvfUnsupportedType {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn OvfFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfFault => Some(from.as_any_ref().downcast_ref::<OvfFault>()?),
            StructType::OvfConsumerCallbackFault => Some(from.as_any_ref().downcast_ref::<OvfConsumerCallbackFault>()?),
            StructType::OvfConsumerCommunicationError => Some(from.as_any_ref().downcast_ref::<OvfConsumerCommunicationError>()?),
            StructType::OvfConsumerFault => Some(from.as_any_ref().downcast_ref::<OvfConsumerFault>()?),
            StructType::OvfConsumerInvalidSection => Some(from.as_any_ref().downcast_ref::<OvfConsumerInvalidSection>()?),
            StructType::OvfConsumerUndeclaredSection => Some(from.as_any_ref().downcast_ref::<OvfConsumerUndeclaredSection>()?),
            StructType::OvfConsumerUndefinedPrefix => Some(from.as_any_ref().downcast_ref::<OvfConsumerUndefinedPrefix>()?),
            StructType::OvfExport => Some(from.as_any_ref().downcast_ref::<OvfExport>()?),
            StructType::ConnectedIso => Some(from.as_any_ref().downcast_ref::<ConnectedIso>()?),
            StructType::OvfDuplicatedPropertyIdExport => Some(from.as_any_ref().downcast_ref::<OvfDuplicatedPropertyIdExport>()?),
            StructType::OvfDuplicatedPropertyIdImport => Some(from.as_any_ref().downcast_ref::<OvfDuplicatedPropertyIdImport>()?),
            StructType::OvfExportFailed => Some(from.as_any_ref().downcast_ref::<OvfExportFailed>()?),
            StructType::OvfHardwareExport => Some(from.as_any_ref().downcast_ref::<OvfHardwareExport>()?),
            StructType::OvfConnectedDevice => Some(from.as_any_ref().downcast_ref::<OvfConnectedDevice>()?),
            StructType::OvfConnectedDeviceFloppy => Some(from.as_any_ref().downcast_ref::<OvfConnectedDeviceFloppy>()?),
            StructType::OvfConnectedDeviceIso => Some(from.as_any_ref().downcast_ref::<OvfConnectedDeviceIso>()?),
            StructType::OvfUnableToExportDisk => Some(from.as_any_ref().downcast_ref::<OvfUnableToExportDisk>()?),
            StructType::OvfUnknownDeviceBacking => Some(from.as_any_ref().downcast_ref::<OvfUnknownDeviceBacking>()?),
            StructType::OvfUnsupportedDeviceExport => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedDeviceExport>()?),
            StructType::OvfPropertyExport => Some(from.as_any_ref().downcast_ref::<OvfPropertyExport>()?),
            StructType::OvfPropertyNetworkExport => Some(from.as_any_ref().downcast_ref::<OvfPropertyNetworkExport>()?),
            StructType::OvfImport => Some(from.as_any_ref().downcast_ref::<OvfImport>()?),
            StructType::OvfCpuCompatibility => Some(from.as_any_ref().downcast_ref::<OvfCpuCompatibility>()?),
            StructType::OvfCpuCompatibilityCheckNotSupported => Some(from.as_any_ref().downcast_ref::<OvfCpuCompatibilityCheckNotSupported>()?),
            StructType::OvfHardwareCheck => Some(from.as_any_ref().downcast_ref::<OvfHardwareCheck>()?),
            StructType::OvfImportFailed => Some(from.as_any_ref().downcast_ref::<OvfImportFailed>()?),
            StructType::OvfMappedOsId => Some(from.as_any_ref().downcast_ref::<OvfMappedOsId>()?),
            StructType::OvfMissingHardware => Some(from.as_any_ref().downcast_ref::<OvfMissingHardware>()?),
            StructType::OvfNetworkMappingNotSupported => Some(from.as_any_ref().downcast_ref::<OvfNetworkMappingNotSupported>()?),
            StructType::OvfUnsupportedDiskProvisioning => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedDiskProvisioning>()?),
            StructType::OvfInvalidPackage => Some(from.as_any_ref().downcast_ref::<OvfInvalidPackage>()?),
            StructType::OvfAttribute => Some(from.as_any_ref().downcast_ref::<OvfAttribute>()?),
            StructType::OvfInvalidValue => Some(from.as_any_ref().downcast_ref::<OvfInvalidValue>()?),
            StructType::OvfInvalidValueConfiguration => Some(from.as_any_ref().downcast_ref::<OvfInvalidValueConfiguration>()?),
            StructType::OvfInvalidValueEmpty => Some(from.as_any_ref().downcast_ref::<OvfInvalidValueEmpty>()?),
            StructType::OvfInvalidValueFormatMalformed => Some(from.as_any_ref().downcast_ref::<OvfInvalidValueFormatMalformed>()?),
            StructType::OvfInvalidValueReference => Some(from.as_any_ref().downcast_ref::<OvfInvalidValueReference>()?),
            StructType::OvfMissingAttribute => Some(from.as_any_ref().downcast_ref::<OvfMissingAttribute>()?),
            StructType::OvfConstraint => Some(from.as_any_ref().downcast_ref::<OvfConstraint>()?),
            StructType::OvfDiskOrderConstraint => Some(from.as_any_ref().downcast_ref::<OvfDiskOrderConstraint>()?),
            StructType::OvfHostResourceConstraint => Some(from.as_any_ref().downcast_ref::<OvfHostResourceConstraint>()?),
            StructType::OvfElement => Some(from.as_any_ref().downcast_ref::<OvfElement>()?),
            StructType::OvfDuplicateElement => Some(from.as_any_ref().downcast_ref::<OvfDuplicateElement>()?),
            StructType::OvfDuplicatedElementBoundary => Some(from.as_any_ref().downcast_ref::<OvfDuplicatedElementBoundary>()?),
            StructType::OvfElementInvalidValue => Some(from.as_any_ref().downcast_ref::<OvfElementInvalidValue>()?),
            StructType::OvfMissingElement => Some(from.as_any_ref().downcast_ref::<OvfMissingElement>()?),
            StructType::OvfMissingElementNormalBoundary => Some(from.as_any_ref().downcast_ref::<OvfMissingElementNormalBoundary>()?),
            StructType::OvfUnexpectedElement => Some(from.as_any_ref().downcast_ref::<OvfUnexpectedElement>()?),
            StructType::OvfWrongElement => Some(from.as_any_ref().downcast_ref::<OvfWrongElement>()?),
            StructType::OvfProperty => Some(from.as_any_ref().downcast_ref::<OvfProperty>()?),
            StructType::OvfPropertyNetwork => Some(from.as_any_ref().downcast_ref::<OvfPropertyNetwork>()?),
            StructType::OvfPropertyQualifier => Some(from.as_any_ref().downcast_ref::<OvfPropertyQualifier>()?),
            StructType::OvfPropertyQualifierDuplicate => Some(from.as_any_ref().downcast_ref::<OvfPropertyQualifierDuplicate>()?),
            StructType::OvfPropertyQualifierIgnored => Some(from.as_any_ref().downcast_ref::<OvfPropertyQualifierIgnored>()?),
            StructType::OvfPropertyType => Some(from.as_any_ref().downcast_ref::<OvfPropertyType>()?),
            StructType::OvfPropertyValue => Some(from.as_any_ref().downcast_ref::<OvfPropertyValue>()?),
            StructType::OvfWrongNamespace => Some(from.as_any_ref().downcast_ref::<OvfWrongNamespace>()?),
            StructType::OvfXmlFormat => Some(from.as_any_ref().downcast_ref::<OvfXmlFormat>()?),
            StructType::OvfSystemFault => Some(from.as_any_ref().downcast_ref::<OvfSystemFault>()?),
            StructType::OvfDiskMappingNotFound => Some(from.as_any_ref().downcast_ref::<OvfDiskMappingNotFound>()?),
            StructType::OvfHostValueNotParsed => Some(from.as_any_ref().downcast_ref::<OvfHostValueNotParsed>()?),
            StructType::OvfInternalError => Some(from.as_any_ref().downcast_ref::<OvfInternalError>()?),
            StructType::OvfToXmlUnsupportedElement => Some(from.as_any_ref().downcast_ref::<OvfToXmlUnsupportedElement>()?),
            StructType::OvfUnknownDevice => Some(from.as_any_ref().downcast_ref::<OvfUnknownDevice>()?),
            StructType::OvfUnknownEntity => Some(from.as_any_ref().downcast_ref::<OvfUnknownEntity>()?),
            StructType::OvfUnsupportedDeviceBackingInfo => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedDeviceBackingInfo>()?),
            StructType::OvfUnsupportedDeviceBackingOption => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedDeviceBackingOption>()?),
            StructType::OvfUnsupportedPackage => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedPackage>()?),
            StructType::OvfInvalidVmName => Some(from.as_any_ref().downcast_ref::<OvfInvalidVmName>()?),
            StructType::OvfNoHostNic => Some(from.as_any_ref().downcast_ref::<OvfNoHostNic>()?),
            StructType::OvfNoSupportedHardwareFamily => Some(from.as_any_ref().downcast_ref::<OvfNoSupportedHardwareFamily>()?),
            StructType::OvfUnsupportedAttribute => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedAttribute>()?),
            StructType::OvfUnsupportedAttributeValue => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedAttributeValue>()?),
            StructType::OvfUnsupportedElement => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedElement>()?),
            StructType::OvfNoSpaceOnController => Some(from.as_any_ref().downcast_ref::<OvfNoSpaceOnController>()?),
            StructType::OvfUnsupportedElementValue => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedElementValue>()?),
            StructType::OvfUnsupportedSection => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedSection>()?),
            StructType::OvfUnsupportedSubType => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedSubType>()?),
            StructType::OvfUnsupportedType => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedType>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfFault => Ok(from.as_any_box().downcast::<OvfFault>()?),
            StructType::OvfConsumerCallbackFault => Ok(from.as_any_box().downcast::<OvfConsumerCallbackFault>()?),
            StructType::OvfConsumerCommunicationError => Ok(from.as_any_box().downcast::<OvfConsumerCommunicationError>()?),
            StructType::OvfConsumerFault => Ok(from.as_any_box().downcast::<OvfConsumerFault>()?),
            StructType::OvfConsumerInvalidSection => Ok(from.as_any_box().downcast::<OvfConsumerInvalidSection>()?),
            StructType::OvfConsumerUndeclaredSection => Ok(from.as_any_box().downcast::<OvfConsumerUndeclaredSection>()?),
            StructType::OvfConsumerUndefinedPrefix => Ok(from.as_any_box().downcast::<OvfConsumerUndefinedPrefix>()?),
            StructType::OvfExport => Ok(from.as_any_box().downcast::<OvfExport>()?),
            StructType::ConnectedIso => Ok(from.as_any_box().downcast::<ConnectedIso>()?),
            StructType::OvfDuplicatedPropertyIdExport => Ok(from.as_any_box().downcast::<OvfDuplicatedPropertyIdExport>()?),
            StructType::OvfDuplicatedPropertyIdImport => Ok(from.as_any_box().downcast::<OvfDuplicatedPropertyIdImport>()?),
            StructType::OvfExportFailed => Ok(from.as_any_box().downcast::<OvfExportFailed>()?),
            StructType::OvfHardwareExport => Ok(from.as_any_box().downcast::<OvfHardwareExport>()?),
            StructType::OvfConnectedDevice => Ok(from.as_any_box().downcast::<OvfConnectedDevice>()?),
            StructType::OvfConnectedDeviceFloppy => Ok(from.as_any_box().downcast::<OvfConnectedDeviceFloppy>()?),
            StructType::OvfConnectedDeviceIso => Ok(from.as_any_box().downcast::<OvfConnectedDeviceIso>()?),
            StructType::OvfUnableToExportDisk => Ok(from.as_any_box().downcast::<OvfUnableToExportDisk>()?),
            StructType::OvfUnknownDeviceBacking => Ok(from.as_any_box().downcast::<OvfUnknownDeviceBacking>()?),
            StructType::OvfUnsupportedDeviceExport => Ok(from.as_any_box().downcast::<OvfUnsupportedDeviceExport>()?),
            StructType::OvfPropertyExport => Ok(from.as_any_box().downcast::<OvfPropertyExport>()?),
            StructType::OvfPropertyNetworkExport => Ok(from.as_any_box().downcast::<OvfPropertyNetworkExport>()?),
            StructType::OvfImport => Ok(from.as_any_box().downcast::<OvfImport>()?),
            StructType::OvfCpuCompatibility => Ok(from.as_any_box().downcast::<OvfCpuCompatibility>()?),
            StructType::OvfCpuCompatibilityCheckNotSupported => Ok(from.as_any_box().downcast::<OvfCpuCompatibilityCheckNotSupported>()?),
            StructType::OvfHardwareCheck => Ok(from.as_any_box().downcast::<OvfHardwareCheck>()?),
            StructType::OvfImportFailed => Ok(from.as_any_box().downcast::<OvfImportFailed>()?),
            StructType::OvfMappedOsId => Ok(from.as_any_box().downcast::<OvfMappedOsId>()?),
            StructType::OvfMissingHardware => Ok(from.as_any_box().downcast::<OvfMissingHardware>()?),
            StructType::OvfNetworkMappingNotSupported => Ok(from.as_any_box().downcast::<OvfNetworkMappingNotSupported>()?),
            StructType::OvfUnsupportedDiskProvisioning => Ok(from.as_any_box().downcast::<OvfUnsupportedDiskProvisioning>()?),
            StructType::OvfInvalidPackage => Ok(from.as_any_box().downcast::<OvfInvalidPackage>()?),
            StructType::OvfAttribute => Ok(from.as_any_box().downcast::<OvfAttribute>()?),
            StructType::OvfInvalidValue => Ok(from.as_any_box().downcast::<OvfInvalidValue>()?),
            StructType::OvfInvalidValueConfiguration => Ok(from.as_any_box().downcast::<OvfInvalidValueConfiguration>()?),
            StructType::OvfInvalidValueEmpty => Ok(from.as_any_box().downcast::<OvfInvalidValueEmpty>()?),
            StructType::OvfInvalidValueFormatMalformed => Ok(from.as_any_box().downcast::<OvfInvalidValueFormatMalformed>()?),
            StructType::OvfInvalidValueReference => Ok(from.as_any_box().downcast::<OvfInvalidValueReference>()?),
            StructType::OvfMissingAttribute => Ok(from.as_any_box().downcast::<OvfMissingAttribute>()?),
            StructType::OvfConstraint => Ok(from.as_any_box().downcast::<OvfConstraint>()?),
            StructType::OvfDiskOrderConstraint => Ok(from.as_any_box().downcast::<OvfDiskOrderConstraint>()?),
            StructType::OvfHostResourceConstraint => Ok(from.as_any_box().downcast::<OvfHostResourceConstraint>()?),
            StructType::OvfElement => Ok(from.as_any_box().downcast::<OvfElement>()?),
            StructType::OvfDuplicateElement => Ok(from.as_any_box().downcast::<OvfDuplicateElement>()?),
            StructType::OvfDuplicatedElementBoundary => Ok(from.as_any_box().downcast::<OvfDuplicatedElementBoundary>()?),
            StructType::OvfElementInvalidValue => Ok(from.as_any_box().downcast::<OvfElementInvalidValue>()?),
            StructType::OvfMissingElement => Ok(from.as_any_box().downcast::<OvfMissingElement>()?),
            StructType::OvfMissingElementNormalBoundary => Ok(from.as_any_box().downcast::<OvfMissingElementNormalBoundary>()?),
            StructType::OvfUnexpectedElement => Ok(from.as_any_box().downcast::<OvfUnexpectedElement>()?),
            StructType::OvfWrongElement => Ok(from.as_any_box().downcast::<OvfWrongElement>()?),
            StructType::OvfProperty => Ok(from.as_any_box().downcast::<OvfProperty>()?),
            StructType::OvfPropertyNetwork => Ok(from.as_any_box().downcast::<OvfPropertyNetwork>()?),
            StructType::OvfPropertyQualifier => Ok(from.as_any_box().downcast::<OvfPropertyQualifier>()?),
            StructType::OvfPropertyQualifierDuplicate => Ok(from.as_any_box().downcast::<OvfPropertyQualifierDuplicate>()?),
            StructType::OvfPropertyQualifierIgnored => Ok(from.as_any_box().downcast::<OvfPropertyQualifierIgnored>()?),
            StructType::OvfPropertyType => Ok(from.as_any_box().downcast::<OvfPropertyType>()?),
            StructType::OvfPropertyValue => Ok(from.as_any_box().downcast::<OvfPropertyValue>()?),
            StructType::OvfWrongNamespace => Ok(from.as_any_box().downcast::<OvfWrongNamespace>()?),
            StructType::OvfXmlFormat => Ok(from.as_any_box().downcast::<OvfXmlFormat>()?),
            StructType::OvfSystemFault => Ok(from.as_any_box().downcast::<OvfSystemFault>()?),
            StructType::OvfDiskMappingNotFound => Ok(from.as_any_box().downcast::<OvfDiskMappingNotFound>()?),
            StructType::OvfHostValueNotParsed => Ok(from.as_any_box().downcast::<OvfHostValueNotParsed>()?),
            StructType::OvfInternalError => Ok(from.as_any_box().downcast::<OvfInternalError>()?),
            StructType::OvfToXmlUnsupportedElement => Ok(from.as_any_box().downcast::<OvfToXmlUnsupportedElement>()?),
            StructType::OvfUnknownDevice => Ok(from.as_any_box().downcast::<OvfUnknownDevice>()?),
            StructType::OvfUnknownEntity => Ok(from.as_any_box().downcast::<OvfUnknownEntity>()?),
            StructType::OvfUnsupportedDeviceBackingInfo => Ok(from.as_any_box().downcast::<OvfUnsupportedDeviceBackingInfo>()?),
            StructType::OvfUnsupportedDeviceBackingOption => Ok(from.as_any_box().downcast::<OvfUnsupportedDeviceBackingOption>()?),
            StructType::OvfUnsupportedPackage => Ok(from.as_any_box().downcast::<OvfUnsupportedPackage>()?),
            StructType::OvfInvalidVmName => Ok(from.as_any_box().downcast::<OvfInvalidVmName>()?),
            StructType::OvfNoHostNic => Ok(from.as_any_box().downcast::<OvfNoHostNic>()?),
            StructType::OvfNoSupportedHardwareFamily => Ok(from.as_any_box().downcast::<OvfNoSupportedHardwareFamily>()?),
            StructType::OvfUnsupportedAttribute => Ok(from.as_any_box().downcast::<OvfUnsupportedAttribute>()?),
            StructType::OvfUnsupportedAttributeValue => Ok(from.as_any_box().downcast::<OvfUnsupportedAttributeValue>()?),
            StructType::OvfUnsupportedElement => Ok(from.as_any_box().downcast::<OvfUnsupportedElement>()?),
            StructType::OvfNoSpaceOnController => Ok(from.as_any_box().downcast::<OvfNoSpaceOnController>()?),
            StructType::OvfUnsupportedElementValue => Ok(from.as_any_box().downcast::<OvfUnsupportedElementValue>()?),
            StructType::OvfUnsupportedSection => Ok(from.as_any_box().downcast::<OvfUnsupportedSection>()?),
            StructType::OvfUnsupportedSubType => Ok(from.as_any_box().downcast::<OvfUnsupportedSubType>()?),
            StructType::OvfUnsupportedType => Ok(from.as_any_box().downcast::<OvfUnsupportedType>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
