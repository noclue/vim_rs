use std::string::ToString;
use crate::resolver::{Result, FieldProcessingType, HierarchyError, NodeData};

static CLASS_FIELDS: phf::Map<&'static str, phf::Map<&'static str, NodeData>> = ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 2),
        (0, 3),
        (0, 9),
        (0, 8),
        (0, 9),
        (0, 165),
        (0, 67),
        (0, 78),
        (0, 0),
        (0, 41),
        (1, 42),
        (0, 1),
        (0, 10),
        (0, 34),
        (0, 14),
        (0, 34),
        (0, 78),
        (0, 50),
        (2, 195),
        (0, 175),
        (0, 48),
        (0, 0),
        (0, 6),
        (3, 16),
        (0, 2),
        (0, 95),
        (3, 24),
        (0, 100),
        (2, 40),
        (0, 0),
        (0, 167),
        (0, 15),
        (1, 45),
        (0, 4),
        (20, 230),
        (4, 164),
        (10, 47),
        (1, 80),
        (40, 206),
        (0, 66),
        (0, 0),
        (0, 0),
        (5, 96),
        (9, 173),
        (1, 166),
        (92, 131),
        (0, 102),
    ],
    entries: &[
        ("VirtualMachineBootOptions", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (4, 4),
        (0, 0),
    ],
    entries: &[
        ("network_boot_protocol", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "networkBootProtocol", doc: Some(r#####"Protocol to attempt during PXE network boot or NetBoot.

See also *VirtualMachineBootOptionsNetworkBootProtocolType_enum*.
"#####) },),
        ("efi_secure_boot_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "efiSecureBootEnabled", doc: Some(r#####"If set to <code>true</code>, the virtual machine's firmware will
perform signature checks of any EFI images loaded during startup, and
will refuse to start any images which do not pass those signature
checks.

When creating a new VM:
\- If vim.vm.FlagInfo.vbsEnabled is set to <code>true</code>,
and this flag is set to <code>false</code> error is returned.
\- If this flag is unset and vim.vm.FlagInfo.vbsEnabled is set to
<code>true</code>, the value of this flag is set to <code>true</code>.
"#####) },),
        ("boot_delay", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "bootDelay", doc: Some(r#####"Delay in milliseconds before starting the boot sequence.

The boot delay specifies a time interval between virtual machine
power on or restart and the beginning of the boot sequence.
"#####) },),
        ("enter_bios_setup", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "enterBIOSSetup", doc: Some(r#####"If set to <code>true</code>, the virtual machine
automatically enters BIOS setup the next time it boots.

The virtual machine resets this flag to <code>false</code>
so that subsequent boots proceed normally.
"#####) },),
        ("boot_retry_delay", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "bootRetryDelay", doc: Some(r#####"Delay in milliseconds before a boot retry.

The boot retry delay
specifies a time interval between virtual machine boot failure
and the subsequent attempt to boot again. The virtual machine
uses this value only if *VirtualMachineBootOptions.bootRetryEnabled* is true.
"#####) },),
        ("boot_order", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::VirtualMachineBootOptionsBootableDeviceTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVirtualMachineBootOptionsBootableDevice"), path_segment: "bootOrder", doc: Some(r#####"Boot order.

Listed devices are used for booting. After list
is exhausted, default BIOS boot device algorithm is used for
booting.
Note that order of the entries in the list is important:
device listed first is used for boot first, if that one
fails second entry is used, and so on.
Platform may have some internal limit on the number of devices
it supports. If bootable device is not reached before platform's
limit is hit, boot will fail. At least single entry is supported
by all products supporting boot order settings.
"#####) },),
        ("boot_retry_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "bootRetryEnabled", doc: Some(r#####"If set to <code>true</code>, a virtual machine that fails
to boot will try again after the *VirtualMachineBootOptions.bootRetryDelay*
time period has expired.

When <code>false</code>,
the virtual machine waits indefinitely for you to initiate
boot retry.
"#####) },),
    ],
}),
        ("HostSystemInfo", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (8, 2),
        (0, 0),
    ],
    entries: &[
        ("family", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "family", doc: Some(r#####"System family identification.

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("uuid", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "uuid", doc: Some(r#####"Hardware BIOS identification.
"#####) },),
        ("serial_number", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "serialNumber", doc: None },),
        ("boot_command_line", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "bootCommandLine", doc: Some(r#####"Command line string to identify different boot options used for host.

Example of different boot options are:
- "runweasel": "System is booted for weasel installation"
- "ks": "System is booted for kickstart installation"
  
***Since:*** vSphere API Release 8.0.3.0
"#####) },),
        ("vendor", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "vendor", doc: Some(r#####"Hardware vendor identification.
"#####) },),
        ("vvol_host_nqn", NodeData { type_decl: "vim_rs::types::structs::HostQualifiedName", type_name: "HostQualifiedName", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "vvolHostNQN", doc: Some(r#####"NVMe qualified name used by Vvol.

A unique name, assigned to each host used by Vvol.
Obtained through vmkctl storage control path while fetching the NVMe info.

***Since:*** vSphere API Release 8.0.0.0
"#####) },),
        ("qualified_name", NodeData { type_decl: "Vec<vim_rs::types::structs::HostQualifiedName>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostQualifiedName"), path_segment: "qualifiedName", doc: Some(r#####"List of qualified names used to identify the host in a specific context.

Unlike the other types of system identification information, these can
potentially change as a result of configuration.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("model", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "model", doc: Some(r#####"System model identification.
"#####) },),
        ("other_identifying_info", NodeData { type_decl: "Vec<vim_rs::types::structs::HostSystemIdentificationInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostSystemIdentificationInfo"), path_segment: "otherIdentifyingInfo", doc: Some(r#####"Other System identification information.

This information may be vendor
specific
"#####) },),
        ("vvol_host_id", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "vvolHostId", doc: Some(r#####"Host id used by Vvol.

The hostd id, obtained through vmkctl storage control path while
fetching the NVMe info.

***Since:*** vSphere API Release 8.0.0.0
"#####) },),
    ],
}),
        ("KeyProviderId", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("id", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "id", doc: Some(r#####"Globally unique ID for the crypto key provider.

Servers with the same ID must provide the same keys.
Cannot be empty.
"#####) },),
    ],
}),
        ("HostConfigSpec", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (1, 9),
        (1, 1),
        (6, 1),
    ],
    entries: &[
        ("datastore_principal", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "datastorePrincipal", doc: Some(r#####"Datastore principal user.
"#####) },),
        ("nas_datastore", NodeData { type_decl: "Vec<vim_rs::types::structs::HostNasVolumeConfig>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostNasVolumeConfig"), path_segment: "nasDatastore", doc: Some(r#####"Configurations to create NAS datastores.
"#####) },),
        ("service", NodeData { type_decl: "Vec<vim_rs::types::structs::HostServiceConfig>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostServiceConfig"), path_segment: "service", doc: Some(r#####"Host service configuration.
"#####) },),
        ("memory", NodeData { type_decl: "vim_rs::types::structs::HostMemorySpec", type_name: "HostMemorySpec", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "memory", doc: Some(r#####"Memory configuration for the host.
"#####) },),
        ("generic_config", NodeData { type_decl: "Vec<vim_rs::types::structs::KeyAnyValue>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfKeyAnyValue"), path_segment: "genericConfig", doc: Some(r#####"Advanced configuration.
"#####) },),
        ("assignable_hardware_config", NodeData { type_decl: "vim_rs::types::structs::HostAssignableHardwareConfig", type_name: "HostAssignableHardwareConfig", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "assignableHardwareConfig", doc: Some(r#####"Assignable Hardware configuration for the host
"#####) },),
        ("network", NodeData { type_decl: "vim_rs::types::structs::HostNetworkConfig", type_name: "HostNetworkConfig", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "network", doc: Some(r#####"Network system information.
"#####) },),
        ("active_directory", NodeData { type_decl: "Vec<vim_rs::types::structs::HostActiveDirectory>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostActiveDirectory"), path_segment: "activeDirectory", doc: Some(r#####"Active Directory configuration change.
"#####) },),
        ("security", NodeData { type_decl: "vim_rs::types::structs::HostSecuritySpec", type_name: "HostSecuritySpec", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "security", doc: Some(r#####"Security specification.
"#####) },),
        ("nic_type_selection", NodeData { type_decl: "Vec<vim_rs::types::structs::HostVirtualNicManagerNicTypeSelection>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostVirtualNicManagerNicTypeSelection"), path_segment: "nicTypeSelection", doc: Some(r#####"Type selection for different VirtualNics.
"#####) },),
        ("user_account", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::HostAccountSpecTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostAccountSpec"), path_segment: "userAccount", doc: Some(r#####"List of users to create/update with new password.
"#####) },),
        ("option", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::OptionValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfOptionValue"), path_segment: "option", doc: Some(r#####"Host configuration options as defined by the
*OptionValue* data object type.
"#####) },),
        ("datetime", NodeData { type_decl: "vim_rs::types::structs::HostDateTimeConfig", type_name: "HostDateTimeConfig", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "datetime", doc: Some(r#####"DateTime Configuration.
"#####) },),
        ("storage_device", NodeData { type_decl: "vim_rs::types::structs::HostStorageDeviceInfo", type_name: "HostStorageDeviceInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "storageDevice", doc: Some(r#####"Storage system information.
"#####) },),
        ("firewall", NodeData { type_decl: "vim_rs::types::structs::HostFirewallConfig", type_name: "HostFirewallConfig", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "firewall", doc: Some(r#####"Firewall configuration.
"#####) },),
        ("datastore_principal_passwd", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "datastorePrincipalPasswd", doc: Some(r#####"Password for the datastore principal.
"#####) },),
        ("usergroup_account", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::HostAccountSpecTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostAccountSpec"), path_segment: "usergroupAccount", doc: Some(r#####"List of users to create/update with new password.
"#####) },),
        ("license", NodeData { type_decl: "vim_rs::types::structs::HostLicenseSpec", type_name: "HostLicenseSpec", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "license", doc: Some(r#####"License configuration for the host.
"#####) },),
        ("graphics_config", NodeData { type_decl: "vim_rs::types::structs::HostGraphicsConfig", type_name: "HostGraphicsConfig", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "graphicsConfig", doc: Some(r#####"Graphics configuration for a host.
"#####) },),
    ],
}),
        ("HostNetCapabilities", ::phf::Map {
    key: 351906021642186605,
    disps: &[
        (4, 3),
        (8, 0),
        (0, 0),
    ],
    entries: &[
        ("dns_config_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "dnsConfigSupported", doc: Some(r#####"The flag to indicate whether DNS configuration for the host is
supported.
"#####) },),
        ("supports_nic_teaming", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "supportsNicTeaming", doc: Some(r#####"The flag to indicate whether or not network adapter teaming is
available.

Multiple network adapters can be bridged to a
virtual switch through a BondBridge. Also, network adapter teaming
policies such as failover order and detection are enabled.
"#####) },),
        ("ip_route_config_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "ipRouteConfigSupported", doc: Some(r#####"The flag to indicate whether ip route configuration for the host
is supported.
"#####) },),
        ("supports_vlan", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "supportsVlan", doc: Some(r#####"The flag to indicate whether or not VLANs can be configured on
PortGroups attached to VirtualSwitch objects.

This allows VLANs for virtual machines without requiring special VLAN
capable hardware switches.
"#####) },),
        ("ip_v_6_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "ipV6Supported", doc: Some(r#####"The flag to indicate whether the host is capable of communicating
using ipv6 protocol
"#####) },),
        ("nic_teaming_policy", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "nicTeamingPolicy", doc: Some(r#####"The available teaming policies if the platform supports network
adapter teaming.
"#####) },),
        ("vswitch_config_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vswitchConfigSupported", doc: Some(r#####"The flag to indicate whether virtual switch configuration is
supported.

This means that operations to add, remove, update virtual
switches are supported.
"#####) },),
        ("dhcp_on_vnic_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "dhcpOnVnicSupported", doc: Some(r#####"This flag indicates whether or not the host is able to support
dhcp configuration for vnics.
"#####) },),
        ("max_port_groups_per_vswitch", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "maxPortGroupsPerVswitch", doc: Some(r#####"The maximum number of port groups supported per virtual switch.

This property will not be set if this value is unlimited.
"#####) },),
        ("can_set_physical_nic_link_speed", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "canSetPhysicalNicLinkSpeed", doc: Some(r#####"The flag to indicate whether or not a physical network
adapter's link speed and duplex settings can be changed through
this API.

For a hosted product, the host uses its physical network
adapters for network connectivity. Configuration of link speed is
done through regular host operations. In ESX Server, the configuration
can be changed through this API.
"#####) },),
        ("backup_nfc_nioc_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "backupNfcNiocSupported", doc: Some(r#####"The flag to indicate whether the host supports Backup NFC NIOC system
traffic, Unset means Backup NFC NIOC system traffic is not supported.

***Since:*** vSphere API Release 7.0.1.0
"#####) },),
        ("supports_network_hints", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "supportsNetworkHints", doc: Some(r#####"The flag to indicate whether or not the host is able
to support
the querying of network hints.
"#####) },),
        ("uses_service_console_nic", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "usesServiceConsoleNic", doc: Some(r#####"The flag to indicate whether or not a service
console network adapter
is used or required.

This means that the system
software has two TCP/IP stacks. As a result, at least two types of
VirtualNics may be created -- the normal VirtualNic and the service
console VirtualNic. If this is not set, then only the VirtualNic type is
supported.
"#####) },),
        ("vnic_config_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vnicConfigSupported", doc: Some(r#####"The flag to indicate whether Virtual NIC configuration is supported.

This means that operations to add, remove, update virtualNic are
supported.
"#####) },),
    ],
}),
        ("VirtualMachineQuestionInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("choice", NodeData { type_decl: "vim_rs::types::structs::ChoiceOption", type_name: "ChoiceOption", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "choice", doc: Some(r#####"List of key-value pairs that specify possible answers.
"#####) },),
        ("text", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "text", doc: Some(r#####"Text that describes the pending question.
"#####) },),
        ("id", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "id", doc: Some(r#####"Identifier with an opaque value that specifies the pending question.
"#####) },),
        ("message", NodeData { type_decl: "Vec<vim_rs::types::structs::VirtualMachineMessage>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVirtualMachineMessage"), path_segment: "message", doc: Some(r#####"The message data for the individual messages that comprise the question.

Only available on servers that support localization.
"#####) },),
    ],
}),
        ("AboutInfo", ::phf::Map {
    key: 351906021642186605,
    disps: &[
        (8, 4),
        (0, 0),
        (3, 2),
    ],
    entries: &[
        ("locale_build", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "localeBuild", doc: Some(r#####"Build number for the current session's locale.

Typically, this is a small number reflecting a
localization change from the normal product build.
"#####) },),
        ("instance_uuid", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "instanceUuid", doc: Some(r#####"A globally unique identifier associated with this service instance.
"#####) },),
        ("build", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "build", doc: Some(r#####"Build string for the server on which this call is made.

For example, x.y.z-num.
This string does not apply to the API.
"#####) },),
        ("license_product_version", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "licenseProductVersion", doc: Some(r#####"The license product version
"#####) },),
        ("vendor", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "vendor", doc: Some(r#####"Name of the vendor of this product.
"#####) },),
        ("patch_level", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "patchLevel", doc: Some(r#####"Patch level for the server.

***Since:*** vSphere API Release 7.0.2.0
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"Short form of the product name.
"#####) },),
        ("license_product_name", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "licenseProductName", doc: Some(r#####"The license product name
"#####) },),
        ("version", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "version", doc: Some(r#####"Dot-separated product version string.

For example, "10.0.2.0".
"#####) },),
        ("locale_version", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "localeVersion", doc: Some(r#####"Version of the message catalog for the current session's locale.
"#####) },),
        ("full_name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "fullName", doc: Some(r#####"The complete product name, including the version information.
"#####) },),
        ("api_type", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "apiType", doc: Some(r#####"Indicates whether or not the service instance represents a
standalone host.

If the service instance represents a standalone host, then the physical
inventory for that service instance is fixed to that single host.
VirtualCenter server provides additional features over single hosts.
For example, VirtualCenter offers multi-host management.

Examples of values are:
- "VirtualCenter" - For a VirtualCenter instance.
- "HostAgent" - For host agent on an ESX Server or VMware Server host.
"#####) },),
        ("product_line_id", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "productLineId", doc: Some(r#####"The product ID is a unique identifier for a product line.

Examples of values are:
- "gsx" - For the VMware Server product.
- "esx" - For the ESX product.
- "embeddedEsx" - For the ESXi product.
- "esxio" - For the ESXio product.
- "vpx" - For the VirtualCenter product.
"#####) },),
        ("api_version", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "apiVersion", doc: Some(r#####"The newest long-term supported API version provided by the server.

The version format is "x.y.z.a", where "x", "y", and "z" are numbers
that do not exceed 99, and "a" does not exceed 9999.
"#####) },),
        ("os_type", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "osType", doc: Some(r#####"Operating system type and architecture.

Examples of values are:
- "win32-x86" - For x86-based Windows systems.
- "linux-x86" - For x86-based Linux systems.
- "vmnix-x86" - For the x86 ESX Server microkernel.
- "vmnix-arm64" - For the arm64 ESX Server microkernel.
"#####) },),
    ],
}),
        ("ClusterComputeResourceHostConfigurationProfile", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("lockdown_mode", NodeData { type_decl: "vim_rs::types::enums::HostLockdownModeEnum", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("HostLockdownMode"), path_segment: "lockdownMode", doc: Some(r#####"Desired lockdown mode
"#####) },),
        ("date_time_config", NodeData { type_decl: "vim_rs::types::structs::HostDateTimeConfig", type_name: "HostDateTimeConfig", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "dateTimeConfig", doc: Some(r#####"Date and time settings
"#####) },),
    ],
}),
        ("HostSystemComplianceCheckState", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("check_time", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveDateTime"), path_segment: "checkTime", doc: Some(r#####"The compliance check starting time for running state; compliance
check finish time for others.
"#####) },),
        ("state", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "state", doc: Some(r#####"The compliance check operation state.

See
*ComplianceResultStatus_enum* for the valid values.
"#####) },),
    ],
}),
        ("HostConfigInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 32),
        (0, 4),
        (14, 39),
        (0, 6),
        (2, 43),
        (2, 0),
        (1, 52),
        (0, 51),
        (2, 20),
        (3, 52),
        (3, 32),
        (8, 55),
    ],
    entries: &[
        ("product", NodeData { type_decl: "vim_rs::types::structs::AboutInfo", type_name: "AboutInfo", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "product", doc: Some(r#####"Information about a product.
"#####) },),
        ("wake_on_lan_capable", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "wakeOnLanCapable", doc: Some(r#####"Indicates if a host is wake on lan capable.

A host is deemed wake on lan capable if there exists at least one
physical network card that is both backing the vmotion interface and
is itself wake on lan capable.
"#####) },),
        ("hyper_thread", NodeData { type_decl: "vim_rs::types::structs::HostHyperThreadScheduleInfo", type_name: "HostHyperThreadScheduleInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "hyperThread", doc: Some(r#####"If hyperthreading is supported, this is the CPU configuration for
optimizing hyperthreading.
"#####) },),
        ("masked_feature_capability", NodeData { type_decl: "Vec<vim_rs::types::structs::HostFeatureCapability>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostFeatureCapability"), path_segment: "maskedFeatureCapability", doc: Some(r#####"Array of the feature capabilities that the host has after the
mask has been applied.
"#####) },),
        ("multipath_state", NodeData { type_decl: "vim_rs::types::structs::HostMultipathStateInfo", type_name: "HostMultipathStateInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "multipathState", doc: Some(r#####"Storage multipath state information.
"#####) },),
        ("host", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "host", doc: Some(r#####"A reference to a managed object on a host.

Refers instance of *HostSystem*.
"#####) },),
        ("v_flash_config_info", NodeData { type_decl: "vim_rs::types::structs::HostVFlashManagerVFlashConfigInfo", type_name: "HostVFlashManagerVFlashConfigInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "vFlashConfigInfo", doc: Some(r#####"Host vFlash configuration information
"#####) },),
        ("feature_capability", NodeData { type_decl: "Vec<vim_rs::types::structs::HostFeatureCapability>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostFeatureCapability"), path_segment: "featureCapability", doc: Some(r#####"Array of host feature capabilities.

This is expected to change
infrequently. It may change while host is in maintenance mode
and between reboots if hardware, firmware, or a device driver
is changed or upgraded.
"#####) },),
        ("auto_start", NodeData { type_decl: "vim_rs::types::structs::HostAutoStartManagerConfig", type_name: "HostAutoStartManagerConfig", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "autoStart", doc: Some(r#####"AutoStart configuration.
"#####) },),
        ("authentication_manager_info", NodeData { type_decl: "vim_rs::types::structs::HostAuthenticationManagerInfo", type_name: "HostAuthenticationManagerInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "authenticationManagerInfo", doc: Some(r#####"Current authentication configuration.
"#####) },),
        ("authentication_data", NodeData { type_decl: "Vec<vim_rs::types::structs::HostAuthenticationInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostAuthenticationInfo"), path_segment: "authenticationData", doc: Some(r#####"Authentication info registered on this host.

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("shared_gpu_capabilities", NodeData { type_decl: "Vec<vim_rs::types::structs::HostSharedGpuCapabilities>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostSharedGpuCapabilities"), path_segment: "sharedGpuCapabilities", doc: Some(r#####"Array of shared passthru GPU capablities.

See also *HostSharedGpuCapabilities*.
"#####) },),
        ("shared_passthru_gpu_types", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "sharedPassthruGpuTypes", doc: Some(r#####"Array of shared passthru GPU types.

These GPU types may be enabled
when specific host hardware is present.
"#####) },),
        ("system_file", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "systemFile", doc: Some(r#####"Datastore paths of files used by the host system on
mounted volumes, for instance, the COS vmdk file of the
host.

For information on datastore paths, see *Datastore*.
"#####) },),
        ("flags", NodeData { type_decl: "vim_rs::types::structs::HostFlagInfo", type_name: "HostFlagInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "flags", doc: Some(r#####"Additional flags for a host.
"#####) },),
        ("script_check_sum", NodeData { type_decl: "Vec<u8>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBinary"), path_segment: "scriptCheckSum", doc: Some(r#####"A checksum of overhead computation script.

(For VMware internal usage only)
"#####) },),
        ("capabilities", NodeData { type_decl: "vim_rs::types::structs::HostNetCapabilities", type_name: "HostNetCapabilities", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "capabilities", doc: Some(r#####"Capability vector indicating the available network features.
"#####) },),
        ("ipmi", NodeData { type_decl: "vim_rs::types::structs::HostIpmiInfo", type_name: "HostIpmiInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "ipmi", doc: Some(r#####"IPMI (Intelligent Platform Management Interface) info for the host.
"#####) },),
        ("console_reservation", NodeData { type_decl: "vim_rs::types::structs::ServiceConsoleReservationInfo", type_name: "ServiceConsoleReservationInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "consoleReservation", doc: Some(r#####"Memory configuration.
"#####) },),
        ("active_diagnostic_partition", NodeData { type_decl: "vim_rs::types::structs::HostDiagnosticPartition", type_name: "HostDiagnosticPartition", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "activeDiagnosticPartition", doc: Some(r#####"The diagnostic partition that will be set as the current
diagnostic partition on the host.
"#####) },),
        ("option", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::OptionValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfOptionValue"), path_segment: "option", doc: Some(r#####"Host configuration options as defined by the
*OptionValue* data object type.
"#####) },),
        ("option_def", NodeData { type_decl: "Vec<vim_rs::types::structs::OptionDef>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfOptionDef"), path_segment: "optionDef", doc: Some(r#####"A list of supported options.
"#####) },),
        ("deployment_info", NodeData { type_decl: "vim_rs::types::structs::HostDeploymentInfo", type_name: "HostDeploymentInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "deploymentInfo", doc: Some(r#####"Deployment information about the host.
"#####) },),
        ("date_time_info", NodeData { type_decl: "vim_rs::types::structs::HostDateTimeInfo", type_name: "HostDateTimeInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "dateTimeInfo", doc: Some(r#####"Date/Time related configuration
"#####) },),
        ("firewall", NodeData { type_decl: "vim_rs::types::structs::HostFirewallInfo", type_name: "HostFirewallInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "firewall", doc: Some(r#####"Firewall configuration.
"#####) },),
        ("assignable_hardware_config", NodeData { type_decl: "vim_rs::types::structs::HostAssignableHardwareConfig", type_name: "HostAssignableHardwareConfig", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "assignableHardwareConfig", doc: Some(r#####"Configured assignable hardware device attributes.
"#####) },),
        ("vsan_host_config", NodeData { type_decl: "Box<dyn vim_rs::types::traits::VsanHostConfigInfoTrait>", type_name: "VsanHostConfigInfo", is_optional: true, processing_type: FieldProcessingType::Trait, path_segment: "vsanHostConfig", doc: Some(r#####"VSAN configuration for a host.
"#####) },),
        ("datastore_principal", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "datastorePrincipal", doc: Some(r#####"Datastore principal user
"#####) },),
        ("offload_capabilities", NodeData { type_decl: "vim_rs::types::structs::HostNetOffloadCapabilities", type_name: "HostNetOffloadCapabilities", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "offloadCapabilities", doc: Some(r#####"Deprecated as of VI API 4.0, the system defaults will be used.

capabilities to offload operations either to the host or to physical
hardware when a virtual machine is transmitting on a network
"#####) },),
        ("virtual_nic_manager_info", NodeData { type_decl: "vim_rs::types::structs::HostVirtualNicManagerInfo", type_name: "HostVirtualNicManagerInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "virtualNicManagerInfo", doc: Some(r#####"VirtualNic manager information.
"#####) },),
        ("graphics_config", NodeData { type_decl: "vim_rs::types::structs::HostGraphicsConfig", type_name: "HostGraphicsConfig", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "graphicsConfig", doc: Some(r#####"Graphics configuration for a host.
"#####) },),
        ("assignable_hardware_binding", NodeData { type_decl: "Vec<vim_rs::types::structs::HostAssignableHardwareBinding>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostAssignableHardwareBinding"), path_segment: "assignableHardwareBinding", doc: Some(r#####"Information describing Assignable Hardware device bindings on host.

See *HostAssignableHardwareBinding*.
"#####) },),
        ("datastore_capabilities", NodeData { type_decl: "vim_rs::types::structs::HostDatastoreSystemCapabilities", type_name: "HostDatastoreSystemCapabilities", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "datastoreCapabilities", doc: Some(r#####"Capability vector indicating available datastore features.
"#####) },),
        ("vmotion", NodeData { type_decl: "vim_rs::types::structs::HostVMotionInfo", type_name: "HostVMotionInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "vmotion", doc: Some(r#####"Deprecated as of VI API 4.0, use *HostConfigInfo.virtualNicManagerInfo*.

VMotion system information.
"#####) },),
        ("admin_disabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "adminDisabled", doc: Some(r#####"Deprecated as of vSphere API 6.0, use *HostConfigInfo.lockdownMode*.

If the flag is true, the permissions on the host have been modified such
that it is only accessible through local console or an authorized
centralized management application.

This flag is affected by the
*HostSystem.EnterLockdownMode* and
*HostSystem.ExitLockdownMode* operations.

This flag is supported in VirtualCenter only. The value returned from host
should be ignored.

See also *HostSystem.EnterLockdownMode*, *HostSystem.ExitLockdownMode*.
"#####) },),
        ("sriov_device_pool", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::HostSriovDevicePoolInfoTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostSriovDevicePoolInfo"), path_segment: "sriovDevicePool", doc: Some(r#####"Information on SRIOV device pools present on host.
"#####) },),
        ("lockdown_mode", NodeData { type_decl: "vim_rs::types::enums::HostLockdownModeEnum", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("HostLockdownMode"), path_segment: "lockdownMode", doc: Some(r#####"Indicates the current lockdown mode of the host as reported by
*HostAccessManager.lockdownMode*.

See also *HostAccessManager.ChangeLockdownMode*.
"#####) },),
        ("cpu_scheduler", NodeData { type_decl: "vim_rs::types::structs::HostCpuSchedulerInfo", type_name: "HostCpuSchedulerInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "cpuScheduler", doc: Some(r#####"Information about the CPU scheduler on the host.

***Since:*** vSphere API Release 8.0.3.0
"#####) },),
        ("graphics_info", NodeData { type_decl: "Vec<vim_rs::types::structs::HostGraphicsInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostGraphicsInfo"), path_segment: "graphicsInfo", doc: Some(r#####"The list of graphics devices available on this host.
"#####) },),
        ("system_swap_configuration", NodeData { type_decl: "vim_rs::types::structs::HostSystemSwapConfiguration", type_name: "HostSystemSwapConfiguration", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "systemSwapConfiguration", doc: Some(r#####"The system swap configuration specifies which options are currently
enabled.  

See also *HostSystemSwapConfiguration*.
"#####) },),
        ("cache_configuration_info", NodeData { type_decl: "Vec<vim_rs::types::structs::HostCacheConfigurationInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostCacheConfigurationInfo"), path_segment: "cacheConfigurationInfo", doc: Some(r#####"Host solid stats drive cache configuration information.
"#####) },),
        ("ssl_thumbprint_info", NodeData { type_decl: "vim_rs::types::structs::HostSslThumbprintInfo", type_name: "HostSslThumbprintInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "sslThumbprintInfo", doc: Some(r#####"Deprecated as of vSphere API 5.0, use *HostConfigInfo.sslThumbprintData* instead.

SSL Thumbprint info for hosts registered on this host.
"#####) },),
        ("io_filter_info", NodeData { type_decl: "Vec<vim_rs::types::structs::HostIoFilterInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostIoFilterInfo"), path_segment: "ioFilterInfo", doc: Some(r#####"Information of the IO Filters installed on the host.

See *HostIoFilterInfo*.
"#####) },),
        ("host_config_check_sum", NodeData { type_decl: "Vec<u8>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBinary"), path_segment: "hostConfigCheckSum", doc: Some(r#####"A checksum of host configuration option set.

(For VMware internal usage only)
"#####) },),
        ("network", NodeData { type_decl: "vim_rs::types::structs::HostNetworkInfo", type_name: "HostNetworkInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "network", doc: Some(r#####"Network system information.
"#####) },),
        ("feature_version", NodeData { type_decl: "Vec<vim_rs::types::structs::HostFeatureVersionInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostFeatureVersionInfo"), path_segment: "featureVersion", doc: Some(r#####"List of feature-specific version information.

Each element refers
to the version information for a specific feature.
"#####) },),
        ("service", NodeData { type_decl: "vim_rs::types::structs::HostServiceInfo", type_name: "HostServiceInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "service", doc: Some(r#####"Host service configuration.
"#####) },),
        ("virtual_machine_reservation", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineMemoryReservationInfo", type_name: "VirtualMachineMemoryReservationInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "virtualMachineReservation", doc: Some(r#####"Virtual machine memory configuration.
"#####) },),
        ("ssl_thumbprint_data", NodeData { type_decl: "Vec<vim_rs::types::structs::HostSslThumbprintInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostSslThumbprintInfo"), path_segment: "sslThumbprintData", doc: Some(r#####"SSL Thumbprints registered on this host.
"#####) },),
        ("local_swap_datastore", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "localSwapDatastore", doc: Some(r#####"Datastore visible to this host that may be used to store virtual
machine swapfiles, for virtual machines executing on this host.

The
value of this property is set or unset by invoking
*HostDatastoreSystem.UpdateLocalSwapDatastore*.
The policy for using this datastore is determined by the compute
resource configuration's
*vmSwapPlacement*
property in concert with each individual virtual machine configuration's
*swapPlacement* property.

Note: Using a host-specific swap location may degrade VMotion performance.

Refers instance of *Datastore*.
"#####) },),
        ("description_tree_check_sum", NodeData { type_decl: "Vec<u8>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBinary"), path_segment: "descriptionTreeCheckSum", doc: Some(r#####"A checksum of the Assignable Hardware Description Tree.

(For VMware internal usage only)
"#####) },),
        ("power_system_info", NodeData { type_decl: "vim_rs::types::structs::PowerSystemInfo", type_name: "PowerSystemInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "powerSystemInfo", doc: Some(r#####"Host power management information.
"#####) },),
        ("certificate", NodeData { type_decl: "Vec<i8>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfByte"), path_segment: "certificate", doc: Some(r#####"Full Host Certificate in PEM format, if known
"#####) },),
        ("system_resources", NodeData { type_decl: "vim_rs::types::structs::HostSystemResourceInfo", type_name: "HostSystemResourceInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "systemResources", doc: Some(r#####"Reference for the system resource hierarchy, used for configuring
the set of resources reserved to the system and unavailable to
virtual machines.
"#####) },),
        ("domain_list", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "domainList", doc: Some(r#####"List of Windows domains available for user searches, if the underlying
system supports windows domain membership.

See *UserDirectory.domainList*.
"#####) },),
        ("pci_passthru_info", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::HostPciPassthruInfoTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostPciPassthruInfo"), path_segment: "pciPassthruInfo", doc: Some(r#####"PCI passthrough information.
"#####) },),
        ("file_system_volume", NodeData { type_decl: "vim_rs::types::structs::HostFileSystemVolumeInfo", type_name: "HostFileSystemVolumeInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "fileSystemVolume", doc: Some(r#####"Storage system file system volume information.
"#####) },),
        ("storage_device", NodeData { type_decl: "vim_rs::types::structs::HostStorageDeviceInfo", type_name: "HostStorageDeviceInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "storageDevice", doc: Some(r#####"Storage system information.
"#####) },),
        ("power_system_capability", NodeData { type_decl: "vim_rs::types::structs::PowerSystemCapability", type_name: "PowerSystemCapability", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "powerSystemCapability", doc: Some(r#####"Host power management capability.
"#####) },),
    ],
}),
        ("VAppProductInfo", ::phf::Map {
    key: 12213676231523076107,
    disps: &[
        (7, 0),
        (3, 4),
    ],
    entries: &[
        ("instance_id", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "instanceId", doc: Some(r#####"Class name for this attribute.

Valid values for instanceId:
Any string except any white-space characters.
"#####) },),
        ("version", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "version", doc: Some(r#####"Short version of the product, for example, 1.0.
"#####) },),
        ("product_url", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "productUrl", doc: Some(r#####"URL to product homepage.
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"Name of the product.
"#####) },),
        ("vendor_url", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "vendorUrl", doc: Some(r#####"URL to vendor homepage.
"#####) },),
        ("app_url", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "appUrl", doc: Some(r#####"URL to entry-point for application.

This is often specified using
a macro, for example, http://${app.ip}/, where app.ip is a defined property
on the virtual machine or vApp container.
"#####) },),
        ("key", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "key", doc: Some(r#####"A unique key for the product section
"#####) },),
        ("vendor", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "vendor", doc: Some(r#####"Vendor of the product.
"#####) },),
        ("class_id", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "classId", doc: Some(r#####"Class name for this attribute.

Valid values for classId:
Any string except any white-space characters.
"#####) },),
        ("full_version", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "fullVersion", doc: Some(r#####"Full-version of the product, for example, 1.0-build 12323.
"#####) },),
    ],
}),
        ("VirtualMachineDefaultPowerOpInfo", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (1, 3),
        (0, 0),
    ],
    entries: &[
        ("standby_action", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "standbyAction", doc: Some(r#####"Behavior of virtual machine when it receives the S1 ACPI call.
"#####) },),
        ("default_power_off_type", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "defaultPowerOffType", doc: Some(r#####"Default operation for power off: soft or hard
"#####) },),
        ("suspend_type", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "suspendType", doc: Some(r#####"Describes the default suspend type for this virtual machine.

The possible values are specified by the PowerOpType.
- hard - Perform suspend by using the Suspend method.
- soft - Perform suspend by using the StandbyGuest method.
- preset - The preset value is specified in the defaultSuspendType
  section.
  
This setting is advisory and clients can choose to ignore it.
"#####) },),
        ("power_off_type", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "powerOffType", doc: Some(r#####"Describes the default power off type for this virtual machine.

The possible values are specified by the PowerOpType.
- hard - Perform power off by using the PowerOff method.
- soft - Perform power off by using the ShutdownGuest method.
- preset - The preset value is specified in the defaultPowerOffType
  section.
  
This setting is advisory and clients can choose to ignore it.
"#####) },),
        ("default_suspend_type", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "defaultSuspendType", doc: Some(r#####"Default operation for suspend: soft or hard
"#####) },),
        ("reset_type", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "resetType", doc: Some(r#####"Describes the default reset type for this virtual machine.

The possible values are specified by the PowerOpType.
- hard - Perform reset by using the Reset method.
- soft - Perform reset by using the RebootGuest method.
- preset - The preset value is specified in the defaultResetType
  section.
  
This setting is advisory and clients can choose to ignore it.
"#####) },),
        ("default_reset_type", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "defaultResetType", doc: Some(r#####"Default operation for reset: soft or hard
"#####) },),
    ],
}),
        ("LatencySensitivity", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("sensitivity", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "sensitivity", doc: Some(r#####"Deprecated as of vSphere version 5.5, this field is deprecated.

The custom absolute latency-sensitivity value of the application.

This value will be used only when the latency-sensitivity
*LatencySensitivity.level* property is is set to
<code>custom</code>. It is ignored in all other cases.

The unit of this value is micro-seconds and the application is more
latency sensitive when this value is smaller. For example, if the
absolute latency-sensitivity is 2000us, the kernel will
try to schedule the virtual machine in a way so that its scheduling
latency is not more than 2ms.
"#####) },),
        ("level", NodeData { type_decl: "vim_rs::types::enums::LatencySensitivitySensitivityLevelEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("LatencySensitivitySensitivityLevel"), path_segment: "level", doc: Some(r#####"The nominal latency-sensitive level of the application.
"#####) },),
    ],
}),
        ("DVSContactInfo", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"The name of the person who is responsible for the switch.
"#####) },),
        ("contact", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "contact", doc: Some(r#####"The contact information for the person.
"#####) },),
    ],
}),
        ("HostAuthenticationManagerInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("auth_config", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::HostAuthenticationStoreInfoTrait>>", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ArrayOfHostAuthenticationStoreInfo"), path_segment: "authConfig", doc: Some(r#####"An array containing entries for local authentication and host
Active Directory authentication.
- *HostLocalAuthenticationInfo* - Local authentication is always enabled.
- *HostActiveDirectoryInfo* - Host Active Directory authentication information
  includes the name of the domain, membership status,
  and a list of other domains trusted by the membership domain.
"#####) },),
    ],
}),
        ("VirtualApp", ::phf::Map {
    key: 351906021642186605,
    disps: &[
        (17, 22),
        (0, 0),
        (6, 22),
        (2, 0),
        (1, 12),
        (14, 7),
    ],
    entries: &[
        ("v_app_config", NodeData { type_decl: "vim_rs::types::structs::VAppConfigInfo", type_name: "VAppConfigInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "vAppConfig", doc: Some(r#####"Configuration of this package.

***Required privileges:*** System.Read
"#####) },),
        ("owner", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "owner", doc: Some(r#####"The ComputeResource to which this set of one or more nested resource pools
belong.

***Required privileges:*** System.View
"#####) },),
        ("recent_task", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "recentTask", doc: Some(r#####"The set of recent tasks operating on this managed entity.

This is a subset
of *TaskManager.recentTask* belong to this entity. A task in this
list could be in one of the four states: pending, running, success or error.

This property can be used to deduce intermediate power states for
a virtual machine entity. For example, if the current powerState is "poweredOn"
and there is a running task performing the "suspend" operation, then the virtual
machine's intermediate state might be described as "suspending."

Most tasks (such as power operations) obtain exclusive access to the virtual
machine, so it is unusual for this list to contain more than one running task.
One exception, however, is the task of cloning a virtual machine.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("overall_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "overallStatus", doc: Some(r#####"General health of this managed entity.

The overall status of the managed entity is computed as the worst status
among its alarms and the configuration issues detected on the entity.
The status is reported as one of the following values:
- red: The entity has alarms or configuration issues with a red status.
- yellow: The entity does not have alarms or configuration issues with a
  red status, and has at least one with a yellow status.
- green: The entity does not have alarms or configuration issues with a
  red or yellow status, and has at least one with a green status.
- gray: All of the entity's alarms have a gray status and the
  configuration status of the entity is not being monitored.
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("parent", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "parent", doc: Some(r#####"Parent of this entity.

This value is null for the root object and for
*VirtualMachine* objects that are part of
a *VirtualApp*.

***Required privileges:*** System.View
"#####) },),
        ("parent_v_app", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "parentVApp", doc: Some(r#####"Reference to the parent vApp.
"#####) },),
        ("runtime", NodeData { type_decl: "vim_rs::types::structs::ResourcePoolRuntimeInfo", type_name: "ResourcePoolRuntimeInfo", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "runtime", doc: Some(r#####"Runtime information about a resource pool.

The *ResourcePoolResourceUsage* information within
*ResourcePoolRuntimeInfo* can be transiently stale.
Use *ResourcePool.RefreshRuntime* method to
update the information.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("effective_role", NodeData { type_decl: "Vec<i32>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfInt"), path_segment: "effectiveRole", doc: Some(r#####"Access rights the current session has to this entity.

***Required privileges:*** System.View
"#####) },),
        ("tag", NodeData { type_decl: "Vec<vim_rs::types::structs::Tag>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfTag"), path_segment: "tag", doc: Some(r#####"The set of tags associated with this managed entity.

Experimental. Subject to change.

***Required privileges:*** System.View
"#####) },),
        ("summary", NodeData { type_decl: "Box<dyn vim_rs::types::traits::ResourcePoolSummaryTrait>", type_name: "ResourcePoolSummary", is_optional: false, processing_type: FieldProcessingType::Trait, path_segment: "summary", doc: Some(r#####"Basic information about a resource pool.

In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("namespace", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "namespace", doc: Some(r#####"The namespace with which the ResourcePool is associated.

Namespace is a
vAPI resource which divides cluster resources and allows administrators
to give Kubernetes environments to their development teams.
This property is set only at the time of creation and cannot change.

***Required privileges:*** System.View
"#####) },),
        ("network", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "network", doc: Some(r#####"A collection of references to the subset of network objects that
is used by this virtual machine.

***Required privileges:*** System.View
"#####) },),
        ("child_link", NodeData { type_decl: "Vec<vim_rs::types::structs::VirtualAppLinkInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVirtualAppLinkInfo"), path_segment: "childLink", doc: Some(r#####"Deprecated as of vSphere API 5.1.

List of linked children.
"#####) },),
        ("config_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "configStatus", doc: Some(r#####"The configStatus indicates whether or not the system has detected a configuration
issue involving this entity.

For example, it might have detected a
duplicate IP address or MAC address, or a host in a cluster
might be out of compliance. The meanings of the configStatus values are:
- red: A problem has been detected involving the entity.
- yellow: A problem is about to occur or a transient condition
  has occurred (For example, reconfigure fail-over policy).
- green: No configuration issues have been detected.
- gray: The configuration status of the entity is not being monitored.
  
A green status indicates only that a problem has not been detected;
it is not a guarantee that the entity is problem-free.

The *configIssue* property contains a list of the
problems that have been detected.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("vm", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "vm", doc: Some(r#####"The set of virtual machines associated with this resource pool.

***Required privileges:*** System.View
"#####) },),
        ("disabled_method", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "disabledMethod", doc: Some(r#####"List of operations that are disabled, given the current runtime
state of the entity.

For example, a power-on operation always fails if a
virtual machine is already powered on. This list can be used by clients to
enable or disable operations in a graphical user interface.

Note: This list is determined by the current runtime state of an entity,
not by its permissions.

This list may include the following operations for a HostSystem:
- *HostSystem.EnterMaintenanceMode_Task*
- *HostSystem.ExitMaintenanceMode_Task*
- *HostSystem.RebootHost_Task*
- *HostSystem.ShutdownHost_Task*
- *HostSystem.ReconnectHost_Task*
- *HostSystem.DisconnectHost_Task*
  
This list may include the following operations for a VirtualMachine:
- *VirtualMachine.AnswerVM*
- *ManagedEntity.Rename_Task*
- *VirtualMachine.CloneVM_Task*
- *VirtualMachine.PowerOffVM_Task*
- *VirtualMachine.PowerOnVM_Task*
- *VirtualMachine.SuspendVM_Task*
- *VirtualMachine.ResetVM_Task*
- *VirtualMachine.ReconfigVM_Task*
- *VirtualMachine.RelocateVM_Task*
- *VirtualMachine.MigrateVM_Task*
- *VirtualMachine.CustomizeVM_Task*
- *VirtualMachine.ShutdownGuest*
- *VirtualMachine.StandbyGuest*
- *VirtualMachine.RebootGuest*
- *VirtualMachine.CreateSnapshot_Task*
- *VirtualMachine.RemoveAllSnapshots_Task*
- *VirtualMachine.RevertToCurrentSnapshot_Task*
- *VirtualMachine.MarkAsTemplate*
- *VirtualMachine.MarkAsVirtualMachine*
- *VirtualMachine.ResetGuestInformation*
- *VirtualMachine.MountToolsInstaller*
- *VirtualMachine.UnmountToolsInstaller*
- *ManagedEntity.Destroy_Task*
- *VirtualMachine.UpgradeVM_Task*
- *VirtualMachine.ExportVm*
  
This list may include the following operations for a ResourcePool:
- *ResourcePool.ImportVApp*
- *ResourcePool.CreateChildVM_Task*
- *ResourcePool.UpdateConfig*
- *Folder.CreateVM_Task*
- *ManagedEntity.Destroy_Task*
- *ManagedEntity.Rename_Task*
  
This list may include the following operations for a VirtualApp:
- *ManagedEntity.Destroy_Task*
- *VirtualApp.CloneVApp_Task*
- *VirtualApp.unregisterVApp_Task*
- *VirtualApp.ExportVApp*
- *VirtualApp.PowerOnVApp_Task*
- *VirtualApp.PowerOffVApp_Task*
- *VirtualApp.UpdateVAppConfig*
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("parent_folder", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "parentFolder", doc: Some(r#####"A reference to the parent folder in the VM and Template folder hierarchy.

This
is only set for a root vApp. A root vApp is a vApp that is not a child of
another vApp.

***Required privileges:*** System.View
"#####) },),
        ("config", NodeData { type_decl: "vim_rs::types::structs::ResourceConfigSpec", type_name: "ResourceConfigSpec", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "config", doc: Some(r#####"Configuration of this resource pool.
"#####) },),
        ("alarm_actions_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "alarmActionsEnabled", doc: Some(r#####"Whether alarm actions are enabled for this entity.

True if enabled; false otherwise.

***Required privileges:*** System.Read
"#####) },),
        ("child_configuration", NodeData { type_decl: "Vec<vim_rs::types::structs::ResourceConfigSpec>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfResourceConfigSpec"), path_segment: "childConfiguration", doc: Some(r#####"The resource configuration of all direct children (VirtualMachine and
ResourcePool) of this resource group.

Property collector update notifications might not be generated for this
property. To listen for the child configuration change, please create
PropertyCollector filter on the child entities directly.
"#####) },),
        ("value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "value", doc: Some(r#####"List of custom field values.

Each value uses a key to associate
an instance of a *CustomFieldStringValue* with
a custom field definition.

***Required privileges:*** System.View
"#####) },),
        ("config_issue", NodeData { type_decl: "Vec<vim_rs::types::structs::Event>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfEvent"), path_segment: "configIssue", doc: Some(r#####"Current configuration issues that have been detected for this entity.

Typically,
these issues have already been logged as events. The entity stores these
events as long as they are still current. The
*configStatus* property provides an overall status
based on these events.
"#####) },),
        ("triggered_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "triggeredAlarmState", doc: Some(r#####"A set of alarm states for alarms triggered by this entity
or by its descendants.

Triggered alarms are propagated up the inventory hierarchy
so that a user can readily tell when a descendant has triggered an alarm.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.

***Required privileges:*** System.View
"#####) },),
        ("datastore", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "datastore", doc: Some(r#####"A collection of references to the subset of datastore objects used by this
vApp.

***Required privileges:*** System.View
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"Name of this entity, unique relative to its parent.

Any / (slash), \\ (backslash), character used in this
name element will be escaped. Similarly, any % (percent) character used in
this name element will be escaped, unless it is used to start an escape
sequence. A slash is escaped as %2F or %2f. A backslash is escaped as %5C or
%5c, and a percent is escaped as %25.

***Required privileges:*** System.View
"#####) },),
        ("resource_pool", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "resourcePool", doc: Some(r#####"The set of child resource pools.

***Required privileges:*** System.View
"#####) },),
        ("declared_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "declaredAlarmState", doc: Some(r#####"A set of alarm states for alarms that apply to this managed entity.

The set includes alarms defined on this entity
and alarms inherited from the parent entity,
or from any ancestors in the inventory hierarchy.

Alarms are inherited if they can be triggered by this entity or its descendants.
This set does not include alarms that are defined on descendants of this entity.

***Required privileges:*** System.View
"#####) },),
        ("permission", NodeData { type_decl: "Vec<vim_rs::types::structs::Permission>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfPermission"), path_segment: "permission", doc: Some(r#####"List of permissions defined for this entity.
"#####) },),
        ("available_field", NodeData { type_decl: "Vec<vim_rs::types::structs::CustomFieldDef>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldDef"), path_segment: "availableField", doc: Some(r#####"List of custom field definitions that are valid for the object's type.

The fields are sorted by *CustomFieldDef.name*.

***Required privileges:*** System.View
"#####) },),
        ("custom_value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "customValue", doc: Some(r#####"Custom field values.

***Required privileges:*** System.View
"#####) },),
    ],
}),
        ("HostHyperThreadScheduleInfo", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("config", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "config", doc: Some(r#####"The flag to indicate whether or not the CPU scheduler
should treat hyperthreads as
schedulable resources the next time the CPU scheduler starts.
- This property is set to "true" by successfully invoking the
  *enableHyperThreading()* method.
- This property is set to "false" by successfully invoking the
  *disableHyperthreading()* method.
"#####) },),
        ("available", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "available", doc: Some(r#####"The flag to indicate whether or not hyperthreading
optimization is available on the system.

This property
is set by VMware prior to installation.
"#####) },),
        ("active", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "active", doc: Some(r#####"The flag to indicate whether or not the CPU scheduler is
currently treating
hyperthreads as schedulable resources.

Setting this property
involves a successful invocation of either the
*enableHyperThreading()* method ("true") or the
*disableHyperthreading()* method
("false"). The property is set once the system is rebooted.
"#####) },),
    ],
}),
        ("ResourcePoolQuickStats", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (3, 0),
        (3, 8),
        (4, 11),
    ],
    entries: &[
        ("static_cpu_entitlement", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "staticCpuEntitlement", doc: Some(r#####"The static CPU resource entitlement for a virtual machine.

This value is
calculated based on this virtual machine's resource reservations, shares
and limit, and doesn't take into account current usage. This is the worst
case CPU allocation for this virtual machine, that is, the amount of CPU
resource this virtual machine would receive if all virtual machines running
in the cluster went to maximum consumption. Units are MHz.
"#####) },),
        ("consumed_overhead_memory", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "consumedOverheadMemory", doc: Some(r#####"The amount of overhead memory, in MB, currently being consumed to run a VM.

This value is limited by the overhead memory reservation for a VM, stored
in *ResourcePoolQuickStats.overheadMemory*.
"#####) },),
        ("swapped_memory", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "swappedMemory", doc: Some(r#####"The portion of memory, in MB, that is granted to a virtual machine from the
host's swap space.

This is a sign that there is memory pressure on the host.
"#####) },),
        ("private_memory", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "privateMemory", doc: Some(r#####"The portion of memory, in MB, that is granted to a virtual machine from
non-shared host memory.
"#####) },),
        ("host_memory_usage", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "hostMemoryUsage", doc: Some(r#####"Host memory utilization statistics, in MB.

This
is also known as consummed host memory. This is between 0 and
the configured resource limit. Valid while a virtual machine is
running. This includes the overhead memory of a virtual machine.
"#####) },),
        ("overall_cpu_usage", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "overallCpuUsage", doc: Some(r#####"Basic CPU performance statistics, in MHz.
"#####) },),
        ("ballooned_memory", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "balloonedMemory", doc: Some(r#####"The size of the balloon driver in a virtual machine, in MB.

The host will
inflate the balloon driver to reclaim physical memory from a virtual machine.
This is a sign that there is memory pressure on the host.
"#####) },),
        ("static_memory_entitlement", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "staticMemoryEntitlement", doc: Some(r#####"The static memory resource entitlement for a virtual machine.

This value is
calculated based on this virtual machine's resource reservations, shares
and limit, and doesn't take into account current usage. This is the worst
case memory allocation for this virtual machine, that is, the amount of
memory this virtual machine would receive if all virtual machines running
in the cluster went to maximum consumption. Units are MB.
"#####) },),
        ("overall_cpu_demand", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "overallCpuDemand", doc: Some(r#####"Basic CPU performance statistics, in MHz.
"#####) },),
        ("compressed_memory", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "compressedMemory", doc: Some(r#####"The amount of compressed memory currently consumed by VM, in KB.
"#####) },),
        ("distributed_cpu_entitlement", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "distributedCpuEntitlement", doc: Some(r#####"This is the amount of CPU resource, in MHz, that this VM is entitled to, as
calculated by DRS.

Valid only for a VM managed by DRS.
"#####) },),
        ("guest_memory_usage", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "guestMemoryUsage", doc: Some(r#####"Guest memory utilization statistics, in MB.

This
is also known as active guest memory. The number
can be between 0 and the configured memory size of
a virtual machine.
"#####) },),
        ("shared_memory", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "sharedMemory", doc: Some(r#####"The portion of memory, in MB, that is granted to a virtual machine from host
memory that is shared between VMs.
"#####) },),
        ("overhead_memory", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "overheadMemory", doc: Some(r#####"The amount of memory resource (in MB) that will be used by
a virtual machine above its guest memory requirements.

This value is set if and only if a virtual machine is registered
on a host that supports memory resource allocation features.
For powered off VMs, this is the minimum overhead required to
power on the VM on the registered host.
For powered on VMs, this is the current overhead reservation, a
value which is almost always larger than the minimum overhead, and
which grows with time.

See also *HostSystem.QueryMemoryOverheadEx*.
"#####) },),
        ("distributed_memory_entitlement", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "distributedMemoryEntitlement", doc: Some(r#####"This is the amount of memory, in MB, that this VM is entitled to, as
calculated by DRS.

Valid only for a VM managed by DRS.
"#####) },),
    ],
}),
        ("HostNetOffloadCapabilities", ::phf::Map {
    key: 2980949210194914378,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("zero_copy_xmit", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "zeroCopyXmit", doc: Some(r#####"(Optional) The flag to indicate whether or not zero copy
transmits are supported.
"#####) },),
        ("tcp_segmentation", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "tcpSegmentation", doc: Some(r#####"(Optional) The flag to indicate whether or not TCP segmentation
offloading (TSO) is supported.
"#####) },),
        ("csum_offload", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "csumOffload", doc: Some(r#####"(Optional) The flag to indicate whether or not checksum
offloading is supported.
"#####) },),
    ],
}),
        ("Folder", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
        (5, 7),
        (6, 9),
        (4, 1),
    ],
    entries: &[
        ("value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "value", doc: Some(r#####"List of custom field values.

Each value uses a key to associate
an instance of a *CustomFieldStringValue* with
a custom field definition.

***Required privileges:*** System.View
"#####) },),
        ("parent", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "parent", doc: Some(r#####"Parent of this entity.

This value is null for the root object and for
*VirtualMachine* objects that are part of
a *VirtualApp*.

***Required privileges:*** System.View
"#####) },),
        ("config_issue", NodeData { type_decl: "Vec<vim_rs::types::structs::Event>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfEvent"), path_segment: "configIssue", doc: Some(r#####"Current configuration issues that have been detected for this entity.

Typically,
these issues have already been logged as events. The entity stores these
events as long as they are still current. The
*configStatus* property provides an overall status
based on these events.
"#####) },),
        ("recent_task", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "recentTask", doc: Some(r#####"The set of recent tasks operating on this managed entity.

This is a subset
of *TaskManager.recentTask* belong to this entity. A task in this
list could be in one of the four states: pending, running, success or error.

This property can be used to deduce intermediate power states for
a virtual machine entity. For example, if the current powerState is "poweredOn"
and there is a running task performing the "suspend" operation, then the virtual
machine's intermediate state might be described as "suspending."

Most tasks (such as power operations) obtain exclusive access to the virtual
machine, so it is unusual for this list to contain more than one running task.
One exception, however, is the task of cloning a virtual machine.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("triggered_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "triggeredAlarmState", doc: Some(r#####"A set of alarm states for alarms triggered by this entity
or by its descendants.

Triggered alarms are propagated up the inventory hierarchy
so that a user can readily tell when a descendant has triggered an alarm.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.

***Required privileges:*** System.View
"#####) },),
        ("declared_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "declaredAlarmState", doc: Some(r#####"A set of alarm states for alarms that apply to this managed entity.

The set includes alarms defined on this entity
and alarms inherited from the parent entity,
or from any ancestors in the inventory hierarchy.

Alarms are inherited if they can be triggered by this entity or its descendants.
This set does not include alarms that are defined on descendants of this entity.

***Required privileges:*** System.View
"#####) },),
        ("externally_managed_folder_info", NodeData { type_decl: "vim_rs::types::structs::FolderExternallyManagedFolderInfo", type_name: "FolderExternallyManagedFolderInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "externallyManagedFolderInfo", doc: Some(r#####"The information of externally managed folder.

This property is only set for the externally managed folder.

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("tag", NodeData { type_decl: "Vec<vim_rs::types::structs::Tag>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfTag"), path_segment: "tag", doc: Some(r#####"The set of tags associated with this managed entity.

Experimental. Subject to change.

***Required privileges:*** System.View
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"Name of this entity, unique relative to its parent.

Any / (slash), \\ (backslash), character used in this
name element will be escaped. Similarly, any % (percent) character used in
this name element will be escaped, unless it is used to start an escape
sequence. A slash is escaped as %2F or %2f. A backslash is escaped as %5C or
%5c, and a percent is escaped as %25.

***Required privileges:*** System.View
"#####) },),
        ("child_type", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "childType", doc: Some(r#####"Specifies the object types a folder may contain.

When you create a folder, it inherits its childType from the parent folder
in which it is created. childType is an array of strings. Each array entry
identifies a set of object types - Folder and one or more managed object
types. The following list shows childType values for the different folders:
- { "vim.Folder", "vim.Datacenter" } - Identifies the root folder
  and its descendant folders. Data center folders can contain
  child data center folders and Datacenter managed objects.
  Datacenter objects contain virtual machine, compute resource,
  network entity, and datastore folders.
- { "vim.Folder", "vim.Virtualmachine", "vim.VirtualApp" } - Identifies
  a virtual machine folder. A virtual machine folder may contain child
  virtual machine folders. It also can contain VirtualMachine managed objects,
  templates, and VirtualApp managed objects.
- { "vim.Folder", "vim.ComputeResource" } - Identifies a
  compute resource folder, which contains child compute resource folders
  and ComputeResource hierarchies.
- { "vim.Folder", "vim.Network" } - Identifies a network entity folder.
  Network entity folders on a vCenter Server can contain Network,
  DistributedVirtualSwitch, and DistributedVirtualPortgroup managed
  objects. Network entity folders on an ESXi host can contain only
  Network objects.
- { "vim.Folder", "vim.Datastore" } - Identifies a datastore folder.
  Datastore folders can contain child datastore folders and Datastore
  managed objects.
  
***Required privileges:*** System.View
"#####) },),
        ("namespace", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "namespace", doc: Some(r#####"The namespace with which the Folder is associated.

Namespace is a vAPI
resource which divides cluster resources and allows administrators to
give Kubernetes environments to their development teams.
This property is set only at the time of creation and cannot change.

***Required privileges:*** System.View
"#####) },),
        ("config_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "configStatus", doc: Some(r#####"The configStatus indicates whether or not the system has detected a configuration
issue involving this entity.

For example, it might have detected a
duplicate IP address or MAC address, or a host in a cluster
might be out of compliance. The meanings of the configStatus values are:
- red: A problem has been detected involving the entity.
- yellow: A problem is about to occur or a transient condition
  has occurred (For example, reconfigure fail-over policy).
- green: No configuration issues have been detected.
- gray: The configuration status of the entity is not being monitored.
  
A green status indicates only that a problem has not been detected;
it is not a guarantee that the entity is problem-free.

The *configIssue* property contains a list of the
problems that have been detected.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("available_field", NodeData { type_decl: "Vec<vim_rs::types::structs::CustomFieldDef>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldDef"), path_segment: "availableField", doc: Some(r#####"List of custom field definitions that are valid for the object's type.

The fields are sorted by *CustomFieldDef.name*.

***Required privileges:*** System.View
"#####) },),
        ("custom_value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "customValue", doc: Some(r#####"Custom field values.

***Required privileges:*** System.View
"#####) },),
        ("alarm_actions_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "alarmActionsEnabled", doc: Some(r#####"Whether alarm actions are enabled for this entity.

True if enabled; false otherwise.

***Required privileges:*** System.Read
"#####) },),
        ("effective_role", NodeData { type_decl: "Vec<i32>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfInt"), path_segment: "effectiveRole", doc: Some(r#####"Access rights the current session has to this entity.

***Required privileges:*** System.View
"#####) },),
        ("disabled_method", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "disabledMethod", doc: Some(r#####"List of operations that are disabled, given the current runtime
state of the entity.

For example, a power-on operation always fails if a
virtual machine is already powered on. This list can be used by clients to
enable or disable operations in a graphical user interface.

Note: This list is determined by the current runtime state of an entity,
not by its permissions.

This list may include the following operations for a HostSystem:
- *HostSystem.EnterMaintenanceMode_Task*
- *HostSystem.ExitMaintenanceMode_Task*
- *HostSystem.RebootHost_Task*
- *HostSystem.ShutdownHost_Task*
- *HostSystem.ReconnectHost_Task*
- *HostSystem.DisconnectHost_Task*
  
This list may include the following operations for a VirtualMachine:
- *VirtualMachine.AnswerVM*
- *ManagedEntity.Rename_Task*
- *VirtualMachine.CloneVM_Task*
- *VirtualMachine.PowerOffVM_Task*
- *VirtualMachine.PowerOnVM_Task*
- *VirtualMachine.SuspendVM_Task*
- *VirtualMachine.ResetVM_Task*
- *VirtualMachine.ReconfigVM_Task*
- *VirtualMachine.RelocateVM_Task*
- *VirtualMachine.MigrateVM_Task*
- *VirtualMachine.CustomizeVM_Task*
- *VirtualMachine.ShutdownGuest*
- *VirtualMachine.StandbyGuest*
- *VirtualMachine.RebootGuest*
- *VirtualMachine.CreateSnapshot_Task*
- *VirtualMachine.RemoveAllSnapshots_Task*
- *VirtualMachine.RevertToCurrentSnapshot_Task*
- *VirtualMachine.MarkAsTemplate*
- *VirtualMachine.MarkAsVirtualMachine*
- *VirtualMachine.ResetGuestInformation*
- *VirtualMachine.MountToolsInstaller*
- *VirtualMachine.UnmountToolsInstaller*
- *ManagedEntity.Destroy_Task*
- *VirtualMachine.UpgradeVM_Task*
- *VirtualMachine.ExportVm*
  
This list may include the following operations for a ResourcePool:
- *ResourcePool.ImportVApp*
- *ResourcePool.CreateChildVM_Task*
- *ResourcePool.UpdateConfig*
- *Folder.CreateVM_Task*
- *ManagedEntity.Destroy_Task*
- *ManagedEntity.Rename_Task*
  
This list may include the following operations for a VirtualApp:
- *ManagedEntity.Destroy_Task*
- *VirtualApp.CloneVApp_Task*
- *VirtualApp.unregisterVApp_Task*
- *VirtualApp.ExportVApp*
- *VirtualApp.PowerOnVApp_Task*
- *VirtualApp.PowerOffVApp_Task*
- *VirtualApp.UpdateVAppConfig*
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("overall_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "overallStatus", doc: Some(r#####"General health of this managed entity.

The overall status of the managed entity is computed as the worst status
among its alarms and the configuration issues detected on the entity.
The status is reported as one of the following values:
- red: The entity has alarms or configuration issues with a red status.
- yellow: The entity does not have alarms or configuration issues with a
  red status, and has at least one with a yellow status.
- green: The entity does not have alarms or configuration issues with a
  red or yellow status, and has at least one with a green status.
- gray: All of the entity's alarms have a gray status and the
  configuration status of the entity is not being monitored.
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("child_entity", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "childEntity", doc: Some(r#####"An array of managed object references.

Each entry is a reference to a child entity.

***Required privileges:*** System.View
"#####) },),
        ("permission", NodeData { type_decl: "Vec<vim_rs::types::structs::Permission>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfPermission"), path_segment: "permission", doc: Some(r#####"List of permissions defined for this entity.
"#####) },),
    ],
}),
        ("HostSystem", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (7, 13),
        (29, 32),
        (0, 14),
        (0, 0),
        (0, 7),
        (0, 7),
        (27, 24),
    ],
    entries: &[
        ("overall_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "overallStatus", doc: Some(r#####"General health of this managed entity.

The overall status of the managed entity is computed as the worst status
among its alarms and the configuration issues detected on the entity.
The status is reported as one of the following values:
- red: The entity has alarms or configuration issues with a red status.
- yellow: The entity does not have alarms or configuration issues with a
  red status, and has at least one with a yellow status.
- green: The entity does not have alarms or configuration issues with a
  red or yellow status, and has at least one with a green status.
- gray: All of the entity's alarms have a gray status and the
  configuration status of the entity is not being monitored.
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("tag", NodeData { type_decl: "Vec<vim_rs::types::structs::Tag>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfTag"), path_segment: "tag", doc: Some(r#####"The set of tags associated with this managed entity.

Experimental. Subject to change.

***Required privileges:*** System.View
"#####) },),
        ("value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "value", doc: Some(r#####"List of custom field values.

Each value uses a key to associate
an instance of a *CustomFieldStringValue* with
a custom field definition.

***Required privileges:*** System.View
"#####) },),
        ("hardware", NodeData { type_decl: "vim_rs::types::structs::HostHardwareInfo", type_name: "HostHardwareInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "hardware", doc: Some(r#####"Hardware configuration of the host.

This might not be available for a
disconnected host.
"#####) },),
        ("summary", NodeData { type_decl: "vim_rs::types::structs::HostListSummary", type_name: "HostListSummary", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "summary", doc: Some(r#####"Basic information about the host, including connection state.
"#####) },),
        ("disabled_method", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "disabledMethod", doc: Some(r#####"List of operations that are disabled, given the current runtime
state of the entity.

For example, a power-on operation always fails if a
virtual machine is already powered on. This list can be used by clients to
enable or disable operations in a graphical user interface.

Note: This list is determined by the current runtime state of an entity,
not by its permissions.

This list may include the following operations for a HostSystem:
- *HostSystem.EnterMaintenanceMode_Task*
- *HostSystem.ExitMaintenanceMode_Task*
- *HostSystem.RebootHost_Task*
- *HostSystem.ShutdownHost_Task*
- *HostSystem.ReconnectHost_Task*
- *HostSystem.DisconnectHost_Task*
  
This list may include the following operations for a VirtualMachine:
- *VirtualMachine.AnswerVM*
- *ManagedEntity.Rename_Task*
- *VirtualMachine.CloneVM_Task*
- *VirtualMachine.PowerOffVM_Task*
- *VirtualMachine.PowerOnVM_Task*
- *VirtualMachine.SuspendVM_Task*
- *VirtualMachine.ResetVM_Task*
- *VirtualMachine.ReconfigVM_Task*
- *VirtualMachine.RelocateVM_Task*
- *VirtualMachine.MigrateVM_Task*
- *VirtualMachine.CustomizeVM_Task*
- *VirtualMachine.ShutdownGuest*
- *VirtualMachine.StandbyGuest*
- *VirtualMachine.RebootGuest*
- *VirtualMachine.CreateSnapshot_Task*
- *VirtualMachine.RemoveAllSnapshots_Task*
- *VirtualMachine.RevertToCurrentSnapshot_Task*
- *VirtualMachine.MarkAsTemplate*
- *VirtualMachine.MarkAsVirtualMachine*
- *VirtualMachine.ResetGuestInformation*
- *VirtualMachine.MountToolsInstaller*
- *VirtualMachine.UnmountToolsInstaller*
- *ManagedEntity.Destroy_Task*
- *VirtualMachine.UpgradeVM_Task*
- *VirtualMachine.ExportVm*
  
This list may include the following operations for a ResourcePool:
- *ResourcePool.ImportVApp*
- *ResourcePool.CreateChildVM_Task*
- *ResourcePool.UpdateConfig*
- *Folder.CreateVM_Task*
- *ManagedEntity.Destroy_Task*
- *ManagedEntity.Rename_Task*
  
This list may include the following operations for a VirtualApp:
- *ManagedEntity.Destroy_Task*
- *VirtualApp.CloneVApp_Task*
- *VirtualApp.unregisterVApp_Task*
- *VirtualApp.ExportVApp*
- *VirtualApp.PowerOnVApp_Task*
- *VirtualApp.PowerOffVApp_Task*
- *VirtualApp.UpdateVAppConfig*
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("datastore_browser", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "datastoreBrowser", doc: Some(r#####"DatastoreBrowser to browse datastores for this host.

***Required privileges:*** System.View
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"Name of this entity, unique relative to its parent.

Any / (slash), \\ (backslash), character used in this
name element will be escaped. Similarly, any % (percent) character used in
this name element will be escaped, unless it is used to start an escape
sequence. A slash is escaped as %2F or %2f. A backslash is escaped as %5C or
%5c, and a percent is escaped as %25.

***Required privileges:*** System.View
"#####) },),
        ("runtime", NodeData { type_decl: "vim_rs::types::structs::HostRuntimeInfo", type_name: "HostRuntimeInfo", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "runtime", doc: Some(r#####"Runtime state information about the host such as connection state.
"#####) },),
        ("config_manager", NodeData { type_decl: "vim_rs::types::structs::HostConfigManager", type_name: "HostConfigManager", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "configManager", doc: Some(r#####"Host configuration systems.

In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("available_field", NodeData { type_decl: "Vec<vim_rs::types::structs::CustomFieldDef>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldDef"), path_segment: "availableField", doc: Some(r#####"List of custom field definitions that are valid for the object's type.

The fields are sorted by *CustomFieldDef.name*.

***Required privileges:*** System.View
"#####) },),
        ("config_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "configStatus", doc: Some(r#####"The configStatus indicates whether or not the system has detected a configuration
issue involving this entity.

For example, it might have detected a
duplicate IP address or MAC address, or a host in a cluster
might be out of compliance. The meanings of the configStatus values are:
- red: A problem has been detected involving the entity.
- yellow: A problem is about to occur or a transient condition
  has occurred (For example, reconfigure fail-over policy).
- green: No configuration issues have been detected.
- gray: The configuration status of the entity is not being monitored.
  
A green status indicates only that a problem has not been detected;
it is not a guarantee that the entity is problem-free.

The *configIssue* property contains a list of the
problems that have been detected.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("triggered_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "triggeredAlarmState", doc: Some(r#####"A set of alarm states for alarms triggered by this entity
or by its descendants.

Triggered alarms are propagated up the inventory hierarchy
so that a user can readily tell when a descendant has triggered an alarm.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.

***Required privileges:*** System.View
"#####) },),
        ("vm", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "vm", doc: Some(r#####"List of virtual machines associated with this host.
"#####) },),
        ("compliance_check_result", NodeData { type_decl: "vim_rs::types::structs::ComplianceResult", type_name: "ComplianceResult", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "complianceCheckResult", doc: Some(r#####"The host profile compliance check result.
"#####) },),
        ("capability", NodeData { type_decl: "vim_rs::types::structs::HostCapability", type_name: "HostCapability", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "capability", doc: Some(r#####"Host capabilities.

This might not be available for a
disconnected host.
"#####) },),
        ("system_resources", NodeData { type_decl: "vim_rs::types::structs::HostSystemResourceInfo", type_name: "HostSystemResourceInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "systemResources", doc: Some(r#####"Reference for the system resource hierarchy, used for configuring the set of
resources reserved to the system and unavailable to virtual machines.
"#####) },),
        ("datastore", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "datastore", doc: Some(r#####"A collection of references to the subset of datastore objects in the datacenter
that are available in this HostSystem.

***Required privileges:*** System.View
"#####) },),
        ("precheck_remediation_result", NodeData { type_decl: "vim_rs::types::structs::ApplyHostProfileConfigurationSpec", type_name: "ApplyHostProfileConfigurationSpec", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "precheckRemediationResult", doc: Some(r#####"The host profile precheck-remediation result.
"#####) },),
        ("network", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "network", doc: Some(r#####"A collection of references to the subset of network objects in the datacenter that
are available in this HostSystem.

***Required privileges:*** System.View
"#####) },),
        ("parent", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "parent", doc: Some(r#####"Parent of this entity.

This value is null for the root object and for
*VirtualMachine* objects that are part of
a *VirtualApp*.

***Required privileges:*** System.View
"#####) },),
        ("recent_task", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "recentTask", doc: Some(r#####"The set of recent tasks operating on this managed entity.

This is a subset
of *TaskManager.recentTask* belong to this entity. A task in this
list could be in one of the four states: pending, running, success or error.

This property can be used to deduce intermediate power states for
a virtual machine entity. For example, if the current powerState is "poweredOn"
and there is a running task performing the "suspend" operation, then the virtual
machine's intermediate state might be described as "suspending."

Most tasks (such as power operations) obtain exclusive access to the virtual
machine, so it is unusual for this list to contain more than one running task.
One exception, however, is the task of cloning a virtual machine.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("licensable_resource", NodeData { type_decl: "vim_rs::types::structs::HostLicensableResourceInfo", type_name: "HostLicensableResourceInfo", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "licensableResource", doc: Some(r#####"Information about all licensable resources, currently present on this host.

This information is used mostly by the modules, manipulating information
in the *LicenseManager*. Developers of such modules
should use this property instead of *hardware*.

NOTE:
The values in this property may not be accurate for pre-5.0 hosts when returned by vCenter 5.0
"#####) },),
        ("declared_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "declaredAlarmState", doc: Some(r#####"A set of alarm states for alarms that apply to this managed entity.

The set includes alarms defined on this entity
and alarms inherited from the parent entity,
or from any ancestors in the inventory hierarchy.

Alarms are inherited if they can be triggered by this entity or its descendants.
This set does not include alarms that are defined on descendants of this entity.

***Required privileges:*** System.View
"#####) },),
        ("permission", NodeData { type_decl: "Vec<vim_rs::types::structs::Permission>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfPermission"), path_segment: "permission", doc: Some(r#####"List of permissions defined for this entity.
"#####) },),
        ("compliance_check_state", NodeData { type_decl: "vim_rs::types::structs::HostSystemComplianceCheckState", type_name: "HostSystemComplianceCheckState", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "complianceCheckState", doc: Some(r#####"The host profile compliance check state.
"#####) },),
        ("answer_file_validation_state", NodeData { type_decl: "vim_rs::types::structs::AnswerFileStatusResult", type_name: "AnswerFileStatusResult", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "answerFileValidationState", doc: Some(r#####"Host answer file validation state.
"#####) },),
        ("config_issue", NodeData { type_decl: "Vec<vim_rs::types::structs::Event>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfEvent"), path_segment: "configIssue", doc: Some(r#####"Current configuration issues that have been detected for this entity.

Typically,
these issues have already been logged as events. The entity stores these
events as long as they are still current. The
*configStatus* property provides an overall status
based on these events.
"#####) },),
        ("remediation_state", NodeData { type_decl: "vim_rs::types::structs::HostSystemRemediationState", type_name: "HostSystemRemediationState", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "remediationState", doc: Some(r#####"The host profile remediation state.
"#####) },),
        ("custom_value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "customValue", doc: Some(r#####"Custom field values.

***Required privileges:*** System.View
"#####) },),
        ("alarm_actions_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "alarmActionsEnabled", doc: Some(r#####"Whether alarm actions are enabled for this entity.

True if enabled; false otherwise.

***Required privileges:*** System.Read
"#####) },),
        ("config", NodeData { type_decl: "vim_rs::types::structs::HostConfigInfo", type_name: "HostConfigInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "config", doc: Some(r#####"Host configuration information.

This might not be available for a disconnected
host.
"#####) },),
        ("remediation_result", NodeData { type_decl: "vim_rs::types::structs::ApplyHostProfileConfigurationResult", type_name: "ApplyHostProfileConfigurationResult", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "remediationResult", doc: Some(r#####"The host profile remediation result.
"#####) },),
        ("effective_role", NodeData { type_decl: "Vec<i32>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfInt"), path_segment: "effectiveRole", doc: Some(r#####"Access rights the current session has to this entity.

***Required privileges:*** System.View
"#####) },),
        ("answer_file_validation_result", NodeData { type_decl: "vim_rs::types::structs::AnswerFileStatusResult", type_name: "AnswerFileStatusResult", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "answerFileValidationResult", doc: Some(r#####"Host answer file validation result.
"#####) },),
    ],
}),
        ("VirtualMachine", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 13),
        (22, 6),
        (0, 0),
        (23, 18),
        (7, 19),
        (13, 22),
        (1, 21),
    ],
    entries: &[
        ("datastore", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "datastore", doc: Some(r#####"A collection of references to the subset of datastore objects in the datacenter
that is used by this virtual machine.

***Required privileges:*** System.View
"#####) },),
        ("triggered_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "triggeredAlarmState", doc: Some(r#####"A set of alarm states for alarms triggered by this entity
or by its descendants.

Triggered alarms are propagated up the inventory hierarchy
so that a user can readily tell when a descendant has triggered an alarm.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.

***Required privileges:*** System.View
"#####) },),
        ("parent", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "parent", doc: Some(r#####"Parent of this entity.

This value is null for the root object and for
*VirtualMachine* objects that are part of
a *VirtualApp*.

***Required privileges:*** System.View
"#####) },),
        ("network", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "network", doc: Some(r#####"A collection of references to the subset of network objects in the datacenter that
is used by this virtual machine.

***Required privileges:*** System.View
"#####) },),
        ("config_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "configStatus", doc: Some(r#####"The configStatus indicates whether or not the system has detected a configuration
issue involving this entity.

For example, it might have detected a
duplicate IP address or MAC address, or a host in a cluster
might be out of compliance. The meanings of the configStatus values are:
- red: A problem has been detected involving the entity.
- yellow: A problem is about to occur or a transient condition
  has occurred (For example, reconfigure fail-over policy).
- green: No configuration issues have been detected.
- gray: The configuration status of the entity is not being monitored.
  
A green status indicates only that a problem has not been detected;
it is not a guarantee that the entity is problem-free.

The *configIssue* property contains a list of the
problems that have been detected.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("parent_v_app", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "parentVApp", doc: Some(r#####"Reference to the parent vApp.
"#####) },),
        ("alarm_actions_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "alarmActionsEnabled", doc: Some(r#####"Whether alarm actions are enabled for this entity.

True if enabled; false otherwise.

***Required privileges:*** System.Read
"#####) },),
        ("summary", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineSummary", type_name: "VirtualMachineSummary", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "summary", doc: Some(r#####"Basic information about this virtual machine.

This includes:
- runtimeInfo
- guest
- basic configuration
- alarms
- performance information
"#####) },),
        ("overall_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "overallStatus", doc: Some(r#####"General health of this managed entity.

The overall status of the managed entity is computed as the worst status
among its alarms and the configuration issues detected on the entity.
The status is reported as one of the following values:
- red: The entity has alarms or configuration issues with a red status.
- yellow: The entity does not have alarms or configuration issues with a
  red status, and has at least one with a yellow status.
- green: The entity does not have alarms or configuration issues with a
  red or yellow status, and has at least one with a green status.
- gray: All of the entity's alarms have a gray status and the
  configuration status of the entity is not being monitored.
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("custom_value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "customValue", doc: Some(r#####"Custom field values.

***Required privileges:*** System.View
"#####) },),
        ("tag", NodeData { type_decl: "Vec<vim_rs::types::structs::Tag>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfTag"), path_segment: "tag", doc: Some(r#####"The set of tags associated with this managed entity.

Experimental. Subject to change.

***Required privileges:*** System.View
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"Name of this entity, unique relative to its parent.

Any / (slash), \\ (backslash), character used in this
name element will be escaped. Similarly, any % (percent) character used in
this name element will be escaped, unless it is used to start an escape
sequence. A slash is escaped as %2F or %2f. A backslash is escaped as %5C or
%5c, and a percent is escaped as %25.

***Required privileges:*** System.View
"#####) },),
        ("available_field", NodeData { type_decl: "Vec<vim_rs::types::structs::CustomFieldDef>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldDef"), path_segment: "availableField", doc: Some(r#####"List of custom field definitions that are valid for the object's type.

The fields are sorted by *CustomFieldDef.name*.

***Required privileges:*** System.View
"#####) },),
        ("recent_task", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "recentTask", doc: Some(r#####"The set of recent tasks operating on this managed entity.

This is a subset
of *TaskManager.recentTask* belong to this entity. A task in this
list could be in one of the four states: pending, running, success or error.

This property can be used to deduce intermediate power states for
a virtual machine entity. For example, if the current powerState is "poweredOn"
and there is a running task performing the "suspend" operation, then the virtual
machine's intermediate state might be described as "suspending."

Most tasks (such as power operations) obtain exclusive access to the virtual
machine, so it is unusual for this list to contain more than one running task.
One exception, however, is the task of cloning a virtual machine.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("disabled_method", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "disabledMethod", doc: Some(r#####"List of operations that are disabled, given the current runtime
state of the entity.

For example, a power-on operation always fails if a
virtual machine is already powered on. This list can be used by clients to
enable or disable operations in a graphical user interface.

Note: This list is determined by the current runtime state of an entity,
not by its permissions.

This list may include the following operations for a HostSystem:
- *HostSystem.EnterMaintenanceMode_Task*
- *HostSystem.ExitMaintenanceMode_Task*
- *HostSystem.RebootHost_Task*
- *HostSystem.ShutdownHost_Task*
- *HostSystem.ReconnectHost_Task*
- *HostSystem.DisconnectHost_Task*
  
This list may include the following operations for a VirtualMachine:
- *VirtualMachine.AnswerVM*
- *ManagedEntity.Rename_Task*
- *VirtualMachine.CloneVM_Task*
- *VirtualMachine.PowerOffVM_Task*
- *VirtualMachine.PowerOnVM_Task*
- *VirtualMachine.SuspendVM_Task*
- *VirtualMachine.ResetVM_Task*
- *VirtualMachine.ReconfigVM_Task*
- *VirtualMachine.RelocateVM_Task*
- *VirtualMachine.MigrateVM_Task*
- *VirtualMachine.CustomizeVM_Task*
- *VirtualMachine.ShutdownGuest*
- *VirtualMachine.StandbyGuest*
- *VirtualMachine.RebootGuest*
- *VirtualMachine.CreateSnapshot_Task*
- *VirtualMachine.RemoveAllSnapshots_Task*
- *VirtualMachine.RevertToCurrentSnapshot_Task*
- *VirtualMachine.MarkAsTemplate*
- *VirtualMachine.MarkAsVirtualMachine*
- *VirtualMachine.ResetGuestInformation*
- *VirtualMachine.MountToolsInstaller*
- *VirtualMachine.UnmountToolsInstaller*
- *ManagedEntity.Destroy_Task*
- *VirtualMachine.UpgradeVM_Task*
- *VirtualMachine.ExportVm*
  
This list may include the following operations for a ResourcePool:
- *ResourcePool.ImportVApp*
- *ResourcePool.CreateChildVM_Task*
- *ResourcePool.UpdateConfig*
- *Folder.CreateVM_Task*
- *ManagedEntity.Destroy_Task*
- *ManagedEntity.Rename_Task*
  
This list may include the following operations for a VirtualApp:
- *ManagedEntity.Destroy_Task*
- *VirtualApp.CloneVApp_Task*
- *VirtualApp.unregisterVApp_Task*
- *VirtualApp.ExportVApp*
- *VirtualApp.PowerOnVApp_Task*
- *VirtualApp.PowerOffVApp_Task*
- *VirtualApp.UpdateVAppConfig*
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("effective_role", NodeData { type_decl: "Vec<i32>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfInt"), path_segment: "effectiveRole", doc: Some(r#####"Access rights the current session has to this entity.

***Required privileges:*** System.View
"#####) },),
        ("storage", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineStorageInfo", type_name: "VirtualMachineStorageInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "storage", doc: Some(r#####"Storage space used by the virtual machine, split by datastore.

Can be explicitly refreshed by the *VirtualMachine.RefreshStorageInfo* operation.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("layout_ex", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineFileLayoutEx", type_name: "VirtualMachineFileLayoutEx", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "layoutEx", doc: Some(r#####"Detailed information about the files that comprise this virtual machine.

Can be explicitly refreshed by the *VirtualMachine.RefreshStorageInfo* operation.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("snapshot", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineSnapshotInfo", type_name: "VirtualMachineSnapshotInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "snapshot", doc: Some(r#####"Current snapshot and tree.

The property is valid if snapshots have been created
for this virtual machine.

The contents of this property change in response to the methods:
- *createSnapshot*
- *revertToCurrentSnapshot*
- *remove*
- *revert*
- *removeAllSnapshots*
"#####) },),
        ("guest_heartbeat_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "guestHeartbeatStatus", doc: Some(r#####"The guest heartbeat.

The heartbeat status is classified as:
- gray - VMware Tools are not installed or not running.
- red - No heartbeat. Guest operating system may have stopped responding.
- yellow - Intermittent heartbeat. May be due to guest load.
- green - Guest operating system is responding normally.
  
The guest heartbeat is a statistics metric. Alarms can be configured on
this metric to trigger emails or other actions.
"#####) },),
        ("permission", NodeData { type_decl: "Vec<vim_rs::types::structs::Permission>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfPermission"), path_segment: "permission", doc: Some(r#####"List of permissions defined for this entity.
"#####) },),
        ("config_issue", NodeData { type_decl: "Vec<vim_rs::types::structs::Event>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfEvent"), path_segment: "configIssue", doc: Some(r#####"Current configuration issues that have been detected for this entity.

Typically,
these issues have already been logged as events. The entity stores these
events as long as they are still current. The
*configStatus* property provides an overall status
based on these events.
"#####) },),
        ("guest", NodeData { type_decl: "vim_rs::types::structs::GuestInfo", type_name: "GuestInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "guest", doc: Some(r#####"Information about VMware Tools and about the virtual machine
from the perspective of VMware Tools.

Information about the guest operating system is available in VirtualCenter. Guest
operating system information reflects the last known state of the virtual machine.
For powered on machines, this is current information. For powered off machines,
this is the last recorded state before the virtual machine was powered off.
"#####) },),
        ("declared_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "declaredAlarmState", doc: Some(r#####"A set of alarm states for alarms that apply to this managed entity.

The set includes alarms defined on this entity
and alarms inherited from the parent entity,
or from any ancestors in the inventory hierarchy.

Alarms are inherited if they can be triggered by this entity or its descendants.
This set does not include alarms that are defined on descendants of this entity.

***Required privileges:*** System.View
"#####) },),
        ("capability", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineCapability", type_name: "VirtualMachineCapability", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "capability", doc: Some(r#####"Information about the runtime capabilities of this virtual machine.
"#####) },),
        ("environment_browser", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "environmentBrowser", doc: Some(r#####"The current virtual machine's environment browser object.

This contains
information on all the configurations that can be used on the
virtual machine. This is identical to the environment browser on
the *ComputeResource* to which this virtual machine belongs.
"#####) },),
        ("value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "value", doc: Some(r#####"List of custom field values.

Each value uses a key to associate
an instance of a *CustomFieldStringValue* with
a custom field definition.

***Required privileges:*** System.View
"#####) },),
        ("layout", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineFileLayout", type_name: "VirtualMachineFileLayout", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "layout", doc: Some(r#####"Deprecated as of vSphere API 4.0, use *VirtualMachine.layoutEx* instead.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.

Detailed information about the files that comprise this virtual machine.
"#####) },),
        ("config", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineConfigInfo", type_name: "VirtualMachineConfigInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "config", doc: Some(r#####"Configuration of this virtual machine, including the name and UUID.

This property is set when a virtual machine is created or when
the *reconfigVM* method is called.

The virtual machine configuration is not guaranteed to be available.
For example, the configuration information would be unavailable
if the server is unable to access the virtual machine files on disk,
and is often also unavailable during the initial phases of
virtual machine creation.
"#####) },),
        ("runtime", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineRuntimeInfo", type_name: "VirtualMachineRuntimeInfo", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "runtime", doc: Some(r#####"Execution state and history for this virtual machine.

The contents of this property change when:
- the virtual machine's power state changes.
- an execution message is pending.
- an event occurs.
"#####) },),
        ("resource_pool", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "resourcePool", doc: Some(r#####"The current resource pool that specifies resource allocation
for this virtual machine.

This property is set when a virtual machine is created or associated with
a different resource pool.

Returns null if the virtual machine is a template or the session has no access
to the resource pool.
"#####) },),
        ("resource_config", NodeData { type_decl: "vim_rs::types::structs::ResourceConfigSpec", type_name: "ResourceConfigSpec", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "resourceConfig", doc: Some(r#####"The resource configuration for a virtual machine.

The shares
in this specification are evaluated relative to the resource pool
to which it is assigned. This will return null if the product
the virtual machine is registered on does not support resource
configuration.

To retrieve the configuration, you typically use
*childConfiguration*.

To change the configuration, use
*ResourcePool.UpdateChildResourceConfiguration*.
"#####) },),
        ("root_snapshot", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "rootSnapshot", doc: Some(r#####"The roots of all snapshot trees for the virtual machine.
"#####) },),
    ],
}),
        ("ResourceConfigSpec", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
        (1, 0),
    ],
    entries: &[
        ("memory_allocation", NodeData { type_decl: "vim_rs::types::structs::ResourceAllocationInfo", type_name: "ResourceAllocationInfo", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "memoryAllocation", doc: Some(r#####"Resource allocation for memory.
"#####) },),
        ("last_modified", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveDateTime"), path_segment: "lastModified", doc: Some(r#####"Timestamp when the resources were last modified.

This is ignored when
the object is used to update a configuration.
"#####) },),
        ("scale_descendants_shares", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "scaleDescendantsShares", doc: Some(r#####"Specifies the scaling behavior of the shares of all descendant resource
pools under a given resource pool.

See *ResourceConfigSpecScaleSharesBehavior_enum* for possible values. If any
scaling behavior other than *disabled* is
specified, the system will scale the CPU and memory shares allocated to
each descendant resource pool with the total shares of all powered on
virtual machines under each respective pool. The system will also use the
*SharesInfo* set on each descendant resource pool as a
multiplier for the scale. If a resource pool's shares are already
scalable through the *ResourceConfigSpec.scaleDescendantsShares* setting on an ancestor
resource pool, the system will not allow *ResourceConfigSpec.scaleDescendantsShares* to be set on the resource
pool. The *ResourcePoolRuntimeInfo.sharesScalable* property
indicates whether or not a resource pool's shares are scalable. This
property does not apply to virtual machines.
"#####) },),
        ("entity", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "entity", doc: Some(r#####"Reference to the entity with this resource specification:
either a VirtualMachine or a ResourcePool.

Refers instance of *ManagedEntity*.
"#####) },),
        ("change_version", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "changeVersion", doc: Some(r#####"The changeVersion is a unique identifier for a given version
of the configuration.

Each change to the configuration will
update this value. This is typically implemented as an ever
increasing count or a time-stamp. However, a client should
always treat this as an opaque string.

If specified when updating the resource config., the
changes will only be applied if the current changeVersion matches the
specified changeVersion. This field can be used to guard against updates that
has happened between the configInfo was read and until it is applied.
"#####) },),
        ("cpu_allocation", NodeData { type_decl: "vim_rs::types::structs::ResourceAllocationInfo", type_name: "ResourceAllocationInfo", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "cpuAllocation", doc: Some(r#####"Resource allocation for CPU.
"#####) },),
    ],
}),
        ("VirtualMachineQuickStats", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (0, 19),
        (1, 0),
        (17, 6),
        (0, 7),
        (1, 7),
    ],
    entries: &[
        ("private_memory", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "privateMemory", doc: Some(r#####"The portion of memory, in MB, that is granted to this VM from non-shared
host memory.
"#####) },),
        ("compressed_memory", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "compressedMemory", doc: Some(r#####"The amount of compressed memory currently consumed by VM, in Kb.
"#####) },),
        ("active_memory", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "activeMemory", doc: Some(r#####"The amount of memory that was recently touched by the VM, in MB.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("ft_latency_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "ftLatencyStatus", doc: Some(r#####"The latency status of the fault tolerance VM.

ftLatencyStatus is determined by the value of ftSecondaryLatency.
ftLatencyStatus is:
green, if ftSecondaryLatency is less than or equal to 2 seconds;
yellow, if ftSecondaryLatency is greater than 2 seconds,
and less than or equal to 6 seconds;
red, if ftSecondaryLatency is greater than 6 seconds;
gray, if ftSecondaryLatency is unknown.
"#####) },),
        ("static_memory_entitlement", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "staticMemoryEntitlement", doc: Some(r#####"The static memory resource entitlement for a virtual machine.

This value is
calculated based on this virtual machine's resource reservations, shares
and limit, and doesn't take into account current usage. This is the worst
case memory allocation for this virtual machine, that is, the amount of
memory this virtual machine would receive if all virtual machines running
in the cluster went to maximum consumption. Units are MB.
"#####) },),
        ("overall_cpu_demand", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "overallCpuDemand", doc: Some(r#####"Basic CPU performance statistics, in MHz.

Valid while the virtual machine is running.
"#####) },),
        ("ssd_swapped_memory", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "ssdSwappedMemory", doc: Some(r#####"The amount of memory swapped to fast disk device such as
SSD, in KB.
"#####) },),
        ("swapped_memory", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "swappedMemory", doc: Some(r#####"The portion of memory, in MB, that is granted to this VM from the host's swap
space.

This is a sign that there is memory pressure on the host.
"#####) },),
        ("distributed_memory_entitlement", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "distributedMemoryEntitlement", doc: Some(r#####"This is the amount of memory, in MB, that this VM is entitled to, as
calculated by DRS.

Valid only for a VM managed by DRS.
"#####) },),
        ("memory_tier_stats", NodeData { type_decl: "Vec<vim_rs::types::structs::VirtualMachineQuickStatsMemoryTierStats>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVirtualMachineQuickStatsMemoryTierStats"), path_segment: "memoryTierStats", doc: Some(r#####"Stats for each physical memory tier.

A physical memory tier consists of one or
more logical memory tiers of the same *HostMemoryTierType_enum*. For
example, the logical tiers can be tier0 (DRAM), tier1 (DRAM), and tier2 (PMEM),
while the physical tiers are just DRAM and PMEM.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("shared_memory", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "sharedMemory", doc: Some(r#####"The portion of memory, in MB, that is granted to this VM from host memory
that is shared between VMs.
"#####) },),
        ("static_cpu_entitlement", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "staticCpuEntitlement", doc: Some(r#####"The static CPU resource entitlement for a virtual machine.

This value is
calculated based on this virtual machine's resource reservations, shares
and limit, and doesn't take into account current usage. This is the worst
case CPU allocation for this virtual machine, that is, the amount of CPU
resource this virtual machine would receive if all virtual machines running
in the cluster went to maximum consumption. Units are MHz.
"#####) },),
        ("ft_secondary_latency", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "ftSecondaryLatency", doc: Some(r#####"The amount of time in wallclock that the VCPU of the secondary fault
tolerance VM is behind the VCPU of the primary VM.

The unit is millisecond.
"#####) },),
        ("overall_cpu_usage", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "overallCpuUsage", doc: Some(r#####"Basic CPU performance statistics, in MHz.

Valid while the virtual machine is running.
"#####) },),
        ("distributed_cpu_entitlement", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "distributedCpuEntitlement", doc: Some(r#####"This is the amount of CPU resource, in MHz, that this VM is entitled to, as
calculated by DRS.

Valid only for a VM managed by DRS.
"#####) },),
        ("overall_cpu_readiness", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "overallCpuReadiness", doc: Some(r#####"Percentage of time that the virtual machine was ready, but could not
get scheduled to run on the physical CPU.

Valid while the virtual machine is running.
"#####) },),
        ("consumed_overhead_memory", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "consumedOverheadMemory", doc: Some(r#####"The amount of consumed overhead memory, in MB, for this VM.
"#####) },),
        ("ballooned_memory", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "balloonedMemory", doc: Some(r#####"The size of the balloon driver in the VM, in MB.

The host will inflate the
balloon driver to reclaim physical memory from the VM. This is a sign that
there is memory pressure on the host.
"#####) },),
        ("uptime_seconds", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "uptimeSeconds", doc: Some(r#####"The system uptime of the VM in seconds.
"#####) },),
        ("ft_log_bandwidth", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "ftLogBandwidth", doc: Some(r#####"The network bandwidth used for logging between the
primary and secondary fault tolerance VMs.

The unit is kilobytes per second.
"#####) },),
        ("host_memory_usage", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "hostMemoryUsage", doc: Some(r#####"Host memory utilization statistics, in MB.

This
is also known as consumed host memory. This is between 0 and
the configured resource limit. Valid while the virtual machine is
running. This includes the overhead memory of the VM.
"#####) },),
        ("granted_memory", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "grantedMemory", doc: Some(r#####"Amount of host physical memory that is mapped for a virtual machine,
in MB.

The number can be between 0 and the configured memory size of
the virtual machine. Valid while the virtual machine is running.
"#####) },),
        ("guest_memory_usage", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "guestMemoryUsage", doc: Some(r#####"Guest memory utilization statistics, in MB.

This
is also known as active guest memory. The number
can be between 0 and the configured memory size of
the virtual machine. Valid while the virtual machine is
running.
"#####) },),
        ("guest_heartbeat_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "guestHeartbeatStatus", doc: Some(r#####"Guest operating system heartbeat metric.

See *VirtualMachine.guestHeartbeatStatus* for a description.
"#####) },),
    ],
}),
        ("DVSVendorSpecificConfig", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("inherited", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "inherited", doc: Some(r#####"Whether the configuration is set to inherited value.
"#####) },),
        ("key_value", NodeData { type_decl: "Vec<vim_rs::types::structs::DistributedVirtualSwitchKeyedOpaqueBlob>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfDistributedVirtualSwitchKeyedOpaqueBlob"), path_segment: "keyValue", doc: Some(r#####"An opaque binary blob that stores vendor specific configuration.
"#####) },),
    ],
}),
        ("ComputeResource", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (0, 15),
        (0, 19),
        (3, 0),
        (2, 9),
        (1, 20),
        (11, 3),
    ],
    entries: &[
        ("declared_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "declaredAlarmState", doc: Some(r#####"A set of alarm states for alarms that apply to this managed entity.

The set includes alarms defined on this entity
and alarms inherited from the parent entity,
or from any ancestors in the inventory hierarchy.

Alarms are inherited if they can be triggered by this entity or its descendants.
This set does not include alarms that are defined on descendants of this entity.

***Required privileges:*** System.View
"#####) },),
        ("configuration_ex", NodeData { type_decl: "Box<dyn vim_rs::types::traits::ComputeResourceConfigInfoTrait>", type_name: "ComputeResourceConfigInfo", is_optional: false, processing_type: FieldProcessingType::Trait, path_segment: "configurationEx", doc: Some(r#####"Configuration of the compute resource; applies to both standalone hosts
and clusters.

For a cluster this property will return a
*ClusterConfigInfoEx* object.
"#####) },),
        ("config_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "configStatus", doc: Some(r#####"The configStatus indicates whether or not the system has detected a configuration
issue involving this entity.

For example, it might have detected a
duplicate IP address or MAC address, or a host in a cluster
might be out of compliance. The meanings of the configStatus values are:
- red: A problem has been detected involving the entity.
- yellow: A problem is about to occur or a transient condition
  has occurred (For example, reconfigure fail-over policy).
- green: No configuration issues have been detected.
- gray: The configuration status of the entity is not being monitored.
  
A green status indicates only that a problem has not been detected;
it is not a guarantee that the entity is problem-free.

The *configIssue* property contains a list of the
problems that have been detected.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("recent_task", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "recentTask", doc: Some(r#####"The set of recent tasks operating on this managed entity.

This is a subset
of *TaskManager.recentTask* belong to this entity. A task in this
list could be in one of the four states: pending, running, success or error.

This property can be used to deduce intermediate power states for
a virtual machine entity. For example, if the current powerState is "poweredOn"
and there is a running task performing the "suspend" operation, then the virtual
machine's intermediate state might be described as "suspending."

Most tasks (such as power operations) obtain exclusive access to the virtual
machine, so it is unusual for this list to contain more than one running task.
One exception, however, is the task of cloning a virtual machine.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("overall_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "overallStatus", doc: Some(r#####"General health of this managed entity.

The overall status of the managed entity is computed as the worst status
among its alarms and the configuration issues detected on the entity.
The status is reported as one of the following values:
- red: The entity has alarms or configuration issues with a red status.
- yellow: The entity does not have alarms or configuration issues with a
  red status, and has at least one with a yellow status.
- green: The entity does not have alarms or configuration issues with a
  red or yellow status, and has at least one with a green status.
- gray: All of the entity's alarms have a gray status and the
  configuration status of the entity is not being monitored.
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("effective_role", NodeData { type_decl: "Vec<i32>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfInt"), path_segment: "effectiveRole", doc: Some(r#####"Access rights the current session has to this entity.

***Required privileges:*** System.View
"#####) },),
        ("environment_browser", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "environmentBrowser", doc: Some(r#####"The environment browser object that identifies the environments that are supported
on this compute resource.

***Required privileges:*** System.View
"#####) },),
        ("custom_value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "customValue", doc: Some(r#####"Custom field values.

***Required privileges:*** System.View
"#####) },),
        ("lifecycle_managed", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "lifecycleManaged", doc: Some(r#####"Flag indicating whether or not the lifecycle of the compute resource is
managed.

Once it is enabled, it cannot be disabled.
This property can be set only at the time of creation or through the
*ComputeResource.EnableLifecycleManagement* method.

***Required privileges:*** System.View
"#####) },),
        ("tag", NodeData { type_decl: "Vec<vim_rs::types::structs::Tag>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfTag"), path_segment: "tag", doc: Some(r#####"The set of tags associated with this managed entity.

Experimental. Subject to change.

***Required privileges:*** System.View
"#####) },),
        ("disabled_method", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "disabledMethod", doc: Some(r#####"List of operations that are disabled, given the current runtime
state of the entity.

For example, a power-on operation always fails if a
virtual machine is already powered on. This list can be used by clients to
enable or disable operations in a graphical user interface.

Note: This list is determined by the current runtime state of an entity,
not by its permissions.

This list may include the following operations for a HostSystem:
- *HostSystem.EnterMaintenanceMode_Task*
- *HostSystem.ExitMaintenanceMode_Task*
- *HostSystem.RebootHost_Task*
- *HostSystem.ShutdownHost_Task*
- *HostSystem.ReconnectHost_Task*
- *HostSystem.DisconnectHost_Task*
  
This list may include the following operations for a VirtualMachine:
- *VirtualMachine.AnswerVM*
- *ManagedEntity.Rename_Task*
- *VirtualMachine.CloneVM_Task*
- *VirtualMachine.PowerOffVM_Task*
- *VirtualMachine.PowerOnVM_Task*
- *VirtualMachine.SuspendVM_Task*
- *VirtualMachine.ResetVM_Task*
- *VirtualMachine.ReconfigVM_Task*
- *VirtualMachine.RelocateVM_Task*
- *VirtualMachine.MigrateVM_Task*
- *VirtualMachine.CustomizeVM_Task*
- *VirtualMachine.ShutdownGuest*
- *VirtualMachine.StandbyGuest*
- *VirtualMachine.RebootGuest*
- *VirtualMachine.CreateSnapshot_Task*
- *VirtualMachine.RemoveAllSnapshots_Task*
- *VirtualMachine.RevertToCurrentSnapshot_Task*
- *VirtualMachine.MarkAsTemplate*
- *VirtualMachine.MarkAsVirtualMachine*
- *VirtualMachine.ResetGuestInformation*
- *VirtualMachine.MountToolsInstaller*
- *VirtualMachine.UnmountToolsInstaller*
- *ManagedEntity.Destroy_Task*
- *VirtualMachine.UpgradeVM_Task*
- *VirtualMachine.ExportVm*
  
This list may include the following operations for a ResourcePool:
- *ResourcePool.ImportVApp*
- *ResourcePool.CreateChildVM_Task*
- *ResourcePool.UpdateConfig*
- *Folder.CreateVM_Task*
- *ManagedEntity.Destroy_Task*
- *ManagedEntity.Rename_Task*
  
This list may include the following operations for a VirtualApp:
- *ManagedEntity.Destroy_Task*
- *VirtualApp.CloneVApp_Task*
- *VirtualApp.unregisterVApp_Task*
- *VirtualApp.ExportVApp*
- *VirtualApp.PowerOnVApp_Task*
- *VirtualApp.PowerOffVApp_Task*
- *VirtualApp.UpdateVAppConfig*
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("config_issue", NodeData { type_decl: "Vec<vim_rs::types::structs::Event>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfEvent"), path_segment: "configIssue", doc: Some(r#####"Current configuration issues that have been detected for this entity.

Typically,
these issues have already been logged as events. The entity stores these
events as long as they are still current. The
*configStatus* property provides an overall status
based on these events.
"#####) },),
        ("host", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "host", doc: Some(r#####"List of hosts that are part of this compute resource.

If the compute resource is a
standalone type, then this list contains just one element.

***Required privileges:*** System.View
"#####) },),
        ("available_field", NodeData { type_decl: "Vec<vim_rs::types::structs::CustomFieldDef>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldDef"), path_segment: "availableField", doc: Some(r#####"List of custom field definitions that are valid for the object's type.

The fields are sorted by *CustomFieldDef.name*.

***Required privileges:*** System.View
"#####) },),
        ("summary", NodeData { type_decl: "Box<dyn vim_rs::types::traits::ComputeResourceSummaryTrait>", type_name: "ComputeResourceSummary", is_optional: false, processing_type: FieldProcessingType::Trait, path_segment: "summary", doc: Some(r#####"Basic runtime information about a compute resource.

This information is used on
summary screens and in list views.
"#####) },),
        ("resource_pool", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "resourcePool", doc: Some(r#####"Reference to root resource pool.

***Required privileges:*** System.View
"#####) },),
        ("network_boot_mode", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "networkBootMode", doc: Some(r#####"Managed property indicating whether and what kind of netwoork boot mode
is configured for this compute resource.

Supported values are enumerated
in *ComputeResourceNetworkBootMode_enum*.
This property can be configured via the
*ComputeResource.EnableNetworkBoot_Task* method or during compute
resource creation through the
*ComputeResource.networkBootMode* property.

***Since:*** vSphere API Release 9.0.0.0

***Required privileges:*** System.View
"#####) },),
        ("triggered_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "triggeredAlarmState", doc: Some(r#####"A set of alarm states for alarms triggered by this entity
or by its descendants.

Triggered alarms are propagated up the inventory hierarchy
so that a user can readily tell when a descendant has triggered an alarm.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.

***Required privileges:*** System.View
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"Name of this entity, unique relative to its parent.

Any / (slash), \\ (backslash), character used in this
name element will be escaped. Similarly, any % (percent) character used in
this name element will be escaped, unless it is used to start an escape
sequence. A slash is escaped as %2F or %2f. A backslash is escaped as %5C or
%5c, and a percent is escaped as %25.

***Required privileges:*** System.View
"#####) },),
        ("config_manager_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "configManagerEnabled", doc: Some(r#####"Flag indicating whether or not desired configuration
management platform is enabled on the compute resource.

This property can be set only at the time of creation or through the
*ComputeResource.EnableConfigurationManagement* method.

***Since:*** vSphere API Release 8.0.0.0

***Required privileges:*** System.View
"#####) },),
        ("alarm_actions_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "alarmActionsEnabled", doc: Some(r#####"Whether alarm actions are enabled for this entity.

True if enabled; false otherwise.

***Required privileges:*** System.Read
"#####) },),
        ("parent", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "parent", doc: Some(r#####"Parent of this entity.

This value is null for the root object and for
*VirtualMachine* objects that are part of
a *VirtualApp*.

***Required privileges:*** System.View
"#####) },),
        ("value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "value", doc: Some(r#####"List of custom field values.

Each value uses a key to associate
an instance of a *CustomFieldStringValue* with
a custom field definition.

***Required privileges:*** System.View
"#####) },),
        ("datastore", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "datastore", doc: Some(r#####"The datastore property is the subset of datastore objects in the datacenter
available in this ComputeResource.

This property is computed as the aggregate set of datastores available from all
the hosts that are part of this compute resource.

***Required privileges:*** System.View
"#####) },),
        ("network", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "network", doc: Some(r#####"The subset of network objects available in the datacenter that is available in
this ComputeResource.

This property is computed as the aggregate set of networks available from all the
hosts that are part of this compute resource.

***Required privileges:*** System.View
"#####) },),
        ("permission", NodeData { type_decl: "Vec<vim_rs::types::structs::Permission>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfPermission"), path_segment: "permission", doc: Some(r#####"List of permissions defined for this entity.
"#####) },),
    ],
}),
        ("VirtualMachineConsolePreferences", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("enter_full_screen_on_power_on", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "enterFullScreenOnPowerOn", doc: Some(r#####"Enter full screen mode when this virtual machine is powered on.
"#####) },),
        ("close_on_power_off_or_suspend", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "closeOnPowerOffOrSuspend", doc: Some(r#####"Close the console application when the virtual machine is powered off
or suspended.
"#####) },),
        ("power_on_when_opened", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "powerOnWhenOpened", doc: Some(r#####"Power on the virtual machine when it is opened in the console.
"#####) },),
    ],
}),
        ("FolderExternallyManagedFolderInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("id", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "id", doc: Some(r#####"The ID of the folder in the external system.
"#####) },),
        ("r#type", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "type", doc: Some(r#####"The type of the externally managed folder.

See *FolderExternallyManagedFolderType_enum* for supported values.
"#####) },),
    ],
}),
        ("HostConfigSummary", ::phf::Map {
    key: 1937371814602216758,
    disps: &[
        (2, 0),
        (6, 0),
    ],
    entries: &[
        ("vmotion_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vmotionEnabled", doc: Some(r#####"The flag to indicate whether or not VMotion is enabled on this host.
"#####) },),
        ("feature_version", NodeData { type_decl: "Vec<vim_rs::types::structs::HostFeatureVersionInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostFeatureVersionInfo"), path_segment: "featureVersion", doc: Some(r#####"List of feature-specific version information.

Each element refers
to the version information for a specific feature.
"#####) },),
        ("ssl_thumbprint", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "sslThumbprint", doc: Some(r#####"The SSL thumbprint of the host, if known.
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"The name of the host.
"#####) },),
        ("fault_tolerance_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "faultToleranceEnabled", doc: Some(r#####"The flag to indicate whether or not Fault Tolerance logging is enabled on this host.
"#####) },),
        ("agent_vm_network", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "agentVmNetwork", doc: Some(r#####"Management network for Agent VMs.

Refers instance of *Network*.
"#####) },),
        ("port", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "port", doc: Some(r#####"The port number.
"#####) },),
        ("ssl_certificate", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "sslCertificate", doc: Some(r#####"The SSL certificate of the host, if known.

Note: *HostConfigSummary.sslThumbprint* and *HostConfigSummary.sslCertificate* parameters are
mutually exclusive, and should never be used simultaneously.

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("agent_vm_datastore", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "agentVmDatastore", doc: Some(r#####"Datastore used to deploy Agent VMs on for this host.

Refers instance of *Datastore*.
"#####) },),
        ("product", NodeData { type_decl: "vim_rs::types::structs::AboutInfo", type_name: "AboutInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "product", doc: Some(r#####"Information about the software running on the host, if known.

The current supported hosts are ESX Server 2.0.1 (and later) and VMware Server
2.0.0 (and later).
"#####) },),
    ],
}),
        ("VirtualMachineVirtualNumaInfo", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("vnuma_on_cpu_hotadd_exposed", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vnumaOnCpuHotaddExposed", doc: Some(r#####"Whether virtual NUMA topology is exposed when CPU hotadd is
enabled.
"#####) },),
        ("auto_cores_per_numa_node", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "autoCoresPerNumaNode", doc: Some(r#####"Whether coresPerNode is determined automatically.
"#####) },),
        ("cores_per_numa_node", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "coresPerNumaNode", doc: Some(r#####"Cores per NUMA node.

When this virtual machine is powered off and "autoCoresPerNumaNode"
is True, coresPerNumaNode will be assigned during power-on and this
field should be ignored.
In other cases, this field represents the virtual NUMA node size
seen by the guest.
"#####) },),
    ],
}),
        ("LicenseSource", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
    ],
    entries: &[
    ],
}),
        ("HostCpuPowerManagementInfo", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("hardware_support", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "hardwareSupport", doc: Some(r#####"Information about supported CPU power management.
"#####) },),
        ("current_policy", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "currentPolicy", doc: Some(r#####"Information about current CPU power management policy.
"#####) },),
    ],
}),
        ("HostSgxInfo", ::phf::Map {
    key: 12213676231523076107,
    disps: &[
        (3, 0),
    ],
    entries: &[
        ("sgx_state", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "sgxState", doc: Some(r#####"SGX state of the host.

The set of supported values are described
in *HostSgxInfoSgxStates_enum*.
"#####) },),
        ("total_epc_memory", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "totalEpcMemory", doc: Some(r#####"Size of physical EPC in bytes.
"#####) },),
        ("registration_info", NodeData { type_decl: "vim_rs::types::structs::HostSgxRegistrationInfo", type_name: "HostSgxRegistrationInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "registrationInfo", doc: Some(r#####"***Since:*** vSphere API Release 8.0.0.1
"#####) },),
        ("flc_mode", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "flcMode", doc: Some(r#####"FLC mode of the host.

The set of supported values are
described in *HostSgxInfoFlcModes_enum*.
"#####) },),
        ("le_pub_key_hash", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "lePubKeyHash", doc: Some(r#####"Public key hash of the provider launch enclave.

This is the SHA256
digest of the SIGSTRUCT.MODULUS(MR\_SIGNER) of the provider launch
enclave. This attribute is set only if attribute flcMode is
locked.
"#####) },),
    ],
}),
        ("DatastoreCapability", ::phf::Map {
    key: 2126027241312876569,
    disps: &[
        (2, 3),
        (6, 0),
        (0, 2),
    ],
    entries: &[
        ("raw_disk_mappings_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "rawDiskMappingsSupported", doc: Some(r#####"Indicates whether or not raw disk mappings can be created on this datastore.
"#####) },),
        ("se_sparse_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "seSparseSupported", doc: Some(r#####"Indicates whether the datastore supports the Flex-SE(SeSparse) feature.
"#####) },),
        ("top_level_directory_create_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "topLevelDirectoryCreateSupported", doc: Some(r#####"Indicates whether the datastore supports traditional top-level
directory creation.

See also *DatastoreNamespaceManager*.
"#####) },),
        ("vmdk_expand_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vmdkExpandSupported", doc: Some(r#####"On certain datastores (e.g.

2016 PMEM datastore) VMDK expand is not supported.
This field tells user if VMDK on this datastore can be expanded or not.
If value is undefined, then it should be read as supported.
"#####) },),
        ("storage_iorm_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "storageIORMSupported", doc: Some(r#####"Deprecated as of vSphere8.0 U3, and there is no replacement for it.

Indicates whether the datastore supports Storage I/O Resource Management.
"#####) },),
        ("vsan_sparse_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vsanSparseSupported", doc: Some(r#####"Indicates whether the datastore supports the vsanSparse feature.
"#####) },),
        ("directory_hierarchy_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "directoryHierarchySupported", doc: Some(r#####"Indicates whether or not directories can be created on this datastore.
"#####) },),
        ("vmfs_sparse_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vmfsSparseSupported", doc: Some(r#####"Indicates whether the datastore supports the vmfsSparse feature.

True for VMFS3/VMFS5/NFS/NFS41, False for VMFS6.
If value is undefined, then it should be read as supported.
"#####) },),
        ("upit_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "upitSupported", doc: Some(r#####"Deprecated as of vSphere API 8.0, and there is no replacement for it.

Indicates whether the datastore supports the upit feature.
"#####) },),
        ("per_file_thin_provisioning_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "perFileThinProvisioningSupported", doc: Some(r#####"Indicates whether or not the datastore supports thin provisioning on a per file
basis.

When thin provisioning is used, backing storage is lazily allocated.

This is supported by VMFS3. VMFS2 always allocates storage eagerly. Thus, this
value is false for VMFS2. Most NAS systems always use thin provisioning.
They do not support configuring this on a per file basis, so for NAS systems
this value is also false.
"#####) },),
        ("native_snapshot_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "nativeSnapshotSupported", doc: Some(r#####"Indicates whether the datastore supports native snapshot feature which is
based on Copy-On-Write.
"#####) },),
        ("clustered_vmdk_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "clusteredVmdkSupported", doc: Some(r#####"Indicates whether the datastore supports clustered VMDK feature.
"#####) },),
    ],
}),
        ("DVSConfigInfo", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (0, 14),
        (0, 13),
        (2, 0),
        (1, 8),
        (3, 3),
        (11, 19),
    ],
    entries: &[
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"Name of the switch.
"#####) },),
        ("host", NodeData { type_decl: "Vec<vim_rs::types::structs::DistributedVirtualSwitchHostMember>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfDistributedVirtualSwitchHostMember"), path_segment: "host", doc: Some(r#####"Hosts that join the switch.
"#####) },),
        ("uplink_port_policy", NodeData { type_decl: "Box<dyn vim_rs::types::traits::DvsUplinkPortPolicyTrait>", type_name: "DVSUplinkPortPolicy", is_optional: false, processing_type: FieldProcessingType::Trait, path_segment: "uplinkPortPolicy", doc: Some(r#####"Uplink port policy.
"#####) },),
        ("infrastructure_traffic_resource_config", NodeData { type_decl: "Vec<vim_rs::types::structs::DvsHostInfrastructureTrafficResource>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfDvsHostInfrastructureTrafficResource"), path_segment: "infrastructureTrafficResourceConfig", doc: Some(r#####"Host infrastructure traffic class resource configuration.
"#####) },),
        ("description", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "description", doc: Some(r#####"Description string for the switch.
"#####) },),
        ("create_time", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveDateTime"), path_segment: "createTime", doc: Some(r#####"Create time of the switch.
"#####) },),
        ("net_resource_pool_traffic_resource_config", NodeData { type_decl: "Vec<vim_rs::types::structs::DvsHostInfrastructureTrafficResource>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfDvsHostInfrastructureTrafficResource"), path_segment: "netResourcePoolTrafficResourceConfig", doc: Some(r#####"Dynamic Host infrastructure traffic class resource configuration.
"#####) },),
        ("contact", NodeData { type_decl: "vim_rs::types::structs::DvsContactInfo", type_name: "DVSContactInfo", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "contact", doc: Some(r#####"Human operator contact information.
"#####) },),
        ("switch_ip_address", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "switchIpAddress", doc: Some(r#####"IP address for the switch, specified using IPv4 dot notation.

The
utility of this address is defined by other switch features.
"#####) },),
        ("extension_key", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "extensionKey", doc: Some(r#####"Key of the extension registered by the remote server that
controls the switch.
"#####) },),
        ("target_info", NodeData { type_decl: "vim_rs::types::structs::DistributedVirtualSwitchProductSpec", type_name: "DistributedVirtualSwitchProductSpec", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "targetInfo", doc: Some(r#####"Intended vendor, product, and version information for the
implementation module of the switch.
"#####) },),
        ("pnic_capacity_ratio_for_reservation", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "pnicCapacityRatioForReservation", doc: Some(r#####"The percentage of physical nic link speed
*PhysicalNicLinkInfo.speedMb*
available for infrastructure traffic reservation.

If this value is 75, then for a 1Gbps physical nic, only
750Mbps is allowed for all infrastructure traffic reservations.
"#####) },),
        ("num_standalone_ports", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "numStandalonePorts", doc: Some(r#####"Number of standalone ports in the switch.

Standalone ports are
ports that do not belong to any portgroup.
"#####) },),
        ("config_version", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "configVersion", doc: Some(r#####"Version string of the configuration.
"#####) },),
        ("default_port_config", NodeData { type_decl: "Box<dyn vim_rs::types::traits::DvPortSettingTrait>", type_name: "DVPortSetting", is_optional: false, processing_type: FieldProcessingType::Trait, path_segment: "defaultPortConfig", doc: Some(r#####"Default configuration for the ports in the switch, if the port
does not inherit configuration from the parent portgroup or has
its own configuration.
"#####) },),
        ("network_resource_management_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "networkResourceManagementEnabled", doc: Some(r#####"Boolean to indicate if network I/O control is enabled on the
switch.
"#####) },),
        ("vendor_specific_config", NodeData { type_decl: "Vec<vim_rs::types::structs::DistributedVirtualSwitchKeyedOpaqueBlob>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfDistributedVirtualSwitchKeyedOpaqueBlob"), path_segment: "vendorSpecificConfig", doc: Some(r#####"Opaque binary blob that stores vendor specific configuration.
"#####) },),
        ("num_ports", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "numPorts", doc: Some(r#####"Current number of ports, not including conflict ports.
"#####) },),
        ("uuid", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "uuid", doc: Some(r#####"Generated UUID of the switch.

Unique across vCenter Server
inventory and instances.
"#####) },),
        ("default_proxy_switch_max_num_ports", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "defaultProxySwitchMaxNumPorts", doc: Some(r#####"Default host proxy switch maximum port number
"#####) },),
        ("uplink_portgroup", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "uplinkPortgroup", doc: Some(r#####"List of uplink portgroups.

When adding host members, the server
uses the *DVSConfigInfo.uplinkPortPolicy* to create a number of
uplink ports for the host. If portgroups are shown here,
those uplink ports will be added to the portgroups, with uplink ports
evenly spread among the portgroups.

Refers instances of *DistributedVirtualPortgroup*.
"#####) },),
        ("product_info", NodeData { type_decl: "vim_rs::types::structs::DistributedVirtualSwitchProductSpec", type_name: "DistributedVirtualSwitchProductSpec", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "productInfo", doc: Some(r#####"Vendor, product, and version information for the implementation
module of the switch.
"#####) },),
        ("health_check_config", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::DvsHealthCheckConfigTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfDVSHealthCheckConfig"), path_segment: "healthCheckConfig", doc: Some(r#####"VDS health check configuration.
"#####) },),
        ("policy", NodeData { type_decl: "vim_rs::types::structs::DvsPolicy", type_name: "DVSPolicy", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "policy", doc: Some(r#####"Usage policy of the switch.
"#####) },),
        ("network_resource_control_version", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "networkResourceControlVersion", doc: Some(r#####"Network resource control version of the switch.

Possible value can be of
*DistributedVirtualSwitchNetworkResourceControlVersion_enum*.
"#####) },),
        ("max_ports", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "maxPorts", doc: Some(r#####"Maximum number of ports allowed in the switch,
not including conflict ports.
"#####) },),
        ("vm_vnic_network_resource_pool", NodeData { type_decl: "Vec<vim_rs::types::structs::DvsVmVnicNetworkResourcePool>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfDVSVmVnicNetworkResourcePool"), path_segment: "vmVnicNetworkResourcePool", doc: Some(r#####"The Virtual NIC network resource pool information for the switch.
"#####) },),
    ],
}),
        ("HostLicenseSpec", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("source", NodeData { type_decl: "Box<dyn vim_rs::types::traits::LicenseSourceTrait>", type_name: "LicenseSource", is_optional: true, processing_type: FieldProcessingType::Trait, path_segment: "source", doc: Some(r#####"License source to be used
"#####) },),
        ("edition_key", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "editionKey", doc: Some(r#####"License edition to use
"#####) },),
        ("enabled_feature_key", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "enabledFeatureKey", doc: Some(r#####"Enabled features
"#####) },),
        ("disabled_feature_key", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "disabledFeatureKey", doc: Some(r#####"Disabled features.

When an edition is set, all the features in it
are enabled by default. The following parameter gives a finer
control on which features are disabled.
"#####) },),
    ],
}),
        ("VirtualMachineFlagInfo", ::phf::Map {
    key: 14108922650502679131,
    disps: &[
        (5, 12),
        (2, 0),
        (0, 5),
        (5, 5),
    ],
    entries: &[
        ("cbrc_cache_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "cbrcCacheEnabled", doc: Some(r#####"Flag to specify whether common CBRC digest cache is enabled for this
virtual machine.

The common CBRC cache is shared between the hot added disks in the VM.
If this flag is set to 'true' the VM will allocate a commont digest
cache on power on.
"#####) },),
        ("disk_uuid_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "diskUuidEnabled", doc: Some(r#####"Indicates whether disk UUIDs are being used by this virtual machine.

If this flag is set to false, disk UUIDs are not exposed to the guest.

Since products before ESX 3.1 do not support disk UUIDs, moving virtual
machines from a platform that supports UUID to a platform that does
not support UUIDs could result in unspecified guest behavior. For
virtual machines where the ability to move to older platforms is
important, this flag should be set to false. If the value is unset,
the behavior 'false' will be used.
"#####) },),
        ("fault_tolerance_type", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "faultToleranceType", doc: Some(r#####"Indicates the type of fault tolerance type the virtual machine is
configured to use.

*VirtualMachineFaultToleranceType_enum* represents the set of
possible values.
"#####) },),
        ("snapshot_locked", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "snapshotLocked", doc: Some(r#####"Flag to specify whether the snapshot tree is locked for this virtual machine.
"#####) },),
        ("record_replay_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "recordReplayEnabled", doc: Some(r#####"Deprecated as of vSphere API 6.0.

Flag to specify whether record and replay operations are
allowed for this virtual machine.

If this flag is set to 'true', instruction virtualization
will use hardware virtualization (HV) support. I.e.,
virtualExecUsage will be set to 'hvOn'.
If this flag is set to 'false' for a virtual machine that
already has a recording, replay will be disallowed, though
the recording will be preserved.
If the value is unset, the behavior 'false' will be used.
"#####) },),
        ("disable_acceleration", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "disableAcceleration", doc: Some(r#####"Flag to turn off video acceleration for a virtual machine console window.
"#####) },),
        ("use_toe", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "useToe", doc: Some(r#####"Flag to specify whether or not to use TOE (TCP/IP Offloading).
"#####) },),
        ("vbs_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vbsEnabled", doc: Some(r#####"Flag to specify if Virtualization-based security
is enabled for this virtual machine.

If set to <code>true</code> when creating a new VM, the following VM
properties might be modified automatically:
\- If vim.vm.FlagInfo.vvtdEnabled is not set to <code>false</code>,
it is set to <code>true</code>. Else error is returned.
\- If vim.vm.ConfigSpec.nestedHVEnabled is not set to <code>false</code>,
it is set to <code>true</code>. Else error is returned.
\- If vim.vm.BootOptions.efiSecureBootEnabled is not set to
<code>false</code>, it is set to <code>true</code>. Else error is
returned.
\- If vim.vm.firmware is not set to <code>bios</code>, it is set
to <code>efi</code>. Else error is returned.
"#####) },),
        ("monitor_type", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "monitorType", doc: Some(r#####"vmx process type.

See *VirtualMachineFlagInfoMonitorType_enum*
for possible values for this property.
"#####) },),
        ("enable_logging", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "enableLogging", doc: Some(r#####"Flag to enable logging for a virtual machine.
"#####) },),
        ("run_with_debug_info", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "runWithDebugInfo", doc: Some(r#####"Deprecated as of VI API 2.5, use *VirtualMachineFlagInfo.monitorType*.

Flag to specify whether or not to run in debug mode.
"#####) },),
        ("vvtd_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vvtdEnabled", doc: Some(r#####"Flag to specify if Intel Virtualization Technology for Directed I/O
is enabled for this virtual machine.

When creating a new VM:
\- If vim.vm.FlagInfo.vbsEnabled is set to <code>true</code>,
and this flag is set to <code>false</code> error is returned.
\- If this flag is unset and vim.vm.FlagInfo.vbsEnabled is set to
<code>true</code>, the value of this flag is set to <code>true</code>.
"#####) },),
        ("virtual_exec_usage", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "virtualExecUsage", doc: Some(r#####"Indicates whether or not the system will try to use Hardware
Virtualization (HV) support for instruction virtualization,
if available.

By default, VMware software will determine whether or not
to use hardware virtualization support based on various factors such
as the guest operating system type and the physical hardware.
Certain workloads can benefit from explicitly turning
hardware virtualization support on or off.
If the value is unset, the value will default to hvAuto.

*VirtualMachineFlagInfoVirtualExecUsage_enum* represents the set of
possible values.

New processors can enable two hardware acceleration technologies
for virtualization, one for instruction virtualization and the
other for MMU virtualization. Intel names its hardware-assisted
instruction virtualization as VT, and its hardware-assisted
MMU virtualization as EPT. AMD calls them as AMD-V and RVI,
respectively. For details on these technologies, please refer
to documents from the processor vendors.

*VirtualMachineFlagInfo.virtualExecUsage* controls instruction
virtualization; while *VirtualMachineFlagInfo.virtualMmuUsage*
controls MMU virtualization. "On" allows hardware acceleration,
while "off" only allows software solution.

There are four meaningful combinations.

(hvAuto, automatic) - The host chooses which feature to use.
(hvOn, on) - Use both VT/AMD-V and EPT/RVI.
(hvOn, off) - Use VT/AMD-V but do not use EPT/RVI.
(hvOff, off) - Do not use any of these hardware acceleration technologies.
"#####) },),
        ("virtual_mmu_usage", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "virtualMmuUsage", doc: Some(r#####"Indicates whether or not the system will try to use nested page
table hardware support, if available.

By default, VMware software will determine whether or not
to use nested page table hardware support based on various factors such
as the guest operating system type and the physical hardware.
Certain workloads can benefit from explicitly turning
nested page table hardware support on or off; this can be set using
nptUsage flag.
If the value is unset, the value will default to automatic.

*VirtualMachineFlagInfoVirtualMmuUsage_enum* represents the set of
possible values.
"#####) },),
        ("snapshot_power_off_behavior", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "snapshotPowerOffBehavior", doc: Some(r#####"Specifies the power-off behavior for a virtual machine that has
a snapshot.

If the value is unset, the behavior 'powerOff' will
be used.

See also *VirtualMachinePowerOffBehavior_enum*.
"#####) },),
        ("snapshot_disabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "snapshotDisabled", doc: Some(r#####"Deprecated as of vSphere API 4.0. The flag is ignored by the server.

Flag to specify whether snapshots are disabled for this virtual
machine.
"#####) },),
        ("ht_sharing", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "htSharing", doc: Some(r#####"Deprecated as of vSphere API 6.7.

Specifies how the VCPUs of a virtual machine are allowed to
share physical cores on a hyperthreaded system.

Two VCPUs are
"sharing" a core if they are both running on logical CPUs of
the core at the same time.

See also *VirtualMachineHtSharing_enum*.
"#####) },),
    ],
}),
        ("ServiceConsoleReservationInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("service_console_reserved_cfg", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "serviceConsoleReservedCfg", doc: Some(r#####"The amount of memory that should be reserved for the service console on
the next boot.
"#####) },),
        ("unreserved", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "unreserved", doc: Some(r#####"The amount of memory that is not reserved for use by the service console.
"#####) },),
        ("service_console_reserved", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "serviceConsoleReserved", doc: Some(r#####"The amount of memory that is currently reserved for the service console.
"#####) },),
    ],
}),
        ("VAppIPAssignmentInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("ip_protocol", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "ipProtocol", doc: Some(r#####"Specifies the chosen IP protocol for this deployment.

This must be one of the
values in the supportedIpProtocol field.

Reconfigure privilege: VApp.InstanceConfig
"#####) },),
        ("ip_allocation_policy", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "ipAllocationPolicy", doc: Some(r#####"Specifies how IP allocation should be managed by the VI platform.

This is
typically specified by the deployer. The set of valid options for the policy
is based on the capabilities of the vApp software, as specified by the
supportedAllocationSchemes property.

Reconfigure privilege: VApp.InstanceConfig
"#####) },),
        ("supported_allocation_scheme", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "supportedAllocationScheme", doc: Some(r#####"Specifies the IP allocation schemes supported by the guest software.

When updating this field, an empty array will be interpreted as no changes.
An array of the form \[""\] will clear all settings. Otherwise, the supplied
value will overwrite the current setting.

Reconfigure privilege: VApp.ApplicationConfig
"#####) },),
        ("supported_ip_protocol", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "supportedIpProtocol", doc: Some(r#####"Specifies the IP protocols supported by the guest software.

When updating this field, an empty array will be interpreted as no changes.
An array of the form \[""\] will clear all settings. Otherwise, the supplied
value will overwrite the current setting.

Reconfigure privilege: VApp.ApplicationConfig
"#####) },),
    ],
}),
        ("HostFirewallInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("ruleset", NodeData { type_decl: "Vec<vim_rs::types::structs::HostFirewallRuleset>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostFirewallRuleset"), path_segment: "ruleset", doc: Some(r#####"List of configured rulesets.
"#####) },),
        ("default_policy", NodeData { type_decl: "vim_rs::types::structs::HostFirewallDefaultPolicy", type_name: "HostFirewallDefaultPolicy", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "defaultPolicy", doc: Some(r#####"Default firewall policy.
"#####) },),
    ],
}),
        ("HostSslThumbprintInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("owner_tag", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "ownerTag", doc: Some(r#####"The tag associated with this registration.

Owner tags allow
multiple entities to register the same thumbprint without
interfering with each other on the life cycle of the thumbprint with
their unique tags.
Each solution should use a unique tag to identify itself.
"#####) },),
        ("ssl_thumbprints", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "sslThumbprints", doc: Some(r#####"Specify the SSL thumbprints to register on the host.
"#####) },),
        ("principal", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "principal", doc: Some(r#####"The principal used for the login session
"#####) },),
    ],
}),
        ("HostFirewallConfig", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("rule", NodeData { type_decl: "Vec<vim_rs::types::structs::HostFirewallConfigRuleSetConfig>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostFirewallConfigRuleSetConfig"), path_segment: "rule", doc: Some(r#####"Rules determining firewall settings.
"#####) },),
        ("default_blocking_policy", NodeData { type_decl: "vim_rs::types::structs::HostFirewallDefaultPolicy", type_name: "HostFirewallDefaultPolicy", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "defaultBlockingPolicy", doc: Some(r#####"Default settings for the firewall,
used for ports that are not explicitly opened.
"#####) },),
    ],
}),
        ("HostIpRouteConfig", ::phf::Map {
    key: 7485420634051515786,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("ip_v_6_default_gateway", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "ipV6DefaultGateway", doc: Some(r#####"The default ipv6 gateway address
"#####) },),
        ("gateway_device", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "gatewayDevice", doc: Some(r#####"The gateway device.

This applies to service console gateway only, it
is ignored otherwise.
"#####) },),
        ("default_gateway", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "defaultGateway", doc: Some(r#####"The default gateway address.
"#####) },),
        ("ip_v_6_gateway_device", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "ipV6GatewayDevice", doc: Some(r#####"The ipv6 gateway device.

This applies to service console gateway only, it
"#####) },),
    ],
}),
        ("VirtualMachineContentLibraryItemInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("content_library_item_version", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "contentLibraryItemVersion", doc: Some(r#####"The content library item version is determined and
managed by content library and this field stamps the version
provided by CL to the VM.
"#####) },),
        ("content_library_item_uuid", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "contentLibraryItemUuid", doc: Some(r#####"The content library item UUID
"#####) },),
    ],
}),
        ("HostFileSystemVolumeInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("mount_info", NodeData { type_decl: "Vec<vim_rs::types::structs::HostFileSystemMountInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostFileSystemMountInfo"), path_segment: "mountInfo", doc: Some(r#####"The list of file system volumes mounted on the host.
"#####) },),
        ("volume_type_list", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "volumeTypeList", doc: Some(r#####"The list of supported file system volume types.
"#####) },),
    ],
}),
        ("HostRuntimeInfoNetworkRuntimeInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("network_resource_runtime", NodeData { type_decl: "vim_rs::types::structs::HostNetworkResourceRuntime", type_name: "HostNetworkResourceRuntime", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "networkResourceRuntime", doc: Some(r#####"The network resource runtime information
"#####) },),
        ("net_stack_instance_runtime_info", NodeData { type_decl: "Vec<vim_rs::types::structs::HostRuntimeInfoNetStackInstanceRuntimeInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostRuntimeInfoNetStackInstanceRuntimeInfo"), path_segment: "netStackInstanceRuntimeInfo", doc: Some(r#####"The list of network stack runtime info
"#####) },),
    ],
}),
        ("VirtualMachineRuntimeInfoDasProtectionState", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("das_protected", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "dasProtected", doc: Some(r#####"Whether vSphere HA is protecting a virtual machine (VM).

If a
VM is protected, vSphere HA will enforce any availability
features that have been enabled for this VM. For
example, if the VM is running on a host
that fails and the VM is configured to be restarted on a failure,
then vSphere HA will attempt to restart the VM on another host.
Similarly, if you enable VM/Application Health Monitoring
for this VM, vSphere HA will monitor the heartbeats of the
VM and reset the VM when needed, as dictated by the configured
policy settings.
"#####) },),
    ],
}),
        ("HostServiceInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("service", NodeData { type_decl: "Vec<vim_rs::types::structs::HostService>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostService"), path_segment: "service", doc: Some(r#####"List of configured services.
"#####) },),
    ],
}),
        ("ResourcePoolRuntimeInfo", ::phf::Map {
    key: 345707026197253659,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("cpu", NodeData { type_decl: "vim_rs::types::structs::ResourcePoolResourceUsage", type_name: "ResourcePoolResourceUsage", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "cpu", doc: Some(r#####"Runtime resource usage for CPU.

Values are in Mhz.
"#####) },),
        ("memory", NodeData { type_decl: "vim_rs::types::structs::ResourcePoolResourceUsage", type_name: "ResourcePoolResourceUsage", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "memory", doc: Some(r#####"Runtime resource usage for memory.

Values are in bytes.
"#####) },),
        ("shares_scalable", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "sharesScalable", doc: Some(r#####"The scaling behavior of the shares of a given resource pool.

See *ResourceConfigSpecScaleSharesBehavior_enum* for possible values. The
system will automatically compute this property based on the *ResourceConfigSpec.scaleDescendantsShares* setting on every
ancestor resource pool. This property does not apply to virtual
machines.
"#####) },),
        ("overall_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "overallStatus", doc: Some(r#####"Deprecated as of vSphere API 6.5.
Use *ManagedEntity.overallStatus*.

Overall health of the tree.

See header for description of various
statuses and when they are set.
"#####) },),
    ],
}),
        ("VirtualMachineRuntimeInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 12),
        (0, 0),
        (1, 3),
        (0, 13),
        (7, 27),
        (0, 12),
        (15, 12),
    ],
    entries: &[
        ("num_mks_connections", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "numMksConnections", doc: Some(r#####"Number of active MKS connections to this virtual machine.
"#####) },),
        ("v_flash_cache_allocation", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "vFlashCacheAllocation", doc: Some(r#####"Deprecated since vSphere 7.0 because vFlash Read Cache
end of availability.

Specifies the total allocated vFlash resource for the vFlash caches associated with VM's
VMDKs when VM is powered on, in bytes.
"#####) },),
        ("feature_requirement", NodeData { type_decl: "Vec<vim_rs::types::structs::VirtualMachineFeatureRequirement>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVirtualMachineFeatureRequirement"), path_segment: "featureRequirement", doc: Some(r#####"These requirements must have equivalent host capabilities
*HostConfigInfo.featureCapability* in order to power on,
resume, or migrate to the host.
"#####) },),
        ("need_secondary_reason", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "needSecondaryReason", doc: Some(r#####"If set, indicates the reason the virtual machine needs a secondary.
"#####) },),
        ("boot_time", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveDateTime"), path_segment: "bootTime", doc: Some(r#####"The timestamp when the virtual machine was most recently powered on.

This property is updated when the virtual machine is powered on
from the poweredOff state, and is cleared when the virtual machine is
powered off. This property is not updated when a virtual machine is resumed
from a suspended state.
"#####) },),
        ("op_notification_timeout", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "opNotificationTimeout", doc: Some(r#####"Operation notification timeout in seconds.

Specifies the maximum time duration the application may take to
prepare for the operation after it has been notified.
This property is set only for powered on VMs.

***Since:*** vSphere API Release 8.0.0.1
"#####) },),
        ("offline_feature_requirement", NodeData { type_decl: "Vec<vim_rs::types::structs::VirtualMachineFeatureRequirement>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVirtualMachineFeatureRequirement"), path_segment: "offlineFeatureRequirement", doc: Some(r#####"These requirements must have equivalent host capabilities
*HostConfigInfo.featureCapability* in order to power on.
"#####) },),
        ("consolidation_needed", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "consolidationNeeded", doc: Some(r#####"Whether any disk of the virtual machine requires consolidation.

This can happen for example when a snapshot is deleted but its
associated disk is not committed back to the base disk.
Use *VirtualMachine.ConsolidateVMDisks_Task* to consolidate if
needed.
"#####) },),
        ("suspended_to_memory", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "suspendedToMemory", doc: Some(r#####"Whether the virtual machine is suspended to memory, or not.

***Since:*** vSphere API Release 7.0.2.0
"#####) },),
        ("record_replay_state", NodeData { type_decl: "vim_rs::types::enums::VirtualMachineRecordReplayStateEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("VirtualMachineRecordReplayState"), path_segment: "recordReplayState", doc: Some(r#####"Deprecated as of vSphere API 6.0.

Record / replay state of this virtual machine.
"#####) },),
        ("quiesced_fork_parent", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "quiescedForkParent", doc: Some(r#####"This flag indicates whether a parent virtual machine is in a fork ready
state.

A persistent instant clone child can be created only when this flag
is true. While a non-persistent instant clone child can be created
independent of this flag, it can only be powered on if this flag is true.
"#####) },),
        ("feature_mask", NodeData { type_decl: "Vec<vim_rs::types::structs::HostFeatureMask>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostFeatureMask"), path_segment: "featureMask", doc: Some(r#####"The masks applied to an individual virtual machine as a result of its
configuration.
"#####) },),
        ("suspend_interval", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "suspendInterval", doc: Some(r#####"The total time the virtual machine has been suspended
since it was initially powered on.

This time excludes the current period,
if the virtual machine is currently suspended. This property is updated
when the virtual machine resumes, and is reset to zero when the virtual machine
is powered off.
"#####) },),
        ("question", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineQuestionInfo", type_name: "VirtualMachineQuestionInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "question", doc: Some(r#####"The current question, if any, that is blocking the virtual machine's execution.
"#####) },),
        ("memory_overhead", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "memoryOverhead", doc: Some(r#####"Deprecated as of vSphere API 4.1, use the *PerformanceManager*
memory overhead counter to get this value.

The amount of memory resource (in bytes) that will be used by
the virtual machine above its guest memory requirements.

This value is set if and only if the virtual machine is registered
on a host that supports memory resource allocation features.

For powered off VMs, this is the minimum overhead required to
power on the VM on the registered host.

For powered on VMs, this is the current overhead reservation, a
value which is almost always larger than the minimum overhead, and
which grows with time.

See also *HostSystem.QueryMemoryOverheadEx*.
"#####) },),
        ("vm_failover_in_progress", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vmFailoverInProgress", doc: Some(r#####"Represents if the vm is currently being failed over by FDM

***Since:*** vSphere API Release 7.0.2.0
"#####) },),
        ("snapshot_in_background", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "snapshotInBackground", doc: Some(r#####"Whether a snapshot operation is in progress in the background, or not.
"#####) },),
        ("device", NodeData { type_decl: "Vec<vim_rs::types::structs::VirtualMachineDeviceRuntimeInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVirtualMachineDeviceRuntimeInfo"), path_segment: "device", doc: Some(r#####"Per-device runtime info.

This array will be empty if the host
software does not provide runtime info for any of the device
types currently in use by the virtual machine.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter.
Since this property is on a DataObject, an update returned by WaitForUpdatesEx may
contain values for this property when some other property on the DataObject changes.
If this update is a result of a call to WaitForUpdatesEx with a non-empty
version parameter, the value for this property may not be current.
"#####) },),
        ("connection_state", NodeData { type_decl: "vim_rs::types::enums::VirtualMachineConnectionStateEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("VirtualMachineConnectionState"), path_segment: "connectionState", doc: Some(r#####"Indicates whether or not the virtual machine is available for management.
"#####) },),
        ("fault_tolerance_state", NodeData { type_decl: "vim_rs::types::enums::VirtualMachineFaultToleranceStateEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("VirtualMachineFaultToleranceState"), path_segment: "faultToleranceState", doc: Some(r#####"The fault tolerance state of the virtual machine.
"#####) },),
        ("min_required_evc_mode_key", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "minRequiredEVCModeKey", doc: Some(r#####"For a powered-on or suspended virtual machine in a cluster with Enhanced
VMotion Compatibility (EVC) enabled, this identifies the least-featured
EVC mode (among those for the appropriate CPU vendor) that could admit
the virtual machine.

See *EVCMode*. Until vSphere 6.5, this
property will be unset if the virtual machine is powered off or is not in
an EVC cluster.

This property may be used as a general indicator of the CPU feature
baseline currently in use by the virtual machine. However, the virtual
machine may be suppressing some of the features present in the CPU
feature baseline of the indicated mode, either explicitly (in the
virtual machine's configured
*cpuFeatureMask*) or implicitly
(in the default masks for the
*GuestOsDescriptor* appropriate for the
virtual machine's configured guest OS).
"#####) },),
        ("instant_clone_frozen", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "instantCloneFrozen", doc: Some(r#####"Whether the virtual machine is frozen for instant clone, or not.
"#####) },),
        ("crypto_state", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "cryptoState", doc: Some(r#####"Encryption state of the virtual machine.

Valid values are enumerated by the
*CryptoState* type.
"#####) },),
        ("iommu_active", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "iommuActive", doc: Some(r#####"Indicates whether there is active IOMMU domain in the current VM.

***Since:*** vSphere API Release 8.0.1.0
"#####) },),
        ("power_state", NodeData { type_decl: "vim_rs::types::enums::VirtualMachinePowerStateEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("VirtualMachinePowerState"), path_segment: "powerState", doc: Some(r#####"The current power state of the virtual machine.
"#####) },),
        ("clean_power_off", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "cleanPowerOff", doc: Some(r#####"For a powered off virtual machine, indicates whether the virtual
machine's last shutdown was an orderly power off or not.

Unset if
the virtual machine is running or suspended.
"#####) },),
        ("paused", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "paused", doc: Some(r#####"Whether the virtual machine is paused, or not.
"#####) },),
        ("max_cpu_usage", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "maxCpuUsage", doc: Some(r#####"Current upper-bound on CPU usage.

The upper-bound is based on the host
the virtual machine is current running on, as well as limits configured
on the virtual machine itself or any parent resource pool.
Valid while the virtual machine is running.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter.
Since this property is on a DataObject, an update returned by WaitForUpdatesEx may
contain values for this property when some other property on the DataObject changes.
If this update is a result of a call to WaitForUpdatesEx with a non-empty
version parameter, the value for this property may not be current.
"#####) },),
        ("tools_installer_mounted", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "toolsInstallerMounted", doc: Some(r#####"Flag to indicate whether or not the VMware Tools installer
is mounted as a CD-ROM.
"#####) },),
        ("max_memory_usage", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "maxMemoryUsage", doc: Some(r#####"Current upper-bound on memory usage.

The upper-bound is based on memory
configuration of the virtual machine, as well as limits configured
on the virtual machine itself or any parent resource pool.
Valid while the virtual machine is running.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter.
Since this property is on a DataObject, an update returned by WaitForUpdatesEx may
contain values for this property when some other property on the DataObject changes.
If this update is a result of a call to WaitForUpdatesEx with a non-empty
version parameter, the value for this property may not be current.
"#####) },),
        ("online_standby", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "onlineStandby", doc: Some(r#####"This property indicates whether the guest has gone into one of the
s1, s2 or s3 standby modes, false indicates the guest is awake.
"#####) },),
        ("das_vm_protection", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineRuntimeInfoDasProtectionState", type_name: "VirtualMachineRuntimeInfoDasProtectionState", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "dasVmProtection", doc: Some(r#####"The vSphere HA protection state for a virtual machine.

Property
is unset if vSphere HA is not enabled.
"#####) },),
        ("suspend_time", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveDateTime"), path_segment: "suspendTime", doc: Some(r#####"The timestamp when the virtual machine was most recently suspended.

This property is updated every time the virtual machine is suspended.
"#####) },),
        ("host", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "host", doc: Some(r#####"The host that is responsible for running a virtual machine.

This property is null if the virtual machine is not running and is
not assigned to run on a particular host.

Refers instance of *HostSystem*.
"#####) },),
    ],
}),
        ("Task", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("available_field", NodeData { type_decl: "Vec<vim_rs::types::structs::CustomFieldDef>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldDef"), path_segment: "availableField", doc: Some(r#####"List of custom field definitions that are valid for the object's type.

The fields are sorted by *CustomFieldDef.name*.

***Required privileges:*** System.View
"#####) },),
        ("value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "value", doc: Some(r#####"List of custom field values.

Each value uses a key to associate
an instance of a *CustomFieldStringValue* with
a custom field definition.

***Required privileges:*** System.View
"#####) },),
        ("info", NodeData { type_decl: "vim_rs::types::structs::TaskInfo", type_name: "TaskInfo", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "info", doc: Some(r#####"Detailed information about this task.
"#####) },),
    ],
}),
        ("HostVFlashManagerVFlashConfigInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("v_flash_resource_config_info", NodeData { type_decl: "vim_rs::types::structs::HostVFlashManagerVFlashResourceConfigInfo", type_name: "HostVFlashManagerVFlashResourceConfigInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "vFlashResourceConfigInfo", doc: Some(r#####"vFlash resource configuration information
"#####) },),
        ("v_flash_cache_config_info", NodeData { type_decl: "vim_rs::types::structs::HostVFlashManagerVFlashCacheConfigInfo", type_name: "HostVFlashManagerVFlashCacheConfigInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "vFlashCacheConfigInfo", doc: Some(r#####"vFlash cache configuration information
"#####) },),
    ],
}),
        ("HostListSummaryQuickStats", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 2),
        (0, 0),
    ],
    entries: &[
        ("overall_cpu_usage", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "overallCpuUsage", doc: Some(r#####"Aggregated CPU usage across all cores on the host in MHz.

This is only
available if the host is connected.
"#####) },),
        ("available_p_mem_capacity", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "availablePMemCapacity", doc: Some(r#####"Deprecated as of vSphere 9.0 APIs with no replacement.

The available capacity in MB.
"#####) },),
        ("uptime", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "uptime", doc: Some(r#####"The system uptime of the host in seconds.
"#####) },),
        ("distributed_memory_fairness", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "distributedMemoryFairness", doc: Some(r#####"The fairness of distributed memory resource allocation on the host.
"#####) },),
        ("distributed_cpu_fairness", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "distributedCpuFairness", doc: Some(r#####"The fairness of distributed CPU resource allocation on the host.
"#####) },),
        ("overall_memory_usage", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "overallMemoryUsage", doc: Some(r#####"Physical memory usage on the host in MB.

This is only available if the
host is connected.
"#####) },),
    ],
}),
        ("VirtualMachineAffinityInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("affinity_set", NodeData { type_decl: "Vec<i32>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfInt"), path_segment: "affinitySet", doc: Some(r#####"List of nodes (processors for CPU, NUMA nodes for memory) that
may be used by the virtual machine.

If the array is empty when
modifying the affinity setting, then any existing affinity is removed.
"#####) },),
    ],
}),
        ("ClusterConfigInfo", ::phf::Map {
    key: 4066803471364472071,
    disps: &[
        (3, 0),
    ],
    entries: &[
        ("das_vm_config", NodeData { type_decl: "Vec<vim_rs::types::structs::ClusterDasVmConfigInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfClusterDasVmConfigInfo"), path_segment: "dasVmConfig", doc: Some(r#####"List of virtual machine configurations for the vSphere HA
service.

Each entry applies to one virtual machine.

If a virtual machine is not specified in this array, the service uses
the default settings for that virtual machine.
"#####) },),
        ("drs_vm_config", NodeData { type_decl: "Vec<vim_rs::types::structs::ClusterDrsVmConfigInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfClusterDrsVmConfigInfo"), path_segment: "drsVmConfig", doc: Some(r#####"List of virtual machine configurations for the VMware DRS
service.

Each entry applies to one virtual machine.

If a virtual machine is not specified in this array, the service uses
the default settings for that virtual machine.
"#####) },),
        ("drs_config", NodeData { type_decl: "vim_rs::types::structs::ClusterDrsConfigInfo", type_name: "ClusterDrsConfigInfo", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "drsConfig", doc: Some(r#####"Cluster-wide configuration of the VMware DRS service.
"#####) },),
        ("das_config", NodeData { type_decl: "vim_rs::types::structs::ClusterDasConfigInfo", type_name: "ClusterDasConfigInfo", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "dasConfig", doc: Some(r#####"Cluster-wide configuration of the vSphere HA service.
"#####) },),
        ("rule", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::ClusterRuleInfoTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfClusterRuleInfo"), path_segment: "rule", doc: Some(r#####"Cluster-wide rules.
"#####) },),
    ],
}),
        ("DistributedVirtualSwitch", ::phf::Map {
    key: 12213676231523076107,
    disps: &[
        (1, 0),
        (2, 1),
        (5, 0),
        (0, 19),
        (15, 22),
    ],
    entries: &[
        ("custom_value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "customValue", doc: Some(r#####"Custom field values.

***Required privileges:*** System.View
"#####) },),
        ("triggered_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "triggeredAlarmState", doc: Some(r#####"A set of alarm states for alarms triggered by this entity
or by its descendants.

Triggered alarms are propagated up the inventory hierarchy
so that a user can readily tell when a descendant has triggered an alarm.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.

***Required privileges:*** System.View
"#####) },),
        ("disabled_method", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "disabledMethod", doc: Some(r#####"List of operations that are disabled, given the current runtime
state of the entity.

For example, a power-on operation always fails if a
virtual machine is already powered on. This list can be used by clients to
enable or disable operations in a graphical user interface.

Note: This list is determined by the current runtime state of an entity,
not by its permissions.

This list may include the following operations for a HostSystem:
- *HostSystem.EnterMaintenanceMode_Task*
- *HostSystem.ExitMaintenanceMode_Task*
- *HostSystem.RebootHost_Task*
- *HostSystem.ShutdownHost_Task*
- *HostSystem.ReconnectHost_Task*
- *HostSystem.DisconnectHost_Task*
  
This list may include the following operations for a VirtualMachine:
- *VirtualMachine.AnswerVM*
- *ManagedEntity.Rename_Task*
- *VirtualMachine.CloneVM_Task*
- *VirtualMachine.PowerOffVM_Task*
- *VirtualMachine.PowerOnVM_Task*
- *VirtualMachine.SuspendVM_Task*
- *VirtualMachine.ResetVM_Task*
- *VirtualMachine.ReconfigVM_Task*
- *VirtualMachine.RelocateVM_Task*
- *VirtualMachine.MigrateVM_Task*
- *VirtualMachine.CustomizeVM_Task*
- *VirtualMachine.ShutdownGuest*
- *VirtualMachine.StandbyGuest*
- *VirtualMachine.RebootGuest*
- *VirtualMachine.CreateSnapshot_Task*
- *VirtualMachine.RemoveAllSnapshots_Task*
- *VirtualMachine.RevertToCurrentSnapshot_Task*
- *VirtualMachine.MarkAsTemplate*
- *VirtualMachine.MarkAsVirtualMachine*
- *VirtualMachine.ResetGuestInformation*
- *VirtualMachine.MountToolsInstaller*
- *VirtualMachine.UnmountToolsInstaller*
- *ManagedEntity.Destroy_Task*
- *VirtualMachine.UpgradeVM_Task*
- *VirtualMachine.ExportVm*
  
This list may include the following operations for a ResourcePool:
- *ResourcePool.ImportVApp*
- *ResourcePool.CreateChildVM_Task*
- *ResourcePool.UpdateConfig*
- *Folder.CreateVM_Task*
- *ManagedEntity.Destroy_Task*
- *ManagedEntity.Rename_Task*
  
This list may include the following operations for a VirtualApp:
- *ManagedEntity.Destroy_Task*
- *VirtualApp.CloneVApp_Task*
- *VirtualApp.unregisterVApp_Task*
- *VirtualApp.ExportVApp*
- *VirtualApp.PowerOnVApp_Task*
- *VirtualApp.PowerOffVApp_Task*
- *VirtualApp.UpdateVAppConfig*
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("permission", NodeData { type_decl: "Vec<vim_rs::types::structs::Permission>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfPermission"), path_segment: "permission", doc: Some(r#####"List of permissions defined for this entity.
"#####) },),
        ("network_resource_pool", NodeData { type_decl: "Vec<vim_rs::types::structs::DvsNetworkResourcePool>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfDVSNetworkResourcePool"), path_segment: "networkResourcePool", doc: Some(r#####"Deprecated as of vSphere API 6.0
Use *DVSConfigInfo.vmVnicNetworkResourcePool*
to get the Virtual NIC resource pool information.
Use *DVSConfigInfo.infrastructureTrafficResourceConfig*
to get the host infrastructure resource information.

Network resource pool information for the switch.
"#####) },),
        ("capability", NodeData { type_decl: "vim_rs::types::structs::DvsCapability", type_name: "DVSCapability", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "capability", doc: Some(r#####"Capability of the switch.

Capabilities are indicated at the port,
portgroup and switch levels, and for version-specific features.
When you retrieve this property from an ESXi host,
*DistributedVirtualSwitch.capability*.*DVSCapability.dvsOperationSupported*
should always be set to false.
"#####) },),
        ("tag", NodeData { type_decl: "Vec<vim_rs::types::structs::Tag>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfTag"), path_segment: "tag", doc: Some(r#####"The set of tags associated with this managed entity.

Experimental. Subject to change.

***Required privileges:*** System.View
"#####) },),
        ("runtime", NodeData { type_decl: "vim_rs::types::structs::DvsRuntimeInfo", type_name: "DVSRuntimeInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "runtime", doc: Some(r#####"Runtime information of the distributed virtual switch.
"#####) },),
        ("effective_role", NodeData { type_decl: "Vec<i32>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfInt"), path_segment: "effectiveRole", doc: Some(r#####"Access rights the current session has to this entity.

***Required privileges:*** System.View
"#####) },),
        ("summary", NodeData { type_decl: "vim_rs::types::structs::DvsSummary", type_name: "DVSSummary", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "summary", doc: Some(r#####"Summary of the switch.
"#####) },),
        ("portgroup", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "portgroup", doc: Some(r#####"Portgroups that are defined on the switch.
"#####) },),
        ("config_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "configStatus", doc: Some(r#####"The configStatus indicates whether or not the system has detected a configuration
issue involving this entity.

For example, it might have detected a
duplicate IP address or MAC address, or a host in a cluster
might be out of compliance. The meanings of the configStatus values are:
- red: A problem has been detected involving the entity.
- yellow: A problem is about to occur or a transient condition
  has occurred (For example, reconfigure fail-over policy).
- green: No configuration issues have been detected.
- gray: The configuration status of the entity is not being monitored.
  
A green status indicates only that a problem has not been detected;
it is not a guarantee that the entity is problem-free.

The *configIssue* property contains a list of the
problems that have been detected.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("config", NodeData { type_decl: "Box<dyn vim_rs::types::traits::DvsConfigInfoTrait>", type_name: "DVSConfigInfo", is_optional: false, processing_type: FieldProcessingType::Trait, path_segment: "config", doc: Some(r#####"Switch configuration data.
"#####) },),
        ("value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "value", doc: Some(r#####"List of custom field values.

Each value uses a key to associate
an instance of a *CustomFieldStringValue* with
a custom field definition.

***Required privileges:*** System.View
"#####) },),
        ("config_issue", NodeData { type_decl: "Vec<vim_rs::types::structs::Event>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfEvent"), path_segment: "configIssue", doc: Some(r#####"Current configuration issues that have been detected for this entity.

Typically,
these issues have already been logged as events. The entity stores these
events as long as they are still current. The
*configStatus* property provides an overall status
based on these events.
"#####) },),
        ("declared_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "declaredAlarmState", doc: Some(r#####"A set of alarm states for alarms that apply to this managed entity.

The set includes alarms defined on this entity
and alarms inherited from the parent entity,
or from any ancestors in the inventory hierarchy.

Alarms are inherited if they can be triggered by this entity or its descendants.
This set does not include alarms that are defined on descendants of this entity.

***Required privileges:*** System.View
"#####) },),
        ("overall_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "overallStatus", doc: Some(r#####"General health of this managed entity.

The overall status of the managed entity is computed as the worst status
among its alarms and the configuration issues detected on the entity.
The status is reported as one of the following values:
- red: The entity has alarms or configuration issues with a red status.
- yellow: The entity does not have alarms or configuration issues with a
  red status, and has at least one with a yellow status.
- green: The entity does not have alarms or configuration issues with a
  red or yellow status, and has at least one with a green status.
- gray: All of the entity's alarms have a gray status and the
  configuration status of the entity is not being monitored.
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"Name of this entity, unique relative to its parent.

Any / (slash), \\ (backslash), character used in this
name element will be escaped. Similarly, any % (percent) character used in
this name element will be escaped, unless it is used to start an escape
sequence. A slash is escaped as %2F or %2f. A backslash is escaped as %5C or
%5c, and a percent is escaped as %25.

***Required privileges:*** System.View
"#####) },),
        ("recent_task", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "recentTask", doc: Some(r#####"The set of recent tasks operating on this managed entity.

This is a subset
of *TaskManager.recentTask* belong to this entity. A task in this
list could be in one of the four states: pending, running, success or error.

This property can be used to deduce intermediate power states for
a virtual machine entity. For example, if the current powerState is "poweredOn"
and there is a running task performing the "suspend" operation, then the virtual
machine's intermediate state might be described as "suspending."

Most tasks (such as power operations) obtain exclusive access to the virtual
machine, so it is unusual for this list to contain more than one running task.
One exception, however, is the task of cloning a virtual machine.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("available_field", NodeData { type_decl: "Vec<vim_rs::types::structs::CustomFieldDef>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldDef"), path_segment: "availableField", doc: Some(r#####"List of custom field definitions that are valid for the object's type.

The fields are sorted by *CustomFieldDef.name*.

***Required privileges:*** System.View
"#####) },),
        ("uuid", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "uuid", doc: Some(r#####"Generated UUID of the switch.

Unique across vCenter Server
inventory and instances.
"#####) },),
        ("parent", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "parent", doc: Some(r#####"Parent of this entity.

This value is null for the root object and for
*VirtualMachine* objects that are part of
a *VirtualApp*.

***Required privileges:*** System.View
"#####) },),
        ("alarm_actions_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "alarmActionsEnabled", doc: Some(r#####"Whether alarm actions are enabled for this entity.

True if enabled; false otherwise.

***Required privileges:*** System.Read
"#####) },),
    ],
}),
        ("HostIpConfigIpV6AddressConfiguration", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dhcp_v_6_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "dhcpV6Enabled", doc: Some(r#####"The flag to indicate whether or not DHCP (dynamic host
control protocol) is enabled to obtain an ipV6 address.

If this property is set to true, an ipV6 address is configured through dhcpV6.
"#####) },),
        ("ip_v_6_address", NodeData { type_decl: "Vec<vim_rs::types::structs::HostIpConfigIpV6Address>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostIpConfigIpV6Address"), path_segment: "ipV6Address", doc: Some(r#####"Ipv6 adrresses configured on the interface.

The global addresses can be configured
through DHCP, stateless or manual configuration. Link local addresses can be
only configured with the origin set to
*other*.
"#####) },),
        ("auto_configuration_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "autoConfigurationEnabled", doc: Some(r#####"Specify if IPv6 address and routing information information
be enabled or not as per RFC 2462.
"#####) },),
    ],
}),
        ("HostDateTimeConfig", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
        (0, 6),
    ],
    entries: &[
        ("disable_events", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "disableEvents", doc: Some(r#####"When Network Time service or Precision Time service are enabled
any detecteced failures will result in Events being sent to
Virtual Center.

Use this setting to disable Time Events.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "enabled", doc: Some(r#####"Bring Time services subsystem up or down.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("reset_to_factory_defaults", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "resetToFactoryDefaults", doc: Some(r#####"When this property is present and set true the existing
configuration for Time Services will be reset to factory default.

The protocol property when set defines the scope of what is reset.
If additional configuration beyond protocol is provided host
will first perform factory reset followed by applying any configuration
present.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("time_zone", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "timeZone", doc: Some(r#####"The time zone of the host.

Must be one of the values of
*HostDateTimeSystemTimeZone.key*
"#####) },),
        ("disable_fallback", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "disableFallback", doc: Some(r#####"When in PrecisionTimeSync, NTP configuration as set will be
running as backup.

Use this setting to prevent NTP from becoming
the primary time protocol in the event of a PTP service failure.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("ntp_config", NodeData { type_decl: "vim_rs::types::structs::HostNtpConfig", type_name: "HostNtpConfig", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "ntpConfig", doc: Some(r#####"The NTP configuration on the host.
"#####) },),
        ("ptp_config", NodeData { type_decl: "vim_rs::types::structs::HostPtpConfig", type_name: "HostPtpConfig", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "ptpConfig", doc: Some(r#####"The PTP configuration on the host.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("protocol", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "protocol", doc: Some(r#####"Specify which network time configuration to discipline vmkernel clock.

See *HostDateTimeInfoProtocol_enum* for supported values.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
    ],
}),
        ("DvsFilterPolicy", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("inherited", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "inherited", doc: Some(r#####"Whether the configuration is set to inherited value.
"#####) },),
        ("filter_config", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::DvsFilterConfigTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfDvsFilterConfig"), path_segment: "filterConfig", doc: Some(r#####"List of Network Filter Configurations.

In an update operation, the array can contain all
*DvsTrafficFilterConfigSpec* objects
or all *DvsFilterConfig* and
*DvsTrafficFilterConfig*
object, but not mixed of Config and Spec objects. If array of
*DvsFilterConfigSpec* and *DvsTrafficFilterConfigSpec* is used
for updating Network Filter then only the Network Filters
matching *DistributedVirtualPort.key* /
*DistributedVirtualPort.key*
is updated.
If array of *DvsFilterConfig* and
*DvsTrafficFilterConfig*
is used for updating port settings, the Network Filter
settings will be overridden with the new array specified. The
specified array should only contain *DvsFilterConfig* and
*DvsTrafficFilterConfig* objects with *InheritablePolicy.inherited* /
*InheritablePolicy.inherited* set to false.
*DvsFilterConfig*/*DvsTrafficFilterConfig* objects with
*InheritablePolicy.inherited*/*InheritablePolicy.inherited* as
true in the specified array will be ignored. The updated result will
include *DvsFilterConfig*/*DvsTrafficFilterConfig* objects
inherited from parent, if any.
"#####) },),
    ],
}),
        ("DvsResourceRuntimeInfo", ::phf::Map {
    key: 14108922650502679131,
    disps: &[
        (3, 0),
    ],
    entries: &[
        ("allocated_resource", NodeData { type_decl: "Vec<vim_rs::types::structs::DvsVnicAllocatedResource>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfDvsVnicAllocatedResource"), path_segment: "allocatedResource", doc: Some(r#####"The reservation taken by *VirtualEthernetCard* of which the
backing is not associdated with any *DVSVmVnicNetworkResourcePool*
"#####) },),
        ("available", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "available", doc: Some(r#####"Available: Current available resource for reservation (capacity - usage).

Units in Mbits/s.
"#####) },),
        ("vm_vnic_network_resource_pool_runtime", NodeData { type_decl: "Vec<vim_rs::types::structs::DvsVmVnicNetworkResourcePoolRuntimeInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfDvsVmVnicNetworkResourcePoolRuntimeInfo"), path_segment: "vmVnicNetworkResourcePoolRuntime", doc: Some(r#####"The runtime information of *DVSVmVnicNetworkResourcePool*.
"#####) },),
        ("capacity", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "capacity", doc: Some(r#####"Capacity: Total Reservation allocated for Virtual Machine
Traffic for this switch.

Units in Mbits/s.
"#####) },),
        ("usage", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "usage", doc: Some(r#####"usage: Current total usage.

This is the sum of all reservations
allocated to *DVSVmVnicNetworkResourcePool* on this switch and the
sum of reservation taken by *VirtualEthernetCard* whose
backing is not associdated with any *DVSVmVnicNetworkResourcePool*.
Units in Mbits/s.
"#####) },),
    ],
}),
        ("StringPolicy", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("inherited", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "inherited", doc: Some(r#####"Whether the configuration is set to inherited value.
"#####) },),
        ("value", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "value", doc: Some(r#####"The String value that is either set or inherited.
"#####) },),
    ],
}),
        ("DVSFeatureCapability", ::phf::Map {
    key: 12213676231523076107,
    disps: &[
        (1, 0),
        (3, 0),
    ],
    entries: &[
        ("backup_restore_capability", NodeData { type_decl: "vim_rs::types::structs::DvsBackupRestoreCapability", type_name: "DVSBackupRestoreCapability", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "backupRestoreCapability", doc: Some(r#####"Backup, restore, and rollback capabilities.

Backup and restore
are supported only for *VmwareDistributedVirtualSwitch*.
Rollback is supported for *VmwareDistributedVirtualSwitch*
and *DistributedVirtualPortgroup*.
For information about backup and restore, see the
*DistributedVirtualSwitchManager* methods
*DistributedVirtualSwitchManager.DVSManagerExportEntity_Task* and
*DistributedVirtualSwitchManager.DVSManagerImportEntity_Task*.
For information about rollback, see the
*DistributedVirtualSwitch*.*DistributedVirtualSwitch.DVSRollback_Task*
and *DistributedVirtualPortgroup*.*DistributedVirtualPortgroup.DVPortgroupRollback_Task*
methods.
"#####) },),
        ("network_filter_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "networkFilterSupported", doc: Some(r#####"Indicates whether Network Filter feature is
supported in vSphere Distributed Switch.
"#####) },),
        ("network_resource_management_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "networkResourceManagementSupported", doc: Some(r#####"Deprecated as of vSphere API 5.0, use
<code>networkResourceManagementCapability</code>.*DVSNetworkResourceManagementCapability.networkResourceManagementSupported*.

Indicates whether network I/O control is
supported on the vSphere Distributed Switch.
"#####) },),
        ("health_check_capability", NodeData { type_decl: "Box<dyn vim_rs::types::traits::DvsHealthCheckCapabilityTrait>", type_name: "DVSHealthCheckCapability", is_optional: true, processing_type: FieldProcessingType::Trait, path_segment: "healthCheckCapability", doc: Some(r#####"Health check capabilities supported by a *VmwareDistributedVirtualSwitch*.
"#####) },),
        ("network_resource_pool_high_share_value", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "networkResourcePoolHighShareValue", doc: Some(r#####"Deprecated as of vSphere API 5.0, use
<code>networkResourceManagementCapability</code>.*DVSNetworkResourceManagementCapability.networkResourcePoolHighShareValue*.

This is the value for *high*
in *DVSNetworkResourcePoolAllocationInfo.shares*.

This
implicitly defines the legal range of share values to be between 1 and this.
This also defines values for other level types, such as
*normal* being one half of this value and
*low* being one fourth of this value.
"#####) },),
        ("vm_direct_path_gen_2_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vmDirectPathGen2Supported", doc: Some(r#####"Deprecated as of vSphere API 8.0. VMDirectPath Gen 2 is no longer supported and
there is no replacement.

Indicates whether VMDirectPath Gen 2 is supported on the
distributed virtual switch.

See
*HostCapability*.*HostCapability.vmDirectPathGen2Supported*
and *PhysicalNic*.*PhysicalNic.vmDirectPathGen2Supported*.

For a third-party distributed switch implementation, you can
specify this property during switch creation or when you call the
*DistributedVirtualSwitch.UpdateDvsCapability* method.

VMDirectPath Gen 2 is supported in
vSphere Distributed Switch Version 4.1 or later.
"#####) },),
        ("nic_teaming_policy", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "nicTeamingPolicy", doc: Some(r#####"The available teaming modes for the vSphere Distributed Switch.

The
value can be one or more of
*DistributedVirtualSwitchNicTeamingPolicyMode_enum*.
"#####) },),
        ("rollback_capability", NodeData { type_decl: "vim_rs::types::structs::DvsRollbackCapability", type_name: "DVSRollbackCapability", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "rollbackCapability", doc: Some(r#####"Host rollback capability.

If <code>rollbackCapability</code>.*DVSRollbackCapability.rollbackSupported*
is true, network operations that disconnect the the host are rolled back.
"#####) },),
        ("mac_learning_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "macLearningSupported", doc: Some(r#####"Indicates whether MAC learning feature is
supported in vSphere Distributed Switch.
"#####) },),
        ("network_resource_management_capability", NodeData { type_decl: "vim_rs::types::structs::DvsNetworkResourceManagementCapability", type_name: "DVSNetworkResourceManagementCapability", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "networkResourceManagementCapability", doc: Some(r#####"Network resource management capabilities supported by a
distributed virtual switch.
"#####) },),
    ],
}),
        ("ApplyHostProfileConfigurationResult", ::phf::Map {
    key: 12213676231523076107,
    disps: &[
        (4, 0),
    ],
    entries: &[
        ("errors", NodeData { type_decl: "Vec<vim_rs::types::structs::MethodFault>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfMethodFault"), path_segment: "errors", doc: Some(r#####"If <code>status</code> is <code>fail</code>, this property contains
a list of status error message objects.
"#####) },),
        ("start_time", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveDateTime"), path_segment: "startTime", doc: Some(r#####"Time that the host config apply starts.
"#####) },),
        ("complete_time", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveDateTime"), path_segment: "completeTime", doc: Some(r#####"Time that the host config apply completes.
"#####) },),
        ("status", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "status", doc: Some(r#####"Status of the remediation.

See
*ApplyHostProfileConfigurationResultStatus_enum*
for valid values.
"#####) },),
        ("host", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "host", doc: Some(r#####"Host to be remediated.

Refers instance of *HostSystem*.
"#####) },),
    ],
}),
        ("VirtualMachineFileLayoutEx", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("file", NodeData { type_decl: "Vec<vim_rs::types::structs::VirtualMachineFileLayoutExFileInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVirtualMachineFileLayoutExFileInfo"), path_segment: "file", doc: Some(r#####"Information about all the files that constitute the virtual machine
including configuration files, disks, swap file, suspend file, log files,
core files, memory file etc.

*VirtualMachineFileLayoutExFileType_enum* lists the
different file-types that make a virtual machine.
"#####) },),
        ("snapshot", NodeData { type_decl: "Vec<vim_rs::types::structs::VirtualMachineFileLayoutExSnapshotLayout>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVirtualMachineFileLayoutExSnapshotLayout"), path_segment: "snapshot", doc: Some(r#####"Layout of each snapshot of the virtual machine.
"#####) },),
        ("disk", NodeData { type_decl: "Vec<vim_rs::types::structs::VirtualMachineFileLayoutExDiskLayout>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVirtualMachineFileLayoutExDiskLayout"), path_segment: "disk", doc: Some(r#####"Layout of each virtual disk attached to the virtual machine.

For a virtual machine with snaphots, this property gives only those disks
that are attached to it at the current point of running.
"#####) },),
        ("timestamp", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveDateTime"), path_segment: "timestamp", doc: Some(r#####"Time when values in this structure were last updated.
"#####) },),
    ],
}),
        ("HostSystemHealthInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("numeric_sensor_info", NodeData { type_decl: "Vec<vim_rs::types::structs::HostNumericSensorInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostNumericSensorInfo"), path_segment: "numericSensorInfo", doc: Some(r#####"Health information provided by the power probes.
"#####) },),
    ],
}),
        ("HostNetworkConfig", ::phf::Map {
    key: 7485420634051515786,
    disps: &[
        (4, 2),
        (1, 5),
        (0, 0),
    ],
    entries: &[
        ("dhcp", NodeData { type_decl: "Vec<vim_rs::types::structs::HostDhcpServiceConfig>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostDhcpServiceConfig"), path_segment: "dhcp", doc: Some(r#####"Dynamic Host Control Protocol (DHCP) Service instances configured
on the host.
"#####) },),
        ("net_stack_spec", NodeData { type_decl: "Vec<vim_rs::types::structs::HostNetworkConfigNetStackSpec>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostNetworkConfigNetStackSpec"), path_segment: "netStackSpec", doc: Some(r#####"The list of network stack instance spec
"#####) },),
        ("dns_config", NodeData { type_decl: "Box<dyn vim_rs::types::traits::HostDnsConfigTrait>", type_name: "HostDnsConfig", is_optional: true, processing_type: FieldProcessingType::Trait, path_segment: "dnsConfig", doc: Some(r#####"Deprecated as of vSphere API 5.5, which is moved to
the default NetStackInstance.

Client-side DNS configuration for the host.

The DNS configuration is
global to the entire host.
"#####) },),
        ("proxy_switch", NodeData { type_decl: "Vec<vim_rs::types::structs::HostProxySwitchConfig>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostProxySwitchConfig"), path_segment: "proxySwitch", doc: Some(r#####"Host proxy switches configured on the host.
"#####) },),
        ("migration_status", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "migrationStatus", doc: Some(r#####"Current status of NVDS to VDS migration.

See *HostNetworkConfig*.*HostNetworkConfigMigrationStatus_enum*
for supported values.

***Since:*** vSphere API Release 7.0.2.0
"#####) },),
        ("console_ip_route_config", NodeData { type_decl: "Box<dyn vim_rs::types::traits::HostIpRouteConfigTrait>", type_name: "HostIpRouteConfig", is_optional: true, processing_type: FieldProcessingType::Trait, path_segment: "consoleIpRouteConfig", doc: Some(r#####"IP route configuration of the service console.
"#####) },),
        ("vswitch", NodeData { type_decl: "Vec<vim_rs::types::structs::HostVirtualSwitchConfig>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostVirtualSwitchConfig"), path_segment: "vswitch", doc: Some(r#####"Virtual switches configured on the host.
"#####) },),
        ("pnic", NodeData { type_decl: "Vec<vim_rs::types::structs::PhysicalNicConfig>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfPhysicalNicConfig"), path_segment: "pnic", doc: Some(r#####"Physical network adapters as seen by the primary operating system.
"#####) },),
        ("ip_v_6_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "ipV6Enabled", doc: Some(r#####"Enable or disable IPv6 protocol on this system.

This property must be set by itself, no other property can accompany
this change. Following the successful change, the system should be rebooted to
have the change take effect.
"#####) },),
        ("nat", NodeData { type_decl: "Vec<vim_rs::types::structs::HostNatServiceConfig>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostNatServiceConfig"), path_segment: "nat", doc: Some(r#####"Network address translation (NAT) Service instances configured
on the host.
"#####) },),
        ("route_table_config", NodeData { type_decl: "vim_rs::types::structs::HostIpRouteTableConfig", type_name: "HostIpRouteTableConfig", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "routeTableConfig", doc: Some(r#####"Deprecated as of vSphere API 5.5, which is moved to
the default NetStackInstance.

IP routing table configuration of the host.
"#####) },),
        ("portgroup", NodeData { type_decl: "Vec<vim_rs::types::structs::HostPortGroupConfig>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostPortGroupConfig"), path_segment: "portgroup", doc: Some(r#####"Port groups configured on the host.
"#####) },),
        ("ip_route_config", NodeData { type_decl: "Box<dyn vim_rs::types::traits::HostIpRouteConfigTrait>", type_name: "HostIpRouteConfig", is_optional: true, processing_type: FieldProcessingType::Trait, path_segment: "ipRouteConfig", doc: Some(r#####"Deprecated as of vSphere API 5.5, which is moved to
the default NetStackInstance.

IP route configuration of the host.
"#####) },),
        ("vnic", NodeData { type_decl: "Vec<vim_rs::types::structs::HostVirtualNicConfig>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostVirtualNicConfig"), path_segment: "vnic", doc: Some(r#####"Virtual network adapters configured for use by the host
operating system network adapter.
"#####) },),
        ("console_vnic", NodeData { type_decl: "Vec<vim_rs::types::structs::HostVirtualNicConfig>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostVirtualNicConfig"), path_segment: "consoleVnic", doc: Some(r#####"Virtual network adapters configured for use by the Service
Console.
"#####) },),
    ],
}),
        ("VmwareDistributedVirtualSwitch", ::phf::Map {
    key: 12213676231523076107,
    disps: &[
        (1, 0),
        (2, 1),
        (5, 0),
        (0, 19),
        (15, 22),
    ],
    entries: &[
        ("custom_value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "customValue", doc: Some(r#####"Custom field values.

***Required privileges:*** System.View
"#####) },),
        ("triggered_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "triggeredAlarmState", doc: Some(r#####"A set of alarm states for alarms triggered by this entity
or by its descendants.

Triggered alarms are propagated up the inventory hierarchy
so that a user can readily tell when a descendant has triggered an alarm.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.

***Required privileges:*** System.View
"#####) },),
        ("disabled_method", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "disabledMethod", doc: Some(r#####"List of operations that are disabled, given the current runtime
state of the entity.

For example, a power-on operation always fails if a
virtual machine is already powered on. This list can be used by clients to
enable or disable operations in a graphical user interface.

Note: This list is determined by the current runtime state of an entity,
not by its permissions.

This list may include the following operations for a HostSystem:
- *HostSystem.EnterMaintenanceMode_Task*
- *HostSystem.ExitMaintenanceMode_Task*
- *HostSystem.RebootHost_Task*
- *HostSystem.ShutdownHost_Task*
- *HostSystem.ReconnectHost_Task*
- *HostSystem.DisconnectHost_Task*
  
This list may include the following operations for a VirtualMachine:
- *VirtualMachine.AnswerVM*
- *ManagedEntity.Rename_Task*
- *VirtualMachine.CloneVM_Task*
- *VirtualMachine.PowerOffVM_Task*
- *VirtualMachine.PowerOnVM_Task*
- *VirtualMachine.SuspendVM_Task*
- *VirtualMachine.ResetVM_Task*
- *VirtualMachine.ReconfigVM_Task*
- *VirtualMachine.RelocateVM_Task*
- *VirtualMachine.MigrateVM_Task*
- *VirtualMachine.CustomizeVM_Task*
- *VirtualMachine.ShutdownGuest*
- *VirtualMachine.StandbyGuest*
- *VirtualMachine.RebootGuest*
- *VirtualMachine.CreateSnapshot_Task*
- *VirtualMachine.RemoveAllSnapshots_Task*
- *VirtualMachine.RevertToCurrentSnapshot_Task*
- *VirtualMachine.MarkAsTemplate*
- *VirtualMachine.MarkAsVirtualMachine*
- *VirtualMachine.ResetGuestInformation*
- *VirtualMachine.MountToolsInstaller*
- *VirtualMachine.UnmountToolsInstaller*
- *ManagedEntity.Destroy_Task*
- *VirtualMachine.UpgradeVM_Task*
- *VirtualMachine.ExportVm*
  
This list may include the following operations for a ResourcePool:
- *ResourcePool.ImportVApp*
- *ResourcePool.CreateChildVM_Task*
- *ResourcePool.UpdateConfig*
- *Folder.CreateVM_Task*
- *ManagedEntity.Destroy_Task*
- *ManagedEntity.Rename_Task*
  
This list may include the following operations for a VirtualApp:
- *ManagedEntity.Destroy_Task*
- *VirtualApp.CloneVApp_Task*
- *VirtualApp.unregisterVApp_Task*
- *VirtualApp.ExportVApp*
- *VirtualApp.PowerOnVApp_Task*
- *VirtualApp.PowerOffVApp_Task*
- *VirtualApp.UpdateVAppConfig*
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("permission", NodeData { type_decl: "Vec<vim_rs::types::structs::Permission>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfPermission"), path_segment: "permission", doc: Some(r#####"List of permissions defined for this entity.
"#####) },),
        ("network_resource_pool", NodeData { type_decl: "Vec<vim_rs::types::structs::DvsNetworkResourcePool>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfDVSNetworkResourcePool"), path_segment: "networkResourcePool", doc: Some(r#####"Deprecated as of vSphere API 6.0
Use *DVSConfigInfo.vmVnicNetworkResourcePool*
to get the Virtual NIC resource pool information.
Use *DVSConfigInfo.infrastructureTrafficResourceConfig*
to get the host infrastructure resource information.

Network resource pool information for the switch.
"#####) },),
        ("capability", NodeData { type_decl: "vim_rs::types::structs::DvsCapability", type_name: "DVSCapability", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "capability", doc: Some(r#####"Capability of the switch.

Capabilities are indicated at the port,
portgroup and switch levels, and for version-specific features.
When you retrieve this property from an ESXi host,
*DistributedVirtualSwitch.capability*.*DVSCapability.dvsOperationSupported*
should always be set to false.
"#####) },),
        ("tag", NodeData { type_decl: "Vec<vim_rs::types::structs::Tag>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfTag"), path_segment: "tag", doc: Some(r#####"The set of tags associated with this managed entity.

Experimental. Subject to change.

***Required privileges:*** System.View
"#####) },),
        ("runtime", NodeData { type_decl: "vim_rs::types::structs::DvsRuntimeInfo", type_name: "DVSRuntimeInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "runtime", doc: Some(r#####"Runtime information of the distributed virtual switch.
"#####) },),
        ("effective_role", NodeData { type_decl: "Vec<i32>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfInt"), path_segment: "effectiveRole", doc: Some(r#####"Access rights the current session has to this entity.

***Required privileges:*** System.View
"#####) },),
        ("summary", NodeData { type_decl: "vim_rs::types::structs::DvsSummary", type_name: "DVSSummary", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "summary", doc: Some(r#####"Summary of the switch.
"#####) },),
        ("portgroup", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "portgroup", doc: Some(r#####"Portgroups that are defined on the switch.
"#####) },),
        ("config_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "configStatus", doc: Some(r#####"The configStatus indicates whether or not the system has detected a configuration
issue involving this entity.

For example, it might have detected a
duplicate IP address or MAC address, or a host in a cluster
might be out of compliance. The meanings of the configStatus values are:
- red: A problem has been detected involving the entity.
- yellow: A problem is about to occur or a transient condition
  has occurred (For example, reconfigure fail-over policy).
- green: No configuration issues have been detected.
- gray: The configuration status of the entity is not being monitored.
  
A green status indicates only that a problem has not been detected;
it is not a guarantee that the entity is problem-free.

The *configIssue* property contains a list of the
problems that have been detected.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("config", NodeData { type_decl: "Box<dyn vim_rs::types::traits::DvsConfigInfoTrait>", type_name: "DVSConfigInfo", is_optional: false, processing_type: FieldProcessingType::Trait, path_segment: "config", doc: Some(r#####"Switch configuration data.
"#####) },),
        ("value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "value", doc: Some(r#####"List of custom field values.

Each value uses a key to associate
an instance of a *CustomFieldStringValue* with
a custom field definition.

***Required privileges:*** System.View
"#####) },),
        ("config_issue", NodeData { type_decl: "Vec<vim_rs::types::structs::Event>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfEvent"), path_segment: "configIssue", doc: Some(r#####"Current configuration issues that have been detected for this entity.

Typically,
these issues have already been logged as events. The entity stores these
events as long as they are still current. The
*configStatus* property provides an overall status
based on these events.
"#####) },),
        ("declared_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "declaredAlarmState", doc: Some(r#####"A set of alarm states for alarms that apply to this managed entity.

The set includes alarms defined on this entity
and alarms inherited from the parent entity,
or from any ancestors in the inventory hierarchy.

Alarms are inherited if they can be triggered by this entity or its descendants.
This set does not include alarms that are defined on descendants of this entity.

***Required privileges:*** System.View
"#####) },),
        ("overall_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "overallStatus", doc: Some(r#####"General health of this managed entity.

The overall status of the managed entity is computed as the worst status
among its alarms and the configuration issues detected on the entity.
The status is reported as one of the following values:
- red: The entity has alarms or configuration issues with a red status.
- yellow: The entity does not have alarms or configuration issues with a
  red status, and has at least one with a yellow status.
- green: The entity does not have alarms or configuration issues with a
  red or yellow status, and has at least one with a green status.
- gray: All of the entity's alarms have a gray status and the
  configuration status of the entity is not being monitored.
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"Name of this entity, unique relative to its parent.

Any / (slash), \\ (backslash), character used in this
name element will be escaped. Similarly, any % (percent) character used in
this name element will be escaped, unless it is used to start an escape
sequence. A slash is escaped as %2F or %2f. A backslash is escaped as %5C or
%5c, and a percent is escaped as %25.

***Required privileges:*** System.View
"#####) },),
        ("recent_task", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "recentTask", doc: Some(r#####"The set of recent tasks operating on this managed entity.

This is a subset
of *TaskManager.recentTask* belong to this entity. A task in this
list could be in one of the four states: pending, running, success or error.

This property can be used to deduce intermediate power states for
a virtual machine entity. For example, if the current powerState is "poweredOn"
and there is a running task performing the "suspend" operation, then the virtual
machine's intermediate state might be described as "suspending."

Most tasks (such as power operations) obtain exclusive access to the virtual
machine, so it is unusual for this list to contain more than one running task.
One exception, however, is the task of cloning a virtual machine.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("available_field", NodeData { type_decl: "Vec<vim_rs::types::structs::CustomFieldDef>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldDef"), path_segment: "availableField", doc: Some(r#####"List of custom field definitions that are valid for the object's type.

The fields are sorted by *CustomFieldDef.name*.

***Required privileges:*** System.View
"#####) },),
        ("uuid", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "uuid", doc: Some(r#####"Generated UUID of the switch.

Unique across vCenter Server
inventory and instances.
"#####) },),
        ("parent", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "parent", doc: Some(r#####"Parent of this entity.

This value is null for the root object and for
*VirtualMachine* objects that are part of
a *VirtualApp*.

***Required privileges:*** System.View
"#####) },),
        ("alarm_actions_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "alarmActionsEnabled", doc: Some(r#####"Whether alarm actions are enabled for this entity.

True if enabled; false otherwise.

***Required privileges:*** System.Read
"#####) },),
    ],
}),
        ("GuestInfo", ::phf::Map {
    key: 12213676231523076107,
    disps: &[
        (1, 5),
        (16, 23),
        (0, 6),
        (0, 4),
        (0, 0),
        (0, 0),
    ],
    entries: &[
        ("tools_version", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "toolsVersion", doc: Some(r#####"Current version of VMware Tools, if known.
"#####) },),
        ("app_state", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "appState", doc: Some(r#####"Application state.

If vSphere HA is enabled and the vm is configured for Application Monitoring
and this field's value is "appStateNeedReset" then HA will attempt immediately reset
the vm.
There are some system conditions which may delay the immediate reset. The immediate
reset will be performed as soon as allowed by vSphere HA and ESX. If during these
conditions the value is changed to appStateOk the reset will be cancelled.

See also *GuestInfoAppStateType_enum*.
"#####) },),
        ("generation_info", NodeData { type_decl: "Vec<vim_rs::types::structs::GuestInfoNamespaceGenerationInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfGuestInfoNamespaceGenerationInfo"), path_segment: "generationInfo", doc: Some(r#####"A list of namespaces and their corresponding generation numbers.

Only namespaces with non-zero
*VirtualMachineNamespaceManagerCreateSpec.maxSizeEventsFromGuest*
are guaranteed to be present here.
Use *VirtualMachineNamespaceManager.ListNamespaces* to retrieve list of
namespaces.
"#####) },),
        ("hw_version", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "hwVersion", doc: Some(r#####"The hardware version string for this virtual machine.
"#####) },),
        ("customization_info", NodeData { type_decl: "vim_rs::types::structs::GuestInfoCustomizationInfo", type_name: "GuestInfoCustomizationInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "customizationInfo", doc: Some(r#####"Guest OS Customization status info.

***Since:*** vSphere API Release 7.0.2.0
"#####) },),
        ("guest_state", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "guestState", doc: Some(r#####"Operation mode of guest operating system.

One of:
- "running" - Guest is running normally.
- "shuttingdown" - Guest has a pending shutdown command.
- "resetting" - Guest has a pending reset command.
- "standby" - Guest has a pending standby command.
- "notrunning" - Guest is not running.
- "unknown" - Guest information is not available.
"#####) },),
        ("ip_stack", NodeData { type_decl: "Vec<vim_rs::types::structs::GuestStackInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfGuestStackInfo"), path_segment: "ipStack", doc: Some(r#####"Guest information about IP networking stack, if known.
"#####) },),
        ("tools_running_status", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "toolsRunningStatus", doc: Some(r#####"Current running status of VMware Tools in the guest operating system,
if known.

The set of possible values is described in
*VirtualMachineToolsRunningStatus_enum*
"#####) },),
        ("screen", NodeData { type_decl: "vim_rs::types::structs::GuestScreenInfo", type_name: "GuestScreenInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "screen", doc: Some(r#####"Guest screen resolution info, if known.
"#####) },),
        ("tools_status", NodeData { type_decl: "vim_rs::types::enums::VirtualMachineToolsStatusEnum", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("VirtualMachineToolsStatus"), path_segment: "toolsStatus", doc: Some(r#####"Deprecated as of vSphere API 4.0 use *GuestInfo.toolsVersionStatus* and
*GuestInfo.toolsRunningStatus*.

Current status of VMware Tools in the guest operating system, if known.
"#####) },),
        ("net", NodeData { type_decl: "Vec<vim_rs::types::structs::GuestNicInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfGuestNicInfo"), path_segment: "net", doc: Some(r#####"Guest information about network adapters, if known.
"#####) },),
        ("guest_id", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "guestId", doc: Some(r#####"Guest operating system identifier (short name), if known.
"#####) },),
        ("guest_detailed_data", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "guestDetailedData", doc: Some(r#####"Guest OS Detailed data.

The guest detailed data string is a property list (space separated,
name='value' pairs where the value is embedded in single quotes) of
metadata provided by the guest OS when sufficiently recent tools are
installed. The fields supplied will vary between distributions and
distribution versions. The order of these fields is not guaranteed.

The guest detailed data string is not available before a virtual machine
is first powered on. The guest must first be booted and a supported
version of tools must be run to record the data (approximately 30-60
seconds after booting). Once the guest detailed data string has been
recorded, it will be available whether the virtual machine is powered off
or on.

Available fields:
<table summary="Available fields">
<thead>
<tr>
<th>Name</th>
<th>Description</th>
<th>Tools version first available</th>
</tr>
</thead>
<tbody>
<tr>
<td>architecture</td>
<td>'Arm' or 'X86'</td>
<td>11.2.0</td>
</tr>
<tr>
<td>bitness</td>
<td>'32' or '64'</td>
<td>11.2.0</td>
</tr>
<tr>
<td>buildNumber</td>
<td>OS build number</td>
<td>11.2.0</td>
</tr>
<tr>
<td>cpeString</td>
<td>NIST Common Platform Enumeration Specification string, a standardized identifier for the OS</td>
<td>12.2.0</td>
</tr>
<tr>
<td>distroAddlVersion</td>
<td>Longer OS version string that may contain additional info (e.g. version name)</td>
<td>12.2.0</td>
</tr>
<tr>
<td>distroName</td>
<td>OS distribution name</td>
<td>11.2.0</td>
</tr>
<tr>
<td>distroVersion</td>
<td>OS version string</td>
<td>11.2.0</td>
</tr>
<tr>
<td>familyName</td>
<td>OS family name (Windows, Linux, etc.)</td>
<td>11.2.0</td>
</tr>
<tr>
<td>kernelVersion</td>
<td>Linux kernel version, Windows 10+ patch number, or Windows build number</td>
<td>11.2.0</td>
</tr>
<tr>
<td>prettyName</td>
<td>Officially specified distro "pretty name"</td>
<td>11.2.0</td>
</tr>
</tbody>
</table>

***Since:*** vSphere API Release 8.0.2.0
"#####) },),
        ("tools_version_status", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "toolsVersionStatus", doc: Some(r#####"Deprecated as of vSphere API 5.1 use *GuestInfo.toolsVersionStatus2*.

Current version status of VMware Tools in the guest operating system,
if known.

The set of possible values is described in
*VirtualMachineToolsVersionStatus_enum* for vSphere API 5.0.
"#####) },),
        ("tools_install_type", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "toolsInstallType", doc: Some(r#####"Current installation type of VMware Tools in the guest operating system.

The set of possible values is described in
*VirtualMachineToolsInstallType_enum*
"#####) },),
        ("guest_operations_ready", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "guestOperationsReady", doc: Some(r#####"Guest Operations availability.

If true, the virtual machine is ready to process guest operations.
"#####) },),
        ("tools_version_status_2", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "toolsVersionStatus2", doc: Some(r#####"Current version status of VMware Tools in the guest operating system,
if known.

The set of possible values is described in
*VirtualMachineToolsVersionStatus_enum*
"#####) },),
        ("host_name", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "hostName", doc: Some(r#####"Hostname of the guest operating system, if known.
"#####) },),
        ("app_heartbeat_status", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "appHeartbeatStatus", doc: Some(r#####"Application heartbeat status.

Please see *VirtualMachineAppHeartbeatStatusType_enum*
"#####) },),
        ("guest_family", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "guestFamily", doc: Some(r#####"Guest operating system family, if known.
"#####) },),
        ("guest_full_name", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "guestFullName", doc: Some(r#####"Guest operating system full name, if known.
"#####) },),
        ("guest_state_change_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "guestStateChangeSupported", doc: Some(r#####"State change support.

If true, the virtual machine is ready to process soft power operations.
"#####) },),
        ("interactive_guest_operations_ready", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "interactiveGuestOperationsReady", doc: Some(r#####"Interactive Guest Operations availability.

If true, the virtual machine is ready to process guest operations
as the user interacting with the guest desktop.
"#####) },),
        ("ip_address", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "ipAddress", doc: Some(r#####"Primary IP address assigned to the guest operating system, if known.
"#####) },),
        ("disk", NodeData { type_decl: "Vec<vim_rs::types::structs::GuestDiskInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfGuestDiskInfo"), path_segment: "disk", doc: Some(r#####"Guest information about disks.

You can obtain Linux guest disk information for the following file system
types: Ext2, Ext3, Ext4, ReiserFS, XFS, Btrfs, NTFS, VFAT, UFS, PCFS, HFS,
and MS-DOS.

NOTE: Installing a more recent version of VMware Tools in the guest may help
obtain disk information for more file system types. Please refer the VMware Tools
User Guide for up-to-date supported file system types.
"#####) },),
        ("guest_kernel_crashed", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "guestKernelCrashed", doc: Some(r#####"Guest operating system's kernel crash state.

If true, the guest operating system's kernel has crashed.
"#####) },),
    ],
}),
        ("DistributedVirtualPortgroup", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (5, 3),
        (4, 13),
        (0, 0),
        (6, 19),
        (1, 21),
    ],
    entries: &[
        ("parent", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "parent", doc: Some(r#####"Parent of this entity.

This value is null for the root object and for
*VirtualMachine* objects that are part of
a *VirtualApp*.

***Required privileges:*** System.View
"#####) },),
        ("config", NodeData { type_decl: "vim_rs::types::structs::DvPortgroupConfigInfo", type_name: "DVPortgroupConfigInfo", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "config", doc: Some(r#####"Configuration of the portgroup.
"#####) },),
        ("config_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "configStatus", doc: Some(r#####"The configStatus indicates whether or not the system has detected a configuration
issue involving this entity.

For example, it might have detected a
duplicate IP address or MAC address, or a host in a cluster
might be out of compliance. The meanings of the configStatus values are:
- red: A problem has been detected involving the entity.
- yellow: A problem is about to occur or a transient condition
  has occurred (For example, reconfigure fail-over policy).
- green: No configuration issues have been detected.
- gray: The configuration status of the entity is not being monitored.
  
A green status indicates only that a problem has not been detected;
it is not a guarantee that the entity is problem-free.

The *configIssue* property contains a list of the
problems that have been detected.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"Name of this entity, unique relative to its parent.

Any / (slash), \\ (backslash), character used in this
name element will be escaped. Similarly, any % (percent) character used in
this name element will be escaped, unless it is used to start an escape
sequence. A slash is escaped as %2F or %2f. A backslash is escaped as %5C or
%5c, and a percent is escaped as %25.

***Required privileges:*** System.View
"#####) },),
        ("custom_value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "customValue", doc: Some(r#####"Custom field values.

***Required privileges:*** System.View
"#####) },),
        ("value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "value", doc: Some(r#####"List of custom field values.

Each value uses a key to associate
an instance of a *CustomFieldStringValue* with
a custom field definition.

***Required privileges:*** System.View
"#####) },),
        ("overall_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "overallStatus", doc: Some(r#####"General health of this managed entity.

The overall status of the managed entity is computed as the worst status
among its alarms and the configuration issues detected on the entity.
The status is reported as one of the following values:
- red: The entity has alarms or configuration issues with a red status.
- yellow: The entity does not have alarms or configuration issues with a
  red status, and has at least one with a yellow status.
- green: The entity does not have alarms or configuration issues with a
  red or yellow status, and has at least one with a green status.
- gray: All of the entity's alarms have a gray status and the
  configuration status of the entity is not being monitored.
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("host", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "host", doc: Some(r#####"Hosts attached to this network.
"#####) },),
        ("declared_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "declaredAlarmState", doc: Some(r#####"A set of alarm states for alarms that apply to this managed entity.

The set includes alarms defined on this entity
and alarms inherited from the parent entity,
or from any ancestors in the inventory hierarchy.

Alarms are inherited if they can be triggered by this entity or its descendants.
This set does not include alarms that are defined on descendants of this entity.

***Required privileges:*** System.View
"#####) },),
        ("permission", NodeData { type_decl: "Vec<vim_rs::types::structs::Permission>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfPermission"), path_segment: "permission", doc: Some(r#####"List of permissions defined for this entity.
"#####) },),
        ("alarm_actions_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "alarmActionsEnabled", doc: Some(r#####"Whether alarm actions are enabled for this entity.

True if enabled; false otherwise.

***Required privileges:*** System.Read
"#####) },),
        ("summary", NodeData { type_decl: "Box<dyn vim_rs::types::traits::NetworkSummaryTrait>", type_name: "NetworkSummary", is_optional: false, processing_type: FieldProcessingType::Trait, path_segment: "summary", doc: Some(r#####"Properties of a network.
"#####) },),
        ("key", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "key", doc: Some(r#####"Generated UUID of the portgroup.
"#####) },),
        ("port_keys", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "portKeys", doc: Some(r#####"Port keys for the set of ports in the portgroup.
"#####) },),
        ("triggered_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "triggeredAlarmState", doc: Some(r#####"A set of alarm states for alarms triggered by this entity
or by its descendants.

Triggered alarms are propagated up the inventory hierarchy
so that a user can readily tell when a descendant has triggered an alarm.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.

***Required privileges:*** System.View
"#####) },),
        ("recent_task", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "recentTask", doc: Some(r#####"The set of recent tasks operating on this managed entity.

This is a subset
of *TaskManager.recentTask* belong to this entity. A task in this
list could be in one of the four states: pending, running, success or error.

This property can be used to deduce intermediate power states for
a virtual machine entity. For example, if the current powerState is "poweredOn"
and there is a running task performing the "suspend" operation, then the virtual
machine's intermediate state might be described as "suspending."

Most tasks (such as power operations) obtain exclusive access to the virtual
machine, so it is unusual for this list to contain more than one running task.
One exception, however, is the task of cloning a virtual machine.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("available_field", NodeData { type_decl: "Vec<vim_rs::types::structs::CustomFieldDef>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldDef"), path_segment: "availableField", doc: Some(r#####"List of custom field definitions that are valid for the object's type.

The fields are sorted by *CustomFieldDef.name*.

***Required privileges:*** System.View
"#####) },),
        ("config_issue", NodeData { type_decl: "Vec<vim_rs::types::structs::Event>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfEvent"), path_segment: "configIssue", doc: Some(r#####"Current configuration issues that have been detected for this entity.

Typically,
these issues have already been logged as events. The entity stores these
events as long as they are still current. The
*configStatus* property provides an overall status
based on these events.
"#####) },),
        ("tag", NodeData { type_decl: "Vec<vim_rs::types::structs::Tag>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfTag"), path_segment: "tag", doc: Some(r#####"The set of tags associated with this managed entity.

Experimental. Subject to change.

***Required privileges:*** System.View
"#####) },),
        ("vm", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "vm", doc: Some(r#####"Virtual machines using this network.
"#####) },),
        ("effective_role", NodeData { type_decl: "Vec<i32>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfInt"), path_segment: "effectiveRole", doc: Some(r#####"Access rights the current session has to this entity.

***Required privileges:*** System.View
"#####) },),
        ("disabled_method", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "disabledMethod", doc: Some(r#####"List of operations that are disabled, given the current runtime
state of the entity.

For example, a power-on operation always fails if a
virtual machine is already powered on. This list can be used by clients to
enable or disable operations in a graphical user interface.

Note: This list is determined by the current runtime state of an entity,
not by its permissions.

This list may include the following operations for a HostSystem:
- *HostSystem.EnterMaintenanceMode_Task*
- *HostSystem.ExitMaintenanceMode_Task*
- *HostSystem.RebootHost_Task*
- *HostSystem.ShutdownHost_Task*
- *HostSystem.ReconnectHost_Task*
- *HostSystem.DisconnectHost_Task*
  
This list may include the following operations for a VirtualMachine:
- *VirtualMachine.AnswerVM*
- *ManagedEntity.Rename_Task*
- *VirtualMachine.CloneVM_Task*
- *VirtualMachine.PowerOffVM_Task*
- *VirtualMachine.PowerOnVM_Task*
- *VirtualMachine.SuspendVM_Task*
- *VirtualMachine.ResetVM_Task*
- *VirtualMachine.ReconfigVM_Task*
- *VirtualMachine.RelocateVM_Task*
- *VirtualMachine.MigrateVM_Task*
- *VirtualMachine.CustomizeVM_Task*
- *VirtualMachine.ShutdownGuest*
- *VirtualMachine.StandbyGuest*
- *VirtualMachine.RebootGuest*
- *VirtualMachine.CreateSnapshot_Task*
- *VirtualMachine.RemoveAllSnapshots_Task*
- *VirtualMachine.RevertToCurrentSnapshot_Task*
- *VirtualMachine.MarkAsTemplate*
- *VirtualMachine.MarkAsVirtualMachine*
- *VirtualMachine.ResetGuestInformation*
- *VirtualMachine.MountToolsInstaller*
- *VirtualMachine.UnmountToolsInstaller*
- *ManagedEntity.Destroy_Task*
- *VirtualMachine.UpgradeVM_Task*
- *VirtualMachine.ExportVm*
  
This list may include the following operations for a ResourcePool:
- *ResourcePool.ImportVApp*
- *ResourcePool.CreateChildVM_Task*
- *ResourcePool.UpdateConfig*
- *Folder.CreateVM_Task*
- *ManagedEntity.Destroy_Task*
- *ManagedEntity.Rename_Task*
  
This list may include the following operations for a VirtualApp:
- *ManagedEntity.Destroy_Task*
- *VirtualApp.CloneVApp_Task*
- *VirtualApp.unregisterVApp_Task*
- *VirtualApp.ExportVApp*
- *VirtualApp.PowerOnVApp_Task*
- *VirtualApp.PowerOffVApp_Task*
- *VirtualApp.UpdateVAppConfig*
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
    ],
}),
        ("HostDateTimeSystemTimeZone", ::phf::Map {
    key: 7485420634051515786,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("gmt_offset", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "gmtOffset", doc: Some(r#####"The GMT offset in seconds that is currently applicable to the timezone
(with respect to the current time on the host).
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"The time zone name.
"#####) },),
        ("description", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "description", doc: Some(r#####"Description of the time zone.
"#####) },),
        ("key", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "key", doc: Some(r#####"The identifier for the time zone.
"#####) },),
    ],
}),
        ("DVSNetworkResourceManagementCapability", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (0, 0),
        (4, 0),
    ],
    entries: &[
        ("network_resource_management_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "networkResourceManagementSupported", doc: Some(r#####"Indicates whether network I/O control is
supported on the vSphere Distributed Switch.

Network I/O control
is supported in vSphere Distributed Switch Version 4.1 or later.
"#####) },),
        ("user_defined_infra_traffic_pool_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "userDefinedInfraTrafficPoolSupported", doc: Some(r#####"Indicates whether user defined infrastructure traffic pool
supported in vSphere Distributed Switch.
"#####) },),
        ("user_defined_network_resource_pools_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "userDefinedNetworkResourcePoolsSupported", doc: Some(r#####"Indicates whether the switch supports creating user defined resource
pools.

This feature is supported in vSphere Distributed
Switch Version 5.0 or later.
"#####) },),
        ("qos_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "qosSupported", doc: Some(r#####"Indicates whether Qos Tag(802.1p priority tag)is supported on the
vSphere Distributed Switch.

Qos Tag is supported in vSphere
Distributed Switch Version 5.0 or later.
"#####) },),
        ("network_resource_control_version_3_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "networkResourceControlVersion3Supported", doc: Some(r#####"Flag to indicate whether Network Resource Control version 3 is supported.

The API supported by Network Resouce Control version 3 include:
1. VM virtual NIC network resource specification
   *VirtualEthernetCardResourceAllocation*
2. VM virtual NIC network resource pool specification
   *DVSVmVnicNetworkResourcePool*
3. Host infrastructure traffic network resource specification
   *DvsHostInfrastructureTrafficResource*
   
Network Resource Control version 3 is supported for Switch Version 6.0 or later.
"#####) },),
        ("network_resource_pool_high_share_value", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "networkResourcePoolHighShareValue", doc: Some(r#####"High share level (*SharesLevel_enum*.*high*)
for *DVSNetworkResourcePoolAllocationInfo*.*DVSNetworkResourcePoolAllocationInfo.shares*.

The <code>networkResourcePoolHighshareValue</code> property implicitly defines
the legal range of share values to be between 1 and this value.
This property also defines values for other level types, such as
*normal* being one half of this value and
*low* being one fourth of this value.
This feature is supported in vSphere Distributed
Switch Version 4.1 or later.
"#####) },),
    ],
}),
        ("ClusterDasFdmHostState", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("state", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "state", doc: Some(r#####"The Availability State of a host based on information
reported by the entity given by the
*ClusterDasFdmHostState.stateReporter* property.

See
*ClusterDasFdmAvailabilityState_enum* for the set of
states.
"#####) },),
        ("state_reporter", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "stateReporter", doc: Some(r#####"The entity reporting the state of the host.

If the reporter is a host,
the property reports which host, whereas if the reporter is vCenter Server,
the property is unset.

Refers instance of *HostSystem*.
"#####) },),
    ],
}),
        ("ResourceAllocationInfo", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("reservation", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "reservation", doc: Some(r#####"Amount of resource that is guaranteed available to the virtual machine or
resource pool.

Reserved resources are not wasted if they are not used. If
the utilization is less than the reservation, the resources can be utilized by
other running virtual machines. Units are MB for memory, MHz for CPU.
"#####) },),
        ("expandable_reservation", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "expandableReservation", doc: Some(r#####"In a resource pool with an expandable reservation, the reservation on a resource
pool can grow beyond the specified value, if the parent resource pool has
unreserved resources.

A non-expandable reservation is called a fixed
reservation. This property is invalid for virtual machines.
"#####) },),
        ("overhead_limit", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "overheadLimit", doc: Some(r#####"The maximum allowed overhead memory.

For a powered on virtual
machine, the overhead memory reservation cannot be larger than its
overheadLimit. This property is only applicable to powered on
virtual machines and is not persisted across reboots. This property
is not applicable for resource pools. If set to -1, then there is
no limit on reservation. Units are MB.

Note: For vCenter Server use only. Not available for other clients
at this time.
The server will throw an exception if you attempt to set
this property.
"#####) },),
        ("limit", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "limit", doc: Some(r#####"The utilization of a virtual machine/resource pool will not exceed this limit, even
if there are available resources.

This is typically used to ensure a consistent
performance of virtual machines / resource pools independent of available resources.
If set to -1, then there is no fixed limit on resource usage (only bounded by available
resources and shares). Units are MB for memory, MHz for CPU.
"#####) },),
        ("shares", NodeData { type_decl: "vim_rs::types::structs::SharesInfo", type_name: "SharesInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "shares", doc: Some(r#####"Memory shares are used in case of resource contention.
"#####) },),
    ],
}),
        ("HostPersistentMemoryInfo", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("volume_uuid", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "volumeUUID", doc: Some(r#####"Unique persistent memory host indentifier.
"#####) },),
        ("capacity_in_mb", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "capacityInMB", doc: Some(r#####"Amount of configured persistent memory available on a host in MB.
"#####) },),
    ],
}),
        ("HostDnsConfig", ::phf::Map {
    key: 8694567506910003252,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("virtual_nic_device", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "virtualNicDevice", doc: Some(r#####"If DHCP is enabled, the DHCP DNS of the vmkernel nic will override
the system's DNS.

This field applies to both IPv4 and IPv6 DNS settings
if *ipv6VirtualNicDevice*
is unset, otherwise it is applicable only for IPv4 setting.
This field is ignored if DHCP is disabled by the
*dhcp* property.
"#####) },),
        ("address", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "address", doc: Some(r#####"The IP addresses of the DNS servers, placed in order of preference.

**Note**: When DHCP is not enabled, the property can be set
explicitly. When DHCP is enabled, the property reflects the current
DNS configuration, but cannot be set.
"#####) },),
        ("ipv_6_virtual_nic_device", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "ipv6VirtualNicDevice", doc: Some(r#####"If DHCP is enabled, the IPv6 DHCP DNS of the vmkernel nic will override
the system's IPv6 DNS.

This field is ignored if DHCP is disabled by the
*dhcp* property.
"#####) },),
        ("search_domain", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "searchDomain", doc: Some(r#####"The domain in which to search for hosts, placed in order of preference.

**Note**: When DHCP is not enabled, the property can be set
explicitly. When DHCP is enabled, the property reflects the current
DNS configuration, but cannot be set.
"#####) },),
        ("domain_name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "domainName", doc: Some(r#####"The domain name portion of the DNS name.

For example, "vmware.com".

**Note**: When DHCP is not enabled, the property can be set
explicitly. When DHCP is enabled, the property reflects the current
DNS configuration, but cannot be set.
"#####) },),
        ("host_name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "hostName", doc: Some(r#####"The host name portion of DNS name.

For example, "esx01".

**Note**: When DHCP is not enabled, the property can be set
explicitly. When DHCP is enabled, the property reflects the current
DNS configuration, but cannot be set.
The hostName can't have character '.' in it when set explicitly.
"#####) },),
        ("dhcp", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "dhcp", doc: Some(r#####"The flag to indicate whether or not DHCP (dynamic host control
protocol) is used to determine DNS configuration automatically.
"#####) },),
    ],
}),
        ("HostSystemSwapConfiguration", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("option", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::HostSystemSwapConfigurationSystemSwapOptionTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostSystemSwapConfigurationSystemSwapOption"), path_segment: "option", doc: Some(r#####"The currently enabled options.

When this property contains only one value and this value is *HostSystemSwapConfigurationDisabledOption*,
this indicates that the system swap is disabled.  
If the *HostSystemSwapConfigurationDisabledOption* option is
used together with some other option in call to *HostSystem.UpdateSystemSwapConfiguration*, a
*InvalidArgument* is thrown.  
It is not allowed to have duplicate values in this array. If so a
*InvalidArgument* is thrown.
"#####) },),
    ],
}),
        ("VirtualMachineStorageInfo", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("per_datastore_usage", NodeData { type_decl: "Vec<vim_rs::types::structs::VirtualMachineUsageOnDatastore>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVirtualMachineUsageOnDatastore"), path_segment: "perDatastoreUsage", doc: Some(r#####"Storage space used by this virtual machine on all datastores that it
is located on.

Total storage space committed to this virtual machine across all datastores is
simply an aggregate of the property
*VirtualMachineUsageOnDatastore.committed*.

See also *VirtualMachineStorageSummary.committed*.
"#####) },),
        ("timestamp", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveDateTime"), path_segment: "timestamp", doc: Some(r#####"Time when values in this structure were last updated.
"#####) },),
    ],
}),
        ("VirtualMachineSummary", ::phf::Map {
    key: 7485420634051515786,
    disps: &[
        (1, 0),
        (2, 3),
    ],
    entries: &[
        ("runtime", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineRuntimeInfo", type_name: "VirtualMachineRuntimeInfo", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "runtime", doc: Some(r#####"Runtime and state information of a running virtual machine.

Most of this information is also available when a virtual machine is powered off.
In that case, it contains information from the last run, if available.
"#####) },),
        ("quick_stats", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineQuickStats", type_name: "VirtualMachineQuickStats", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "quickStats", doc: Some(r#####"A set of statistics that are typically updated with near real-time regularity.

This data object type does not support notification, for scalability reasons.
Therefore, changes in QuickStats do not generate property collector updates. To
monitor statistics values, use the statistics and alarms modules instead.
"#####) },),
        ("config", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineConfigSummary", type_name: "VirtualMachineConfigSummary", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "config", doc: Some(r#####"Basic configuration information about the virtual machine.

This information
is not available when the virtual machine is unavailable, for instance, when
it is being created or deleted.
"#####) },),
        ("custom_value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "customValue", doc: Some(r#####"Custom field values.
"#####) },),
        ("guest", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineGuestSummary", type_name: "VirtualMachineGuestSummary", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "guest", doc: Some(r#####"Guest operating system and VMware Tools information.

See *VirtualMachine.guest* for more information.
"#####) },),
        ("vm", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "vm", doc: Some(r#####"Reference to the virtual machine managed object.

Refers instance of *VirtualMachine*.
"#####) },),
        ("overall_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "overallStatus", doc: Some(r#####"Overall alarm status on this node.

In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter.
Since this property is on a DataObject, an update returned by WaitForUpdatesEx may
contain values for this property when some other property on the DataObject changes.
If this update is a result of a call to WaitForUpdatesEx with a non-empty
version parameter, the value for this property may not be current.
"#####) },),
        ("storage", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineStorageSummary", type_name: "VirtualMachineStorageSummary", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "storage", doc: Some(r#####"Storage information of the virtual machine.

It can be explicitly refreshed
with the *VirtualMachine.RefreshStorageInfo* operation.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter.
Since this property is on a DataObject, an update returned by WaitForUpdatesEx may
contain values for this property when some other property on the DataObject changes.
If this update is a result of a call to WaitForUpdatesEx with a non-empty
version parameter, the value for this property may not be current.
"#####) },),
    ],
}),
        ("DVSHealthCheckCapability", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
    ],
    entries: &[
    ],
}),
        ("DatastoreInfo", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
        (2, 8),
        (0, 3),
    ],
    entries: &[
        ("max_virtual_disk_capacity", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "maxVirtualDiskCapacity", doc: Some(r#####"The maximum capacity of a virtual disk which can be created on this volume.
"#####) },),
        ("logical_sector_size", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "logicalSectorSize", doc: Some(r#####"The logical sector size of the datastore.

If not set,
the default is 512 bytes.

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"The name of the datastore.
"#####) },),
        ("supported_v_disk_formats", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "supportedVDiskFormats", doc: Some(r#####"A list of virtual disk format type which can be supported
on that datastore.

Supported values are *native_512* and
*native_4k*.

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("url", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "url", doc: Some(r#####"The unique locator for the datastore.
"#####) },),
        ("container_id", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "containerId", doc: Some(r#####"The unique container ID of the datastore, if applicable.
"#####) },),
        ("free_space", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "freeSpace", doc: Some(r#####"Free space of this datastore, in bytes.

The server periodically updates this
value. It can be explicitly refreshed with the Refresh operation.
"#####) },),
        ("physical_sector_size", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "physicalSectorSize", doc: Some(r#####"The physical sector size of the datastore.

If not set,
the default is 512 bytes.

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("max_memory_file_size", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "maxMemoryFileSize", doc: Some(r#####"The maximum size of a snapshot or a swap file that can reside on this file system volume.
"#####) },),
        ("alias_of", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "aliasOf", doc: Some(r#####"vSAN datastore container that this datastore is alias of.

If this
field is unset then this datastore is not alias of any other vSAN
datastore.
See *DatastoreInfo.containerId*.
"#####) },),
        ("max_file_size", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "maxFileSize", doc: Some(r#####"The maximum size of a file that can reside on this file system volume.
"#####) },),
        ("timestamp", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveDateTime"), path_segment: "timestamp", doc: Some(r#####"Time when the free-space and capacity values in *DatastoreInfo* and
*DatastoreSummary* were updated.
"#####) },),
    ],
}),
        ("HostFlagInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("background_snapshots_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "backgroundSnapshotsEnabled", doc: Some(r#####"Flag to specify whether background snapshots are enabled.
"#####) },),
    ],
}),
        ("HostMemorySpec", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("service_console_reservation", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "serviceConsoleReservation", doc: Some(r#####"Service Console reservation in bytes.
"#####) },),
    ],
}),
        ("LocalizableMessage", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("message", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "message", doc: Some(r#####"Message in session locale.

Use vim.SessionManager.setLocale() to change the session locale.
"#####) },),
        ("arg", NodeData { type_decl: "Vec<vim_rs::types::structs::KeyAnyValue>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfKeyAnyValue"), path_segment: "arg", doc: Some(r#####"Substitution arguments for variables in the localized message.
"#####) },),
        ("key", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "key", doc: Some(r#####"Unique key identifying the message in the localized message catalog.
"#####) },),
    ],
}),
        ("HostVffsVolume", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (4, 5),
        (2, 0),
    ],
    entries: &[
        ("uuid", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "uuid", doc: Some(r#####"The universally unique identifier assigned to VFFS.
"#####) },),
        ("r#type", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "type", doc: Some(r#####"FileSystemType of this particular file system
See *HostFileSystemVolumeFileSystemType_enum*
"#####) },),
        ("extent", NodeData { type_decl: "Vec<vim_rs::types::structs::HostScsiDiskPartition>", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ArrayOfHostScsiDiskPartition"), path_segment: "extent", doc: Some(r#####"The list of partition names that comprise this disk's
VFFS extents.
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"Name of the file system volume.
"#####) },),
        ("version", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "version", doc: Some(r#####"Version string.

Contains major and minor version numbers.
"#####) },),
        ("major_version", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "majorVersion", doc: Some(r#####"Major version number of VFFS.
"#####) },),
        ("capacity", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "capacity", doc: Some(r#####"The capacity of the file system volume, in bytes.
"#####) },),
    ],
}),
        ("ClusterVmToolsMonitoringSettings", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (5, 4),
        (0, 0),
    ],
    entries: &[
        ("max_failures", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "maxFailures", doc: Some(r#####"Maximum number of failures and automated resets allowed during the time that
*ClusterVmToolsMonitoringSettings.maxFailureWindow* specifies.

If *ClusterVmToolsMonitoringSettings.maxFailureWindow* is -1
(no window), this represents the absolute number of failures after which
automated response is stopped.

If a virtual machine exceeds this threshold, in-depth problem analysis is
usually needed.

The default value is 3.
"#####) },),
        ("min_up_time", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "minUpTime", doc: Some(r#####"The number of seconds for the virtual machine's heartbeats to stabilize
after the virtual machine has been powered on.

This time should include
the guest operating system boot-up time. The virtual machine monitoring
will begin only after this period.

The default value is 120.
"#####) },),
        ("enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "enabled", doc: Some(r#####"Deprecated as of vSphere API 4.1, use *ClusterVmToolsMonitoringSettings.vmMonitoring*.

Flag indicating whether or not the Virtual Machine Health Monitoring
service is enabled.

The Server does not use this property.
"#####) },),
        ("vm_monitoring", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "vmMonitoring", doc: Some(r#####"Indicates the type of virtual machine monitoring.

Specify a string value corresponding to one of the
following *ClusterDasConfigInfoVmMonitoringState_enum* values:
- <code>vmMonitoringDisabled</code> (the default value)
- <code>vmMonitoringOnly</code>
- <code>vmAndAppMonitoring</code>
  
The individual VMware Tools setting for virtual machine monitoring depends on
the HA Virtual Machine Health Monitoring Service level that is
defined for the cluster
(*ClusterDasConfigInfo*.*ClusterDasConfigInfo.vmMonitoring*).
The following list indicates the supported VMware Tools <code>vmMonitoring</code> values
according to the cluster configuration.
- If the cluster configuration specifies <code>vmMonitoringDisabled</code>,
  the Service is disabled and the HA Service ignores the VMware Tools monitoring setting.
- If the cluster configuration specifies <code>vmMonitoringOnly</code>,
  the Service supports <code>vmMonitoringOnly</code> or <code>vmMonitoringDisabled</code> only.
- If the cluster configuration specifies <code>vmAndAppMonitoring</code>,
  you can use any of the *ClusterDasConfigInfoVmMonitoringState_enum* values.
  
The *ClusterVmToolsMonitoringSettings.clusterSettings* value has no
effect on the constraint imposed by the HA Virtual Machine Health Monitoring Service
level that is defined for the cluster
(*ClusterDasConfigInfo*.*ClusterDasConfigInfo.vmMonitoring*).

Application monitoring events are generated regardless of the
currently configured type of virtual machine monitoring.
You can use these events even if monitoring is being disabled
or set to <code>vmMonitoringOnly</code>.
"#####) },),
        ("cluster_settings", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "clusterSettings", doc: Some(r#####"Flag indicating whether to use the cluster settings or the per VM settings.

The default value is true.
"#####) },),
        ("max_failure_window", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "maxFailureWindow", doc: Some(r#####"The number of seconds for the window during which up to *ClusterVmToolsMonitoringSettings.maxFailures*
resets can occur before automated responses stop.

If set to -1, no failure window is specified.

The default value is -1.
"#####) },),
        ("failure_interval", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "failureInterval", doc: Some(r#####"If no heartbeat has been received for at least the specified number of seconds,
the virtual machine is declared as failed.

The default value is 30.
"#####) },),
    ],
}),
        ("HostPlugStoreTopology", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("target", NodeData { type_decl: "Vec<vim_rs::types::structs::HostPlugStoreTopologyTarget>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostPlugStoreTopologyTarget"), path_segment: "target", doc: Some(r#####"Partial list of targets as seen by the host.

The list of targets
may not be exhaustive on the host.
"#####) },),
        ("device", NodeData { type_decl: "Vec<vim_rs::types::structs::HostPlugStoreTopologyDevice>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostPlugStoreTopologyDevice"), path_segment: "device", doc: Some(r#####"List of devices in the plug store inventory.
"#####) },),
        ("adapter", NodeData { type_decl: "Vec<vim_rs::types::structs::HostPlugStoreTopologyAdapter>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostPlugStoreTopologyAdapter"), path_segment: "adapter", doc: Some(r#####"List of host bus adapters in the plug store inventory.
"#####) },),
        ("path", NodeData { type_decl: "Vec<vim_rs::types::structs::HostPlugStoreTopologyPath>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostPlugStoreTopologyPath"), path_segment: "path", doc: Some(r#####"List of paths in the plug store inventory.
"#####) },),
        ("plugin", NodeData { type_decl: "Vec<vim_rs::types::structs::HostPlugStoreTopologyPlugin>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostPlugStoreTopologyPlugin"), path_segment: "plugin", doc: Some(r#####"List of plugins in the plug store inventory.
"#####) },),
    ],
}),
        ("VsanHostFaultDomainInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"The configured VSAN fault domain.

The length of fault domain name should not exceed 256.
Empty string indicates that the default fault domain is used.
"#####) },),
    ],
}),
        ("ToolsConfigInfo", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (7, 12),
        (2, 0),
        (5, 1),
    ],
    entries: &[
        ("after_resume", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "afterResume", doc: Some(r#####"Flag to specify whether or not scripts should run
after the virtual machine resumes.
"#####) },),
        ("before_guest_reboot", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "beforeGuestReboot", doc: Some(r#####"Flag to specify whether or not scripts should run
before the virtual machine reboots.
"#####) },),
        ("customization_key_id", NodeData { type_decl: "vim_rs::types::structs::CryptoKeyId", type_name: "CryptoKeyId", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "customizationKeyId", doc: Some(r#####"When set, provides the id of the key used to encrypt the customization
package attached to the VM.
"#####) },),
        ("sync_time_with_host", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "syncTimeWithHost", doc: Some(r#####"Flag to specify whether or not the tools program will periodically
synchronize guest time with host time.

Periodical synchronization is
only allowed if *ToolsConfigInfo.syncTimeWithHostAllowed*
is not set to <code>false</code>.
"#####) },),
        ("last_install_info", NodeData { type_decl: "vim_rs::types::structs::ToolsConfigInfoToolsLastInstallInfo", type_name: "ToolsConfigInfoToolsLastInstallInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "lastInstallInfo", doc: Some(r#####"Information about the last tools upgrade attempt if applicable.

This information is maintained by the server and is ignored if set by the client.
"#####) },),
        ("tools_install_type", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "toolsInstallType", doc: Some(r#####"Installation type of VMware Tools in the guest operating system.

The set of possible values is described in
*VirtualMachineToolsInstallType_enum*
"#####) },),
        ("after_power_on", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "afterPowerOn", doc: Some(r#####"Flag to specify whether or not scripts should run
after the virtual machine powers on.
"#####) },),
        ("tools_upgrade_policy", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "toolsUpgradePolicy", doc: Some(r#####"Tools upgrade policy setting for the virtual machine.

See also *UpgradePolicy_enum*.
"#####) },),
        ("pending_customization", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "pendingCustomization", doc: Some(r#####"When set, this indicates that a customization operation is pending on the VM.

The value represents the filename of the customization package on the host.
"#####) },),
        ("sync_time_with_host_allowed", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "syncTimeWithHostAllowed", doc: Some(r#####"Indicates whether or not the tools program is allowed to synchronize
guest time with host time.

When set to <code>false</code>, disallows
tool periodic time synchronization as well as guest time step corrections
due to one-off events like resume from suspend.

***Since:*** vSphere API Release 7.0.1.0
"#####) },),
        ("tools_version", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "toolsVersion", doc: Some(r#####"Version of VMware Tools installed on the guest operating system.
"#####) },),
        ("before_guest_standby", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "beforeGuestStandby", doc: Some(r#####"Flag to specify whether or not scripts should run
before the virtual machine suspends.
"#####) },),
        ("before_guest_shutdown", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "beforeGuestShutdown", doc: Some(r#####"Flag to specify whether or not scripts should run
before the virtual machine powers off.
"#####) },),
    ],
}),
        ("HostHardwareSummary", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (2, 3),
        (10, 1),
        (8, 0),
    ],
    entries: &[
        ("num_cpu_cores", NodeData { type_decl: "i16", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveShort"), path_segment: "numCpuCores", doc: Some(r#####"Number of physical CPU cores on the host.

Physical CPU cores are the
processors contained by a CPU package.
"#####) },),
        ("num_cpu_pkgs", NodeData { type_decl: "i16", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveShort"), path_segment: "numCpuPkgs", doc: Some(r#####"Number of physical CPU packages on the host.

Physical CPU packages are chips
that contain one or more processors. Processors contained by a package are
also known as CPU cores. For example, one dual-core package is comprised of
one chip that contains two CPU cores.
"#####) },),
        ("num_hb_as", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "numHBAs", doc: Some(r#####"The number of host bus adapters (HBAs).
"#####) },),
        ("vendor", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "vendor", doc: Some(r#####"The hardware vendor identification.
"#####) },),
        ("memory_size", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "memorySize", doc: Some(r#####"The physical memory size in bytes.
"#####) },),
        ("model", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "model", doc: Some(r#####"The system model identification.
"#####) },),
        ("num_cpu_threads", NodeData { type_decl: "i16", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveShort"), path_segment: "numCpuThreads", doc: Some(r#####"Number of physical CPU threads on the host.
"#####) },),
        ("family", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "family", doc: Some(r#####"The system family identification.

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("cpu_model", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "cpuModel", doc: Some(r#####"The CPU model.
"#####) },),
        ("uuid", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "uuid", doc: Some(r#####"The hardware BIOS identification.
"#####) },),
        ("other_identifying_info", NodeData { type_decl: "Vec<vim_rs::types::structs::HostSystemIdentificationInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostSystemIdentificationInfo"), path_segment: "otherIdentifyingInfo", doc: Some(r#####"Other identification information.

This information may be vendor
specific.
"#####) },),
        ("num_nics", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "numNics", doc: Some(r#####"The number of network adapters.
"#####) },),
        ("cpu_mhz", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "cpuMhz", doc: Some(r#####"The speed of the CPU cores.

This is an average value if there are multiple
speeds. The product of cpuMhz and numCpuCores is approximately equal to the
sum of the MHz for all the individual cores on the host.
"#####) },),
    ],
}),
        ("VsanHostConfigInfoClusterInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("uuid", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "uuid", doc: Some(r#####"VSAN service cluster UUID, in the string form
"nnnnnnnn-nnnn-nnnn-nnnn-nnnnnnnnnnnn", where n are hexadecimal
digits.

If provided while enabling the VSAN service, this value will be
used for the service cluster UUID. If omitted when enabling the
VSAN service, a suitable UUID will be generated by the platform.
This is a read-only value while the VSAN service is enabled.
"#####) },),
        ("node_uuid", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "nodeUuid", doc: Some(r#####"VSAN node UUID for this host.

This is a read-only value which is populated upon enabling of the
VSAN service.
"#####) },),
    ],
}),
        ("HostVirtualNicManagerInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("net_config", NodeData { type_decl: "Vec<vim_rs::types::structs::VirtualNicManagerNetConfig>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVirtualNicManagerNetConfig"), path_segment: "netConfig", doc: Some(r#####"List of VirtualNicManager network configuration.

See also *VirtualNicManagerNetConfig*This contains the network
configuration for each NicType..
"#####) },),
    ],
}),
        ("FaultToleranceConfigInfo", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("orphaned", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "orphaned", doc: Some(r#####"Indicates whether a secondary VM is orphaned (no longer associated with
the primary VM).
"#####) },),
        ("role", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "role", doc: Some(r#####"The index of the current VM in instanceUuids array starting from 1, so
1 means that it is the primary VM.
"#####) },),
        ("instance_uuids", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "instanceUuids", doc: Some(r#####"The instanceUuid of all the VMs in this fault tolerance group.

The
first element is the instanceUuid of the primary VM.
"#####) },),
        ("config_paths", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "configPaths", doc: Some(r#####"The configuration file path for all the VMs in this fault tolerance
group.
"#####) },),
    ],
}),
        ("VirtualMachineConfigInfoOverheadInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("initial_swap_reservation", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "initialSwapReservation", doc: Some(r#####"Disk space required for virtual machine to be powered on (in bytes).

This space is used by virtualization infrastructure to swap out
virtual machine process memory. Location of the file is specified by
sched.swap.vmxSwapDir virtual machinge advanced config option or
in case it is not specified - current virtual machine home directory
is being used.
"#####) },),
        ("initial_memory_reservation", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "initialMemoryReservation", doc: Some(r#####"Memory overhead required for virtual machine to be powered on (in bytes).
"#####) },),
    ],
}),
        ("ClusterComputeResourceHCIConfigInfo", ::phf::Map {
    key: 8694567506910003252,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("workflow_state", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "workflowState", doc: Some(r#####"Configuration pertinent to state of the HCI workflow.

Valid
values are enumerated by the *HCIWorkflowState*
type.
"#####) },),
        ("dvs_setting", NodeData { type_decl: "Vec<vim_rs::types::structs::ClusterComputeResourceDvsSetting>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfClusterComputeResourceDVSSetting"), path_segment: "dvsSetting", doc: Some(r#####"Contains DVS related information captured while configuring
the cluster.
"#####) },),
        ("host_config_profile", NodeData { type_decl: "vim_rs::types::structs::ClusterComputeResourceHostConfigurationProfile", type_name: "ClusterComputeResourceHostConfigurationProfile", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "hostConfigProfile", doc: Some(r#####"Configuration of host services and host settings.
"#####) },),
        ("configured_hosts", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "configuredHosts", doc: Some(r#####"Contains a list of hosts that are currently configured using
*ClusterComputeResource.ConfigureHCI_Task* and *ClusterComputeResource.ExtendHCI_Task*
method.

A failed host will not be part of this list.

Refers instances of *HostSystem*.
"#####) },),
    ],
}),
        ("TaskReason", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
    ],
    entries: &[
    ],
}),
        ("HostFirewallDefaultPolicy", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("incoming_blocked", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "incomingBlocked", doc: Some(r#####"Flag indicating whether incoming traffic should be blocked by default.
"#####) },),
        ("outgoing_blocked", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "outgoingBlocked", doc: Some(r#####"Flag indicating whether outgoing traffic should be blocked by default.
"#####) },),
    ],
}),
        ("HostRuntimeInfoStateEncryptionInfo", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("require_secure_boot", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "requireSecureBoot", doc: Some(r#####"Indicates if UEFI Secure Boot must be enabled in order for the
state encryption key to be accessible.
"#####) },),
        ("protection_mode", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "protectionMode", doc: Some(r#####"The state encryption key protection mode.

The host state is encrypted with a key that is protected using
one of the modes specified by *HostRuntimeInfoStateEncryptionInfoProtectionMode_enum*.
"#####) },),
        ("require_exec_installed_only", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "requireExecInstalledOnly", doc: Some(r#####"Indicates if the "execInstalledOnly" enforcement must be active
for the state encryption key to be accessible.
"#####) },),
    ],
}),
        ("HostVFlashManagerVFlashResourceConfigInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("vffs", NodeData { type_decl: "vim_rs::types::structs::HostVffsVolume", type_name: "HostVffsVolume", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "vffs", doc: Some(r#####"The contained VFFS volume
"#####) },),
        ("capacity", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "capacity", doc: Some(r#####"Capacity of the vFlash resource.

It is the capacity
of the contained VFFS volume.
"#####) },),
    ],
}),
        ("HostListSummary", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (7, 7),
        (0, 0),
        (2, 2),
    ],
    entries: &[
        ("host", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "host", doc: Some(r#####"The reference to the host-managed object.

Refers instance of *HostSystem*.
"#####) },),
        ("current_evc_graphics_mode_key", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "currentEVCGraphicsModeKey", doc: Some(r#####"The Enhanced VMotion Compatibility Graphics mode that is currently in
effect for this host.

If the host is in a cluster where EVC is active,
this will match the cluster's EVC Graphics mode; otherwise this will
be unset.

See also *Capability.supportedEVCGraphicsMode*.

***Since:*** vSphere API Release 7.0.1.0
"#####) },),
        ("trust_authority_attestation_infos", NodeData { type_decl: "Vec<vim_rs::types::structs::HostTrustAuthorityAttestationInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostTrustAuthorityAttestationInfo"), path_segment: "trustAuthorityAttestationInfos", doc: Some(r#####"The attestation information for the host as retrieved from any Trust
Authority attestation services configured in the host's parent compute
resource.

This field will be set only if there is any Trust Authority
attestation service configured for the host's parent compute resource,
and unset otherwise.

***Since:*** vSphere API Release 7.0.1.0
"#####) },),
        ("quick_stats", NodeData { type_decl: "vim_rs::types::structs::HostListSummaryQuickStats", type_name: "HostListSummaryQuickStats", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "quickStats", doc: Some(r#####"Basic host statistics.
"#####) },),
        ("overall_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "overallStatus", doc: Some(r#####"The overall alarm status of the host.

In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter.
Since this property is on a DataObject, an update returned by WaitForUpdatesEx may
contain values for this property when some other property on the DataObject changes.
If this update is a result of a call to WaitForUpdatesEx with a non-empty
version parameter, the value for this property may not be current.
"#####) },),
        ("hardware", NodeData { type_decl: "vim_rs::types::structs::HostHardwareSummary", type_name: "HostHardwareSummary", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "hardware", doc: Some(r#####"Basic hardware information, if known.
"#####) },),
        ("gateway", NodeData { type_decl: "vim_rs::types::structs::HostListSummaryGatewaySummary", type_name: "HostListSummaryGatewaySummary", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "gateway", doc: Some(r#####"Gateway configuration, if vCenter server manages the host via a gateway
"#####) },),
        ("management_server_ip", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "managementServerIp", doc: Some(r#####"IP address of the VirtualCenter server managing this host, if any.
"#####) },),
        ("runtime", NodeData { type_decl: "vim_rs::types::structs::HostRuntimeInfo", type_name: "HostRuntimeInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "runtime", doc: Some(r#####"Basic runtime information, if known.
"#####) },),
        ("custom_value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "customValue", doc: Some(r#####"The customized field values.
"#####) },),
        ("max_evc_mode_key", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "maxEVCModeKey", doc: Some(r#####"The most capable Enhanced VMotion Compatibility mode supported by the
host hardware and software; unset if this host cannot participate in
any EVC mode.

See also *Capability.supportedEVCMode*.
"#####) },),
        ("tpm_attestation", NodeData { type_decl: "vim_rs::types::structs::HostTpmAttestationInfo", type_name: "HostTpmAttestationInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "tpmAttestation", doc: None },),
        ("config", NodeData { type_decl: "vim_rs::types::structs::HostConfigSummary", type_name: "HostConfigSummary", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "config", doc: Some(r#####"Basic configuration information, if known.
"#####) },),
        ("current_evc_mode_key", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "currentEVCModeKey", doc: Some(r#####"The Enhanced VMotion Compatibility mode that is currently in effect
for this host.

If the host is in a cluster where EVC is active, this
will match the cluster's EVC mode; otherwise this will be unset.

See also *Capability.supportedEVCMode*.
"#####) },),
        ("reboot_required", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "rebootRequired", doc: Some(r#####"Indicates whether or not the host requires a reboot due to a configuration
change.
"#####) },),
    ],
}),
        ("HostIpRouteTableConfig", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("ip_route", NodeData { type_decl: "Vec<vim_rs::types::structs::HostIpRouteOp>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostIpRouteOp"), path_segment: "ipRoute", doc: Some(r#####"The array of Routing ops (routes to be added/removed)
"#####) },),
        ("ipv_6_route", NodeData { type_decl: "Vec<vim_rs::types::structs::HostIpRouteOp>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostIpRouteOp"), path_segment: "ipv6Route", doc: None },),
    ],
}),
        ("HostNetworkInfo", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 15),
        (4, 0),
        (1, 12),
        (0, 9),
        (18, 19),
    ],
    entries: &[
        ("dhcp", NodeData { type_decl: "Vec<vim_rs::types::structs::HostDhcpService>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostDhcpService"), path_segment: "dhcp", doc: Some(r#####"DHCP Service instances configured on the host.
"#####) },),
        ("console_vnic", NodeData { type_decl: "Vec<vim_rs::types::structs::HostVirtualNic>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostVirtualNic"), path_segment: "consoleVnic", doc: Some(r#####"Virtual network adapters configured for use by the service console.

The service
console uses this network access for system management and bootstrapping
services like network boot.
The two sets of virtual network adapters are mutually exclusive.
A virtual network adapter
in this list cannot be used for things like VMotion. Likewise, a
virtual network adapter in the other list cannot be used by the
service console.
"#####) },),
        ("migration_status", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "migrationStatus", doc: Some(r#####"Current status of NVDS to VDS migration.

See *HostNetworkConfig*.*HostNetworkConfigMigrationStatus_enum*
for supported values.

***Since:*** vSphere API Release 7.0.2.0
"#####) },),
        ("rdma_device", NodeData { type_decl: "Vec<vim_rs::types::structs::HostRdmaDevice>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostRdmaDevice"), path_segment: "rdmaDevice", doc: Some(r#####"Remote direct memory access devices, if any are present on the host.
"#####) },),
        ("ip_v_6_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "ipV6Enabled", doc: Some(r#####"Enable or disable IPv6 protocol on this system.
"#####) },),
        ("nsx_transport_node_id", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "nsxTransportNodeId", doc: Some(r#####"The nsx transport node Id
"#####) },),
        ("vnic", NodeData { type_decl: "Vec<vim_rs::types::structs::HostVirtualNic>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostVirtualNic"), path_segment: "vnic", doc: Some(r#####"Virtual network adapters configured on the host (hosted products)
or the vmkernel.

In the hosted architecture, these network adapters are used by the
host to
communicate with the virtual machines running on that host. In the
VMkernel architecture, these virtual network adapters provide the
ESX Server with
external network access through a virtual switch that is bridged to a
physical network adapter. The VMkernel uses these network adapters
for features such as VMotion, NAS, iSCSI, and remote MKS connections.
"#####) },),
        ("proxy_switch", NodeData { type_decl: "Vec<vim_rs::types::structs::HostProxySwitch>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostProxySwitch"), path_segment: "proxySwitch", doc: Some(r#####"Proxy switches configured on the host.
"#####) },),
        ("pnic", NodeData { type_decl: "Vec<vim_rs::types::structs::PhysicalNic>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfPhysicalNic"), path_segment: "pnic", doc: Some(r#####"Physical network adapters as seen by the primary operating system.
"#####) },),
        ("console_ip_route_config", NodeData { type_decl: "Box<dyn vim_rs::types::traits::HostIpRouteConfigTrait>", type_name: "HostIpRouteConfig", is_optional: true, processing_type: FieldProcessingType::Trait, path_segment: "consoleIpRouteConfig", doc: Some(r#####"IP route configuration of the service console.
"#####) },),
        ("vswitch", NodeData { type_decl: "Vec<vim_rs::types::structs::HostVirtualSwitch>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostVirtualSwitch"), path_segment: "vswitch", doc: Some(r#####"Virtual switches configured on the host.
"#####) },),
        ("net_stack_instance", NodeData { type_decl: "Vec<vim_rs::types::structs::HostNetStackInstance>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostNetStackInstance"), path_segment: "netStackInstance", doc: Some(r#####"List of NetStackInstances
"#####) },),
        ("opaque_switch", NodeData { type_decl: "Vec<vim_rs::types::structs::HostOpaqueSwitch>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostOpaqueSwitch"), path_segment: "opaqueSwitch", doc: Some(r#####"List of opaque switches configured on the host.
"#####) },),
        ("ip_route_config", NodeData { type_decl: "Box<dyn vim_rs::types::traits::HostIpRouteConfigTrait>", type_name: "HostIpRouteConfig", is_optional: true, processing_type: FieldProcessingType::Trait, path_segment: "ipRouteConfig", doc: Some(r#####"Deprecated as of vSphere API 5.5, which is moved to
each NetStackInstance. For this property in NetworkInfo,
Get operation will only return its value of default NetStackInstance.

IP route configuration.
"#####) },),
        ("nvds_to_vds_migration_required", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "nvdsToVdsMigrationRequired", doc: Some(r#####"Whether NSX N-VDS to VDS migration is required

***Since:*** vSphere API Release 7.0.2.0
"#####) },),
        ("dns_config", NodeData { type_decl: "Box<dyn vim_rs::types::traits::HostDnsConfigTrait>", type_name: "HostDnsConfig", is_optional: true, processing_type: FieldProcessingType::Trait, path_segment: "dnsConfig", doc: Some(r#####"Deprecated as of vSphere API 5.5, which is moved to
each NetStackInstance. For this property in NetworkInfo,
Get operation will only return its value of default NetStackInstance.

Client-side DNS configuration.
"#####) },),
        ("portgroup", NodeData { type_decl: "Vec<vim_rs::types::structs::HostPortGroup>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostPortGroup"), path_segment: "portgroup", doc: Some(r#####"Port groups configured on the host.
"#####) },),
        ("route_table_info", NodeData { type_decl: "vim_rs::types::structs::HostIpRouteTableInfo", type_name: "HostIpRouteTableInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "routeTableInfo", doc: Some(r#####"Deprecated as of vSphere API 5.5, which is moved to
each NetStackInstance. For this property in NetworkInfo,
Get operation will only return its value of default NetStackInstance.

IP routing table
"#####) },),
        ("nat", NodeData { type_decl: "Vec<vim_rs::types::structs::HostNatService>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostNatService"), path_segment: "nat", doc: Some(r#####"NAT service instances configured on the host.
"#####) },),
        ("at_boot_ip_v_6_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "atBootIpV6Enabled", doc: Some(r#####"If true then dual IPv4/IPv6 stack enabled else IPv4 only.
"#####) },),
        ("opaque_network", NodeData { type_decl: "Vec<vim_rs::types::structs::HostOpaqueNetworkInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostOpaqueNetworkInfo"), path_segment: "opaqueNetwork", doc: Some(r#####"List of opaque networks
"#####) },),
    ],
}),
        ("VirtualMachineForkConfigInfo", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("child_fork_group_id", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "childForkGroupId", doc: Some(r#####"The fork group ID identifies the parent group of which this child
VirtualMachine is a child.

Applicable for child VirtualMachines only.
"#####) },),
        ("parent_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "parentEnabled", doc: Some(r#####"Flag to indicate whether this virtual machine is a parent enabled
virtual machine.

If this vm is not a parent enabled vm this
property will be unset.
When set into the vim.vm.ConfigSpec this flag will be ignored.
"#####) },),
        ("child_type", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "childType", doc: Some(r#####"The flag to indicate the fork child type.

For a persistent child
virtual machine, once it is powered on, it will become a linked
clone of its parent and this flag will be set to 'none'.

See also *VirtualMachineForkConfigInfoChildType_enum*.
"#####) },),
        ("parent_fork_group_id", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "parentForkGroupId", doc: Some(r#####"The fork group ID identifies the parent group which this VirtualMachine
belongs to.

Applicable for parent VirtualMachines only.
"#####) },),
    ],
}),
        ("VirtualMachineGuestSummary", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (8, 2),
        (5, 0),
    ],
    entries: &[
        ("tools_version_status_2", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "toolsVersionStatus2", doc: Some(r#####"Current version status of VMware Tools in the guest operating system,
if known.
"#####) },),
        ("tools_running_status", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "toolsRunningStatus", doc: Some(r#####"Current running status of VMware Tools in the guest operating system,
if known.
"#####) },),
        ("ip_address", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "ipAddress", doc: Some(r#####"Primary IP address assigned to the guest operating system, if known.
"#####) },),
        ("tools_status", NodeData { type_decl: "vim_rs::types::enums::VirtualMachineToolsStatusEnum", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("VirtualMachineToolsStatus"), path_segment: "toolsStatus", doc: Some(r#####"Deprecated as of vSphere API 5.0 use *VirtualMachineGuestSummary.toolsVersionStatus2* and
*VirtualMachineGuestSummary.toolsRunningStatus*.

Current status of VMware Tools in the guest operating system, if known.
"#####) },),
        ("guest_id", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "guestId", doc: Some(r#####"Guest operating system identifier (short name), if known.
"#####) },),
        ("tools_version_status", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "toolsVersionStatus", doc: Some(r#####"Deprecated as of vSphere API 5.0 use *VirtualMachineGuestSummary.toolsVersionStatus2*.

Current version status of VMware Tools in the guest operating system,
if known.
"#####) },),
        ("host_name", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "hostName", doc: Some(r#####"Hostname of the guest operating system, if known.
"#####) },),
        ("guest_full_name", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "guestFullName", doc: Some(r#####"Guest operating system name configured on the virtual machine.
"#####) },),
        ("hw_version", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "hwVersion", doc: Some(r#####"The hardware version string for this virtual machine.
"#####) },),
    ],
}),
        ("DVPortgroupPolicy", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (3, 0),
        (6, 0),
    ],
    entries: &[
        ("vendor_config_override_allowed", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vendorConfigOverrideAllowed", doc: Some(r#####"Allow the *DVPortSetting.vendorSpecificConfig*
setting of an individual port to override the setting in
*DVPortgroupConfigInfo.defaultPortConfig* of
a portgroup.
"#####) },),
        ("network_resource_pool_override_allowed", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "networkResourcePoolOverrideAllowed", doc: Some(r#####"Allow the setting of
*DVPortSetting.networkResourcePoolKey* of an
individual port to override the setting in
*DVPortgroupConfigInfo.defaultPortConfig*
of a portgroup.
"#####) },),
        ("traffic_filter_override_allowed", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "trafficFilterOverrideAllowed", doc: Some(r#####"Allow the setting of
*DVPortSetting.filterPolicy*,
for an individual port to override the setting in
*DVPortgroupConfigInfo.defaultPortConfig* of
a portgroup.
"#####) },),
        ("live_port_moving_allowed", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "livePortMovingAllowed", doc: Some(r#####"Allow a live port to be moved in and out of the portgroup.
"#####) },),
        ("port_config_reset_at_disconnect", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "portConfigResetAtDisconnect", doc: Some(r#####"If true, reset the port network setting back to the portgroup setting
(thus removing the per-port setting) when the port is disconnected from
the connectee.
"#####) },),
        ("block_override_allowed", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "blockOverrideAllowed", doc: Some(r#####"Allow the *DVPortSetting.blocked* setting
of an individual port to override the setting in
*DVPortgroupConfigInfo.defaultPortConfig* of
a portgroup.
"#####) },),
        ("shaping_override_allowed", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "shapingOverrideAllowed", doc: Some(r#####"Allow the *DVPortSetting.inShapingPolicy* or
*DVPortSetting.outShapingPolicy* settings
of an individual port to override the setting in
*DVPortgroupConfigInfo.defaultPortConfig* of
a portgroup.
"#####) },),
    ],
}),
        ("BoolPolicy", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("inherited", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "inherited", doc: Some(r#####"Whether the configuration is set to inherited value.
"#####) },),
        ("value", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "value", doc: Some(r#####"The boolean value that is either set or inherited.
"#####) },),
    ],
}),
        ("SharesInfo", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("shares", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "shares", doc: Some(r#####"The number of shares allocated.

Used to determine resource allocation in case of
resource contention. This value is only set if level is set to custom. If level is
not set to custom, this value is ignored. Therefore, only shares with custom
values can be compared.

There is no unit for this value. It is a relative measure based on the settings
for other resource pools.
"#####) },),
        ("level", NodeData { type_decl: "vim_rs::types::enums::SharesLevelEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("SharesLevel"), path_segment: "level", doc: Some(r#####"The allocation level.

The level is a simplified view of shares.
Levels map to a pre-determined set of numeric values for shares.
If the shares value does not map to a predefined size, then
the level is set as custom.
"#####) },),
    ],
}),
        ("HostScsiDiskPartition", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("disk_name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "diskName", doc: Some(r#####"The SCSI disk device on which a VMware File System (VMFS)
extent resides.

See also *HostScsiDisk*, *ScsiLun.canonicalName*.
"#####) },),
        ("partition", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "partition", doc: Some(r#####"The partition number of the partition on the ScsiDisk.
"#####) },),
    ],
}),
        ("Any", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("type_name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "_typeName", doc: Some(r#####"The type discriminator. Refers to the name of a valid data object type.
"#####) },),
    ],
}),
        ("HostNvmeTopology", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("adapter", NodeData { type_decl: "Vec<vim_rs::types::structs::HostNvmeTopologyInterface>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostNvmeTopologyInterface"), path_segment: "adapter", doc: Some(r#####"The list of NVME interfaces (could be empty).
"#####) },),
    ],
}),
        ("VirtualMachineVirtualDeviceGroups", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("device_group", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::VirtualMachineVirtualDeviceGroupsDeviceGroupTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVirtualMachineVirtualDeviceGroupsDeviceGroup"), path_segment: "deviceGroup", doc: Some(r#####"Information about device groups used by this VM.

When adding a group, all devices that form the group
must be added in the same request.
When removing group, also all devices participating
in the group must be removed.
Modifying existing device group membership is not allowed.
"#####) },),
    ],
}),
        ("HostIpRouteTableInfo", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("ip_route", NodeData { type_decl: "Vec<vim_rs::types::structs::HostIpRouteEntry>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostIpRouteEntry"), path_segment: "ipRoute", doc: Some(r#####"The array of IpRouteEntry
"#####) },),
        ("ipv_6_route", NodeData { type_decl: "Vec<vim_rs::types::structs::HostIpRouteEntry>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostIpRouteEntry"), path_segment: "ipv6Route", doc: None },),
    ],
}),
        ("HostSystemRemediationState", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("state", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "state", doc: Some(r#####"The remediation or precheck remediation operation state.

See
*HostSystemRemediationStateState_enum* for the valid
values.
"#####) },),
        ("operation_time", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveDateTime"), path_segment: "operationTime", doc: Some(r#####"For any "running" state, this is the starting time; for others, this
is the completion time.
"#####) },),
    ],
}),
        ("DVSUplinkPortPolicy", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
    ],
    entries: &[
    ],
}),
        ("VmConfigInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
        (5, 3),
    ],
    entries: &[
        ("eula", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "eula", doc: Some(r#####"End User Liceses Agreements.
"#####) },),
        ("install_boot_required", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "installBootRequired", doc: Some(r#####"Specifies whether the VM needs an initial boot before the deployment is complete.

Not relevant for vApps. This means that the value is always false when reading the
configuration and is ignored when setting the configuration.

If a vApp requires an install boot (because one of its VMs does), this is visible
on the *VirtualAppSummary.installBootRequired* field of the vApp.
"#####) },),
        ("product", NodeData { type_decl: "Vec<vim_rs::types::structs::VAppProductInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVAppProductInfo"), path_segment: "product", doc: Some(r#####"Information about the package content.
"#####) },),
        ("ovf_environment_transport", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "ovfEnvironmentTransport", doc: Some(r#####"List the transports to use for properties.

Supported values are: iso and
com.vmware.guestInfo.
"#####) },),
        ("property", NodeData { type_decl: "Vec<vim_rs::types::structs::VAppPropertyInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVAppPropertyInfo"), path_segment: "property", doc: Some(r#####"List of properties
"#####) },),
        ("ip_assignment", NodeData { type_decl: "vim_rs::types::structs::VAppIpAssignmentInfo", type_name: "VAppIPAssignmentInfo", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "ipAssignment", doc: Some(r#####"IP assignment policy and DHCP support configuration.
"#####) },),
        ("install_boot_stop_delay", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "installBootStopDelay", doc: Some(r#####"Specifies the delay in seconds to wait for the VM to power off after the initial
boot (used only if installBootRequired is true).

A value of 0 means wait forever.

Not relevant for vApps. This means that the value is always false when reading the
configuration and is ignored when setting the configuration.
"#####) },),
        ("ovf_section", NodeData { type_decl: "Vec<vim_rs::types::structs::VAppOvfSectionInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVAppOvfSectionInfo"), path_segment: "ovfSection", doc: Some(r#####"List of uninterpreted OVF meta-data sections.
"#####) },),
    ],
}),
        ("HostAutoStartManagerConfig", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("power_info", NodeData { type_decl: "Vec<vim_rs::types::structs::AutoStartPowerInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAutoStartPowerInfo"), path_segment: "powerInfo", doc: Some(r#####"Lists the auto-start/auto-stop configuration.

If a virtual machine is not
mentioned in this array, it does not participate in auto-start/auto-stop
operations.
"#####) },),
        ("defaults", NodeData { type_decl: "vim_rs::types::structs::AutoStartDefaults", type_name: "AutoStartDefaults", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "defaults", doc: Some(r#####"System defaults for auto-start/auto-stop.
"#####) },),
    ],
}),
        ("PowerSystemCapability", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("available_policy", NodeData { type_decl: "Vec<vim_rs::types::structs::HostPowerPolicy>", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ArrayOfHostPowerPolicy"), path_segment: "availablePolicy", doc: Some(r#####"List of available host power policies.
"#####) },),
    ],
}),
        ("DVPortgroupConfigInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (3, 20),
        (0, 0),
        (9, 12),
        (0, 12),
        (0, 18),
    ],
    entries: &[
        ("policy", NodeData { type_decl: "Box<dyn vim_rs::types::traits::DvPortgroupPolicyTrait>", type_name: "DVPortgroupPolicy", is_optional: false, processing_type: FieldProcessingType::Trait, path_segment: "policy", doc: Some(r#####"Portgroup policy.
"#####) },),
        ("backing_type", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "backingType", doc: Some(r#####"Backing type of portgroup.

See
*DistributedVirtualPortgroup*.*DistributedVirtualPortgroupBackingType_enum*
for possible values.
The default value is "standard"
"#####) },),
        ("uplink", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "uplink", doc: Some(r#####"Indicates whether the portgroup is an uplink portroup.
"#####) },),
        ("transport_zone_name", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "transportZoneName", doc: Some(r#####"The name of transport zone to be associated with a NSX portgroup.
"#####) },),
        ("segment_id", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "segmentId", doc: Some(r#####"The segment ID of logical switch
"#####) },),
        ("config_version", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "configVersion", doc: Some(r#####"Configuration version number.
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"Name of the portgroup.
"#####) },),
        ("auto_expand", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "autoExpand", doc: Some(r#####"If set to true, this property ignores the limit on the number of ports in the
portgroup.

When a Virtual Machine/Host tries to connect to the portgroup and there
are no free ports available in the portgroup, new ports will be automatically
added to the portgroup. The flag is currently supported only for static portgroups.

When this property is set to true, the portgroup becomes a potential candidate for
auto-shrink. Once the portgroup has auto-expanded then its disconnected ports are
likely to be deleted automatically, as a part of auto-shrink step, if there are more
than certain number of free ports. If the portgroup never auto-expanded, then it will
never lose any free ports.
"#####) },),
        ("scope", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "scope", doc: Some(r#####"Deprecated as of vSphere API 5.5.

Eligible entities that can connect to the portgroup.

If unset,
there is no restriction on which entity can connect to the portgroup.
If set, only the entities in the specified list or their child
entities are allowed to connect to the portgroup. If scopes are
defined at both port and portgroup level, they are taken as an "AND"
relationship. If such a relationship doesn't make sense, the
reconfigure operation will raise an exception.

Refers instances of *ManagedEntity*.
"#####) },),
        ("vendor_specific_config", NodeData { type_decl: "Vec<vim_rs::types::structs::DistributedVirtualSwitchKeyedOpaqueBlob>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfDistributedVirtualSwitchKeyedOpaqueBlob"), path_segment: "vendorSpecificConfig", doc: Some(r#####"Opaque binary blob that stores vendor specific configuration.
"#####) },),
        ("distributed_virtual_switch", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "distributedVirtualSwitch", doc: Some(r#####"Distributed virtual switch that the portgroup is defined on.

This property should always be set unless the user's setting
does not have System.Read privilege on the object referred to
by this property.

Refers instance of *DistributedVirtualSwitch*.
"#####) },),
        ("description", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "description", doc: Some(r#####"Description of the portgroup.
"#####) },),
        ("port_name_format", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "portNameFormat", doc: Some(r#####"If set, a name will be automatically generated based on this format
string for a port when it is created in or moved into the portgroup.

The format string can contain meta tags that will be resolved
to the corresponding values in generating a name, if applicable for
the port at the time of name generation.

To insert a meta tag in the format string,
enclose the names defined as meta tag names inside angle brackets.
See *DistributedVirtualPortgroupMetaTagName_enum* for a list of
currently available meta tags. For example,
"redNetwork-&lt;portIndex&gt;" and "&lt;dvsName&gt;-pnic&lt;portIndex&gt;"
result in generated port names like "redNetwork-2" and "switch-pnic3".

If a meta tag is recognized, but there is no applicable value, the tag
will be expanded to empty string. If an arbitrary name appears inside
a "&lt;&gt;" pair and is not recognized as one of the defined meta tags,
the substring is treated as-is and appear unchanged in the generated name.

To prevent a meta tag from being expanded, prefix the meta tag with a
'\\' (backslash). For example, the format string "abc\\&lt;portIndex&gt;def"
results in the generated port name "abc&lt;portIndex&gt;def".
"#####) },),
        ("vm_vnic_network_resource_pool_key", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "vmVnicNetworkResourcePoolKey", doc: Some(r#####"The key of virtual NIC network resource pool to be associated with a portgroup.

The default value for this property is unset, indicating that
this portgroup is not associated with any virtual NIC network resource pool.
To clear the value of this property and revert to unset, set the
*DVPortgroupConfigSpec.vmVnicNetworkResourcePoolKey*
to "-1" in an update operation.
"#####) },),
        ("num_ports", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "numPorts", doc: Some(r#####"Number of ports in the portgroup.
"#####) },),
        ("transport_zone_uuid", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "transportZoneUuid", doc: Some(r#####"The UUID of transport zone to be associated with a NSX portgroup.
"#####) },),
        ("subnet_id", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "subnetId", doc: Some(r#####"ID of the VPC Subnet when the DVPG is backed by a VPC Subnet

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("r#type", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "type", doc: Some(r#####"Type of portgroup.

See
*DistributedVirtualPortgroup*.*DistributedVirtualPortgroupPortgroupType_enum*
for possible values.
"#####) },),
        ("key", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "key", doc: Some(r#####"Key of the portgroup.
"#####) },),
        ("logical_switch_uuid", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "logicalSwitchUuid", doc: Some(r#####"The logical switch UUID, which is used by NSX portgroup
"#####) },),
        ("default_port_config", NodeData { type_decl: "Box<dyn vim_rs::types::traits::DvPortSettingTrait>", type_name: "DVPortSetting", is_optional: true, processing_type: FieldProcessingType::Trait, path_segment: "defaultPortConfig", doc: Some(r#####"Common network setting for all the ports in the portgroup.
"#####) },),
    ],
}),
        ("HostListSummaryGatewaySummary", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("gateway_type", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "gatewayType", doc: Some(r#####"The type of the gateway server used for communication with the host.

This is an opaque string that depends on how the gateway server is
registered with the vCenter Component Manager Service. There might be
several gateway servers for the same type.
"#####) },),
        ("gateway_id", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "gatewayId", doc: Some(r#####"Unique ID of the gateway server used for communication with the host.

This ID must be a unique identifier for the gateway server as
registered in the vCenter Component Manager Service. There must be
only one gateway server with the same ID.
"#####) },),
    ],
}),
        ("HostCapability", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 10),
        (0, 3),
        (1, 11),
        (0, 10),
        (4, 36),
        (0, 5),
        (0, 15),
        (0, 50),
        (0, 31),
        (0, 113),
        (0, 2),
        (0, 2),
        (0, 0),
        (15, 83),
        (0, 37),
        (0, 52),
        (6, 25),
        (0, 31),
        (1, 0),
        (0, 155),
        (0, 1),
        (0, 119),
        (2, 50),
        (0, 8),
        (0, 4),
        (21, 119),
        (1, 93),
        (0, 37),
        (12, 106),
        (2, 71),
        (81, 66),
        (2, 67),
    ],
    entries: &[
        ("max_num_disks_sv_motion", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "maxNumDisksSVMotion", doc: Some(r#####"Maximum number of migrating disks allowed of a migrating VM during SVMotion.

If this capability is not set, then the maximum is considered to be 64.
"#####) },),
        ("mark_perennially_reserved_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "markPerenniallyReservedSupported", doc: Some(r#####"Indicates whether this host supports marking specified LUN as
perennially reserved.
"#####) },),
        ("ah_device_hints_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "ahDeviceHintsSupported", doc: Some(r#####"Indicates if the host supports Assignable Hardware device hints.

***Since:*** vSphere API Release 7.0.2.0
"#####) },),
        ("tpm_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "tpmSupported", doc: Some(r#####"Flag indicating whether this host supports the integrity measurement using
a TPM device.
"#####) },),
        ("tpm_version", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "tpmVersion", doc: Some(r#####"TPM version if supported by this host.
"#####) },),
        ("replay_unsupported_reason", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "replayUnsupportedReason", doc: Some(r#####"Deprecated as of vSphere API 4.1, use
*HostCapability.replayCompatibilityIssues*.

For a host whose CPU doesn't support replay, indicates the reason
for the incompatibility.

*HostReplayUnsupportedReason_enum*
represents the set of possible values.
"#####) },),
        ("iommu_sl_dirty_capable", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "iommuSLDirtyCapable", doc: Some(r#####"Indicates whether this host's IOMMUs are capable of tracking pages
written by device DMAs using dirty bits in the second-level page
tables.

If this value is not specified, it should be considered as
not capable.

***Since:*** vSphere API Release 8.0.0.1
"#####) },),
        ("max_supported_vcpus", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "maxSupportedVcpus", doc: Some(r#####"The maximum number of virtual CPUs supported per virtual machine.

If this capability is not set, the number is unlimited.
"#####) },),
        ("ptp_config_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "ptpConfigSupported", doc: Some(r#####"Indicates whether this host supports PTP (Precision Time Protocol)
service configuration.

See *HostPtpConfig*. If value is
not specified, it should be considered as not supported.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("entitlement_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "entitlementSupported", doc: Some(r#####"Indicates whether this host supports license entitlements

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("encryption_change_on_add_remove_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "encryptionChangeOnAddRemoveSupported", doc: Some(r#####"Indicates whether this host supports changing the encryption state
of a virtual disk when the disk is being added or removed from a
virtual machine configuration.
"#####) },),
        ("vmknic_binding_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vmknicBindingSupported", doc: Some(r#####"Indicates whether vmknic binding is supported over this host.

***Since:*** vSphere API Release 8.0.1.0
"#####) },),
        ("storage_v_motion_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "storageVMotionSupported", doc: Some(r#####"Indicates whether the storage of a powered-on virtual machine may be
relocated.
"#####) },),
        ("virtual_exec_usage_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "virtualExecUsageSupported", doc: Some(r#####"Indicates whether the host supports configuring hardware
virtualization (HV) support for virtual machines.
"#####) },),
        ("scheduled_hardware_upgrade_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "scheduledHardwareUpgradeSupported", doc: Some(r#####"Indicates whether the host supports scheduled hardware upgrades.

See also *VirtualMachineConfigInfo.scheduledHardwareUpgradeInfo*.
"#####) },),
        ("fpt_hotplug_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "fptHotplugSupported", doc: Some(r#####"Indicates whether FPT Hotplug is supported on this host.

***Since:*** vSphere API Release 8.0.1.0
"#####) },),
        ("mark_as_ssd_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "markAsSsdSupported", doc: Some(r#####"Indicates whether mark disk as SSD or Non-SSD is supported
on the host.

See also *HostStorageSystem.MarkAsSsd_Task*, *HostStorageSystem.MarkAsNonSsd_Task*.
"#####) },),
        ("encryption_hbr_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "encryptionHBRSupported", doc: Some(r#####"Indicates whether this host supports encrypting virtual disks with
Host Based Replication enabled.
"#####) },),
        ("max_running_v_ms", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "maxRunningVMs", doc: Some(r#####"The maximum number of virtual machines that can be running
simultaneously on this host.

If this capability is not set, the number of virtual machines
running simultaneously is unlimited.
"#####) },),
        ("max_supported_vm_memory", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "maxSupportedVmMemory", doc: Some(r#####"The maximum amount of virtual memory supported per virtual machine.

If this capability is not set, the size is limited by hardware version
of the virtual machine only.
"#####) },),
        ("san_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "sanSupported", doc: Some(r#####"Flag indicating whether access to SAN devices is supported.
"#####) },),
        ("per_vm_network_traffic_shaping_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "perVMNetworkTrafficShapingSupported", doc: Some(r#####"Indicates whether network traffic shaping on a
per virtual machine basis is supported.
"#####) },),
        ("device_rebind_without_reboot_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "deviceRebindWithoutRebootSupported", doc: Some(r#####"Indicates whether device rebind without reboot is supported.

This is
the capability which enables PCI passthrough and SR-IOV configuration
without reboot.
"#####) },),
        ("max_host_supported_vcpus", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "maxHostSupportedVcpus", doc: Some(r#####"The maximum number of virtual CPUs that can be run on the host.

An unset
value indicates that the value could not be obtained. In contrast to
*HostCapability.maxSupportedVcpus*, this value is the minimum of (i) the maximum
number supported by the hardware and (ii) the maximum number permitted by
the host license.
"#####) },),
        ("iscsi_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "iscsiSupported", doc: Some(r#####"Is access to iSCSI devices supported.
"#####) },),
        ("nvme_vvol_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "nvmeVvolSupported", doc: Some(r#####"Indicates whether mounting of NVMe vvol is supported on this host.

***Since:*** vSphere API Release 8.0.0.0
"#####) },),
        ("v_flash_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vFlashSupported", doc: Some(r#####"Indicates whether the host supports vFlash.
"#####) },),
        ("p_mem_snapshot_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "pMemSnapshotSupported", doc: Some(r#####"Deprecated as of vSphere 9.0 APIs with no replacement.

Indicates whether this host supports snapshots for VMs with virtual
devices backed by persistent memory.

If value is not specified, it should be considered as not supported.
"#####) },),
        ("encryption_v_flash_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "encryptionVFlashSupported", doc: Some(r#####"Indicates whether this host supports encrypting virtual disks with
vFlash cache enabled.
"#####) },),
        ("screenshot_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "screenshotSupported", doc: Some(r#####"Indicates whether the screenshot retrival over https is supported for this host's
virtual machines.

If true, a screenshot can be retrieved at the HTTPS relative path
_/screen?id=&lt;managed object ID of virtual machine or snapshot&gt;_.
If any of the optional parameters 'top', 'left', 'bottom', and 'right' is
specified, the returned image will be cropped from the rectangle with upper left
corner (left, top) and bottom right corner (right - 1, bottom - 1). These values
default to the top, left, bottom and right edges of the image.
The client must use an authenticated session with privilege
VirtualMachine.Interact.ConsoleInteract on the requested virtual machine or,
in the case of a snapshot, the virtual machine associated with that snapshot.
"#####) },),
        ("vm_direct_path_gen_2_unsupported_reason_extended", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "vmDirectPathGen2UnsupportedReasonExtended", doc: Some(r#####"Deprecated as of vSphere API 8.0. VMDirectPath Gen 2 is no longer supported and
there is no replacement.

If *HostCapability.vmDirectPathGen2Supported* is false, this property may
contain an explanation provided by the platform, beyond the reasons (if
any) enumerated in *HostCapability.vmDirectPathGen2UnsupportedReason*.
"#####) },),
        ("vp_status_check_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vpStatusCheckSupported", doc: Some(r#####"Indicates whether VasaProvider Status can be monitored on the host.

***Since:*** vSphere API Release 8.0.3.0
"#####) },),
        ("vmfs_datastore_mount_capable", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vmfsDatastoreMountCapable", doc: Some(r#####"Indicates whether the host is capable of mounting/unmounting
VMFS datastores.
"#####) },),
        ("p_mem_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "pMemSupported", doc: Some(r#####"Deprecated as of vSphere 9.0 APIs with no replacement.

Indicates whether this host supports persistent memory.

If value is not specified, it should be considered as not supported.
"#####) },),
        ("ft_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "ftSupported", doc: Some(r#####"Deprecated as of vSphere API 6.0.

Indicates whether this host supports Fault Tolerance
There can be many reasons why a host does not support Fault
Tolerance, which includes CPU compatibility, product
compatibility as well as other host configuration settings.

For specific reasons, look into
*HostCapability.replayCompatibilityIssues* and
*HostCapability.ftCompatibilityIssues*
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter.
Since this property is on a DataObject, an update returned by WaitForUpdatesEx may
contain values for this property when some other property on the DataObject changes.
If this update is a result of a call to WaitForUpdatesEx with a non-empty
version parameter, the value for this property may not be current.
"#####) },),
        ("v_storage_capable", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vStorageCapable", doc: Some(r#####"Indicates whether the host supports vStorage Hardware
acceleration.
"#####) },),
        ("sgx_registration_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "sgxRegistrationSupported", doc: Some(r#####"Indicates whether this host supports SGX registration.

***Since:*** vSphere API Release 8.0.0.1
"#####) },),
        ("quick_boot_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "quickBootSupported", doc: Some(r#####"Indicates whether this host supports the LoadESX feature
which allows faster reboots.

See also *HostLoadEsxManager.QueryLoadEsxSupported*.
"#####) },),
        ("encrypted_v_motion_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "encryptedVMotionSupported", doc: Some(r#####"Indicates whether this host supports encrypted vMotion.
"#####) },),
        ("firewall_ip_rules_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "firewallIpRulesSupported", doc: Some(r#####"Indicates whether this host supports ip address based restrictions in
the firewall configuration.
"#####) },),
        ("assignable_hardware_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "assignableHardwareSupported", doc: Some(r#####"Indicates whether this host supports Assignable Hardware.
"#####) },),
        ("provisioning_nic_selection_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "provisioningNicSelectionSupported", doc: Some(r#####"Indicates whether a dedicated nic can be selected for vSphere Provisioning
NFC traffic.
"#####) },),
        ("ipmi_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "ipmiSupported", doc: Some(r#####"Flag indicating whether the host supports
IPMI (Intelligent Platform Management Interface).

XXX - Make ipmiSupported optional until there is a compatible hostagent.
"#####) },),
        ("nvme_over_tcp_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "nvmeOverTcpSupported", doc: Some(r#####"Indicates if access to NVMe over TCP devices is supported.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("cpu_memory_resource_configuration_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "cpuMemoryResourceConfigurationSupported", doc: Some(r#####"Flag indicating whether cpu and memory resource configuration is
supported.

If this is set to false,
*ResourcePool.UpdateConfig*,
*ResourcePool.UpdateChildResourceConfiguration*
cannot be used for changing the cpu/memory resource configurations.
"#####) },),
        ("multiple_network_stack_instance_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "multipleNetworkStackInstanceSupported", doc: Some(r#####"Indicates whether the host supports Multiple Instance TCP/IP stack
"#####) },),
        ("cpu_hw_mmu_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "cpuHwMmuSupported", doc: Some(r#####"Indicates whether this host supports hardware-based MMU virtualization.
"#####) },),
        ("datastore_principal_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "datastorePrincipalSupported", doc: Some(r#####"Flag indicating whether datastore principal user
is supported on the host.
"#####) },),
        ("mconnect_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "mconnectSupported", doc: Some(r#####"Indicates whether MCONNECT is supported on this host.

***Since:*** vSphere API Release 8.0.1.0
"#####) },),
        ("ndcm_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "ndcmSupported", doc: Some(r#####"Indicates whether non-disruptive certificate management is supported on this host.

***Since:*** vSphere API Release 8.0.3.0
"#####) },),
        ("vsan_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vsanSupported", doc: Some(r#####"Indicates whether the host supports VSAN functionality.

See also *HostVsanSystem*.
"#####) },),
        ("vr_nfc_nic_selection_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vrNfcNicSelectionSupported", doc: Some(r#####"Indicates whether a dedicated nic can be selected for vSphere Replication
NFC traffic, i.e., from the VR server to the secondary host.
"#####) },),
        ("vm_direct_path_gen_2_unsupported_reason", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "vmDirectPathGen2UnsupportedReason", doc: Some(r#####"Deprecated as of vSphere API 8.0. VMDirectPath Gen 2 is no longer supported and
there is no replacement.

If *HostCapability.vmDirectPathGen2Supported* is false, this array will be
populated with reasons for the lack of support (chosen from
*HostCapabilityVmDirectPathGen2UnsupportedReason_enum*).

If there is a reason for
the lack of support that cannot be described by the available constants,
*HostCapability.vmDirectPathGen2UnsupportedReasonExtended* will be populated
with an additional explanation provided by the platform.

Note that this list of reasons is not guaranteed to be exhaustive.

If the reason "hostNptIncompatibleProduct" is provided, then that will
be the only provided reason, as the host software is incapable of
providing additional information.
"#####) },),
        ("gateway_on_nic_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "gatewayOnNicSupported", doc: Some(r#####"Flag indicating whether default gateway can be configured on a
vmkernel nic.
"#####) },),
        ("max_vcpus_per_ft_vm", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "maxVcpusPerFtVm", doc: Some(r#####"The maximum number of vCPUs allowed for a fault-tolerant virtual machine.
"#####) },),
        ("max_supported_v_ms", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "maxSupportedVMs", doc: Some(r#####"The maximum number of virtual machines that can exist on this host.

If this capability is not set, the number of virtual machines is
unlimited.
"#####) },),
        ("replay_compatibility_issues", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "replayCompatibilityIssues", doc: Some(r#####"Deprecated as of vSphere API 6.0.

For a host which doesn't support replay, indicates all the reasons
for the incompatibility.

*HostReplayUnsupportedReason_enum*
lists the set of possible values.
"#####) },),
        ("assign_hw_pci_config_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "assignHwPciConfigSupported", doc: Some(r#####"Indicates if setting hardwareLabel using PciPassThrough is supported.

***Since:*** vSphere API Release 7.0.2.0
"#####) },),
        ("vlan_tagging_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vlanTaggingSupported", doc: Some(r#####"Is VLAN Tagging supported.
"#####) },),
        ("clone_from_snapshot_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "cloneFromSnapshotSupported", doc: Some(r#####"Indicates whether or not cloning a virtual machine from a snapshot
point is allowed.

This property must be true on the host where the virtual machine
is currently residing. This property need not be true on the
destination host for the clone.

See also *VirtualMachineCloneSpec.snapshot*.
"#####) },),
        ("service_package_info_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "servicePackageInfoSupported", doc: Some(r#####"Indicates whether this host supports package information in service
configuration.
"#####) },),
        ("e_2_e_4_kn_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "e2e4knSupported", doc: Some(r#####"Indicates whether E2E 4KN capability is supported on the host.

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("cim_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "cimSupported", doc: Some(r#####"Indicates whether this host supports CIM

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("background_snapshots_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "backgroundSnapshotsSupported", doc: Some(r#####"Flag indicating whether background snapshots are supported on this host.
"#####) },),
        ("max_virtual_disk_desc_version_supported", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "maxVirtualDiskDescVersionSupported", doc: Some(r#####"Maximum version of vDiskVersion supported by this host.

If this capability is not set, then the maximum is considered to be 6.

***Since:*** vSphere API Release 8.0.1.0
"#####) },),
        ("crypto_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "cryptoSupported", doc: Some(r#####"Flag indicating whether Cryptographer feature is supported.
"#####) },),
        ("nfs_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "nfsSupported", doc: Some(r#####"Is access to NFS devices supported.
"#####) },),
        ("smart_card_authentication_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "smartCardAuthenticationSupported", doc: Some(r#####"Deprecated as of vSphere API 8.0U3, and there is no replacement for it.

Indicates whether this host supports local two-factor user
authentication using smart cards.

See also *HostActiveDirectoryAuthentication.EnableSmartCardAuthentication*.
"#####) },),
        ("storage_policy_change_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "storagePolicyChangeSupported", doc: Some(r#####"Indicates whether this host supports storage policy change.
"#####) },),
        ("suspend_to_memory_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "suspendToMemorySupported", doc: Some(r#####"Indicates whether this host supports suspending virtual machines to memory.

***Since:*** vSphere API Release 7.0.2.0
"#####) },),
        ("max_registered_v_ms", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "maxRegisteredVMs", doc: Some(r#####"The maximum number of registered virtual machines supported by
the host.

If this limit is exceeded, the management agent will be
at risk of running out of system resources. *configIssue* will be posted on
*HostSystem* in this case.

If this capability is not set, the number is unknown.
"#####) },),
        ("suspended_relocate_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "suspendedRelocateSupported", doc: Some(r#####"Indicates whether this host supports relocation of
suspended virtual machines.

Must be true on the source
and destination hosts for the relocation to work.
"#####) },),
        ("vmfs_3_eol_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vmfs3EOLSupported", doc: Some(r#####"Indicates whether this host supports VMFS-3 EOL.

If value is not specified, it should be considered as not supported.
"#####) },),
        ("high_guest_mem_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "highGuestMemSupported", doc: Some(r#####"Is high guest memory supported.
"#####) },),
        ("nfs_41_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "nfs41Supported", doc: Some(r#####"Whether this host supports NFS41 file systems.
"#####) },),
        ("time_config_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "timeConfigSupported", doc: Some(r#####"Indicates whether advanced timekeeping apis are supported

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("maintenance_mode_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "maintenanceModeSupported", doc: Some(r#####"Is maintenance mode supported
"#####) },),
        ("storage_iorm_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "storageIORMSupported", doc: Some(r#####"Deprecated as of vSphere8.0 U3, and there is no replacement for it.

Indicates whether the host supports storage I/O resource
management.
"#####) },),
        ("standby_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "standbySupported", doc: Some(r#####"Flag indicating whether you can put the host in a power down
state, from which it can be powered up automatically.
"#####) },),
        ("ft_vmcp_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "ftVmcpSupported", doc: Some(r#####"Indicates whether this host supports VMCP for Fault Tolerance VMs.
"#####) },),
        ("nvme_batch_operations_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "nvmeBatchOperationsSupported", doc: Some(r#####"Indicates whether batch NVMe controller connection/disconnection is supported.

See *HostStorageSystem.ConnectNvmeControllerEx_Task* and
*HostStorageSystem.DisconnectNvmeControllerEx_Task*.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("nfs_41_krb_5_i_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "nfs41Krb5iSupported", doc: Some(r#####"Whether this host support NFS41 Kerberos 5\* security type.
"#####) },),
        ("virtual_mmu_usage_ignored", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "virtualMmuUsageIgnored", doc: Some(r#####"Indicates that *VirtualMachineFlagInfo.virtualMmuUsage* is
ignored by the host, always operating as if "on" was selected.
"#####) },),
        ("latency_sensitivity_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "latencySensitivitySupported", doc: Some(r#####"Indicates whether the host supports latency sensitivity for the
virtual machines.
"#####) },),
        ("record_replay_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "recordReplaySupported", doc: Some(r#####"Deprecated as of vSphere API 6.0.

Indicates whether this host supports record and replay
"#####) },),
        ("feature_capabilities_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "featureCapabilitiesSupported", doc: Some(r#####"Indicated whether the host supports feature capabilities
for EVC mode.
"#####) },),
        ("virtual_exec_usage_ignored", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "virtualExecUsageIgnored", doc: Some(r#####"Indicates that *VirtualMachineFlagInfo.virtualExecUsage* is
ignored by the host, always operating as if "hvOn" was selected.
"#####) },),
        ("nic_teaming_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "nicTeamingSupported", doc: Some(r#####"Is NIC teaming supported.
"#####) },),
        ("use_feature_reqs_for_old_h_wv", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "useFeatureReqsForOldHWv", doc: Some(r#####"Indicates whether this host uses vmFeatures for compatibility checking
of old (&leq;8) virtual hardware version VMs.
"#####) },),
        ("v_pmc_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vPMCSupported", doc: Some(r#####"Indicates whether the host supports virtual CPU performance counters.
"#####) },),
        ("npiv_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "npivSupported", doc: Some(r#####"Indicate whether this host supports NPIV

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("turn_disk_locator_led_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "turnDiskLocatorLedSupported", doc: Some(r#####"Indicates whether turning on/off local disk LED is supported
on the host.

See also *HostStorageSystem.TurnDiskLocatorLedOn_Task*, *HostStorageSystem.TurnDiskLocatorLedOff_Task*.
"#####) },),
        ("snapshot_relayout_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "snapshotRelayoutSupported", doc: Some(r#####"Indicates whether this host supports unrestricted relocation of virtual
machines with snapshots.

Only needs to be true on the destination host for
the unrestricted relocation to work. The full snapshot relocation does not
restrict the layout of snapshot files or disks of the virtual machine, nor
its power state. If the virtual machine is powered on, a storage vmotion
will be performed to relocate its snapshots and disks.
"#####) },),
        ("inter_vm_communication_through_vmci_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "interVMCommunicationThroughVMCISupported", doc: Some(r#####"Indicates whether the host supports VMCI for communication
between virtual machines.
"#####) },),
        ("encryption_with_snapshots_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "encryptionWithSnapshotsSupported", doc: Some(r#####"Indicates whether this host supports changing the encryption state
state of a virtual machine, or virtual disk, while the virtual
machine has snapshots present.
"#####) },),
        ("p_mem_failover_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "pMemFailoverSupported", doc: Some(r#####"Deprecated as of vSphere 9.0 APIs with no replacement.

Indicates whether this host supports failover for VMs with virtual
devices backed by persistent memory.

If value is not specified, it should be considered as not supported.

***Since:*** vSphere API Release 7.0.2.0
"#####) },),
        ("eight_plus_host_vmfs_shared_access_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "eightPlusHostVmfsSharedAccessSupported", doc: Some(r#####"Indicates whether the host is capable of accessing a VMFS disk
when there are eight or more hosts accessing the disk already.
"#####) },),
        ("one_k_volume_ap_is_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "oneKVolumeAPIsSupported", doc: Some(r#####"Indicates whether this host supports granular datastore cache update.

If value is not specified, it should be considered as not supported.
"#####) },),
        ("encryption_cbrc_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "encryptionCBRCSupported", doc: Some(r#####"Indicates whether this host supports encrypting virtual disks with
Content Based Read Cache (digest disks) enabled.
"#####) },),
        ("ft_compatibility_issues", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "ftCompatibilityIssues", doc: Some(r#####"Deprecated as of vSphere API 6.0.

For a host which doesn't support Fault Tolerance, indicates all the reasons
for the incompatibility.

*HostCapabilityFtUnsupportedReason_enum*
lists the set of possible values.
"#####) },),
        ("hbr_nic_selection_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "hbrNicSelectionSupported", doc: Some(r#####"Indicates whether a dedicated nic can be selected for vSphere Replication
LWD traffic, i.e., from the primary host to the VR server.
"#####) },),
        ("nfs_41_krb_5_p_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "nfs41Krb5pSupported", doc: Some(r#####"Indicates whether this host supports NFS41
encryption using KRB5P protocol.

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("login_by_ssl_thumbprint_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "loginBySSLThumbprintSupported", doc: Some(r#####"Flag indicating whether this host supports SSL thumbprint authentication
"#####) },),
        ("host_access_manager_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "hostAccessManagerSupported", doc: Some(r#####"Whether this host supports HostAccessManager for controlling direct
access to the host and for better lockdown mode management.
"#####) },),
        ("smp_ft_compatibility_issues", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "smpFtCompatibilityIssues", doc: Some(r#####"For a host which doesn't support smp fault tolerance, indicates all the
reasons for the incompatibility.

*HostCapabilityFtUnsupportedReason_enum* lists the set of possible
values.
"#####) },),
        ("remote_device_v_motion_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "remoteDeviceVMotionSupported", doc: Some(r#####"Indicates whether vMotion of a VM with remote devices attached is
supported.

This applies to CD-ROM and floppy devices backed by a
remote client.
"#####) },),
        ("restricted_snapshot_relocate_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "restrictedSnapshotRelocateSupported", doc: Some(r#####"Indicates whether this host supports relocation of
virtual machines with snapshots.

Must be true on the
source and destination hosts for the relocation to work.
Even if this is true, the following conditions must hold:
1\) All the the vm's files are in one directory prior
to the relocate.
2\) All of the vm's files will be in one directory
after the relocate.
3\) The source and destination hosts are the same product
version.
"#####) },),
        ("nested_hv_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "nestedHVSupported", doc: Some(r#####"Indicates whether the host supports nested hardware-assisted virtualization.
"#####) },),
        ("unmap_method_supported", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "unmapMethodSupported", doc: Some(r#####"Indicates which kind of VMFS unmap method is supported.

See
*HostCapabilityUnmapMethodSupported_enum*
"#####) },),
        ("vpxd_vmx_generation_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vpxdVmxGenerationSupported", doc: Some(r#####"Indicates whether this host can populate and consume
*VirtualMachineConfigInfo.vmxRuntimeConfig* property.

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("vsan_dedicated_vmk_nic_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vsanDedicatedVmkNicSupported", doc: Some(r#####"Indicates whether vSAN external vmknic configuration is supported on the host.

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("vmotion_with_storage_v_motion_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vmotionWithStorageVMotionSupported", doc: Some(r#####"Indicates whether the storage of a powered-on virtual machine may be
relocated while simultaneously changing the execution host of the
virtual machine.
"#####) },),
        ("vmotion_across_network_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vmotionAcrossNetworkSupported", doc: Some(r#####"Indicates whether the network of a powered-on virtual machine can be
changed while simultaneously changing the execution host of the
virtual machine.
"#####) },),
        ("txt_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "txtEnabled", doc: Some(r#####"Flag indicating whether Intel TXT is enabled on this host.

TPM attestation may be used to definitively determine the Intel TXT
Measured Launch Environment (MLE) details.
"#####) },),
        ("precision_time_protocol_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "precisionTimeProtocolSupported", doc: Some(r#####"Indicates whether this host supports date time synchronization over
Precision Time Protocol (PTP).
"#####) },),
        ("delta_disk_backings_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "deltaDiskBackingsSupported", doc: Some(r#####"Flag indicating whether explicitly creating arbirary configurations of
delta disk backings is supported.

A delta disk backing is a way to preserve a virtual disk backing
at some point in time. A delta disk backing is a file backing which in
turn points to the original virtual disk backing (the parent). After a delta
disk backing is added, all writes go to the delta disk backing. All reads
first try the delta disk backing and then try the parent backing if needed.

If this property is false, then delta disk backings can only be implicitly
created through using snapshot operations and two virtual machines cannot
safely share a parent disk backing.

If this property is true, then delta disk backings can be explicitly created
and managed, and two virtual machines may safely share a parent disk backing.

In the context above, "safely" means that performing operations on one of the
virtual machines will not affect the operation of the other virtual machine.

See also *VirtualDiskSparseVer1BackingInfo.parent*, *VirtualDiskSparseVer2BackingInfo.parent*, *VirtualDiskFlatVer1BackingInfo.parent*, *VirtualDiskFlatVer2BackingInfo.parent*, *VirtualDiskRawDiskMappingVer1BackingInfo.parent*, *VirtualMachine.PromoteDisks_Task*, *VirtualMachineRelocateSpec.diskMoveType*, *VirtualMachineRelocateSpecDiskLocator.diskMoveType*.
"#####) },),
        ("vsan_nic_mgmt_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vsanNicMgmtSupported", doc: Some(r#####"Indicates whether vSAN nic types can be managed by VirtualNicManager.

***Since:*** vSphere API Release 8.0.2.0
"#####) },),
        ("scaled_screenshot_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "scaledScreenshotSupported", doc: Some(r#####"Indicates whether scaling is supported for screenshots retrieved over https.

If true, screenshot retrieval supports the additional optional
parameters 'width' and 'height'. After cropping, the returned image will be scaled
to these dimensions. If only one of these parameters is specified, default behavior
is to return an image roughly proportional to the source image.
"#####) },),
        ("encryption_fault_tolerance_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "encryptionFaultToleranceSupported", doc: Some(r#####"Indicates whether this host supports enabling Fault Tolerance on
encrypted virtual machines.
"#####) },),
        ("max_mem_mb_per_ft_vm", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "maxMemMBPerFtVm", doc: Some(r#####"Indicates maximum memory allowed for Fault Tolerance virtual machine.
"#####) },),
        ("supported_vmfs_major_version", NodeData { type_decl: "Vec<i32>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfInt"), path_segment: "supportedVmfsMajorVersion", doc: Some(r#####"List of VMFS major versions supported by the host.
"#####) },),
        ("hpp_psp_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "hppPspSupported", doc: Some(r#####"Indicates whether this host supports HPP path selection policy
settings.
"#####) },),
        ("recursive_resource_pools_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "recursiveResourcePoolsSupported", doc: None },),
        ("shutdown_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "shutdownSupported", doc: Some(r#####"Flag indicating whether the host can be powered off
"#####) },),
        ("p_mem_independent_snapshot_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "pMemIndependentSnapshotSupported", doc: Some(r#####"Deprecated as of vSphere 9.0 APIs with no replacement.

Indicates whether this host supports snapshots of VMs configured
with independent vNVDIMMs.

If value is not specified, it should be considered as not supported.
This support does not depend on *HostCapability.pMemSnapshotSupported*.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("virtual_volume_datastore_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "virtualVolumeDatastoreSupported", doc: Some(r#####"Indicates whether this host supports VirtualVolume based Datastore.
"#####) },),
        ("per_vm_swap_files", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "perVmSwapFiles", doc: Some(r#####"Flag indicating whether virtual machine execution on this host involves
a swapfile for each virtual machine.

If true, the swapfile placement
for a powered-on virtual machine is advertised in its FileLayout by
the *swapFile* property.
"#####) },),
        ("mark_as_local_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "markAsLocalSupported", doc: Some(r#####"Indicates whether mark disk as local or remote is supported
on the host.

See also *HostStorageSystem.MarkAsLocal_Task*, *HostStorageSystem.MarkAsNonLocal_Task*.
"#####) },),
        ("vvol_nqn_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vvolNQNSupported", doc: Some(r#####"Indicates whether vVol NQN is supported on this host.

***Since:*** vSphere API Release 8.0.2.0
"#####) },),
        ("pre_assigned_pci_unit_numbers_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "preAssignedPCIUnitNumbersSupported", doc: Some(r#####"Flag to indicate whether the server returns unit numbers in a
pre-assigned range for devices on the PCI bus.

When the server supports this flag, the device unit number namespace is
partitioned by device type. Different types of devices will sit in
a specific range of unit numbers that may not correspond to physical
slots in the pci bus but present a relative ordering of the devices
with respect to other devices of the same type.
Note that this does not mean that the user can set the relative ordering
between device types, but only allows stable orderings between devices
of the same type. The unit number will now clearly represent an ordering
between devices of the same type.
*VirtualDevice.unitNumber*
This property is only available for devices on the pci controller.
"#####) },),
        ("max_supported_ptp_ports", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "maxSupportedPtpPorts", doc: Some(r#####"Number of PTP (Precision Time Protocol) ports supported by this
host (See *HostPtpConfig*).

If this capability is not
set, number of PTP ports in the host is 0.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("host_config_encryption_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "hostConfigEncryptionSupported", doc: Some(r#####"Indicates whether this host supports host configuration encryption.

***Since:*** vSphere API Release 7.0.2.0
"#####) },),
        ("message_bus_proxy_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "messageBusProxySupported", doc: Some(r#####"Indicates whether the message bus proxy is supported
"#####) },),
        ("ultralow_fixed_unmap_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "ultralowFixedUnmapSupported", doc: Some(r#####"Indicates whether ultralow fixed unmap bandwidth is supported on this host.

***Since:*** vSphere API Release 8.0.0.1
"#####) },),
        ("vmknic_binding_on_nf_sv_41", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vmknicBindingOnNFSv41", doc: Some(r#####"Indicates whether vmknic binding is supported on NFSv41 over this host.

***Since:*** vSphere API Release 8.0.3.0
"#####) },),
        ("vm_create_date_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vmCreateDateSupported", doc: Some(r#####"Indicates that createDate feature is supported by the host.
"#####) },),
        ("storage_policy_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "storagePolicySupported", doc: Some(r#####"Indicates that host supports Object-based Storage System and
Storage-Profile based disk provisioning.
"#####) },),
        ("encryption_hot_operation_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "encryptionHotOperationSupported", doc: Some(r#####"Indicates whether this host supports changing the encryption state
of a virtual machine, or virtual disk, while the virtual machine
is powered on.
"#####) },),
        ("vm_direct_path_gen_2_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vmDirectPathGen2Supported", doc: Some(r#####"Deprecated as of vSphere API 8.0. VMDirectPath Gen 2 is no longer supported and
there is no replacement.

Indicates whether the host supports network passthrough using
VMDirectPath Gen 2.

Note that this is a general capability for the host
and is independent of support by a given physical NIC. If false, the
reason(s) for lack of support will be provided in
*HostCapability.vmDirectPathGen2UnsupportedReason* and/or in
*HostCapability.vmDirectPathGen2UnsupportedReasonExtended*.

See also *PhysicalNic.vmDirectPathGen2Supported*.
"#####) },),
        ("reliable_memory_aware", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "reliableMemoryAware", doc: Some(r#####"Indicates that this host uses a reliable memory aware allocation policy.
"#####) },),
        ("user_key_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "userKeySupported", doc: Some(r#####"Indicates whether user-provided private key installation is supported on this host.

***Since:*** vSphere API Release 8.0.3.0
"#####) },),
        ("upit_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "upitSupported", doc: Some(r#####"Deprecated as of vSphere API 8.0, and there is no replacement for it.

Indicate whether this host supports UPIT
"#####) },),
        ("max_supported_simultaneous_threads", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "maxSupportedSimultaneousThreads", doc: Some(r#####"Max supported number of SMT (Simultaneous multithreading) threads.

If value is not specified, it should be considered as not supported.

***Since:*** vSphere API Release 8.0.0.1
"#####) },),
        ("max_host_running_vms", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "maxHostRunningVms", doc: Some(r#####"The maximum number of virtual machines that can be run on the host.

An unset value indicates that the value could not be obtained. In contrast
to *HostCapability.maxRunningVMs*, this value is the minimum of (i) the maximum
number supported by the hardware and (ii) the maximum number permitted by
the host license.
"#####) },),
        ("accel_3_d_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "accel3dSupported", doc: Some(r#####"Indicates if 3D hardware acceleration for virtual machines is supported.
"#####) },),
        ("n_connect_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "nConnectSupported", doc: Some(r#####"Indicates whether NFS41 NCONNECT is supported on this host.

***Since:*** vSphere API Release 8.0.3.0
"#####) },),
        ("nvme_storage_fabric_services_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "nvmeStorageFabricServicesSupported", doc: Some(r#####"Indicates whether NVMe Storage Fabrics Services (StFS) are supported.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("stretched_sc_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "stretchedSCSupported", doc: Some(r#####"Indicates whether "stretched" vVol Storage Container is supported on this host.

***Since:*** vSphere API Release 8.0.3.0
"#####) },),
        ("smp_ft_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "smpFtSupported", doc: Some(r#####"Indicates whether this host supports smp fault tolerance
"#####) },),
        ("unshared_swap_v_motion_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "unsharedSwapVMotionSupported", doc: Some(r#####"Flag indicating whether the host supports participating in a VMotion
where the virtual machine swapfile is not visible to the destination.
"#####) },),
        ("vmotion_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vmotionSupported", doc: Some(r#####"Flag indicating whether you can perform VMotion.
"#####) },),
        ("encryption_rdm_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "encryptionRDMSupported", doc: Some(r#####"Indicates whether this host supports encrypting RDM backed virtual
disks.
"#####) },),
        ("ft_efi_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "ftEfiSupported", doc: Some(r#####"Indicates whether this host supports Fault Tolerance VMs that have
specified UEFI firmware.
"#####) },),
        ("local_swap_datastore_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "localSwapDatastoreSupported", doc: Some(r#####"Flag indicating whether the host supports selecting a datastore that
that may be used to store virtual machine swapfiles.
"#####) },),
        ("supported_cpu_feature", NodeData { type_decl: "Vec<vim_rs::types::structs::HostCpuIdInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostCpuIdInfo"), path_segment: "supportedCpuFeature", doc: Some(r#####"Deprecated as of vSphere API 6.5 use
*featureCapability*.

CPU feature set that is supported by the virtualization platform.

This
feature set may reflect characteristics of the product capabilities and
licensing. For any feature marked '-', reference the
*cpuFeature* array of the host's
HardwareInfo to determine the correct value.
"#####) },),
        ("encrypted_ft_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "encryptedFtSupported", doc: Some(r#####"Indicates whether this host supports encrypted Fault Tolerance.

***Since:*** vSphere API Release 7.0.2.0
"#####) },),
        ("uefi_secure_boot", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "uefiSecureBoot", doc: Some(r#####"Flag indicating that the firmware reports the use of UEFI Secure
Boot during system boot.

TPM attestation may be used to definitively determine the boot
time UEFI Secure Boot state and its complete configuration. An
out-of-band management channel may also be considered.

***Since:*** vSphere API Release 8.0.3.0
"#####) },),
        ("encryption_memory_save_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "encryptionMemorySaveSupported", doc: Some(r#####"Indicates whether this host supports suspending, or creating
with-memory snapshots, encrypted virtual machines.
"#####) },),
        ("reboot_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "rebootSupported", doc: Some(r#####"Flag indicating whether rebooting the host is supported.
"#####) },),
    ],
}),
        ("VirtualMachineVirtualPMem", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("snapshot_mode", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "snapshotMode", doc: Some(r#####"An enum describing behavior of NVDIMM devices when a VM snapshot is taken
and restored.

If unset, taking a VM snapshot will fail when the VM is
configured with NVDIMMs. See *VirtualMachineVirtualPMemSnapshotMode_enum* for supported values.
The snapshot mode applies to all NVDIMMs configured for the VM.
Property is currently only applicable to VMs with virtual NVDIMMs and not
applicable to vPMem disks.
Setting this property will fail if the VM has existing snapshots.
"#####) },),
    ],
}),
        ("DistributedVirtualSwitchProductSpec", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
        (5, 5),
    ],
    entries: &[
        ("build", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "build", doc: Some(r#####"Build string for the server on which this call is made.

For example, x.y.z-num.
This string does not apply to the API.
"#####) },),
        ("vendor", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "vendor", doc: Some(r#####"Name of the vendor of this product.
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"Short form of the product name.
"#####) },),
        ("bundle_id", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "bundleId", doc: Some(r#####"The ID of the bundle if a host component bundle needs to be installed on
the host members to support the functionality of the switch.
"#####) },),
        ("version", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "version", doc: Some(r#####"Dot-separated version string.

For example, "1.2".
"#####) },),
        ("bundle_url", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "bundleUrl", doc: Some(r#####"The URL of the bundle that VMware Update Manager will use to install
the bundle on the host members, if *DistributedVirtualSwitchProductSpec.bundleId* is set.
"#####) },),
        ("forwarding_class", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "forwardingClass", doc: Some(r#####"Forwarding class of the distributed virtual switch.
"#####) },),
    ],
}),
        ("HostMultipathStateInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("path", NodeData { type_decl: "Vec<vim_rs::types::structs::HostMultipathStateInfoPath>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostMultipathStateInfoPath"), path_segment: "path", doc: Some(r#####"List of paths on the system and their path states.
"#####) },),
    ],
}),
        ("VirtualMachineConfigSummary", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 9),
        (0, 8),
        (0, 5),
        (11, 14),
        (12, 0),
    ],
    entries: &[
        ("guest_full_name", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "guestFullName", doc: Some(r#####"Guest operating system name configured on the virtual machine.
"#####) },),
        ("num_ethernet_cards", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "numEthernetCards", doc: Some(r#####"Number of virtual network adapters.
"#####) },),
        ("num_virtual_disks", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "numVirtualDisks", doc: Some(r#####"Number of virtual disks attached to the virtual machine.
"#####) },),
        ("hw_version", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "hwVersion", doc: Some(r#####"The hardware version string for this virtual machine.
"#####) },),
        ("tpm_present", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "tpmPresent", doc: Some(r#####"Is TPM present in a VM?
"#####) },),
        ("cpu_reservation", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "cpuReservation", doc: Some(r#####"Configured CPU reservation in MHz
"#####) },),
        ("vm_path_name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "vmPathName", doc: Some(r#####"Path name to the configuration file for the virtual machine
"#####) },),
        ("template", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "template", doc: Some(r#####"Flag to determine whether or not this virtual machine is a template.
"#####) },),
        ("guest_id", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "guestId", doc: Some(r#####"Guest operating system identifier (short name).
"#####) },),
        ("annotation", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "annotation", doc: Some(r#####"Description for the virtual machine.
"#####) },),
        ("uuid", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "uuid", doc: Some(r#####"Virtual machine BIOS identification.
"#####) },),
        ("memory_size_mb", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "memorySizeMB", doc: Some(r#####"Memory size of the virtual machine, in megabytes.
"#####) },),
        ("ft_info", NodeData { type_decl: "Box<dyn vim_rs::types::traits::FaultToleranceConfigInfoTrait>", type_name: "FaultToleranceConfigInfo", is_optional: true, processing_type: FieldProcessingType::Trait, path_segment: "ftInfo", doc: Some(r#####"Fault Tolerance settings for this virtual machine.

This property will be populated only for fault tolerance virtual
machines and will be left unset for all other virtual machines.
See *FaultToleranceConfigInfo* for a description.
"#####) },),
        ("install_boot_required", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "installBootRequired", doc: Some(r#####"Whether the VM requires a reboot to finish installation.

False if no vApp
meta-data is configured.
"#####) },),
        ("product", NodeData { type_decl: "vim_rs::types::structs::VAppProductInfo", type_name: "VAppProductInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "product", doc: Some(r#####"Product information.

References to properties in the URLs are expanded.
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"Name of the virtual machine.
"#####) },),
        ("managed_by", NodeData { type_decl: "vim_rs::types::structs::ManagedByInfo", type_name: "ManagedByInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "managedBy", doc: Some(r#####"Specifies that this VM is managed by a VC Extension.

See the
*managedBy* property in the ConfigSpec
for more details.
"#####) },),
        ("num_vmiop_backings", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "numVmiopBackings", doc: Some(r#####"Number of VMIOP backed devices attached to the virtual machine.
"#####) },),
        ("memory_reservation", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "memoryReservation", doc: Some(r#####"Configured Memory reservation in MB
"#####) },),
        ("instance_uuid", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "instanceUuid", doc: Some(r#####"VC-specific identifier of the virtual machine
"#####) },),
        ("num_cpu", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "numCpu", doc: Some(r#####"Number of processors in the virtual machine.
"#####) },),
    ],
}),
        ("PodStorageDrsEntry", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("action_history", NodeData { type_decl: "Vec<vim_rs::types::structs::ClusterActionHistory>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfClusterActionHistory"), path_segment: "actionHistory", doc: Some(r#####"The set of actions that have been performed recently.
"#####) },),
        ("storage_drs_config", NodeData { type_decl: "vim_rs::types::structs::StorageDrsConfigInfo", type_name: "StorageDrsConfigInfo", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "storageDrsConfig", doc: Some(r#####"Storage DRS configuration.
"#####) },),
        ("drs_fault", NodeData { type_decl: "Vec<vim_rs::types::structs::ClusterDrsFaults>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfClusterDrsFaults"), path_segment: "drsFault", doc: Some(r#####"A collection of the DRS faults generated in the last Storage DRS invocation.

Each element of the collection is the set of faults generated in one
recommendation.
"#####) },),
        ("recommendation", NodeData { type_decl: "Vec<vim_rs::types::structs::ClusterRecommendation>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfClusterRecommendation"), path_segment: "recommendation", doc: Some(r#####"List of recommended actions for the Storage Pod.

It is
possible that the current set of recommendations may be empty,
either due to not having any running dynamic recommendation
generation module, or since there may be no recommended actions
at this time.
"#####) },),
    ],
}),
        ("Network", ::phf::Map {
    key: 14108922650502679131,
    disps: &[
        (1, 0),
        (0, 17),
        (3, 9),
        (17, 17),
    ],
    entries: &[
        ("permission", NodeData { type_decl: "Vec<vim_rs::types::structs::Permission>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfPermission"), path_segment: "permission", doc: Some(r#####"List of permissions defined for this entity.
"#####) },),
        ("effective_role", NodeData { type_decl: "Vec<i32>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfInt"), path_segment: "effectiveRole", doc: Some(r#####"Access rights the current session has to this entity.

***Required privileges:*** System.View
"#####) },),
        ("config_issue", NodeData { type_decl: "Vec<vim_rs::types::structs::Event>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfEvent"), path_segment: "configIssue", doc: Some(r#####"Current configuration issues that have been detected for this entity.

Typically,
these issues have already been logged as events. The entity stores these
events as long as they are still current. The
*configStatus* property provides an overall status
based on these events.
"#####) },),
        ("vm", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "vm", doc: Some(r#####"Virtual machines using this network.
"#####) },),
        ("parent", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "parent", doc: Some(r#####"Parent of this entity.

This value is null for the root object and for
*VirtualMachine* objects that are part of
a *VirtualApp*.

***Required privileges:*** System.View
"#####) },),
        ("config_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "configStatus", doc: Some(r#####"The configStatus indicates whether or not the system has detected a configuration
issue involving this entity.

For example, it might have detected a
duplicate IP address or MAC address, or a host in a cluster
might be out of compliance. The meanings of the configStatus values are:
- red: A problem has been detected involving the entity.
- yellow: A problem is about to occur or a transient condition
  has occurred (For example, reconfigure fail-over policy).
- green: No configuration issues have been detected.
- gray: The configuration status of the entity is not being monitored.
  
A green status indicates only that a problem has not been detected;
it is not a guarantee that the entity is problem-free.

The *configIssue* property contains a list of the
problems that have been detected.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("summary", NodeData { type_decl: "Box<dyn vim_rs::types::traits::NetworkSummaryTrait>", type_name: "NetworkSummary", is_optional: false, processing_type: FieldProcessingType::Trait, path_segment: "summary", doc: Some(r#####"Properties of a network.
"#####) },),
        ("value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "value", doc: Some(r#####"List of custom field values.

Each value uses a key to associate
an instance of a *CustomFieldStringValue* with
a custom field definition.

***Required privileges:*** System.View
"#####) },),
        ("available_field", NodeData { type_decl: "Vec<vim_rs::types::structs::CustomFieldDef>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldDef"), path_segment: "availableField", doc: Some(r#####"List of custom field definitions that are valid for the object's type.

The fields are sorted by *CustomFieldDef.name*.

***Required privileges:*** System.View
"#####) },),
        ("host", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "host", doc: Some(r#####"Hosts attached to this network.
"#####) },),
        ("disabled_method", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "disabledMethod", doc: Some(r#####"List of operations that are disabled, given the current runtime
state of the entity.

For example, a power-on operation always fails if a
virtual machine is already powered on. This list can be used by clients to
enable or disable operations in a graphical user interface.

Note: This list is determined by the current runtime state of an entity,
not by its permissions.

This list may include the following operations for a HostSystem:
- *HostSystem.EnterMaintenanceMode_Task*
- *HostSystem.ExitMaintenanceMode_Task*
- *HostSystem.RebootHost_Task*
- *HostSystem.ShutdownHost_Task*
- *HostSystem.ReconnectHost_Task*
- *HostSystem.DisconnectHost_Task*
  
This list may include the following operations for a VirtualMachine:
- *VirtualMachine.AnswerVM*
- *ManagedEntity.Rename_Task*
- *VirtualMachine.CloneVM_Task*
- *VirtualMachine.PowerOffVM_Task*
- *VirtualMachine.PowerOnVM_Task*
- *VirtualMachine.SuspendVM_Task*
- *VirtualMachine.ResetVM_Task*
- *VirtualMachine.ReconfigVM_Task*
- *VirtualMachine.RelocateVM_Task*
- *VirtualMachine.MigrateVM_Task*
- *VirtualMachine.CustomizeVM_Task*
- *VirtualMachine.ShutdownGuest*
- *VirtualMachine.StandbyGuest*
- *VirtualMachine.RebootGuest*
- *VirtualMachine.CreateSnapshot_Task*
- *VirtualMachine.RemoveAllSnapshots_Task*
- *VirtualMachine.RevertToCurrentSnapshot_Task*
- *VirtualMachine.MarkAsTemplate*
- *VirtualMachine.MarkAsVirtualMachine*
- *VirtualMachine.ResetGuestInformation*
- *VirtualMachine.MountToolsInstaller*
- *VirtualMachine.UnmountToolsInstaller*
- *ManagedEntity.Destroy_Task*
- *VirtualMachine.UpgradeVM_Task*
- *VirtualMachine.ExportVm*
  
This list may include the following operations for a ResourcePool:
- *ResourcePool.ImportVApp*
- *ResourcePool.CreateChildVM_Task*
- *ResourcePool.UpdateConfig*
- *Folder.CreateVM_Task*
- *ManagedEntity.Destroy_Task*
- *ManagedEntity.Rename_Task*
  
This list may include the following operations for a VirtualApp:
- *ManagedEntity.Destroy_Task*
- *VirtualApp.CloneVApp_Task*
- *VirtualApp.unregisterVApp_Task*
- *VirtualApp.ExportVApp*
- *VirtualApp.PowerOnVApp_Task*
- *VirtualApp.PowerOffVApp_Task*
- *VirtualApp.UpdateVAppConfig*
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("recent_task", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "recentTask", doc: Some(r#####"The set of recent tasks operating on this managed entity.

This is a subset
of *TaskManager.recentTask* belong to this entity. A task in this
list could be in one of the four states: pending, running, success or error.

This property can be used to deduce intermediate power states for
a virtual machine entity. For example, if the current powerState is "poweredOn"
and there is a running task performing the "suspend" operation, then the virtual
machine's intermediate state might be described as "suspending."

Most tasks (such as power operations) obtain exclusive access to the virtual
machine, so it is unusual for this list to contain more than one running task.
One exception, however, is the task of cloning a virtual machine.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("custom_value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "customValue", doc: Some(r#####"Custom field values.

***Required privileges:*** System.View
"#####) },),
        ("declared_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "declaredAlarmState", doc: Some(r#####"A set of alarm states for alarms that apply to this managed entity.

The set includes alarms defined on this entity
and alarms inherited from the parent entity,
or from any ancestors in the inventory hierarchy.

Alarms are inherited if they can be triggered by this entity or its descendants.
This set does not include alarms that are defined on descendants of this entity.

***Required privileges:*** System.View
"#####) },),
        ("overall_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "overallStatus", doc: Some(r#####"General health of this managed entity.

The overall status of the managed entity is computed as the worst status
among its alarms and the configuration issues detected on the entity.
The status is reported as one of the following values:
- red: The entity has alarms or configuration issues with a red status.
- yellow: The entity does not have alarms or configuration issues with a
  red status, and has at least one with a yellow status.
- green: The entity does not have alarms or configuration issues with a
  red or yellow status, and has at least one with a green status.
- gray: All of the entity's alarms have a gray status and the
  configuration status of the entity is not being monitored.
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("alarm_actions_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "alarmActionsEnabled", doc: Some(r#####"Whether alarm actions are enabled for this entity.

True if enabled; false otherwise.

***Required privileges:*** System.Read
"#####) },),
        ("tag", NodeData { type_decl: "Vec<vim_rs::types::structs::Tag>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfTag"), path_segment: "tag", doc: Some(r#####"The set of tags associated with this managed entity.

Experimental. Subject to change.

***Required privileges:*** System.View
"#####) },),
        ("triggered_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "triggeredAlarmState", doc: Some(r#####"A set of alarm states for alarms triggered by this entity
or by its descendants.

Triggered alarms are propagated up the inventory hierarchy
so that a user can readily tell when a descendant has triggered an alarm.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.

***Required privileges:*** System.View
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"Name of this entity, unique relative to its parent.

Any / (slash), \\ (backslash), character used in this
name element will be escaped. Similarly, any % (percent) character used in
this name element will be escaped, unless it is used to start an escape
sequence. A slash is escaped as %2F or %2f. A backslash is escaped as %5C or
%5c, and a percent is escaped as %25.

***Required privileges:*** System.View
"#####) },),
    ],
}),
        ("HostSevInfo", ::phf::Map {
    key: 8694567506910003252,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("snp_state", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "snpState", doc: Some(r#####"State of SEV-SNP (SEV Secure Nested Paging) on the host.

The supported
values are described in *HostSevInfoSevState_enum*.

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("snp_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "snpSupported", doc: Some(r#####"SEV-SNP (SEV Secure Nested Paging) supported

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("max_sev_es_guests", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "maxSevEsGuests", doc: Some(r#####"The maximum number of SEV-ES and SEV-SNP guests supported on this host.
"#####) },),
        ("sev_state", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "sevState", doc: Some(r#####"State of SEV on the host.

The set of supported values are described
in *HostSevInfoSevState_enum*.
"#####) },),
    ],
}),
        ("VirtualMachineCapability", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 13),
        (0, 46),
        (1, 6),
        (1, 0),
        (7, 9),
        (38, 50),
        (0, 27),
        (0, 7),
        (0, 29),
        (0, 33),
        (0, 1),
    ],
    entries: &[
        ("change_tracking_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "changeTrackingSupported", doc: Some(r#####"Indicates that change tracking is supported for virtual disks of this
virtual machine.

However, even if change tracking is supported, it might
not be available for all disks of the virtual machine. For example,
passthru raw disk mappings or disks backed by any Ver1BackingInfo cannot
be tracked.
"#####) },),
        ("snapshot_config_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "snapshotConfigSupported", doc: Some(r#####"Indicates whether or not a virtual machine supports snapshot config.
"#####) },),
        ("pmem_failover_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "pmemFailoverSupported", doc: Some(r#####"Deprecated as of vSphere 9.0 APIs with no replacement.

Indicates support for failover to a dfferent host on VM's with pmem.

Failover is supported when set to true, and unsupported otherwise.

***Since:*** vSphere API Release 7.0.2.0
"#####) },),
        ("boot_options_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "bootOptionsSupported", doc: Some(r#####"Indicates whether boot options can be configured
for this virtual machine.
"#####) },),
        ("require_sgx_attestation_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "requireSgxAttestationSupported", doc: Some(r#####"Whether the VM supports requiring SGX remote attestation.

***Since:*** vSphere API Release 8.0.0.1
"#####) },),
        ("tdx_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "tdxSupported", doc: Some(r#####"Indicates support for INTEL-TDX (Trusted Domain Extensions).

TDX is
supported when set to true, and unsupported otherwise.

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("vm_npiv_wwn_update_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vmNpivWwnUpdateSupported", doc: Some(r#####"Indicates whether the update of NPIV WWNs are supported on the virtual machine.
"#####) },),
        ("npiv_wwn_on_non_rdm_vm_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "npivWwnOnNonRdmVmSupported", doc: Some(r#####"Supports assigning NPIV WWN to virtual machines that don't have RDM disks.
"#####) },),
        ("console_preferences_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "consolePreferencesSupported", doc: Some(r#####"Indicates whether console preferences can be set for this virtual machine.
"#####) },),
        ("powered_off_snapshots_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "poweredOffSnapshotsSupported", doc: Some(r#####"Indicates whether or not a virtual machine supports snapshot operations in
poweredOff state.

This flag doesn't affect vim.VirtualMachine.GetSnapshot,
which is always supported.
"#####) },),
        ("change_mode_disks_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "changeModeDisksSupported", doc: Some(r#####"Indicates support for change mode on virtual disks

***Since:*** vSphere API Release 8.0.0.1
"#####) },),
        ("revert_to_snapshot_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "revertToSnapshotSupported", doc: Some(r#####"Indicates whether or not a virtual machine supports reverting to a snapshot.
"#####) },),
        ("sev_snp_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "sevSnpSupported", doc: Some(r#####"Indicates support for AMD-SEV-SNP (Secure Encrypted Virtualization Secure
Nested Paging).

SEV-SNP is supported when set to true, and unsupported
otherwise.

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("setting_screen_resolution_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "settingScreenResolutionSupported", doc: Some(r#####"Indicates whether of not this virtual machine supports
setting the screen resolution of the console window.

This capability depends on the guest operating system
configured for this virtual machine.
"#####) },),
        ("secure_boot_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "secureBootSupported", doc: Some(r#####"Indicates whether secureBoot is supported for this virtual machine.
"#####) },),
        ("vendor_device_group_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vendorDeviceGroupSupported", doc: Some(r#####"Indicates support for Vendor Device Groups

***Since:*** vSphere API Release 8.0.1.0
"#####) },),
        ("vm_npiv_wwn_disable_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vmNpivWwnDisableSupported", doc: Some(r#####"Indicates whether the NPIV disabling operation is supported the virtual machine.
"#####) },),
        ("quiesced_snapshots_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "quiescedSnapshotsSupported", doc: Some(r#####"Indicates whether or not a virtual machine supports quiesced snapshots.
"#####) },),
        ("record_replay_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "recordReplaySupported", doc: Some(r#####"Deprecated as of vSphere API 6.0.

Indicates whether record and replay functionality is supported on this
virtual machine.
"#####) },),
        ("setting_video_ram_size_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "settingVideoRamSizeSupported", doc: Some(r#####"Flag indicating whether the video ram size of this virtual machine
can be configured.
"#####) },),
        ("memory_reservation_lock_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "memoryReservationLockSupported", doc: Some(r#####"Indicates whether
*memoryReservationLockedToMax*
may be set to true for this virtual machine.
"#####) },),
        ("vm_npiv_wwn_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vmNpivWwnSupported", doc: Some(r#####"Supports virtual machine NPIV WWN.
"#####) },),
        ("suspend_to_memory_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "suspendToMemorySupported", doc: Some(r#####"Indicates whether this virtual machine supports suspending to memory.

***Since:*** vSphere API Release 7.0.2.0
"#####) },),
        ("multiple_snapshots_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "multipleSnapshotsSupported", doc: Some(r#####"Indicates whether or not a virtual machine supports multiple snapshots.

This value is not set when the virtual machine is unavailable, for instance,
when it is being created or deleted.
"#####) },),
        ("sev_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "sevSupported", doc: Some(r#####"Indicates support for AMD-SEV (Secure Encrypted Virtualization).

SEV is
supported when set to true, and unsupported otherwise.

***Since:*** vSphere API Release 7.0.1.0
"#####) },),
        ("guest_auto_lock_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "guestAutoLockSupported", doc: Some(r#####"Indicates whether features like guest OS auto-lock and MKS connection
controls are supported for this virtual machine.
"#####) },),
        ("snapshot_operations_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "snapshotOperationsSupported", doc: Some(r#####"Indicates whether or not a virtual machine supports snapshot operations.
"#####) },),
        ("disk_only_snapshot_on_suspended_vm_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "diskOnlySnapshotOnSuspendedVMSupported", doc: Some(r#####"Indicates whether this virtual machine supports creating disk-only snapshots
in suspended state.

If this capability is not set, the snapshot of a
virtual machine in suspended state will always include memory.
"#####) },),
        ("virtual_mmu_usage_ignored", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "virtualMmuUsageIgnored", doc: Some(r#####"Indicates that *VirtualMachineFlagInfo.virtualMmuUsage* is
ignored by this virtual machine, always operating as if "on" was selected.
"#####) },),
        ("disable_snapshots_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "disableSnapshotsSupported", doc: Some(r#####"Deprecated as of vSphere API 4.0. The value returned from the server is
always false.

Indicates whether or not snapshots can be disabled.
"#####) },),
        ("se_sparse_disk_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "seSparseDiskSupported", doc: Some(r#####"Indicates whether this virtual machine supports the Flex-SE
(space-efficient, sparse) format for virtual disks.
"#####) },),
        ("powered_on_monitor_type_change_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "poweredOnMonitorTypeChangeSupported", doc: Some(r#####"Indicates whether a monitor type change is supported while this virtual
machine is in the poweredOn state.
"#####) },),
        ("boot_retry_options_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "bootRetryOptionsSupported", doc: Some(r#####"Indicates whether automatic boot retry can be
configured for this virtual machine.
"#####) },),
        ("nested_hv_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "nestedHVSupported", doc: Some(r#####"Indicates whether this virtual machine supports nested hardware-assisted
virtualization.
"#####) },),
        ("tools_sync_time_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "toolsSyncTimeSupported", doc: Some(r#####"Indicates whether asking tools to sync time with the host is supported.
"#####) },),
        ("tools_sync_time_allow_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "toolsSyncTimeAllowSupported", doc: Some(r#####"Indicates support for allowing or disallowing all tools time
sync with host.

***Since:*** vSphere API Release 7.0.1.0
"#####) },),
        ("virtual_exec_usage_ignored", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "virtualExecUsageIgnored", doc: Some(r#####"Indicates that *VirtualMachineFlagInfo.virtualExecUsage* is
ignored by this virtual machine, always operating as if "hvOn" was selected.
"#####) },),
        ("virtual_mmu_usage_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "virtualMmuUsageSupported", doc: Some(r#####"Indicates whether or not the use of nested page table hardware support
can be explicitly set.
"#####) },),
        ("memory_snapshots_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "memorySnapshotsSupported", doc: Some(r#####"Indicates whether or not a virtual machine supports memory snapshots.
"#####) },),
        ("v_pmc_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vPMCSupported", doc: Some(r#####"Indicates whether this virtual machine supports virtualized CPU performance
counters.
"#####) },),
        ("per_vm_evc_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "perVmEvcSupported", doc: Some(r#####"Indicates whether this virtual machine supports Per-VM EVC mode.
"#####) },),
        ("lock_snapshots_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "lockSnapshotsSupported", doc: Some(r#####"Indicates whether or not the snapshot tree can be locked.
"#####) },),
        ("s_1_acpi_management_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "s1AcpiManagementSupported", doc: Some(r#####"Indicates whether or not a virtual machine supports ACPI S1 settings management.
"#####) },),
        ("host_based_replication_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "hostBasedReplicationSupported", doc: Some(r#####"Indicates that host based replication is supported on this virtual
machine.

However, even if host based replication is supported,
it might not be available for all disk types. For example, passthru
raw disk mappings can not be replicated.
"#####) },),
        ("swap_placement_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "swapPlacementSupported", doc: Some(r#####"Flag indicating whether the virtual machine has a configurable
*swapfile placement policy*.
"#####) },),
        ("multiple_cores_per_socket_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "multipleCoresPerSocketSupported", doc: Some(r#####"Indicates whether multiple virtual cores per socket is supported on this VM.
"#####) },),
        ("cpu_feature_mask_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "cpuFeatureMaskSupported", doc: Some(r#####"Indicates whether CPU feature requirements masks can be set for this
virtual machine.

Masking for hardware version 9 and newer virtual
machines is controlled by *VirtualMachineCapability.featureRequirementSupported*.
"#####) },),
        ("disk_shares_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "diskSharesSupported", doc: Some(r#####"Indicates whether resource settings for disks can be
applied to this virtual machine.
"#####) },),
        ("feature_requirement_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "featureRequirementSupported", doc: Some(r#####"Indicates whether featureRequirement feature is supported.
"#####) },),
        ("tools_auto_update_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "toolsAutoUpdateSupported", doc: Some(r#####"Supports tools auto-update.
"#####) },),
        ("setting_display_topology_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "settingDisplayTopologySupported", doc: Some(r#####"Indicates whether of not this virtual machine supports
setting the display topology of the console window.

This capability depends on the guest operating system
configured for this virtual machine.
"#####) },),
    ],
}),
        ("VsanHostConfigInfoStorageInfo", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (3, 0),
    ],
    entries: &[
        ("checksum_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "checksumEnabled", doc: Some(r#####"Deprecated this attribute was originally used for indicating whether
hardware checksums is supported on the disks. But in vSphere 2016
hardware checksums are replaced with software implementation,
supported by all disks. This makes current field redundant,
and its value as an input/output is ignored.

Whether checksum is enabled on all the disks in this host.

If any disk is not checksum capable or 520 bps formatted,
we will skip it.
"#####) },),
        ("auto_claim_storage", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "autoClaimStorage", doc: Some(r#####"Deprecated as this configuration will be deprecated, autoclaim
will be no longer supported.

Whether the VSAN service is configured to automatically claim local
unused storage on this host.

When set, the VSAN service will automatically format and use local
disks. Side effects from any disk consumption will be reflected in
*VsanHostConfigInfoStorageInfo.diskMapping*.
If this argument is specified as a host-level configuration input
at the VC-level (see *ClusterConfigInfoEx.vsanHostConfig*),
it will override that of any cluster-level default value.

See also *VsanHostConfigInfoStorageInfo.diskMapping*, *ClusterConfigInfoEx.vsanHostConfig*, *VsanClusterConfigInfo.defaultConfig*.
"#####) },),
        ("disk_mapping", NodeData { type_decl: "Vec<vim_rs::types::structs::VsanHostDiskMapping>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVsanHostDiskMapping"), path_segment: "diskMapping", doc: Some(r#####"Deprecated use *VsanHostConfigInfoStorageInfo.diskMapInfo* instead.

List of *VsanHostDiskMapping* entries in use by the VSAN service.

DiskMappings to be used by the VSAN service may be manually
specified using
*HostVsanSystem.InitializeDisks_Task*.

See also *HostVsanSystem.InitializeDisks_Task*.
"#####) },),
        ("disk_map_info", NodeData { type_decl: "Vec<vim_rs::types::structs::VsanHostDiskMapInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVsanHostDiskMapInfo"), path_segment: "diskMapInfo", doc: Some(r#####"List of *VsanHostDiskMapping* entries with runtime information from
the perspective of this host.
"#####) },),
    ],
}),
        ("ClusterVmComponentProtectionSettings", ::phf::Map {
    key: 351906021642186605,
    disps: &[
        (4, 0),
    ],
    entries: &[
        ("vm_storage_protection_for_pdl", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "vmStorageProtectionForPDL", doc: Some(r#####"VM storage protection setting for storage failures categorized as Permenant Device
Loss (PDL).

PDL indicates storage device failure or LUN removal. In case of PDL,
the failed datastore or device is unlikely to recover. The details of PDL are
described in *HostMountInfoInaccessibleReason_enum*.

This property is meaningful only when vSphere HA is turned on. Valid values are
*disabled*, *warning*,
*restartAggressive* and *clusterDefault*.
The default value is *disabled* for cluster setting and
*clusterDefault* for per-VM setting.

When set to *restartAggressive*, VM Component Protection service
will immediately terminate the VMs impacted by PDL and will attempt to restart the VMs
with best effort. When set to the other values, the behavior is the same as described for
*ClusterVmComponentProtectionSettings.vmStorageProtectionForAPD*.
"#####) },),
        ("vm_terminate_delay_for_apd_sec", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "vmTerminateDelayForAPDSec", doc: Some(r#####"The time interval after an APD timeout has been declared and before VM Component
Protection service will terminate the VM.

The value only applies if
*ClusterVmComponentProtectionSettings.vmStorageProtectionForAPD* is set to *restartConservative* or
*restartAggressive*.

The default value is 180 seconds if not specified. To use cluster setting for a VM override,
set to -1 in per-VM setting.
"#####) },),
        ("vm_reaction_on_apd_cleared", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "vmReactionOnAPDCleared", doc: Some(r#####"Action taken by VM Component Protection service for a powered on VM when APD
condition clears after APD timeout.

This property is meaningful only when vSphere HA is turned on. Valid values are
specified by *ClusterVmComponentProtectionSettingsVmReactionOnAPDCleared_enum*. The default value is
*none* for cluster setting and
*useClusterDefault* for per-VM setting.
"#####) },),
        ("enable_apd_timeout_for_hosts", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "enableAPDTimeoutForHosts", doc: Some(r#####"This property indicates if APD timeout will be enabled for all the hosts
in the cluster when vSphere HA is configured.

The details of APD timeout are
described in *HostMountInfoInaccessibleReason_enum*.

If *ClusterDasConfigInfo.vmComponentProtecting* is *disabled*,
the property will be ignored. Otherwise, for each host in the cluster,
APD timeout will be enabled. Note that no change will be made for a host if it
already had APD timeout enabled.

This property is meaningful only for cluster setting. It is ignored if specified at VM level.
The default value is false if not specified.

Note that this property is not persisted by vSphere backend. It does not impact any cluster
reconfiguration or host operation (such as adding a host to a cluster) that might happen later.
"#####) },),
        ("vm_storage_protection_for_apd", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "vmStorageProtectionForAPD", doc: Some(r#####"VM storage protection setting for storage failures categorized as All Paths
Down (APD).

APD is a condition where a storage has become inaccessible
for unknown reasons. It only indicates loss of connectivity and does not indicate
storage device failure or LUN removal (Permenant Device Loss or PDL). The details
of APD and PDL are described in *HostMountInfoInaccessibleReason_enum*.

This property is meaningful only when vSphere HA is turned on. Valid values are
specified by *ClusterVmComponentProtectionSettingsStorageVmReaction_enum*. The default value is
*disabled* for cluster setting and
*clusterDefault* for per-VM setting.

When an APD condition happens and the host begins timing out I/Os
(@link vim.host.MountInfo.InaccessibleReason#AllPathsDown\_Timeout}, VM Component
Protection service will react based on the specific value of this property:
- ***disabled***, no reaction, i.e., no
  VM failover and no event reporting for the failures.
- ***warning***, service will issue events,
  alarms and/or config issues for component failures.
- ***restartConservative***, service will
  terminate the impacted VMs after a preconfigured time interval
  (*ClusterVmComponentProtectionSettings.vmTerminateDelayForAPDSec*) if they are to be restarted.
- ***restartAggressive***, service might
  terminate the impacted VMs after a preconfigured time interval
  (*ClusterVmComponentProtectionSettings.vmTerminateDelayForAPDSec*). In some cases, a VM is terminated
  even if it may not able to be restarted or lose Fault Tolerance redundancy.
- ***clusterDefault***, service will implement
  cluster default.
"#####) },),
    ],
}),
        ("VsanHostConfigInfoNetworkInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("port", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::VsanHostConfigInfoNetworkInfoPortConfigTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVsanHostConfigInfoNetworkInfoPortConfig"), path_segment: "port", doc: Some(r#####"Set of PortConfig entries for use by the VSAN service, one per
"virtual network" as used by VSAN.
"#####) },),
    ],
}),
        ("DVSBackupRestoreCapability", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("backup_restore_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "backupRestoreSupported", doc: Some(r#####"Indicates whether backup, restore, and rollback are supported.
"#####) },),
    ],
}),
        ("VirtualMachineFileLayout", ::phf::Map {
    key: 14108922650502679131,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("log_file", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "logFile", doc: Some(r#####"A list of files stored in the virtual machine's log directory.

These are
relative paths from the logDirectory. A slash is always used as a
separator.
"#####) },),
        ("swap_file", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "swapFile", doc: Some(r#####"The swapfile specific to this virtual machine, if any.

This is a
complete datastore path, not a relative path.
"#####) },),
        ("snapshot", NodeData { type_decl: "Vec<vim_rs::types::structs::VirtualMachineFileLayoutSnapshotLayout>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVirtualMachineFileLayoutSnapshotLayout"), path_segment: "snapshot", doc: Some(r#####"Files of each snapshot.
"#####) },),
        ("disk", NodeData { type_decl: "Vec<vim_rs::types::structs::VirtualMachineFileLayoutDiskLayout>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVirtualMachineFileLayoutDiskLayout"), path_segment: "disk", doc: Some(r#####"Files making up each virtual disk.
"#####) },),
        ("config_file", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "configFile", doc: Some(r#####"A list of files that makes up the configuration of the virtual machine
(excluding the .vmx file, since that file is represented in the
FileInfo).

These are relative paths from the configuration directory. A
slash is always used as a separator. This list will typically include the
NVRAM file, but could also include other meta-data files.
"#####) },),
    ],
}),
        ("StoragePod", ::phf::Map {
    key: 8694567506910003252,
    disps: &[
        (2, 2),
        (1, 11),
        (2, 0),
        (15, 14),
        (1, 18),
    ],
    entries: &[
        ("parent", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "parent", doc: Some(r#####"Parent of this entity.

This value is null for the root object and for
*VirtualMachine* objects that are part of
a *VirtualApp*.

***Required privileges:*** System.View
"#####) },),
        ("namespace", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "namespace", doc: Some(r#####"The namespace with which the Folder is associated.

Namespace is a vAPI
resource which divides cluster resources and allows administrators to
give Kubernetes environments to their development teams.
This property is set only at the time of creation and cannot change.

***Required privileges:*** System.View
"#####) },),
        ("child_entity", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "childEntity", doc: Some(r#####"An array of managed object references.

Each entry is a reference to a child entity.

***Required privileges:*** System.View
"#####) },),
        ("child_type", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "childType", doc: Some(r#####"Specifies the object types a folder may contain.

When you create a folder, it inherits its childType from the parent folder
in which it is created. childType is an array of strings. Each array entry
identifies a set of object types - Folder and one or more managed object
types. The following list shows childType values for the different folders:
- { "vim.Folder", "vim.Datacenter" } - Identifies the root folder
  and its descendant folders. Data center folders can contain
  child data center folders and Datacenter managed objects.
  Datacenter objects contain virtual machine, compute resource,
  network entity, and datastore folders.
- { "vim.Folder", "vim.Virtualmachine", "vim.VirtualApp" } - Identifies
  a virtual machine folder. A virtual machine folder may contain child
  virtual machine folders. It also can contain VirtualMachine managed objects,
  templates, and VirtualApp managed objects.
- { "vim.Folder", "vim.ComputeResource" } - Identifies a
  compute resource folder, which contains child compute resource folders
  and ComputeResource hierarchies.
- { "vim.Folder", "vim.Network" } - Identifies a network entity folder.
  Network entity folders on a vCenter Server can contain Network,
  DistributedVirtualSwitch, and DistributedVirtualPortgroup managed
  objects. Network entity folders on an ESXi host can contain only
  Network objects.
- { "vim.Folder", "vim.Datastore" } - Identifies a datastore folder.
  Datastore folders can contain child datastore folders and Datastore
  managed objects.
  
***Required privileges:*** System.View
"#####) },),
        ("recent_task", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "recentTask", doc: Some(r#####"The set of recent tasks operating on this managed entity.

This is a subset
of *TaskManager.recentTask* belong to this entity. A task in this
list could be in one of the four states: pending, running, success or error.

This property can be used to deduce intermediate power states for
a virtual machine entity. For example, if the current powerState is "poweredOn"
and there is a running task performing the "suspend" operation, then the virtual
machine's intermediate state might be described as "suspending."

Most tasks (such as power operations) obtain exclusive access to the virtual
machine, so it is unusual for this list to contain more than one running task.
One exception, however, is the task of cloning a virtual machine.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("available_field", NodeData { type_decl: "Vec<vim_rs::types::structs::CustomFieldDef>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldDef"), path_segment: "availableField", doc: Some(r#####"List of custom field definitions that are valid for the object's type.

The fields are sorted by *CustomFieldDef.name*.

***Required privileges:*** System.View
"#####) },),
        ("alarm_actions_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "alarmActionsEnabled", doc: Some(r#####"Whether alarm actions are enabled for this entity.

True if enabled; false otherwise.

***Required privileges:*** System.Read
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"Name of this entity, unique relative to its parent.

Any / (slash), \\ (backslash), character used in this
name element will be escaped. Similarly, any % (percent) character used in
this name element will be escaped, unless it is used to start an escape
sequence. A slash is escaped as %2F or %2f. A backslash is escaped as %5C or
%5c, and a percent is escaped as %25.

***Required privileges:*** System.View
"#####) },),
        ("custom_value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "customValue", doc: Some(r#####"Custom field values.

***Required privileges:*** System.View
"#####) },),
        ("permission", NodeData { type_decl: "Vec<vim_rs::types::structs::Permission>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfPermission"), path_segment: "permission", doc: Some(r#####"List of permissions defined for this entity.
"#####) },),
        ("pod_storage_drs_entry", NodeData { type_decl: "vim_rs::types::structs::PodStorageDrsEntry", type_name: "PodStorageDrsEntry", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "podStorageDrsEntry", doc: Some(r#####"Storage DRS related attributes of the Storage Pod.

***Required privileges:*** System.Read
"#####) },),
        ("triggered_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "triggeredAlarmState", doc: Some(r#####"A set of alarm states for alarms triggered by this entity
or by its descendants.

Triggered alarms are propagated up the inventory hierarchy
so that a user can readily tell when a descendant has triggered an alarm.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.

***Required privileges:*** System.View
"#####) },),
        ("summary", NodeData { type_decl: "vim_rs::types::structs::StoragePodSummary", type_name: "StoragePodSummary", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "summary", doc: Some(r#####"Storage pod summary.

***Required privileges:*** System.View
"#####) },),
        ("overall_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "overallStatus", doc: Some(r#####"General health of this managed entity.

The overall status of the managed entity is computed as the worst status
among its alarms and the configuration issues detected on the entity.
The status is reported as one of the following values:
- red: The entity has alarms or configuration issues with a red status.
- yellow: The entity does not have alarms or configuration issues with a
  red status, and has at least one with a yellow status.
- green: The entity does not have alarms or configuration issues with a
  red or yellow status, and has at least one with a green status.
- gray: All of the entity's alarms have a gray status and the
  configuration status of the entity is not being monitored.
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("declared_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "declaredAlarmState", doc: Some(r#####"A set of alarm states for alarms that apply to this managed entity.

The set includes alarms defined on this entity
and alarms inherited from the parent entity,
or from any ancestors in the inventory hierarchy.

Alarms are inherited if they can be triggered by this entity or its descendants.
This set does not include alarms that are defined on descendants of this entity.

***Required privileges:*** System.View
"#####) },),
        ("effective_role", NodeData { type_decl: "Vec<i32>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfInt"), path_segment: "effectiveRole", doc: Some(r#####"Access rights the current session has to this entity.

***Required privileges:*** System.View
"#####) },),
        ("externally_managed_folder_info", NodeData { type_decl: "vim_rs::types::structs::FolderExternallyManagedFolderInfo", type_name: "FolderExternallyManagedFolderInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "externallyManagedFolderInfo", doc: Some(r#####"The information of externally managed folder.

This property is only set for the externally managed folder.

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "value", doc: Some(r#####"List of custom field values.

Each value uses a key to associate
an instance of a *CustomFieldStringValue* with
a custom field definition.

***Required privileges:*** System.View
"#####) },),
        ("tag", NodeData { type_decl: "Vec<vim_rs::types::structs::Tag>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfTag"), path_segment: "tag", doc: Some(r#####"The set of tags associated with this managed entity.

Experimental. Subject to change.

***Required privileges:*** System.View
"#####) },),
        ("config_issue", NodeData { type_decl: "Vec<vim_rs::types::structs::Event>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfEvent"), path_segment: "configIssue", doc: Some(r#####"Current configuration issues that have been detected for this entity.

Typically,
these issues have already been logged as events. The entity stores these
events as long as they are still current. The
*configStatus* property provides an overall status
based on these events.
"#####) },),
        ("disabled_method", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "disabledMethod", doc: Some(r#####"List of operations that are disabled, given the current runtime
state of the entity.

For example, a power-on operation always fails if a
virtual machine is already powered on. This list can be used by clients to
enable or disable operations in a graphical user interface.

Note: This list is determined by the current runtime state of an entity,
not by its permissions.

This list may include the following operations for a HostSystem:
- *HostSystem.EnterMaintenanceMode_Task*
- *HostSystem.ExitMaintenanceMode_Task*
- *HostSystem.RebootHost_Task*
- *HostSystem.ShutdownHost_Task*
- *HostSystem.ReconnectHost_Task*
- *HostSystem.DisconnectHost_Task*
  
This list may include the following operations for a VirtualMachine:
- *VirtualMachine.AnswerVM*
- *ManagedEntity.Rename_Task*
- *VirtualMachine.CloneVM_Task*
- *VirtualMachine.PowerOffVM_Task*
- *VirtualMachine.PowerOnVM_Task*
- *VirtualMachine.SuspendVM_Task*
- *VirtualMachine.ResetVM_Task*
- *VirtualMachine.ReconfigVM_Task*
- *VirtualMachine.RelocateVM_Task*
- *VirtualMachine.MigrateVM_Task*
- *VirtualMachine.CustomizeVM_Task*
- *VirtualMachine.ShutdownGuest*
- *VirtualMachine.StandbyGuest*
- *VirtualMachine.RebootGuest*
- *VirtualMachine.CreateSnapshot_Task*
- *VirtualMachine.RemoveAllSnapshots_Task*
- *VirtualMachine.RevertToCurrentSnapshot_Task*
- *VirtualMachine.MarkAsTemplate*
- *VirtualMachine.MarkAsVirtualMachine*
- *VirtualMachine.ResetGuestInformation*
- *VirtualMachine.MountToolsInstaller*
- *VirtualMachine.UnmountToolsInstaller*
- *ManagedEntity.Destroy_Task*
- *VirtualMachine.UpgradeVM_Task*
- *VirtualMachine.ExportVm*
  
This list may include the following operations for a ResourcePool:
- *ResourcePool.ImportVApp*
- *ResourcePool.CreateChildVM_Task*
- *ResourcePool.UpdateConfig*
- *Folder.CreateVM_Task*
- *ManagedEntity.Destroy_Task*
- *ManagedEntity.Rename_Task*
  
This list may include the following operations for a VirtualApp:
- *ManagedEntity.Destroy_Task*
- *VirtualApp.CloneVApp_Task*
- *VirtualApp.unregisterVApp_Task*
- *VirtualApp.ExportVApp*
- *VirtualApp.PowerOnVApp_Task*
- *VirtualApp.PowerOffVApp_Task*
- *VirtualApp.UpdateVAppConfig*
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("config_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "configStatus", doc: Some(r#####"The configStatus indicates whether or not the system has detected a configuration
issue involving this entity.

For example, it might have detected a
duplicate IP address or MAC address, or a host in a cluster
might be out of compliance. The meanings of the configStatus values are:
- red: A problem has been detected involving the entity.
- yellow: A problem is about to occur or a transient condition
  has occurred (For example, reconfigure fail-over policy).
- green: No configuration issues have been detected.
- gray: The configuration status of the entity is not being monitored.
  
A green status indicates only that a problem has not been detected;
it is not a guarantee that the entity is problem-free.

The *configIssue* property contains a list of the
problems that have been detected.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
    ],
}),
        ("Datastore", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 10),
        (5, 8),
        (3, 4),
        (14, 4),
        (1, 0),
    ],
    entries: &[
        ("available_field", NodeData { type_decl: "Vec<vim_rs::types::structs::CustomFieldDef>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldDef"), path_segment: "availableField", doc: Some(r#####"List of custom field definitions that are valid for the object's type.

The fields are sorted by *CustomFieldDef.name*.

***Required privileges:*** System.View
"#####) },),
        ("tag", NodeData { type_decl: "Vec<vim_rs::types::structs::Tag>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfTag"), path_segment: "tag", doc: Some(r#####"The set of tags associated with this managed entity.

Experimental. Subject to change.

***Required privileges:*** System.View
"#####) },),
        ("value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "value", doc: Some(r#####"List of custom field values.

Each value uses a key to associate
an instance of a *CustomFieldStringValue* with
a custom field definition.

***Required privileges:*** System.View
"#####) },),
        ("alarm_actions_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "alarmActionsEnabled", doc: Some(r#####"Whether alarm actions are enabled for this entity.

True if enabled; false otherwise.

***Required privileges:*** System.Read
"#####) },),
        ("parent", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "parent", doc: Some(r#####"Parent of this entity.

This value is null for the root object and for
*VirtualMachine* objects that are part of
a *VirtualApp*.

***Required privileges:*** System.View
"#####) },),
        ("config_issue", NodeData { type_decl: "Vec<vim_rs::types::structs::Event>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfEvent"), path_segment: "configIssue", doc: Some(r#####"Current configuration issues that have been detected for this entity.

Typically,
these issues have already been logged as events. The entity stores these
events as long as they are still current. The
*configStatus* property provides an overall status
based on these events.
"#####) },),
        ("custom_value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "customValue", doc: Some(r#####"Custom field values.

***Required privileges:*** System.View
"#####) },),
        ("overall_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "overallStatus", doc: Some(r#####"General health of this managed entity.

The overall status of the managed entity is computed as the worst status
among its alarms and the configuration issues detected on the entity.
The status is reported as one of the following values:
- red: The entity has alarms or configuration issues with a red status.
- yellow: The entity does not have alarms or configuration issues with a
  red status, and has at least one with a yellow status.
- green: The entity does not have alarms or configuration issues with a
  red or yellow status, and has at least one with a green status.
- gray: All of the entity's alarms have a gray status and the
  configuration status of the entity is not being monitored.
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("vm", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "vm", doc: Some(r#####"Virtual machines stored on this datastore.
"#####) },),
        ("info", NodeData { type_decl: "Box<dyn vim_rs::types::traits::DatastoreInfoTrait>", type_name: "DatastoreInfo", is_optional: false, processing_type: FieldProcessingType::Trait, path_segment: "info", doc: Some(r#####"Specific information about the datastore.
"#####) },),
        ("capability", NodeData { type_decl: "vim_rs::types::structs::DatastoreCapability", type_name: "DatastoreCapability", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "capability", doc: Some(r#####"Capabilities of this datastore.
"#####) },),
        ("disabled_method", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "disabledMethod", doc: Some(r#####"List of operations that are disabled, given the current runtime
state of the entity.

For example, a power-on operation always fails if a
virtual machine is already powered on. This list can be used by clients to
enable or disable operations in a graphical user interface.

Note: This list is determined by the current runtime state of an entity,
not by its permissions.

This list may include the following operations for a HostSystem:
- *HostSystem.EnterMaintenanceMode_Task*
- *HostSystem.ExitMaintenanceMode_Task*
- *HostSystem.RebootHost_Task*
- *HostSystem.ShutdownHost_Task*
- *HostSystem.ReconnectHost_Task*
- *HostSystem.DisconnectHost_Task*
  
This list may include the following operations for a VirtualMachine:
- *VirtualMachine.AnswerVM*
- *ManagedEntity.Rename_Task*
- *VirtualMachine.CloneVM_Task*
- *VirtualMachine.PowerOffVM_Task*
- *VirtualMachine.PowerOnVM_Task*
- *VirtualMachine.SuspendVM_Task*
- *VirtualMachine.ResetVM_Task*
- *VirtualMachine.ReconfigVM_Task*
- *VirtualMachine.RelocateVM_Task*
- *VirtualMachine.MigrateVM_Task*
- *VirtualMachine.CustomizeVM_Task*
- *VirtualMachine.ShutdownGuest*
- *VirtualMachine.StandbyGuest*
- *VirtualMachine.RebootGuest*
- *VirtualMachine.CreateSnapshot_Task*
- *VirtualMachine.RemoveAllSnapshots_Task*
- *VirtualMachine.RevertToCurrentSnapshot_Task*
- *VirtualMachine.MarkAsTemplate*
- *VirtualMachine.MarkAsVirtualMachine*
- *VirtualMachine.ResetGuestInformation*
- *VirtualMachine.MountToolsInstaller*
- *VirtualMachine.UnmountToolsInstaller*
- *ManagedEntity.Destroy_Task*
- *VirtualMachine.UpgradeVM_Task*
- *VirtualMachine.ExportVm*
  
This list may include the following operations for a ResourcePool:
- *ResourcePool.ImportVApp*
- *ResourcePool.CreateChildVM_Task*
- *ResourcePool.UpdateConfig*
- *Folder.CreateVM_Task*
- *ManagedEntity.Destroy_Task*
- *ManagedEntity.Rename_Task*
  
This list may include the following operations for a VirtualApp:
- *ManagedEntity.Destroy_Task*
- *VirtualApp.CloneVApp_Task*
- *VirtualApp.unregisterVApp_Task*
- *VirtualApp.ExportVApp*
- *VirtualApp.PowerOnVApp_Task*
- *VirtualApp.PowerOffVApp_Task*
- *VirtualApp.UpdateVAppConfig*
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("config_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "configStatus", doc: Some(r#####"The configStatus indicates whether or not the system has detected a configuration
issue involving this entity.

For example, it might have detected a
duplicate IP address or MAC address, or a host in a cluster
might be out of compliance. The meanings of the configStatus values are:
- red: A problem has been detected involving the entity.
- yellow: A problem is about to occur or a transient condition
  has occurred (For example, reconfigure fail-over policy).
- green: No configuration issues have been detected.
- gray: The configuration status of the entity is not being monitored.
  
A green status indicates only that a problem has not been detected;
it is not a guarantee that the entity is problem-free.

The *configIssue* property contains a list of the
problems that have been detected.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("permission", NodeData { type_decl: "Vec<vim_rs::types::structs::Permission>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfPermission"), path_segment: "permission", doc: Some(r#####"List of permissions defined for this entity.
"#####) },),
        ("host", NodeData { type_decl: "Vec<vim_rs::types::structs::DatastoreHostMount>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfDatastoreHostMount"), path_segment: "host", doc: Some(r#####"Hosts attached to this datastore.
"#####) },),
        ("effective_role", NodeData { type_decl: "Vec<i32>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfInt"), path_segment: "effectiveRole", doc: Some(r#####"Access rights the current session has to this entity.

***Required privileges:*** System.View
"#####) },),
        ("declared_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "declaredAlarmState", doc: Some(r#####"A set of alarm states for alarms that apply to this managed entity.

The set includes alarms defined on this entity
and alarms inherited from the parent entity,
or from any ancestors in the inventory hierarchy.

Alarms are inherited if they can be triggered by this entity or its descendants.
This set does not include alarms that are defined on descendants of this entity.

***Required privileges:*** System.View
"#####) },),
        ("browser", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "browser", doc: Some(r#####"DatastoreBrowser used to browse this datastore.
"#####) },),
        ("triggered_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "triggeredAlarmState", doc: Some(r#####"A set of alarm states for alarms triggered by this entity
or by its descendants.

Triggered alarms are propagated up the inventory hierarchy
so that a user can readily tell when a descendant has triggered an alarm.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.

***Required privileges:*** System.View
"#####) },),
        ("recent_task", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "recentTask", doc: Some(r#####"The set of recent tasks operating on this managed entity.

This is a subset
of *TaskManager.recentTask* belong to this entity. A task in this
list could be in one of the four states: pending, running, success or error.

This property can be used to deduce intermediate power states for
a virtual machine entity. For example, if the current powerState is "poweredOn"
and there is a running task performing the "suspend" operation, then the virtual
machine's intermediate state might be described as "suspending."

Most tasks (such as power operations) obtain exclusive access to the virtual
machine, so it is unusual for this list to contain more than one running task.
One exception, however, is the task of cloning a virtual machine.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("iorm_configuration", NodeData { type_decl: "vim_rs::types::structs::StorageIormInfo", type_name: "StorageIORMInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "iormConfiguration", doc: Some(r#####"Deprecated as of vSphere8.0 U3, and there is no replacement for it.

Configuration of storage I/O resource management for the datastore.

Currently we only support storage I/O resource management on VMFS volumes
of a datastore.

This configuration may not be available if the datastore is not accessible
from any host, or if the datastore does not have VMFS volume.
The configuration can be modified using the method
*StorageResourceManager.ConfigureDatastoreIORM_Task*
"#####) },),
        ("summary", NodeData { type_decl: "vim_rs::types::structs::DatastoreSummary", type_name: "DatastoreSummary", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "summary", doc: Some(r#####"Global properties of the datastore.
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"Name of this entity, unique relative to its parent.

Any / (slash), \\ (backslash), character used in this
name element will be escaped. Similarly, any % (percent) character used in
this name element will be escaped, unless it is used to start an escape
sequence. A slash is escaped as %2F or %2f. A backslash is escaped as %5C or
%5c, and a percent is escaped as %25.

***Required privileges:*** System.View
"#####) },),
    ],
}),
        ("HostBIOSInfo", ::phf::Map {
    key: 14108922650502679131,
    disps: &[
        (2, 4),
        (3, 0),
    ],
    entries: &[
        ("firmware_type", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "firmwareType", doc: Some(r#####"Firmware Type of the host.

The set of supported values is described
in *HostBIOSInfoFirmwareType_enum*

***Since:*** vSphere API Release 8.0.2.0
"#####) },),
        ("firmware_major_release", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "firmwareMajorRelease", doc: None },),
        ("release_date", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveDateTime"), path_segment: "releaseDate", doc: Some(r#####"The release date for the BIOS.
"#####) },),
        ("vendor", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "vendor", doc: Some(r#####"The vendor for the BIOS.
"#####) },),
        ("bios_version", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "biosVersion", doc: Some(r#####"The current BIOS version of the physical chassis
"#####) },),
        ("major_release", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "majorRelease", doc: Some(r#####"BIOS Major Release
"#####) },),
        ("minor_release", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "minorRelease", doc: Some(r#####""BIOS Minor Release
"#####) },),
        ("firmware_minor_release", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "firmwareMinorRelease", doc: Some(r#####"Embedded Controller Firmware Minor Release
"#####) },),
    ],
}),
        ("HostNtpConfig", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("server", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "server", doc: Some(r#####"List of time servers, specified as either IP addresses or
fully qualified domain names (FQDNs).

Each entry may optionally
specify one or more space separated 'server' ntp.conf command options.
Any comments appended to an entry after a '#' will not be retained.
To reset any previously configured servers, submit an NtpConfig
without the server or configFile property set to method
*HostDateTimeSystem.UpdateDateTimeConfig*
"#####) },),
        ("config_file", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "configFile", doc: Some(r#####"Content of ntp.conf host configuration file, split by lines for ntpd version 4.2.8.

Comment lines start with comment marker '#' as per ntp.conf are kept.
When submitting a new ntp commands to this property via
*HostDateTimeSystem.UpdateDateTimeConfig* method, any 'restrict'
or 'drift' commands will be ignored as the those are set to fixed defaults.
"#####) },),
    ],
}),
        ("NetworkSummary", ::phf::Map {
    key: 7485420634051515786,
    disps: &[
        (4, 0),
    ],
    entries: &[
        ("ip_pool_id", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "ipPoolId", doc: Some(r#####"Identifier of the associated IP pool.

Zero if the network is not associated
with an IP pool.
"#####) },),
        ("ip_pool_name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "ipPoolName", doc: Some(r#####"Name of the associated IP pool.

Empty if the network is not associated with an
IP pool.
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"Name of the network.
"#####) },),
        ("accessible", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "accessible", doc: Some(r#####"At least one host is configured to provide this network.
"#####) },),
        ("network", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "network", doc: Some(r#####"Reference to the associated managed object.

Refers instance of *Network*.
"#####) },),
    ],
}),
        ("HostPtpConfig", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("port", NodeData { type_decl: "Vec<vim_rs::types::structs::HostPtpConfigPtpPort>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostPtpConfigPtpPort"), path_segment: "port", doc: Some(r#####"List of PTP port configurations.

See *HostPtpConfigPtpPort*.
"#####) },),
        ("domain", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "domain", doc: Some(r#####"PTP domain number as defined in the IEEE 1588 standard.

Supported
values are in the range 0-255.
"#####) },),
    ],
}),
        ("ClusterDasAdmissionControlPolicy", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("p_mem_admission_control_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "pMemAdmissionControlEnabled", doc: Some(r#####"Deprecated as of vSphere 9.0 APIs with no replacement.

Flag that determines whether strict admission control for persistent
memory is enabled.

By default, this value is false.
This flag can only be set to true if
*ClusterDasConfigInfo.admissionControlEnabled* is set to true.
When you use persistent memory admission control, the following
operations are prevented, if doing so would violate the
*ClusterDasConfigInfo.admissionControlEnabled*.
- Creating a virtual machine with persistent memory.
- Adding a virtual persistent memory device to a virtual machine.
- Increasing the capacity of a virtual persistent memory device.
  
***Since:*** vSphere API Release 7.0.2.0
"#####) },),
        ("resource_reduction_to_tolerate_percent", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "resourceReductionToToleratePercent", doc: Some(r#####"Percentage of resource reduction that a cluster of VMs can tolerate
in case of a failover.
"#####) },),
    ],
}),
        ("StorageDrsPodConfigInfo", ::phf::Map {
    key: 106375038446233661,
    disps: &[
        (2, 0),
        (2, 7),
    ],
    entries: &[
        ("option", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::OptionValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfOptionValue"), path_segment: "option", doc: Some(r#####"Advanced settings.
"#####) },),
        ("enabled", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "enabled", doc: Some(r#####"Flag indicating whether or not storage DRS is enabled.
"#####) },),
        ("rule", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::ClusterRuleInfoTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfClusterRuleInfo"), path_segment: "rule", doc: Some(r#####"Pod-wide rules.
"#####) },),
        ("automation_overrides", NodeData { type_decl: "vim_rs::types::structs::StorageDrsAutomationConfig", type_name: "StorageDrsAutomationConfig", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "automationOverrides", doc: Some(r#####"Configuration settings for fine-grain automation overrides on
the cluster level setting.
"#####) },),
        ("default_intra_vm_affinity", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "defaultIntraVmAffinity", doc: Some(r#####"Specifies whether or not each virtual machine in this pod should have its virtual
disks on the same datastore by default.

If set to true, virtual machines will have
all their virtual disks on the same datastore. If set to false, the virtual disks
of a virtual machine may or may not be placed on the same datastore.
If not set, the default value is true.
You can override the default behavior for a virtual machine
by using the *StorageDrsVmConfigInfo* object.
"#####) },),
        ("io_load_balance_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "ioLoadBalanceEnabled", doc: Some(r#####"Deprecated as of vSphere8.0 U3, and there is no replacement for it.

Flag indicating whether or not storage DRS takes into account storage I/O
workload when making load balancing and initial placement recommendations.
"#####) },),
        ("default_vm_behavior", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "defaultVmBehavior", doc: Some(r#####"Specifies the pod-wide default storage DRS behavior for virtual machines.

For currently supported storage DRS behavior, see *StorageDrsPodConfigInfoBehavior_enum*.
You can override the default behavior for a virtual machine
by using the *StorageDrsVmConfigInfo* object.
"#####) },),
        ("space_load_balance_config", NodeData { type_decl: "vim_rs::types::structs::StorageDrsSpaceLoadBalanceConfig", type_name: "StorageDrsSpaceLoadBalanceConfig", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "spaceLoadBalanceConfig", doc: Some(r#####"The configuration settings for load balancing storage space.
"#####) },),
        ("io_load_balance_config", NodeData { type_decl: "vim_rs::types::structs::StorageDrsIoLoadBalanceConfig", type_name: "StorageDrsIoLoadBalanceConfig", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "ioLoadBalanceConfig", doc: Some(r#####"Deprecated as of vSphere8.0 U3, and there is no replacement for it.

The configuration settings for load balancing I/O workload.

This takes effect only if *StorageDrsPodConfigInfo.ioLoadBalanceEnabled* is <code>true</code>.
"#####) },),
        ("load_balance_interval", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "loadBalanceInterval", doc: Some(r#####"Specify the interval that storage DRS runs to load balance among datastores
within a storage pod.

Unit: minute.
The valid values are from 60 (1 hour) to 43200 (30 days).
If not specified, the default value is 480 (8 hours).
"#####) },),
    ],
}),
        ("HostStorageDeviceInfo", ::phf::Map {
    key: 2980949210194914378,
    disps: &[
        (3, 0),
        (4, 3),
    ],
    entries: &[
        ("scsi_topology", NodeData { type_decl: "vim_rs::types::structs::HostScsiTopology", type_name: "HostScsiTopology", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "scsiTopology", doc: Some(r#####"Storage topology view of SCSI storage devices.

This data object
exists only if storage topology information is available. See the
*ScsiTopology* data object type for
more information.
"#####) },),
        ("software_internet_scsi_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "softwareInternetScsiEnabled", doc: Some(r#####"Indicates if the software iSCSI initiator is enabled on this system
"#####) },),
        ("plug_store_topology", NodeData { type_decl: "vim_rs::types::structs::HostPlugStoreTopology", type_name: "HostPlugStoreTopology", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "plugStoreTopology", doc: Some(r#####"The plug-store topology on the host system.

This data object exists only if
the plug-store system is available and configurable.
"#####) },),
        ("multipath_info", NodeData { type_decl: "vim_rs::types::structs::HostMultipathInfo", type_name: "HostMultipathInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "multipathInfo", doc: Some(r#####"The multipath configuration that controls multipath policy for ScsiLuns.

This data object exists only if path information is available and is
configurable.
"#####) },),
        ("nvme_topology", NodeData { type_decl: "vim_rs::types::structs::HostNvmeTopology", type_name: "HostNvmeTopology", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "nvmeTopology", doc: Some(r#####"Topology view of NVME storage devices.

This data object exists
only if storage topology information is available. See the
*HostNvmeTopology* data object type for more information.
"#####) },),
        ("scsi_lun", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::ScsiLunTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfScsiLun"), path_segment: "scsiLun", doc: Some(r#####"The list of SCSI logical units available on the host.
"#####) },),
        ("host_bus_adapter", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::HostHostBusAdapterTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostHostBusAdapter"), path_segment: "hostBusAdapter", doc: Some(r#####"The list of host bus adapters available on the host.
"#####) },),
    ],
}),
        ("HostVMotionNetConfig", ::phf::Map {
    key: 7485420634051515786,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("candidate_vnic", NodeData { type_decl: "Vec<vim_rs::types::structs::HostVirtualNic>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostVirtualNic"), path_segment: "candidateVnic", doc: Some(r#####"List of VirtualNic objects that may be used for VMotion.

This will be a subset of the list of VirtualNics in
*HostNetworkInfo.vnic*.
"#####) },),
        ("selected_vnic", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "selectedVnic", doc: Some(r#####"VirtualNic that is selected for use in VMotion operations.
"#####) },),
    ],
}),
        ("VirtualMachineMemoryReservationInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("virtual_machine_max", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "virtualMachineMax", doc: Some(r#####"The maximum amount of memory reserved for all running virtual machines,
in bytes.
"#####) },),
        ("allocation_policy", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "allocationPolicy", doc: Some(r#####"Policy for allocating additional memory for virtual machines.

See also *VirtualMachineMemoryAllocationPolicy_enum*.
"#####) },),
        ("virtual_machine_min", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "virtualMachineMin", doc: Some(r#####"The minimum amount of memory reserved for all running virtual machines,
in bytes.
"#####) },),
        ("virtual_machine_reserved", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "virtualMachineReserved", doc: Some(r#####"The amount of memory reserved for all running virtual machines,
in bytes.
"#####) },),
    ],
}),
        ("DVSSummary", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (7, 8),
        (4, 0),
        (1, 8),
    ],
    entries: &[
        ("portgroup_name", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "portgroupName", doc: Some(r#####"The names of the portgroups that are defined on the switch.
"#####) },),
        ("product_info", NodeData { type_decl: "vim_rs::types::structs::DistributedVirtualSwitchProductSpec", type_name: "DistributedVirtualSwitchProductSpec", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "productInfo", doc: Some(r#####"The product information for the implementation of the switch.
"#####) },),
        ("host_member", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "hostMember", doc: Some(r#####"The names of the hosts that join the switch.

Refers instances of *HostSystem*.
"#####) },),
        ("uuid", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "uuid", doc: Some(r#####"The generated UUID of the switch.
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"The name of the switch.
"#####) },),
        ("num_hosts", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "numHosts", doc: Some(r#####"The number of hosts in the switch.

The value of this property
is not affected by the privileges granted to the current user.
"#####) },),
        ("vm", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "vm", doc: Some(r#####"The Virtual Machines with Virtual NICs that connect to the switch.

In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter.
Since this property is on a DataObject, an update returned by WaitForUpdatesEx may
contain values for this property when some other property on the DataObject changes.
If this update is a result of a call to WaitForUpdatesEx with a non-empty
version parameter, the value for this property may not be current.

Refers instances of *VirtualMachine*.
"#####) },),
        ("num_ports", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "numPorts", doc: Some(r#####"Current number of ports, not including conflict ports.
"#####) },),
        ("description", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "description", doc: Some(r#####"A description string of the switch.
"#####) },),
        ("host", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "host", doc: Some(r#####"The hosts with Virtual NICs that connect to the switch.

Refers instances of *HostSystem*.
"#####) },),
        ("contact", NodeData { type_decl: "vim_rs::types::structs::DvsContactInfo", type_name: "DVSContactInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "contact", doc: Some(r#####"The human operator contact information.
"#####) },),
    ],
}),
        ("ClusterDasConfigInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (5, 9),
        (3, 0),
        (0, 0),
    ],
    entries: &[
        ("host_monitoring", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "hostMonitoring", doc: Some(r#####"Determines whether HA restarts virtual machines after a host fails.

The default value is
*ClusterDasConfigInfoServiceState_enum*.*enabled*.
This property is meaningful only when
*ClusterDasConfigInfo*.*ClusterDasConfigInfo.enabled* is <code>true</code>.

When <code>hostMonitoring</code> is
*enabled*, HA restarts virtual machines
after a host fails.

When <code>hostMonitoring</code> is
*disabled*, HA does not restart
virtual machines after a host fails.
The status of Host Monitoring does not affect other services such
as virtual machine Health Monitoring or Fault Tolerance.
The rest of the cluster operations follow normal processing.
No configuration information is lost and re-enabling the service
is a quick operation.
"#####) },),
        ("admission_control_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "admissionControlEnabled", doc: Some(r#####"Flag that determines whether strict admission control is enabled.

When you use admission control, the following operations are
prevented, if doing so would violate the *ClusterDasConfigInfo.admissionControlPolicy*.
- Powering on a virtual machine in the cluster.
- Migrating a virtual machine into the cluster.
- Increasing the CPU or memory reservation of powered-on
  virtual machines in the cluster.
  
With admission control disabled, there is no assurance that
all virtual machines in the HA cluster can be restarted after
a host failure. VMware recommends that you do not disable
admission control, but you might need to do so temporarily,
for the following reasons:
- If you need to violate the failover constraints when there
  are not enough resources to support them (for example,
  if you are placing hosts in standby mode to test them
  for use with DPM).
- If an automated process needs to take actions that might
  temporarily violate the failover constraints (for example,
  as part of an upgrade directed by VMware Update Manager).
- If you need to perform testing or maintenance operations.
"#####) },),
        ("vm_component_protecting", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "vmComponentProtecting", doc: Some(r#####"This property indicates if vSphere HA VM Component Protection service
is enabled.

The default value is *disabled*.

When <code>vmComponentProtecting</code> is set to
*disabled*, reaction to all types of VM
component failures is disabled.

When <code>vmComponentProtecting</code> is set to
*enabled*, VM Component Protection service
will detect and react to component failures. The actual reaction is determined
by *ClusterVmComponentProtectionSettings* which is referenced by both cluster
level configuration (*ClusterDasConfigInfo.defaultVmSettings*) and per-VM
override *ClusterConfigInfoEx.dasVmConfig*.
"#####) },),
        ("default_vm_settings", NodeData { type_decl: "vim_rs::types::structs::ClusterDasVmSettings", type_name: "ClusterDasVmSettings", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "defaultVmSettings", doc: Some(r#####"Cluster-wide defaults for virtual machine HA settings.

When a virtual machine has no HA configuration
(*ClusterDasVmConfigSpec*), it uses the values
specified here.
"#####) },),
        ("vm_monitoring", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "vmMonitoring", doc: Some(r#####"Level of HA Virtual Machine Health Monitoring Service.

You can monitor both guest and application heartbeats, guest heartbeats only,
or you can disable the service. See *ClusterDasConfigInfoVmMonitoringState_enum*.
The default value is *vmMonitoringDisabled*.

The Service level specified for the cluster determines
the possible monitoring settings that you can use for individual virtual machines.
See *ClusterVmToolsMonitoringSettings*.*ClusterVmToolsMonitoringSettings.vmMonitoring*.
"#####) },),
        ("h_b_datastore_candidate_policy", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "hBDatastoreCandidatePolicy", doc: Some(r#####"The policy on what datastores will be used by vCenter Server to choose
heartbeat datastores.

See *ClusterDasConfigInfoHBDatastoreCandidate_enum* for all options.
The default value is
*allFeasibleDsWithUserPreference*.
"#####) },),
        ("heartbeat_datastore", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "heartbeatDatastore", doc: Some(r#####"The list of preferred datastores to use for storage heartbeating.

Each of the specified datastores should be active and mounted
by more than one host. There is no limit on the number of specified
datastores and no priority among them.
The specified datastores will replace those previously specified and
an empty list will delete all such earlier specified ones.

vCenter Server chooses the heartbeat datastores for a host from the
set specified by *ClusterDasConfigInfo.hBDatastoreCandidatePolicy*.
The choice is made based on datastore connectivity and storage array
redundancy (in case of VMFS).

The final set of selected heartbeat datastores is reported via
*ClusterDasAdvancedRuntimeInfo.heartbeatDatastoreInfo*.

Refers instances of *Datastore*.
"#####) },),
        ("failover_level", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "failoverLevel", doc: Some(r#####"Deprecated as of vSphere API 4.0, use
*ClusterFailoverLevelAdmissionControlPolicy* to set
*ClusterDasConfigInfo.admissionControlPolicy*.

Configured failover level.

This is the number of physical host failures
that can be tolerated without impacting the ability to satisfy the
minimums for all running virtual machines. Acceptable values range from one to
four.
"#####) },),
        ("admission_control_policy", NodeData { type_decl: "Box<dyn vim_rs::types::traits::ClusterDasAdmissionControlPolicyTrait>", type_name: "ClusterDasAdmissionControlPolicy", is_optional: true, processing_type: FieldProcessingType::Trait, path_segment: "admissionControlPolicy", doc: Some(r#####"Virtual machine admission control policy for vSphere HA.

The policies specify resource availability for failover support.
- Failover host admission policy
  *ClusterFailoverHostAdmissionControlPolicy* -
  specify one or more dedicated failover hosts.
- Failover level policy
  *ClusterFailoverLevelAdmissionControlPolicy* -
  the limit of host failures for which resources are reserved.
  When you use the failover level policy,
  HA partitions resources into slots. A slot represents the minimum
  CPU and memory resources that are required to support
  any powered on virtual machine in the cluster.
  To retrieve information about partitioned resources, use the
  *ClusterComputeResource.RetrieveDasAdvancedRuntimeInfo*
  method.
- Resources admission policy
  *ClusterFailoverResourcesAdmissionControlPolicy* -
  CPU and memory resources reserved for failover support.
  When you use the resources policy, you can reserve
  a percentage of the aggregate cluster resource for failover.
"#####) },),
        ("option", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::OptionValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfOptionValue"), path_segment: "option", doc: Some(r#####"Advanced settings.
"#####) },),
        ("enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "enabled", doc: Some(r#####"Flag to indicate whether or not vSphere HA feature is enabled.
"#####) },),
    ],
}),
        ("HostRuntimeInfo", ::phf::Map {
    key: 2980949210194914378,
    disps: &[
        (14, 0),
        (3, 0),
        (5, 11),
        (1, 4),
    ],
    entries: &[
        ("vsan_runtime_info", NodeData { type_decl: "vim_rs::types::structs::VsanHostRuntimeInfo", type_name: "VsanHostRuntimeInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "vsanRuntimeInfo", doc: Some(r#####"Host Runtime information related to the VSAN service.

See also *VsanHostRuntimeInfo*.
"#####) },),
        ("host_max_virtual_disk_capacity", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "hostMaxVirtualDiskCapacity", doc: Some(r#####"The maximum theoretical virtual disk capacity supported by this host
"#####) },),
        ("in_maintenance_mode", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "inMaintenanceMode", doc: Some(r#####"The flag to indicate whether or not the host is in maintenance mode.

This
flag is set when the host has entered the maintenance mode. It is not set
during the entering phase of maintenance mode.

See also *HostSystem.EnterMaintenanceMode_Task*, *HostSystem.ExitMaintenanceMode_Task*.
"#####) },),
        ("v_flash_resource_runtime_info", NodeData { type_decl: "vim_rs::types::structs::HostVFlashManagerVFlashResourceRunTimeInfo", type_name: "HostVFlashManagerVFlashResourceRunTimeInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "vFlashResourceRuntimeInfo", doc: Some(r#####"Runtime information of vFlash resource of the host.
"#####) },),
        ("network_runtime_info", NodeData { type_decl: "vim_rs::types::structs::HostRuntimeInfoNetworkRuntimeInfo", type_name: "HostRuntimeInfoNetworkRuntimeInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "networkRuntimeInfo", doc: Some(r#####"This property is for getting network related runtime info
"#####) },),
        ("stateless_nvds_migration_ready", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "statelessNvdsMigrationReady", doc: Some(r#####"Indicating the host is ready for NVDS to VDS migration.

See *HostRuntimeInfoStatelessNvdsMigrationState_enum* for supported values.

***Since:*** vSphere API Release 7.0.2.0
"#####) },),
        ("connection_state", NodeData { type_decl: "vim_rs::types::enums::HostSystemConnectionStateEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("HostSystemConnectionState"), path_segment: "connectionState", doc: Some(r#####"The host connection state.

See the description in the enums for the
*ConnectionState* data object type.
"#####) },),
        ("in_quarantine_mode", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "inQuarantineMode", doc: Some(r#####"The flag to indicate whether or not the host is in quarantine mode.

InfraUpdateHa will recommend to set this flag based on the HealthUpdates
received by the HealthUpdateProviders configured for the cluster. A
host that is reported as degraded will be recommended to enter quarantine
mode, while a host that is reported as healthy will be recommended to
exit quarantine mode. Execution of these recommended actions will set
this flag. Hosts in quarantine mode will be avoided by vSphere DRS as
long as the increased consolidation in the cluster does not negatively
affect VM performance.

See also *HealthUpdateManager*, *ClusterInfraUpdateHaConfigInfo*, *ClusterHostInfraUpdateHaModeAction*.
"#####) },),
        ("crypto_key_id", NodeData { type_decl: "vim_rs::types::structs::CryptoKeyId", type_name: "CryptoKeyId", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "cryptoKeyId", doc: Some(r#####"Crypto Key used for coredump encryption
"#####) },),
        ("state_encryption", NodeData { type_decl: "vim_rs::types::structs::HostRuntimeInfoStateEncryptionInfo", type_name: "HostRuntimeInfoStateEncryptionInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "stateEncryption", doc: Some(r#####"Host persistent state encryption information.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("tpm_pcr_values", NodeData { type_decl: "Vec<vim_rs::types::structs::HostTpmDigestInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostTpmDigestInfo"), path_segment: "tpmPcrValues", doc: Some(r#####"Deprecated as of @released("5.1") this information should be
considered to be neither complete nor reliable.

The array of PCR digest values stored in the TPM device since the last
host boot time.
"#####) },),
        ("crypto_state", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "cryptoState", doc: Some(r#####"Encryption state of the host.

Valid values are enumerated by the
*CryptoState* type.
"#####) },),
        ("power_state", NodeData { type_decl: "vim_rs::types::enums::HostSystemPowerStateEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("HostSystemPowerState"), path_segment: "powerState", doc: Some(r#####"The host power state.

See the description in the enums for the
*PowerState* data object type.
"#####) },),
        ("standby_mode", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "standbyMode", doc: Some(r#####"The host's standby mode.

For valid values see
*HostStandbyMode_enum*. The property is only populated by
vCenter server. If queried directly from a ESX host, the property is
is unset.
"#####) },),
        ("partial_maintenance_mode", NodeData { type_decl: "Vec<vim_rs::types::structs::HostPartialMaintenanceModeRuntimeInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostPartialMaintenanceModeRuntimeInfo"), path_segment: "partialMaintenanceMode", doc: Some(r#####"The following list contains the runtime status for all the partial
maintenance modes currently supported on the host.

***Since:*** vSphere API Release 8.0.3.0
"#####) },),
        ("boot_time", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveDateTime"), path_segment: "bootTime", doc: Some(r#####"The time when the host was booted.
"#####) },),
        ("health_system_runtime", NodeData { type_decl: "vim_rs::types::structs::HealthSystemRuntime", type_name: "HealthSystemRuntime", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "healthSystemRuntime", doc: Some(r#####"Available system health status
"#####) },),
        ("das_host_state", NodeData { type_decl: "vim_rs::types::structs::ClusterDasFdmHostState", type_name: "ClusterDasFdmHostState", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "dasHostState", doc: Some(r#####"The availability state of an active host in a vSphere HA enabled
cluster.

A host is inactive if it is in maintenance or standby mode, or
it has been disconnected from vCenter Server. The active hosts in a cluster
form a vSphere HA fault domain.

The property is unset if vSphere HA is disabled, the host is
in maintenance or standby mode, or the host is disconnected from
vCenter Server. The property is set to hostDown if the host has crashed.
"#####) },),
    ],
}),
        ("HostSecuritySpec", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("admin_password", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "adminPassword", doc: Some(r#####"Administrator password to configure
"#####) },),
        ("add_permission", NodeData { type_decl: "Vec<vim_rs::types::structs::Permission>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfPermission"), path_segment: "addPermission", doc: Some(r#####"Permissions to add
"#####) },),
        ("remove_permission", NodeData { type_decl: "Vec<vim_rs::types::structs::Permission>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfPermission"), path_segment: "removePermission", doc: Some(r#####"Permissions to remove
"#####) },),
    ],
}),
        ("HostCpuSchedulerInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("policy", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "policy", doc: Some(r#####"The *policy* active for CPU Scheduling.
"#####) },),
    ],
}),
        ("HostGraphicsConfig", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("host_default_graphics_type", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "hostDefaultGraphicsType", doc: Some(r#####"The host default graphics type.

See *HostGraphicsConfigGraphicsType_enum* for list
of supported values. This default value can be overridden by specifying
graphics type for an individual device. If host supports a single
graphics type, specifying an individual graphics device is optional.
"#####) },),
        ("shared_passthru_assignment_policy", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "sharedPassthruAssignmentPolicy", doc: Some(r#####"The policy for assigning shared passthrough VMs to a host graphics
device.

See *HostGraphicsConfigSharedPassthruAssignmentPolicy_enum* for list of
supported values.
"#####) },),
        ("device_type", NodeData { type_decl: "Vec<vim_rs::types::structs::HostGraphicsConfigDeviceType>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostGraphicsConfigDeviceType"), path_segment: "deviceType", doc: Some(r#####"Graphics devices and their associated type.
"#####) },),
    ],
}),
        ("DVSRuntimeInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("host_member_runtime", NodeData { type_decl: "Vec<vim_rs::types::structs::HostMemberRuntimeInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostMemberRuntimeInfo"), path_segment: "hostMemberRuntime", doc: Some(r#####"Runtime information of the hosts that joined the switch.
"#####) },),
        ("resource_runtime_info", NodeData { type_decl: "vim_rs::types::structs::DvsResourceRuntimeInfo", type_name: "DvsResourceRuntimeInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "resourceRuntimeInfo", doc: Some(r#####"The bandwidth reservation information for the switch.
"#####) },),
    ],
}),
        ("ClusterDasData", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
    ],
    entries: &[
    ],
}),
        ("ApplyHostProfileConfigurationSpec", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
        (5, 6),
        (2, 5),
    ],
    entries: &[
        ("inapplicable_path", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "inapplicablePath", doc: Some(r#####"List of property paths.

Each path identifies a policy that does not apply
to this host. For example, if the precheck policies for a port group are not satisfied,
the port group will not be created when you apply the profile to the host.
Based on this information, the client might not display that part of the profile tree.
"#####) },),
        ("task_description", NodeData { type_decl: "Vec<vim_rs::types::structs::LocalizableMessage>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfLocalizableMessage"), path_segment: "taskDescription", doc: Some(r#####"Description of tasks that will be performed on the host
to carry out HostProfile application.
"#####) },),
        ("reboot_host", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "rebootHost", doc: Some(r#####"For regular apply, when some of the tasks requires reboot,
that this variable is<code>true</code> indicates that the
reboot automatically happens in the batch profile apply
than that the user will manually reboot the system later.

For stateless host, this variable takes effect only when
the variable <code>rebootStateless</code> above is
<code>false</code>.
"#####) },),
        ("fault_data", NodeData { type_decl: "vim_rs::types::structs::MethodFault", type_name: "MethodFault", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "faultData", doc: Some(r#####"This contains the error details.
"#####) },),
        ("task_list_requirement", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "taskListRequirement", doc: Some(r#####"The task requirements from the results of
*HostProfileManager.GenerateConfigTaskList* method
"#####) },),
        ("reboot_stateless", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "rebootStateless", doc: Some(r#####"For a stateless host, there are two approaches to apply a host
profile:
(1) Reboot the host and apply the host profile at boot time.

(2) Apply the host profile directly from VC. We call this as
regular apply.
The variable rebootStateless allows users to choose the first
approach from the two approaches above:
apply host profile by rebooting this host.
"#####) },),
        ("require_input", NodeData { type_decl: "Vec<vim_rs::types::structs::ProfileDeferredPolicyOptionParameter>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfProfileDeferredPolicyOptionParameter"), path_segment: "requireInput", doc: Some(r#####"List that describes the required input for host configuration and identifies
any policy options that still require parameter data.

Each entry in the list
specifies the path to a policy and a parameter list. If the call to
*HostProfile.ExecuteHostProfile* includes deferred parameters,
the <code>requireInput</code> entries
(<code>requireInput\[\].</code>*ProfileDeferredPolicyOptionParameter.parameter*\[\])
will be populated with the parameter data that was passed to the execute method.
For policies that still require input data, the parameter list in the corresponding
entry will be null.

A vSphere client that displays a GUI can use this information to show the host-specific
configuration policy options. The client can highlight required input fields
and ask the user for data in increments instead of collecting all of the input at once.
For example, in the first pass, the client collects a minimum of user input and
sends that to the Server. The Server evaluates the profile and might decide to
invalidate a particular part of the subtree or enable a new
subtree in the profile. This would result in a new set of invalid paths
(*ProfileExecuteResult.inapplicablePath*\[\]) and
required input property paths
(*ProfileDeferredPolicyOptionParameter*.*ProfileDeferredPolicyOptionParameter.inputPath*).
The client can make a series of calls to the method until it achieves a success status.

When *HostProfile.ExecuteHostProfile* returns a success status,
the <code>requireInput</code> list contains the complete list of parameters,
consisting of the following data:
- Deferred parameter values resolved through successive calls to
  *HostProfile.ExecuteHostProfile*.
- Default parameter values from the host configuration.
- User-specified values that override the defaults.
  
You can specify the returned <code>requireInput</code> list in the
<code>userInput</code> parameter to the
*HostProfileManager*.*HostProfileManager.ApplyHostConfig_Task*
method. The Server will use the list to update the *AnswerFile*
associated with the host.
"#####) },),
        ("host", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "host", doc: Some(r#####"The host to be remediated.

Refers instance of *HostSystem*.
"#####) },),
        ("error", NodeData { type_decl: "Vec<vim_rs::types::structs::ProfileExecuteError>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfProfileExecuteError"), path_segment: "error", doc: Some(r#####"List of errors that were encountered during execute.

This field will be set if status is set to error.
"#####) },),
        ("config_spec", NodeData { type_decl: "vim_rs::types::structs::HostConfigSpec", type_name: "HostConfigSpec", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "configSpec", doc: Some(r#####"Host configuration specification.

This data is valid only if
the <code>status</code> value is <code>success</code>.
See *ProfileExecuteResultStatus_enum*.

Use this data object when you apply the configuration
to a host. See the <code>configSpec</code> parameter to the
*HostProfileManager*.*HostProfileManager.ApplyHostConfig_Task*
method.
"#####) },),
        ("status", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "status", doc: Some(r#####"Status of the profile execution operation.

The value is a string that contains
one of the *ProfileExecuteResultStatus_enum* enumerations.
"#####) },),
    ],
}),
        ("ComputeResourceSummary", ::phf::Map {
    key: 471159234146692604,
    disps: &[
        (8, 0),
        (3, 4),
    ],
    entries: &[
        ("overall_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "overallStatus", doc: Some(r#####"Overall alarm status.

In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter.
Since this property is on a DataObject, an update returned by WaitForUpdatesEx may
contain values for this property when some other property on the DataObject changes.
If this update is a result of a call to WaitForUpdatesEx with a non-empty
version parameter, the value for this property may not be current.
"#####) },),
        ("total_cpu", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "totalCpu", doc: Some(r#####"Aggregated CPU resources of all hosts, in MHz.
"#####) },),
        ("num_hosts", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "numHosts", doc: Some(r#####"Total number of hosts.
"#####) },),
        ("effective_cpu", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "effectiveCpu", doc: Some(r#####"Effective CPU resources (in MHz) available to run virtual machines.

This is the
aggregated effective resource level from all running hosts. Hosts that are in
maintenance mode or are unresponsive are not counted. Resources used by the
VMware Service Console are not included in the aggregate. This value represents
the amount of resources available for the root resource pool for running
virtual machines.
"#####) },),
        ("effective_memory", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "effectiveMemory", doc: Some(r#####"Effective memory resources (in MB) available to run virtual machines.

This is the aggregated effective resource level from all running hosts. Hosts
that are in maintenance mode or are unresponsive are not counted.
Resources used by the VMware Service Console are not included in the aggregate.
This value represents the amount of resources available for the root
resource pool for running virtual machines.
"#####) },),
        ("num_effective_hosts", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "numEffectiveHosts", doc: Some(r#####"Total number of effective hosts.
"#####) },),
        ("total_memory", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "totalMemory", doc: Some(r#####"Aggregated memory resources of all hosts, in bytes.
"#####) },),
        ("num_cpu_threads", NodeData { type_decl: "i16", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveShort"), path_segment: "numCpuThreads", doc: Some(r#####"Aggregated number of CPU threads.
"#####) },),
        ("num_cpu_cores", NodeData { type_decl: "i16", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveShort"), path_segment: "numCpuCores", doc: Some(r#####"Number of physical CPU cores.

Physical CPU cores are the processors contained
by a CPU package.
"#####) },),
    ],
}),
        ("HostIpConfig", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (3, 0),
    ],
    entries: &[
        ("ip_address", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "ipAddress", doc: Some(r#####"The IP address currently used by the network adapter.

All IP addresses are specified using IPv4 dot notation.
For example, "192.168.0.1". Subnet addresses and netmasks are
specified using the same notation.

**Note**: When DHCP is enabled, this property reflects the
current IP configuration and cannot be set. When DHCP is not
enabled, this property can be set explicitly.
"#####) },),
        ("dhcp", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "dhcp", doc: Some(r#####"The flag to indicate whether or not DHCP (dynamic host
control protocol) is enabled.

If this property is set to true,
the ipAddress and the subnetMask strings cannot be set explicitly.
"#####) },),
        ("ip_v_6_config", NodeData { type_decl: "vim_rs::types::structs::HostIpConfigIpV6AddressConfiguration", type_name: "HostIpConfigIpV6AddressConfiguration", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "ipV6Config", doc: Some(r#####"The ipv6 configuration
"#####) },),
        ("subnet_mask", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "subnetMask", doc: Some(r#####"The subnet mask.

**Note**: When DHCP is not enabled, this property can be
set explicitly. When DHCP is enabled, this property reflects the
current IP configuration and cannot be set.
"#####) },),
    ],
}),
        ("HostSgxRegistrationInfo", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (5, 0),
        (0, 3),
    ],
    entries: &[
        ("bios_error", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "biosError", doc: Some(r#####"BIOS error related to SGX host registration.

Set only if SGX registration status is incomplete.
"#####) },),
        ("registration_url", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "registrationUrl", doc: Some(r#####"SGX host registration URL.

Unset if SGX registration status is not applicable
or in case of an internal error.
"#####) },),
        ("last_registered_time", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveDateTime"), path_segment: "lastRegisteredTime", doc: Some(r#####"Timestamp of last successful registration
from this vCenter.
"#####) },),
        ("r#type", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "type", doc: Some(r#####"SGX host registration type.

Valid values come from *HostSgxRegistrationInfoRegistrationType_enum* enum.
Unset if SGX registration status is not applicable,
complete, or in case of an internal error.
"#####) },),
        ("ppid", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "ppid", doc: Some(r#####"Platform Provisioning ID (PPID).

Hex-encoded representation of the the PPID
(Platform Provisioning ID), returned as the response
to a successful registration request. This field
is populated only on the vCenter through which
the host has been registered.
"#####) },),
        ("status", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "status", doc: Some(r#####"SGX host registration status.

Valid values come from *HostSgxRegistrationInfoRegistrationStatus_enum* enum.
Set, except in case of an internal error.
"#####) },),
    ],
}),
        ("HostIpmiInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("login", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "login", doc: Some(r#####"User ID for logging into the BMC.

BMC usernames may be up to 16
characters and must be null terminated. Hence, a login comprises
17 or fewer characters.
"#####) },),
        ("bmc_ip_address", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "bmcIpAddress", doc: Some(r#####"IP address of the BMC on the host.

It should be null terminated.
"#####) },),
        ("password", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "password", doc: Some(r#####"Password for logging into the BMC.

Only used for configuration, returned as unset
while reading. The password can be up to 16 characters and must be null
terminated. Hence, a password comprises 17 or fewer characters.
"#####) },),
        ("bmc_mac_address", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "bmcMacAddress", doc: Some(r#####"MAC address of the BMC on the host.

The MAC address should be of the
form xx:xx:xx:xx:xx:xx where each x is a hex digit. It should be null
terminated.
"#####) },),
    ],
}),
        ("VirtualMachineSgxInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (3, 0),
    ],
    entries: &[
        ("epc_size", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "epcSize", doc: Some(r#####"Size of vEPC, in megabytes.
"#####) },),
        ("flc_mode", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "flcMode", doc: Some(r#####"FLC mode for the virtual machine.

The set of possible values are
described in *VirtualMachineSgxInfoFlcModes_enum*. If no value is specified,
then "unlocked" will be used.
"#####) },),
        ("le_pub_key_hash", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "lePubKeyHash", doc: Some(r#####"Public key hash of the provider launch enclave.

This is the SHA256
digest of the SIGSTRUCT.MODULUS(MR\_SIGNER) of the provider launch
enclave. This hash must only be provided when the launch enclave mode is
"locked", for the other cases the hash is ignored.
"#####) },),
        ("require_attestation", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "requireAttestation", doc: Some(r#####"Indicates whether or not a virtual machine requires remote
attestation.

***Since:*** vSphere API Release 8.0.0.1
"#####) },),
    ],
}),
        ("ClusterDasVmSettings", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("vm_tools_monitoring_settings", NodeData { type_decl: "vim_rs::types::structs::ClusterVmToolsMonitoringSettings", type_name: "ClusterVmToolsMonitoringSettings", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "vmToolsMonitoringSettings", doc: Some(r#####"Configuration for the VM Health Monitoring Service.
"#####) },),
        ("vm_component_protection_settings", NodeData { type_decl: "vim_rs::types::structs::ClusterVmComponentProtectionSettings", type_name: "ClusterVmComponentProtectionSettings", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "vmComponentProtectionSettings", doc: Some(r#####"Configuration for the VM Component Protection Service.
"#####) },),
        ("isolation_response", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "isolationResponse", doc: Some(r#####"Indicates whether or not the virtual machine should be powered off if a
host determines that it is isolated from the rest of the compute
resource.

If not specified at either the cluster level or
the virtual machine level, this will default to <code>powerOff</code>.

See also *ClusterDasVmSettingsIsolationResponse_enum*.
"#####) },),
        ("restart_priority_timeout", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "restartPriorityTimeout", doc: Some(r#####"This setting is used to specify a maximum time the lower priority VMs
should wait for the higher priority VMs to be ready.

If the higher
priority Vms are not ready by this time, then the lower priority VMs
are restarted irrespective of the VM ready state. This timeout can be
used to prevent the failover of lower priority VMs to be stuck
infinitely.

This timeout is not used if ready condition is
*none*

Timeout specified in seconds. To use cluster setting for a VM override,
set to -1 in per-VM.
setting.
"#####) },),
        ("restart_priority", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "restartPriority", doc: Some(r#####"Restart priority for a virtual machine.

If not specified at either the cluster level or
the virtual machine level, this will default to <code>medium</code>.

See also *ClusterDasVmSettingsRestartPriority_enum*.
"#####) },),
    ],
}),
        ("VirtualMachineConfigInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (2, 24),
        (0, 7),
        (0, 41),
        (0, 13),
        (0, 80),
        (0, 2),
        (0, 35),
        (14, 39),
        (0, 82),
        (0, 57),
        (0, 0),
        (0, 45),
        (19, 74),
        (45, 46),
        (0, 81),
        (1, 14),
        (3, 16),
        (1, 0),
    ],
    entries: &[
        ("sev_snp_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "sevSnpEnabled", doc: Some(r#####"SEV-SNP (Secure Encrypted Virtualization Secure Nested paging) enabled or
not.

SEV-SNP is enabled when set to true, and disabled otherwise.

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("v_app_config", NodeData { type_decl: "Box<dyn vim_rs::types::traits::VmConfigInfoTrait>", type_name: "VmConfigInfo", is_optional: true, processing_type: FieldProcessingType::Trait, path_segment: "vAppConfig", doc: Some(r#####"vApp meta-data for the virtual machine
"#####) },),
        ("vcpu_config", NodeData { type_decl: "Vec<vim_rs::types::structs::VirtualMachineVcpuConfig>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVirtualMachineVcpuConfig"), path_segment: "vcpuConfig", doc: Some(r#####"Vcpu configuration.

The <code>vcpuConfig</code> array is indexed
by vcpu number.
"#####) },),
        ("tools", NodeData { type_decl: "vim_rs::types::structs::ToolsConfigInfo", type_name: "ToolsConfigInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "tools", doc: Some(r#####"Configuration of VMware Tools running in the guest operating system.
"#####) },),
        ("pmem_failover_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "pmemFailoverEnabled", doc: Some(r#####"Deprecated as of vSphere 9.0 APIs with no replacement.

Property to indicate PMem HA failover configuration.

\- When set to TRUE, VMs configured to use PMem
will be failed over to other hosts by HA, but the data
in NVDIMM is not persistent.
\- When set to FALSE, VMs configured to use PMem will not
be failed over to other hosts by HA.
Property is currently only applicable to VMs with NVDimms and
will fail to set True if vPMem disks are present.

***Since:*** vSphere API Release 7.0.2.0
"#####) },),
        ("content_lib_item_info", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineContentLibraryItemInfo", type_name: "VirtualMachineContentLibraryItemInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "contentLibItemInfo", doc: Some(r#####"Content Library Item info.
"#####) },),
        ("npiv_temporary_disabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "npivTemporaryDisabled", doc: Some(r#####"This property is used to enable or disable the NPIV capability on a desired
virtual machine on a temporary basis.

When this property is set NPIV Vport
will not be instantiated by the VMX process of the Virtual Machine. When this
property is set port WWNs and node WWNs in the VM configuration are preserved.
"#####) },),
        ("vmx_runtime_config", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::OptionValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfOptionValue"), path_segment: "vmxRuntimeConfig", doc: Some(r#####"Contains properties that are established when the VM
powers-on and are later examined when the VM is resumed to ensure that
the VM is compatible with the suspended device state.

It is only populated while the VM is powered-on.

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("npiv_world_wide_name_type", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "npivWorldWideNameType", doc: Some(r#####"The source that provides/generates the assigned WWNs.

See also *VirtualMachineConfigInfoNpivWwnType_enum*.
"#####) },),
        ("v_flash_cache_reservation", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "vFlashCacheReservation", doc: Some(r#####"Deprecated since vSphere 7.0 because vFlash Read Cache
end of availability.

Specifies the total vFlash resource reservation for the vFlash caches associated
with this VM's virtual disks, in bytes.

This reservation must be allocated to power on the VM.
See *VirtualMachineRuntimeInfo.vFlashCacheAllocation* for allocated
reservation when VM is powered on.
"#####) },),
        ("v_asserts_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vAssertsEnabled", doc: Some(r#####"Indicates whether user-configured virtual asserts will be
triggered during virtual machine replay.
"#####) },),
        ("message_bus_tunnel_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "messageBusTunnelEnabled", doc: Some(r#####"Whether to allow tunneling of clients from the guest VM into the
common message bus on the host network.
"#####) },),
        ("initial_overhead", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineConfigInfoOverheadInfo", type_name: "VirtualMachineConfigInfoOverheadInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "initialOverhead", doc: Some(r#####"Set of values to be used only to perform admission control when
determining if a host has sufficient resources for the virtual
machine to power on.
"#####) },),
        ("network_shaper", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineNetworkShaperInfo", type_name: "VirtualMachineNetworkShaperInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "networkShaper", doc: Some(r#####"Deprecated from vSphere 5.5, shaping policy on VM is not supported.

Resource limits for network.
"#####) },),
        ("hardware", NodeData { type_decl: "vim_rs::types::structs::VirtualHardware", type_name: "VirtualHardware", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "hardware", doc: Some(r#####"Processor, memory, and virtual devices for a virtual machine.
"#####) },),
        ("firmware", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "firmware", doc: Some(r#####"Information about firmware type for this Virtual Machine.

Possible values are described in
*GuestOsDescriptorFirmwareType_enum*
When creating a new VM:
\- If vim.vm.FlagInfo.vbsEnabled is set to <code>true</code> and
this property is set to <code>bios</code>, error is returned.
\- If this property is unset and vim.vm.FlagInfo.vbsEnabled is set
to <code>true</code>, this property is set to <code>efi</code>.
"#####) },),
        ("key_id", NodeData { type_decl: "vim_rs::types::structs::CryptoKeyId", type_name: "CryptoKeyId", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "keyId", doc: Some(r#####"Virtual Machine cryptographic options.
"#####) },),
        ("guest_integrity_info", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineGuestIntegrityInfo", type_name: "VirtualMachineGuestIntegrityInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "guestIntegrityInfo", doc: Some(r#####"Guest integrity platform configuration
"#####) },),
        ("change_tracking_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "changeTrackingEnabled", doc: Some(r#####"Indicates whether changed block tracking for this VM's disks
is active.
"#####) },),
        ("boot_options", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineBootOptions", type_name: "VirtualMachineBootOptions", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "bootOptions", doc: Some(r#####"Configuration options for the boot behavior of the virtual machine.
"#####) },),
        ("numa_info", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineVirtualNumaInfo", type_name: "VirtualMachineVirtualNumaInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "numaInfo", doc: Some(r#####"vNUMA info.

***Since:*** vSphere API Release 8.0.0.1
"#####) },),
        ("guest_monitoring_mode_info", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineGuestMonitoringModeInfo", type_name: "VirtualMachineGuestMonitoringModeInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "guestMonitoringModeInfo", doc: Some(r#####"GMM configuration
"#####) },),
        ("annotation", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "annotation", doc: Some(r#####"Description for the virtual machine.
"#####) },),
        ("vmx_stats_collection_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vmxStatsCollectionEnabled", doc: Some(r#####"Indicates whether VMXStats Collection is enabled/disabled.

\- If TRUE, VMXStats is enabled for the VM and a scoreboard
file is created to store stats for various VMX components.
\- If FALSE, VMXStats is disabled for the VM and there is
no scoreboard file created.

***Since:*** vSphere API Release 7.0.3.1
"#####) },),
        ("reboot_power_off", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "rebootPowerOff", doc: Some(r#####"Whether the next reboot will result in a power off.

***Since:*** vSphere API Release 8.0.0.1
"#####) },),
        ("managed_by", NodeData { type_decl: "vim_rs::types::structs::ManagedByInfo", type_name: "ManagedByInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "managedBy", doc: Some(r#####"Specifies that this VM is managed by a VC Extension.

See the
*managedBy* property in the ConfigSpec
for more details.
"#####) },),
        ("swap_placement", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "swapPlacement", doc: Some(r#####"Virtual machine swapfile placement policy.

This will be unset if the
virtual machine's
*swapPlacementSupported*
capability is false. If swapPlacementSupported is true, the default
policy is "inherit".

See also *VirtualMachineConfigInfoSwapPlacementType_enum*.
"#####) },),
        ("default_power_ops", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineDefaultPowerOpInfo", type_name: "VirtualMachineDefaultPowerOpInfo", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "defaultPowerOps", doc: Some(r#####"Configuration of default power operations.
"#####) },),
        ("console_preferences", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineConsolePreferences", type_name: "VirtualMachineConsolePreferences", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "consolePreferences", doc: Some(r#####"Legacy console viewer preferences when doing power operations.
"#####) },),
        ("v_pmc_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vPMCEnabled", doc: Some(r#####"Indicates whether this VM have vurtual CPU performance counters
enabled.
"#####) },),
        ("device_swap", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineVirtualDeviceSwap", type_name: "VirtualMachineVirtualDeviceSwap", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "deviceSwap", doc: Some(r#####"Status of the device swap operation.

***Since:*** vSphere API Release 8.0.0.1
"#####) },),
        ("latency_sensitivity", NodeData { type_decl: "vim_rs::types::structs::LatencySensitivity", type_name: "LatencySensitivity", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "latencySensitivity", doc: Some(r#####"The latency-sensitivity of the virtual machine.
"#####) },),
        ("hot_plug_memory_increment_size", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "hotPlugMemoryIncrementSize", doc: Some(r#####"Memory, in MB that can be added to a running virtual machine
must be in increments of this value and needs be a
multiple of this value.

This value is determined by the virtual machine and is specified
only if *VirtualMachineConfigSpec.memoryHotAddEnabled*
has been set to true.
"#####) },),
        ("cpu_hot_remove_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "cpuHotRemoveEnabled", doc: Some(r#####"Whether virtual processors can be removed while this
virtual machine is running.
"#####) },),
        ("datastore_url", NodeData { type_decl: "Vec<vim_rs::types::structs::VirtualMachineConfigInfoDatastoreUrlPair>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVirtualMachineConfigInfoDatastoreUrlPair"), path_segment: "datastoreUrl", doc: Some(r#####"Enumerates the set of datastores that this virtual machine is
stored on, as well as the URL identification for each of these.

Changes to datastores do not generate property updates on this
property. However, when this property is retrieved it returns the
current datastore information.
"#####) },),
        ("guest_id", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "guestId", doc: Some(r#####"Guest operating system configured on a virtual machine.

This is a guest identifier that can be used to access the
*GuestOsDescriptor*
list for information about default configuration.
For more information on possible values, see
*VirtualMachineGuestOsIdentifier*.
"#####) },),
        ("npiv_desired_port_wwns", NodeData { type_decl: "i16", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveShort"), path_segment: "npivDesiredPortWwns", doc: Some(r#####"The NPIV port WWNs to be extended from the original list of WWN nummbers.

This
property should be set to desired number which is an aggregate of existing
plus new numbers. Desired Node WWNs should always be greater than the existing
number of port WWNs
"#####) },),
        ("vm_op_notification_timeout", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "vmOpNotificationTimeout", doc: Some(r#####"Operation notification timeout in seconds.

\- Specifies the maximum time the application can take to
prepare for the operation after its been notified. This value is used
only if *VirtualMachineConfigInfo.vmOpNotificationToAppEnabled* is set to TRUE.
\- If *VirtualMachineConfigInfo.vmOpNotificationTimeout* is unset, then it defaults to
cluster/host timeout.

***Since:*** vSphere API Release 8.0.0.1
"#####) },),
        ("files", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineFileInfo", type_name: "VirtualMachineFileInfo", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "files", doc: Some(r#####"Information about the files associated with a virtual machine.

This information does not include files for specific virtual disks or
snapshots.
"#####) },),
        ("pmem", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineVirtualPMem", type_name: "VirtualMachineVirtualPMem", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "pmem", doc: Some(r#####"Deprecated as of vSphere 9.0 APIs with no replacement.

Virtual persistent memory info.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("version", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "version", doc: Some(r#####"The version string for this virtual machine.
"#####) },),
        ("vm_op_notification_to_app_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vmOpNotificationToAppEnabled", doc: Some(r#####"Indicates whether operation notification to applications is
enabled/disabled.

\- When set to TRUE, application running inside the VM will be
notified of operations for which they have registered.
\- If unset or FALSE, new applications are not allowed to register
for notifications and RPCs will no longer be supported from
already registered applications.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("template", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "template", doc: Some(r#####"Flag indicating whether or not a virtual machine is a template.
"#####) },),
        ("memory_hot_add_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "memoryHotAddEnabled", doc: Some(r#####"Whether memory can be added while this virtual machine is running.
"#####) },),
        ("tdx_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "tdxEnabled", doc: Some(r#####"TDX (Trust Domain Extensions) enabled or not.

TDX is enabled
when set to true, and disabled otherwise.

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("scheduled_hardware_upgrade_info", NodeData { type_decl: "vim_rs::types::structs::ScheduledHardwareUpgradeInfo", type_name: "ScheduledHardwareUpgradeInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "scheduledHardwareUpgradeInfo", doc: Some(r#####"Configuration of scheduled hardware upgrades and result from last
attempt to run scheduled hardware upgrade.

See also *ScheduledHardwareUpgradeInfo*.
"#####) },),
        ("device_groups", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineVirtualDeviceGroups", type_name: "VirtualMachineVirtualDeviceGroups", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "deviceGroups", doc: Some(r#####"Assignable hardware device groups.

***Since:*** vSphere API Release 8.0.0.1
"#####) },),
        ("nested_hv_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "nestedHVEnabled", doc: Some(r#####"Indicates whether this VM is configured to use nested
hardware-assisted virtualization.
"#####) },),
        ("ft_encryption_mode", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "ftEncryptionMode", doc: Some(r#####"An enum describing whether encrypted Fault Tolerance is required for this
VM.

See *VirtualMachineConfigSpecEncryptedFtModes_enum* for allowed values.
\- This defaults to opportunistic for a regular VM, and will be set to
required for an encrypted VM.
\- If this property is unset, the mode of encrypted Fault Tolerance
will be set to opportunistic.

***Since:*** vSphere API Release 7.0.2.0
"#####) },),
        ("sgx_info", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineSgxInfo", type_name: "VirtualMachineSgxInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "sgxInfo", doc: Some(r#####"Configuration of SGX, Software Guard Extensions for the VM.
"#####) },),
        ("cpu_allocation", NodeData { type_decl: "vim_rs::types::structs::ResourceAllocationInfo", type_name: "ResourceAllocationInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "cpuAllocation", doc: Some(r#####"Resource limits for CPU.
"#####) },),
        ("sev_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "sevEnabled", doc: Some(r#####"SEV (Secure Encrypted Virtualization) enabled or not.

SEV is enabled
when set to true, and disabled otherwise.

***Since:*** vSphere API Release 7.0.1.0
"#####) },),
        ("cpu_affinity", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineAffinityInfo", type_name: "VirtualMachineAffinityInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "cpuAffinity", doc: Some(r#####"Affinity settings for CPU.
"#####) },),
        ("guest_auto_lock_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "guestAutoLockEnabled", doc: Some(r#####"Indicates whether the guest operating system will logout any active
sessions whenever there are no remote display connections open to
the virtual machine.
"#####) },),
        ("alternate_guest_name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "alternateGuestName", doc: Some(r#####"Used as display name for the operating system if guestId is `other`
or `other-64`.

See also *VirtualMachineConfigInfo.guestFullName*.
"#####) },),
        ("flags", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineFlagInfo", type_name: "VirtualMachineFlagInfo", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "flags", doc: Some(r#####"Additional flags for a virtual machine.
"#####) },),
        ("npiv_node_world_wide_name", NodeData { type_decl: "Vec<i64>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfLong"), path_segment: "npivNodeWorldWideName", doc: Some(r#####"A 64-bit node WWN (World Wide Name).

These WWNs are paired with the
*VirtualMachineConfigInfo.npivPortWorldWideName* to be used by the NPIV VPORTs instantiated for the
virtual machine on the physical HBAs of the host. A pair of node and port WWNs
serves as a unique identifier in accessing a LUN, so that it can be monitored or
controlled by the storage administrator.

If this property contains a single node WWN, the same node WWN is used to pair
with all port WWNs listed in *VirtualMachineConfigInfo.npivPortWorldWideName*. If this property or
*VirtualMachineConfigInfo.npivPortWorldWideName* is empty or unset, NPIV WWN is disabled for the
virtual machine.
"#####) },),
        ("hot_plug_memory_limit", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "hotPlugMemoryLimit", doc: Some(r#####"The maximum amount of memory, in MB, than can be added to a
running virtual machine.

This value is determined by the
virtual machine and is specified only if
*VirtualMachineConfigInfo.memoryHotAddEnabled*
is set to true.
"#####) },),
        ("npiv_port_world_wide_name", NodeData { type_decl: "Vec<i64>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfLong"), path_segment: "npivPortWorldWideName", doc: Some(r#####"A 64-bit port WWN (World Wide Name).

For detail description on WWN, see
*VirtualMachineConfigInfo.npivNodeWorldWideName*.
"#####) },),
        ("npiv_on_non_rdm_disks", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "npivOnNonRdmDisks", doc: Some(r#####"This property is used to check whether the NPIV can be enabled on the Virtual
machine with non-rdm disks in the configuration, so this is potentially not
enabling npiv on vmfs disks.

Also this property is used to check whether RDM
is required to generate WWNs for a virtual machine.
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"Display name of the virtual machine.

Any / (slash), \\ (backslash), character used in this
name element is escaped. Similarly, any % (percent) character used in
this name element is escaped, unless it is used to start an escape
sequence. A slash is escaped as %2F or %2f. A backslash is escaped as %5C or
%5c, and a percent is escaped as %25.
"#####) },),
        ("guest_full_name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "guestFullName", doc: Some(r#####"This is the full name of the guest operating system for the virtual machine.

For example: Windows 2000 Professional.

See also *VirtualMachineConfigInfo.alternateGuestName*.
"#####) },),
        ("memory_reservation_locked_to_max", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "memoryReservationLockedToMax", doc: Some(r#####"If set true, memory resource reservation for this virtual machine will always be
equal to the virtual machine's memory size; increases in memory size will be
rejected when a corresponding reservation increase is not possible.
"#####) },),
        ("location_id", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "locationId", doc: Some(r#####"Hash incorporating the virtual machine's config file location
and the UUID of the host assigned to run the virtual machine.
"#####) },),
        ("metro_ft_host_group", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "metroFtHostGroup", doc: Some(r#####"Indicate the Host Group (*ClusterHostGroup*) for FT
Metro Cluster enabled Virtual Machine.

Based on the selected Host Group, FT can divide the hosts in the cluster
into two groups and ensure to place FT primary and FT secondary in
different groups.

***Since:*** vSphere API Release 8.0.3.0
"#####) },),
        ("migrate_encryption", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "migrateEncryption", doc: Some(r#####"An enum describing whether encrypted vMotion is required for this VM.

See *VirtualMachineConfigSpecEncryptedVMotionModes_enum* for allowed values.
This defaults to opportunistic for a regular VM, and will be set to
required for an encrypted VM.
"#####) },),
        ("metro_ft_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "metroFtEnabled", doc: Some(r#####"Indicates whether FT Metro Cluster is enabled/disabled.

\- If TRUE, FT Metro Cluster is enabled for the VM. An implicit
Anti-HostGroup will be generated from HostGroup defined for FT
primary, then affine the primary with one HostGroup and affine the
secondary with another HostGroup.
\- If FALSE or unset, FT Metro Cluster is disabled for the VM. Both FT
primary and secondary will be put in the same HostGroup.

***Since:*** vSphere API Release 8.0.3.0
"#####) },),
        ("cpu_hot_add_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "cpuHotAddEnabled", doc: Some(r#####"Whether virtual processors can be added while this
virtual machine is running.
"#####) },),
        ("cpu_feature_mask", NodeData { type_decl: "Vec<vim_rs::types::structs::HostCpuIdInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostCpuIdInfo"), path_segment: "cpuFeatureMask", doc: Some(r#####"Specifies CPU feature compatibility masks that override the
defaults from the *GuestOsDescriptor*
of the virtual machine's guest OS.

As of vSphere API 6.5 *FeatureMask*
is the recommended method for masking virtual machines with
hardware version 9 and above (newer). They can be viewed via
*featureMask*.
"#####) },),
        ("npiv_desired_node_wwns", NodeData { type_decl: "i16", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveShort"), path_segment: "npivDesiredNodeWwns", doc: Some(r#####"The NPIV node WWNs to be extended from the original list of WWN nummbers.

This
property should be set to desired number which is an aggregate of existing
plus new numbers. Desired Node WWNs should always be greater than the existing
number of node WWNs
"#####) },),
        ("memory_allocation", NodeData { type_decl: "vim_rs::types::structs::ResourceAllocationInfo", type_name: "ResourceAllocationInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "memoryAllocation", doc: Some(r#####"Resource limits for memory.
"#####) },),
        ("memory_affinity", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineAffinityInfo", type_name: "VirtualMachineAffinityInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "memoryAffinity", doc: Some(r#####"Deprecated since vSphere 6.0.

Affinity settings for memory.
"#####) },),
        ("ft_info", NodeData { type_decl: "Box<dyn vim_rs::types::traits::FaultToleranceConfigInfoTrait>", type_name: "FaultToleranceConfigInfo", is_optional: true, processing_type: FieldProcessingType::Trait, path_segment: "ftInfo", doc: Some(r#####"Fault Tolerance settings for this virtual machine.
"#####) },),
        ("swap_storage_object_id", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "swapStorageObjectId", doc: Some(r#####"Virtual Machine Swap Object Identifier.

With Object-based Storage systems, VM's Swap is backed by an object.
This identifier will be set only if VM swap resided on
object-based storage systems.
"#####) },),
        ("uuid", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "uuid", doc: Some(r#####"128-bit SMBIOS UUID of a virtual machine represented as a hexadecimal string
in "12345678-abcd-1234-cdef-123456789abc" format.
"#####) },),
        ("extra_config", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::OptionValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfOptionValue"), path_segment: "extraConfig", doc: Some(r#####"Additional configuration information for the virtual machine.
"#####) },),
        ("change_version", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "changeVersion", doc: Some(r#####"The changeVersion is a unique identifier for a given version
of the configuration.

Each change to the configuration
updates this value. This is typically implemented as an ever
increasing count or a time-stamp. However, a client should
always treat this as an opaque string.
"#####) },),
        ("vmx_config_checksum", NodeData { type_decl: "Vec<u8>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBinary"), path_segment: "vmxConfigChecksum", doc: Some(r#####"A checksum of vmx config file.
"#####) },),
        ("vm_storage_object_id", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "vmStorageObjectId", doc: Some(r#####"Virtual Machine Object Identifier.

With Object-based Storage systems, Virtual Machine home directory
is backed by an object.
This identifier will be set only if VM directory resided on
object-based storage systems.
"#####) },),
        ("fork_config_info", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineForkConfigInfo", type_name: "VirtualMachineForkConfigInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "forkConfigInfo", doc: Some(r#####"Fork configuration of this virtual machines.

If unset, this virtual machine
is not configured for fork.

See also *VirtualMachineForkConfigInfo*.
"#####) },),
        ("fixed_passthru_hot_plug_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "fixedPassthruHotPlugEnabled", doc: Some(r#####"Indicates whether support to add and remove fixed passthrough
devices when the VM is running is enabled.

When the virtual machine is powered on, this indicates if
support for hot adding and removing fixed passthrough devices
was enabled prior to power on. Otherwise, it indicates whether
it will be enabled when the VM is powered on.
NOTE: When setting this to true, the memory reservation should
be equal to the guest memory size or the option to reserve all
guest memory should be selected. If unset, the current value is
left unchanged.

***Since:*** vSphere API Release 8.0.1.0
"#####) },),
        ("instance_uuid", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "instanceUuid", doc: Some(r#####"VirtualCenter-specific 128-bit UUID of a virtual machine, represented
as a hexademical string.

This identifier is used by VirtualCenter to
uniquely identify all virtual machine instances, including those that
may share the same SMBIOS UUID.
"#####) },),
        ("max_mks_connections", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "maxMksConnections", doc: Some(r#####"Indicates the maximum number of active remote display connections
that the virtual machine will support.
"#####) },),
        ("modified", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveDateTime"), path_segment: "modified", doc: Some(r#####"Last time a virtual machine's configuration was modified.
"#####) },),
        ("rep_config", NodeData { type_decl: "vim_rs::types::structs::ReplicationConfigSpec", type_name: "ReplicationConfigSpec", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "repConfig", doc: Some(r#####"vSphere Replication settings for this virtual machine.

Note this may become deprecated in the future releases. We discourage
any unnecessary dependency on this field.
"#####) },),
        ("create_date", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveDateTime"), path_segment: "createDate", doc: Some(r#####"Time the virtual machine's configuration was created.
"#####) },),
    ],
}),
        ("StorageDrsConfigInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("pod_config", NodeData { type_decl: "vim_rs::types::structs::StorageDrsPodConfigInfo", type_name: "StorageDrsPodConfigInfo", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "podConfig", doc: Some(r#####"Pod-wide configuration of the storage DRS service.
"#####) },),
        ("vm_config", NodeData { type_decl: "Vec<vim_rs::types::structs::StorageDrsVmConfigInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfStorageDrsVmConfigInfo"), path_segment: "vmConfig", doc: Some(r#####"List of virtual machine configurations for the storage DRS
service.

Each entry applies to all the virtual disks of the virtual machine
on this pod.

If a virtual machine is not specified in this array, the service uses
the default settings for that virtual machine.
"#####) },),
    ],
}),
        ("VirtualHardware", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (3, 6),
        (3, 0),
    ],
    entries: &[
        ("motherboard_layout", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "motherboardLayout", doc: Some(r#####"One of motherboardLayout choices.

Default is i440bxHostBridge. See
*VirtualHardware.motherboardLayout*

***Since:*** vSphere API Release 8.0.0.1
"#####) },),
        ("simultaneous_threads", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "simultaneousThreads", doc: Some(r#####"Number of SMT (Simultaneous multithreading) threads.

If unset, then system defaults are in use.

***Since:*** vSphere API Release 8.0.0.1
"#####) },),
        ("auto_cores_per_socket", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "autoCoresPerSocket", doc: Some(r#####"Cores per socket is automatically determined.

***Since:*** vSphere API Release 8.0.0.1
"#####) },),
        ("num_cpu", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "numCPU", doc: Some(r#####"Number of virtual CPUs present in this virtual machine.
"#####) },),
        ("num_cores_per_socket", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "numCoresPerSocket", doc: Some(r#####"Number of cores used to distribute virtual CPUs among sockets
in this virtual machine.

This field should be ignored for powered off VM with
autoCoresPerSocket equals TRUE, because the virtual socket size
will be assigned during power-on.
This field could be unset for releases prior to 7.0 U3, and it
implies numCoresPerSocket is 1.
In other cases, this field represents the actual virtual socket
size seen by the virtual machine.
"#####) },),
        ("memory_mb", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "memoryMB", doc: Some(r#####"Memory size, in MB.
"#####) },),
        ("device", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::VirtualDeviceTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVirtualDevice"), path_segment: "device", doc: Some(r#####"The set of virtual devices belonging to the virtual machine.

This list is unordered.
"#####) },),
        ("virtual_smc_present", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "virtualSMCPresent", doc: Some(r#####"Does this virtual machine have System Management Controller
"#####) },),
        ("virtual_ich_7_m_present", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "virtualICH7MPresent", doc: Some(r#####"Does this virtual machine have Virtual Intel I/O Controller Hub 7
"#####) },),
    ],
}),
        ("ClusterDrsConfigInfo", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (0, 3),
        (0, 0),
    ],
    entries: &[
        ("enable_vm_behavior_overrides", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "enableVmBehaviorOverrides", doc: Some(r#####"Flag that dictates whether DRS Behavior overrides for individual
virtual machines (*ClusterDrsVmConfigInfo*) are enabled.

The default
value is <code>true</code>.

When this flag is <code>true</code>, the
*ClusterConfigSpecEx*.*ClusterConfigSpecEx.drsVmConfigSpec*
values override the *ClusterDrsConfigInfo.defaultVmBehavior*.

When this flag is <code>false</code>, the
*ClusterDrsConfigInfo.defaultVmBehavior* value applies to all virtual
machines, with the following exception: in a cluster that has EVC disabled,
you cannot override the virtual machine setting
(*ClusterConfigSpecEx.drsVmConfigSpec*)
for Fault Tolerance virtual machines.
"#####) },),
        ("enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "enabled", doc: Some(r#####"Deprecated as of vSphere API 7.0.
To disable DRS load balancing, please use the lowest DRS aggressiveness
level, setting *ClusterDrsConfigInfo.vmotionRate* to 5, and/or
setting *ClusterDrsConfigInfo.defaultVmBehavior* to manual.
The former only generates manadatory move recommendations, not load
balancing recommendations. The latter only generates recommendations,
without executing them.
To remove all the child resource pools, please find the root resource
pool *ComputeResource.resourcePool*, and destroy all its
children *ResourcePool.DestroyChildren*.
Vice versa.

Flag indicating whether or not DRS service is enabled.
"#####) },),
        ("default_vm_behavior", NodeData { type_decl: "vim_rs::types::enums::DrsBehaviorEnum", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("DrsBehavior"), path_segment: "defaultVmBehavior", doc: Some(r#####"Specifies the cluster-wide default DRS behavior for virtual machines.

You can override the default behavior for a virtual machine
by using the *ClusterDrsVmConfigInfo* object.
"#####) },),
        ("scale_descendants_shares", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "scaleDescendantsShares", doc: Some(r#####"Specifies the scaling behavior of the shares of all resource pools
in the cluster.

See *ResourceConfigSpecScaleSharesBehavior_enum*
for possible values. If any scaling behavior other than
*disabled* is specified,
the system will scale the CPU and memory shares allocated to each
resource pool with the total shares of all powered on virtual machines
under each respective pool. The system will also use the
*SharesInfo* set on each resource pool as a multiplier for the
scale. Setting the
*ClusterDrsConfigInfo.scaleDescendantsShares* on the cluster
is equivalent to setting the
*ResourceConfigSpec.scaleDescendantsShares* on the root
resource pool.
"#####) },),
        ("vmotion_rate", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "vmotionRate", doc: Some(r#####"Threshold for generated *ClusterRecommendation*s.

DRS generates only those recommendations that are above the
specified vmotionRate. Ratings vary from 1 to 5. This setting applies
to manual, partiallyAutomated, and fullyAutomated
DRS clusters. See *DrsBehavior_enum*.
"#####) },),
        ("option", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::OptionValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfOptionValue"), path_segment: "option", doc: Some(r#####"Advanced settings.
"#####) },),
    ],
}),
        ("HostScsiTopology", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("adapter", NodeData { type_decl: "Vec<vim_rs::types::structs::HostScsiTopologyInterface>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostScsiTopologyInterface"), path_segment: "adapter", doc: Some(r#####"The list of SCSI interfaces.
"#####) },),
    ],
}),
        ("TaskInfo", ::phf::Map {
    key: 2980949210194914378,
    disps: &[
        (2, 12),
        (6, 7),
        (8, 12),
        (7, 2),
        (2, 0),
    ],
    entries: &[
        ("activation_id", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "activationId", doc: Some(r#####"The activation Id is a client-provided token to link an API call with a task.
"#####) },),
        ("event_chain_id", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "eventChainId", doc: Some(r#####"Event chain ID that leads to the corresponding events.
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"The name of the operation that created the task.

This is not set
for internal tasks.
"#####) },),
        ("reason", NodeData { type_decl: "Box<dyn vim_rs::types::traits::TaskReasonTrait>", type_name: "TaskReason", is_optional: false, processing_type: FieldProcessingType::Trait, path_segment: "reason", doc: Some(r#####"Kind of entity responsible for creating this task.
"#####) },),
        ("progress", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "progress", doc: Some(r#####"If the task state is "running", then this property contains a
progress measurement, expressed as percentage completed, from 0 to 100.

If this property is not set, then the command does not report progress.
"#####) },),
        ("cancelable", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "cancelable", doc: Some(r#####"Flag to indicate whether or not the cancel task operation is supported.
"#####) },),
        ("description_id", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "descriptionId", doc: Some(r#####"An identifier for this operation.

This includes publicly visible
internal tasks and is a lookup in the TaskDescription methodInfo
data object.
"#####) },),
        ("root_task_key", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "rootTaskKey", doc: Some(r#####"Tasks can be created by another task and such creation can go on for
multiple levels.

This is the *TaskInfo.key* of the task
that started the chain of tasks.
"#####) },),
        ("progress_details", NodeData { type_decl: "Vec<vim_rs::types::structs::KeyAnyValue>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfKeyAnyValue"), path_segment: "progressDetails", doc: Some(r#####"***Since:*** vSphere API Release 8.0.1.0
"#####) },),
        ("error", NodeData { type_decl: "vim_rs::types::structs::MethodFault", type_name: "MethodFault", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "error", doc: Some(r#####"If the task state is "error", then this property contains the fault code.
"#####) },),
        ("cancelled", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "cancelled", doc: Some(r#####"Flag to indicate whether or not the client requested
cancellation of the task.
"#####) },),
        ("locked", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "locked", doc: Some(r#####"If the state of the task is "running", then this property is a list of
managed entities that the operation has locked, with a shared lock.

Refers instances of *ManagedEntity*.
"#####) },),
        ("start_time", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveDateTime"), path_segment: "startTime", doc: Some(r#####"Time stamp when the task started running.
"#####) },),
        ("description", NodeData { type_decl: "vim_rs::types::structs::LocalizableMessage", type_name: "LocalizableMessage", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "description", doc: Some(r#####"The description field of the task describes the current phase of
operation of the task.

For a task that does a single monolithic
activity, this will be fixed and unchanging.
For tasks that have various substeps, this field will change
as the task progresses from one phase to another.
"#####) },),
        ("queue_time", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveDateTime"), path_segment: "queueTime", doc: Some(r#####"Time stamp when the task was created.
"#####) },),
        ("state", NodeData { type_decl: "vim_rs::types::enums::TaskInfoStateEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("TaskInfoState"), path_segment: "state", doc: Some(r#####"Runtime status of the task.
"#####) },),
        ("change_tag", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "changeTag", doc: Some(r#####"The user entered tag to identify the operations and their side effects
"#####) },),
        ("key", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "key", doc: Some(r#####"The unique key for the task.
"#####) },),
        ("result", NodeData { type_decl: "vim_rs::types::vim_any::VimAny", type_name: "Any", is_optional: true, processing_type: FieldProcessingType::Trait, path_segment: "result", doc: Some(r#####"If the task state is "success", then this property may be used
to hold a return value.
"#####) },),
        ("task", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "task", doc: Some(r#####"The managed object that represents this task.

Refers instance of *Task*.
"#####) },),
        ("parent_task_key", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "parentTaskKey", doc: Some(r#####"Tasks can be created by another task.

This shows *TaskInfo.key* of the task spun off this task. This is to
track causality between tasks.
"#####) },),
        ("complete_time", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveDateTime"), path_segment: "completeTime", doc: Some(r#####"Time stamp when the task was completed (whether success or failure).
"#####) },),
        ("entity", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "entity", doc: Some(r#####"Managed entity to which the operation applies.

Refers instance of *ManagedEntity*.
"#####) },),
        ("entity_name", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "entityName", doc: Some(r#####"The name of the managed entity, locale-specific, retained for the
history collector database.
"#####) },),
    ],
}),
        ("VirtualMachineGuestIntegrityInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "enabled", doc: Some(r#####"Flag to indicate whether guest integrity platform feature is enabled for
this virtual machine.

Guest integrity adds capabilities in the virtual
platform to detect attacks on the guest OS kernel
"#####) },),
    ],
}),
        ("AnswerFileStatusResult", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (3, 0),
    ],
    entries: &[
        ("host", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "host", doc: Some(r#####"Host associated with the answer file.

Refers instance of *HostSystem*.
"#####) },),
        ("error", NodeData { type_decl: "Vec<vim_rs::types::structs::AnswerFileStatusError>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAnswerFileStatusError"), path_segment: "error", doc: Some(r#####"If <code>status</code> is <code>invalid</code>, this property contains a list
of status error objects.
"#####) },),
        ("checked_time", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveDateTime"), path_segment: "checkedTime", doc: Some(r#####"Time that the answer file status was determined.
"#####) },),
        ("status", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "status", doc: Some(r#####"Status of the answer file.

See *HostProfileManagerAnswerFileStatus_enum* for valid values.
"#####) },),
    ],
}),
        ("VirtualMachineVirtualDeviceSwapDeviceSwapInfo", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("status", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "status", doc: Some(r#####"Status of the operation.

One of
*VirtualMachineVirtualDeviceSwapDeviceSwapStatus_enum*
This field is read-only and cannot be modified.
"#####) },),
        ("enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "enabled", doc: Some(r#####"Is the swap operation enabled for this virtual machine.
"#####) },),
        ("applicable", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "applicable", doc: Some(r#####"Is the swap operation applicable to this virtual machine?
Operation is applicable if it is enabled for the virtual
machine, for the host or cluster in which virtual machine
resides, operating system supports device swap, and
virtual machine has controllers that need to be replaced.

This field is read-only and cannot be modified.
"#####) },),
    ],
}),
        ("ComplianceResult", ::phf::Map {
    key: 7485420634051515786,
    disps: &[
        (3, 0),
    ],
    entries: &[
        ("entity", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "entity", doc: Some(r#####"Entity on which the compliance check was carried out.

Entity can be a Cluster, Host and so on.

Refers instance of *ManagedEntity*.
"#####) },),
        ("failure", NodeData { type_decl: "Vec<vim_rs::types::structs::ComplianceFailure>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfComplianceFailure"), path_segment: "failure", doc: Some(r#####"If complianceStatus is non-compliant, failure will
contain additional information about the compliance errors.
"#####) },),
        ("compliance_status", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "complianceStatus", doc: Some(r#####"Indicates the compliance status of the entity.

See @link Status
"#####) },),
        ("profile", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "profile", doc: Some(r#####"Profile for which the ComplianceResult applies

Refers instance of *Profile*.
"#####) },),
        ("check_time", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveDateTime"), path_segment: "checkTime", doc: Some(r#####"Time at which compliance check was last run on the entity
"#####) },),
    ],
}),
        ("VirtualMachineNetworkShaperInfo", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("burst_size", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "burstSize", doc: Some(r#####"Burst size, in bytes.
"#####) },),
        ("peak_bps", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "peakBps", doc: Some(r#####"Peak bandwidth, in bits per second.
"#####) },),
        ("enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "enabled", doc: Some(r#####"Is the shaper enabled?
"#####) },),
        ("average_bps", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "averageBps", doc: Some(r#####"Average bandwidth, in bits per second.
"#####) },),
    ],
}),
        ("GuestScreenInfo", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("height", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "height", doc: Some(r#####"Height of the screen in pixels.
"#####) },),
        ("width", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "width", doc: Some(r#####"Width of the screen in pixels.
"#####) },),
    ],
}),
        ("ClusterDasAdmissionControlInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
    ],
    entries: &[
    ],
}),
        ("StorageIORMInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (2, 1),
        (0, 0),
    ],
    entries: &[
        ("reservable_iops_threshold", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "reservableIopsThreshold", doc: Some(r#####"Deprecated as of vSphere8.0 U3, and there is no replacement for it.

Storage DRS makes storage migration recommendations
if total IOPs reservation for all VMs running on the
datastore is higher than specified threshold value.

This value (if present) overrides
*StorageIORMInfo.reservableIopsThreshold*
"#####) },),
        ("reservation_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "reservationEnabled", doc: Some(r#####"Deprecated as of vSphere8.0 U3, and there is no replacement for it.

Flag indicating whether IO reservations support is enabled.
"#####) },),
        ("stats_aggregation_disabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "statsAggregationDisabled", doc: Some(r#####"Flag indicating whether stats aggregation is disabled.
"#####) },),
        ("percent_of_peak_throughput", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "percentOfPeakThroughput", doc: Some(r#####"The percentage of peak throughput to be used for setting congestion threshold
of a datastore.

Valid values are between 50 to 100. Default value is 90%

For more information, see
*StorageIORMInfo.congestionThreshold*
"#####) },),
        ("enabled", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "enabled", doc: Some(r#####"Flag indicating whether or not the service is enabled.
"#####) },),
        ("congestion_threshold", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "congestionThreshold", doc: Some(r#####"The latency beyond which the storage array is considered congested.

If storage I/O resource management is enabled on a datastore,
the algorithm tries to maintain the latency to be below or
close to this value. The unit is millisecond. The range of
this value is between 5 to 100 milliseconds.
"#####) },),
        ("congestion_threshold_mode", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "congestionThresholdMode", doc: Some(r#####"Mode of congestion threshold specification
For more information, see
*StorageIORMThresholdMode_enum*
"#####) },),
        ("stats_collection_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "statsCollectionEnabled", doc: Some(r#####"Deprecated as of vSphere API 6.5, use *StorageIORMInfo.enabled* instead.

Flag indicating whether service is running in stats collection mode.
"#####) },),
    ],
}),
        ("CryptoKeyId", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("provider_id", NodeData { type_decl: "vim_rs::types::structs::KeyProviderId", type_name: "KeyProviderId", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "providerId", doc: Some(r#####"The provider holding the key data.

May be ignored if the key is known to be stored in another provider.
"#####) },),
        ("key_id", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "keyId", doc: Some(r#####"Unique key ID.

When creating a key may be replaced with the ID generated by the KMS
server.
An empty string must be used when encrypting with a Trusted Key Provider,
because the key is generated at the time of encryption.
"#####) },),
    ],
}),
        ("HostDiagnosticPartition", ::phf::Map {
    key: 8602556344903797927,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("storage_type", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "storageType", doc: Some(r#####"Indicates the storage type of the diagnostic partition.

See also *DiagnosticPartitionStorageType_enum*.
"#####) },),
        ("slots", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "slots", doc: Some(r#####"The number of slots in the diagnostic partition.
"#####) },),
        ("diagnostic_type", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "diagnosticType", doc: Some(r#####"Indicates the type of the diagnostic partition.

See also *DiagnosticPartitionType_enum*.
"#####) },),
        ("id", NodeData { type_decl: "vim_rs::types::structs::HostScsiDiskPartition", type_name: "HostScsiDiskPartition", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "id", doc: Some(r#####"Diagnostic partition identification information.
"#####) },),
    ],
}),
        ("PowerSystemInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("current_policy", NodeData { type_decl: "vim_rs::types::structs::HostPowerPolicy", type_name: "HostPowerPolicy", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "currentPolicy", doc: Some(r#####"Currently selected host power management policy.

This property can have one of the values from
*PowerSystemCapability.availablePolicy*.
"#####) },),
    ],
}),
        ("VirtualMachineFileInfo", ::phf::Map {
    key: 8694567506910003252,
    disps: &[
        (4, 0),
    ],
    entries: &[
        ("snapshot_directory", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "snapshotDirectory", doc: Some(r#####"Path name of the directory that holds suspend and snapshot files
belonging to the virtual machine.

Prior to vSphere 5.0, this
directory also holds snapshot redo files. Starting with vSphere 5.0,
the redo files will stay in the same directory as the snapshotted
disk, thus this directory will no longer hold the snapshot redo
files.

This path name defaults to the same directory as the configuration
file.

ESX Server requires this to indicate a VMFS volume or NAS volume
(for ESX Server 3).
In case the configuration file is not stored on VMFS or NAS, this
property must be set explicitly.
"#####) },),
        ("suspend_directory", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "suspendDirectory", doc: Some(r#####"Some products allow the suspend directory to be different than the
snapshot directory.

On products where this is not possible, setting
of this property is ignored.
"#####) },),
        ("vm_path_name", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "vmPathName", doc: Some(r#####"Path name to the configuration file for the virtual machine, e.g., the
.vmx file.

This also implicitly defines the configuration directory.
"#####) },),
        ("log_directory", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "logDirectory", doc: Some(r#####"Directory to store the log files for the virtual machine.

If not specified,
this defaults to the same directory as the configuration file,
"#####) },),
        ("ft_metadata_directory", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "ftMetadataDirectory", doc: Some(r#####"Directory to store the fault tolerance meta data files for the
virtual machine.
"#####) },),
    ],
}),
        ("VsanHostRuntimeInfo", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("membership_list", NodeData { type_decl: "Vec<vim_rs::types::structs::VsanHostMembershipInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVsanHostMembershipInfo"), path_segment: "membershipList", doc: Some(r#####"This property reports host membership information.
"#####) },),
        ("disk_issues", NodeData { type_decl: "Vec<vim_rs::types::structs::VsanHostRuntimeInfoDiskIssue>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVsanHostRuntimeInfoDiskIssue"), path_segment: "diskIssues", doc: Some(r#####"List of disk issues detected on this host.

To retrieve more information on the issues, use
*HostVsanSystem.QueryDisksForVsan*.
"#####) },),
        ("access_gen_no", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "accessGenNo", doc: Some(r#####"Generation number tracking object accessibility.
"#####) },),
    ],
}),
        ("HealthSystemRuntime", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("system_health_info", NodeData { type_decl: "vim_rs::types::structs::HostSystemHealthInfo", type_name: "HostSystemHealthInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "systemHealthInfo", doc: Some(r#####"Available system health information
"#####) },),
        ("hardware_status_info", NodeData { type_decl: "vim_rs::types::structs::HostHardwareStatusInfo", type_name: "HostHardwareStatusInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "hardwareStatusInfo", doc: Some(r#####"Available hardware health information
"#####) },),
    ],
}),
        ("VirtualMachineGuestMonitoringModeInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("gmm_file", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "gmmFile", doc: None },),
        ("gmm_appliance", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "gmmAppliance", doc: None },),
    ],
}),
        ("ResourcePoolResourceUsage", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
        (0, 1),
    ],
    entries: &[
        ("reservation_used_for_vm", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "reservationUsedForVm", doc: Some(r#####"Total amount of resources that have been used to satisfy the reservation
requirements of running virtual machines in this resource pool or any of its
child resource pools.
"#####) },),
        ("unreserved_for_vm", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "unreservedForVm", doc: Some(r#####"Total amount of resources available to satisfy a reservation for
a child virtual machine.

In general, this should be the same as
*ResourcePoolResourceUsage.unreservedForPool*. However, in the overcommitted case, this
is limited by the remaining available resources at the root
node.
"#####) },),
        ("overall_usage", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "overallUsage", doc: Some(r#####"Deprecated as of vSphere API 6.5.
Use *ResourcePoolQuickStats.overallCpuUsage* and
*ResourcePoolQuickStats.hostMemoryUsage*.

Close to real-time resource usage of all running child virtual
machines, including virtual machines in child resource pools.
"#####) },),
        ("reservation_used", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "reservationUsed", doc: Some(r#####"Total amount of resources that have been used to satisfy the
reservation requirements of all descendants of this
resource pool (includes both resource pools and virtual
machines).
"#####) },),
        ("unreserved_for_pool", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "unreservedForPool", doc: Some(r#####"Total amount of resources available to satisfy a reservation
for a child resource pool.

In the undercommitted state, this is
limited by the capacity at the root node. In the overcommitted case,
this could be higher since we do not perform the dynamic capacity
checks.
"#####) },),
        ("max_usage", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "maxUsage", doc: Some(r#####"Current upper-bound on usage.

The upper-bound is based on the limit configured
on this resource pool, as well as limits configured on any parent resource
pool.
"#####) },),
    ],
}),
        ("HostNumaInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("num_nodes", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "numNodes", doc: Some(r#####"The number of NUMA nodes on the host.

The value is 0 if the
host is not NUMA-capable.
"#####) },),
        ("numa_node", NodeData { type_decl: "Vec<vim_rs::types::structs::HostNumaNode>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostNumaNode"), path_segment: "numaNode", doc: Some(r#####"Information about each of the NUMA nodes on the host.

The array is empty if the host is not NUMA-capable.
"#####) },),
        ("r#type", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "type", doc: Some(r#####"Deprecated as of vSphere API 5.1, this property is always
set to "NUMA".

The type of NUMA technology.
"#####) },),
    ],
}),
        ("ResourcePoolSummary", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"Name of resource pool.
"#####) },),
        ("configured_memory_mb", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "configuredMemoryMB", doc: Some(r#####"Total configured memory of all virtual machines in the resource pool, in MB.
"#####) },),
        ("runtime", NodeData { type_decl: "vim_rs::types::structs::ResourcePoolRuntimeInfo", type_name: "ResourcePoolRuntimeInfo", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "runtime", doc: Some(r#####"Current runtime state of the resource pool.
"#####) },),
        ("quick_stats", NodeData { type_decl: "vim_rs::types::structs::ResourcePoolQuickStats", type_name: "ResourcePoolQuickStats", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "quickStats", doc: Some(r#####"A set of statistics that are typically updated with near real-time regularity.

This data object type does not support notification, for scalability reasons.
Therefore, changes in QuickStats do not generate property collector updates.
To monitor statistics values, use the statistics and alarms modules instead.
"#####) },),
        ("config", NodeData { type_decl: "vim_rs::types::structs::ResourceConfigSpec", type_name: "ResourceConfigSpec", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "config", doc: Some(r#####"Current configuration of the resource pool.
"#####) },),
    ],
}),
        ("DVPortSetting", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
        (1, 0),
    ],
    entries: &[
        ("vm_direct_path_gen_2_allowed", NodeData { type_decl: "vim_rs::types::structs::BoolPolicy", type_name: "BoolPolicy", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "vmDirectPathGen2Allowed", doc: Some(r#####"Deprecated as of vSphere API 8.0. VMDirectPath Gen 2 is no longer supported and
there is no replacement.

Indicates whether this port is allowed to do VMDirectPath Gen2 network passthrough.

Direct path capability is defined at host, switch, and device levels.
See the <code>vmDirectPathGen2Supported</code> properties on the
*DVSFeatureCapability*,
*HostCapability*, *PhysicalNic*,
and *VirtualEthernetCardOption* objects.
"#####) },),
        ("network_resource_pool_key", NodeData { type_decl: "vim_rs::types::structs::StringPolicy", type_name: "StringPolicy", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "networkResourcePoolKey", doc: Some(r#####"Deprecated as of vSphere API 6.0
Use *DVPortgroupConfigInfo.vmVnicNetworkResourcePoolKey* instead
to reference the virtual NIC network resource pool.

The key of user defined network resource pool to be associated with a port.

The default value for this property is "-1", indicating that
this port is not associated with any network resource pool.
"#####) },),
        ("in_shaping_policy", NodeData { type_decl: "vim_rs::types::structs::DvsTrafficShapingPolicy", type_name: "DVSTrafficShapingPolicy", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "inShapingPolicy", doc: Some(r#####"Network shaping policy for controlling throughput of inbound traffic.
"#####) },),
        ("blocked", NodeData { type_decl: "vim_rs::types::structs::BoolPolicy", type_name: "BoolPolicy", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "blocked", doc: Some(r#####"Indicates whether this port is blocked.

If a port is blocked,
packet forwarding is stopped.
"#####) },),
        ("out_shaping_policy", NodeData { type_decl: "vim_rs::types::structs::DvsTrafficShapingPolicy", type_name: "DVSTrafficShapingPolicy", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "outShapingPolicy", doc: Some(r#####"Network shaping policy for controlling throughput of outbound traffic.
"#####) },),
        ("filter_policy", NodeData { type_decl: "vim_rs::types::structs::DvsFilterPolicy", type_name: "DvsFilterPolicy", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "filterPolicy", doc: Some(r#####"Configuration for Network Filter Policy.
"#####) },),
        ("vendor_specific_config", NodeData { type_decl: "vim_rs::types::structs::DvsVendorSpecificConfig", type_name: "DVSVendorSpecificConfig", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "vendorSpecificConfig", doc: Some(r#####"Opaque binary blob that stores vendor specific configuration.
"#####) },),
    ],
}),
        ("DVSRollbackCapability", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("rollback_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "rollbackSupported", doc: Some(r#####"Indicates whether rollback is supported on the distributed switch.
"#####) },),
    ],
}),
        ("StorageDrsIoLoadBalanceConfig", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("io_load_imbalance_threshold", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "ioLoadImbalanceThreshold", doc: Some(r#####"Storage DRS makes storage migration recommendations if
I/O load imbalance level is higher than the specified threshold.

Unit: a number.
The valid values are in the range of 1 to 100. If not specified,
the default value is 5.
"#####) },),
        ("reservable_iops_threshold", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "reservableIopsThreshold", doc: Some(r#####"Storage DRS makes storage migration recommendations if total
IOPs reservation of all VMs running on a datastore is higher than
the specified threshold.

Storage DRS recommends migration out of
all such datastores, if more than one datastore exceed their reserved
IOPs threshold.

This is an advanced option, and should only be used if Storage DRS
estimated IOPs capacity is incorrect for datastores. The value
should be based on conservative estimate of storage performance,
and ideally should be set to about 50-60% of worse case peak
performance of backing LUN.
"#####) },),
        ("io_latency_threshold", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "ioLatencyThreshold", doc: Some(r#####"Storage DRS makes storage migration recommendations if
I/O latency on one (or more) of the datastores is higher than
the specified threshold.

Unit: millisecond.
The valid values are in the range of 5 to 100. If not specified,
the default value is 15.
"#####) },),
        ("reservable_percent_threshold", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "reservablePercentThreshold", doc: Some(r#####"Storage DRS makes storage migration recommendations if total
IOPs reservation of all VMs running on a datastore is higher than
the specified threshold.

Storage DRS recommends migration out of
all such datastores, if more than one datastore exceed their reserved
IOPs threshold.

The actual Iops used to determine threshold are computed from Storage
DRS estimation of IOPs capacity of a datastore. The absolute value
may change over time, according to storage response to workloads.

The valid values are in the range of 30 (i.e., 30%) to 100 (i.e., 100%).
If not specified, the default value is 60%.
"#####) },),
        ("reservable_threshold_mode", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "reservableThresholdMode", doc: Some(r#####"Determines which reservation threshold specification to use.

See *StorageDrsPodConfigInfoBehavior_enum*. If unspecified, the
mode is assumed automatic by default. Storage DRS uses
percentage value in that case.
If mode is specified, but corresponding reservationThreshold
value is absent, option specific defaults are used.
"#####) },),
    ],
}),
        ("ChoiceOption", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("choice_info", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::ElementDescriptionTrait>>", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ArrayOfElementDescription"), path_segment: "choiceInfo", doc: Some(r#####"The set of possible selections and descriptions.
"#####) },),
        ("default_index", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "defaultIndex", doc: Some(r#####"The index in ChoiceOption.value that serves as the default value.
"#####) },),
        ("value_is_readonly", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "valueIsReadonly", doc: Some(r#####"The flag to indicate whether or not a user
can modify a value belonging to this option type.

If
the flag is not set, the value can be modified.
"#####) },),
    ],
}),
        ("MethodFault", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("fault_message", NodeData { type_decl: "Vec<vim_rs::types::structs::LocalizableMessage>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfLocalizableMessage"), path_segment: "faultMessage", doc: Some(r#####"Message which has details about the error
Message can also contain a key to message catalog which
can be used to generate better localized messages.
"#####) },),
        ("fault_cause", NodeData { type_decl: "vim_rs::types::structs::MethodFault", type_name: "MethodFault", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "faultCause", doc: Some(r#####"Fault which is the cause of this fault.
"#####) },),
    ],
}),
        ("HostDateTimeInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 10),
        (0, 0),
        (10, 9),
    ],
    entries: &[
        ("service_sync", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "serviceSync", doc: Some(r#####"Report true if time is synchronized with remote time source
For PrecisionTimeSync this is obtained from PTP Port Status value.

For NetworkTimeProtocol this obtained from Leap Indicator value.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("ntp_run_time", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "ntpRunTime", doc: Some(r#####"Provides the total seconds ntpd process has been running for.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("last_sync_time", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveDateTime"), path_segment: "lastSyncTime", doc: Some(r#####"Timestamp when time services were last in sync with remote clock.

If not set, time services have never established synchronization.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("disable_fallback", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "disableFallback", doc: Some(r#####"When not disabled, if PrecisionTimeSync is configured, then the
NTP configuration can run as backup.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("ntp_duration", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "ntpDuration", doc: Some(r#####"Provides a duration in simplified, human-readable form
for the lifetime of the ntp service.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("system_clock_protocol", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "systemClockProtocol", doc: Some(r#####"The system clock synchronization protocol.

See *HostDateTimeInfoProtocol_enum* for possible values.
"#####) },),
        ("ptp_run_time", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "ptpRunTime", doc: Some(r#####"Provides the total seconds ptpd process has been running for.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("ntp_config", NodeData { type_decl: "vim_rs::types::structs::HostNtpConfig", type_name: "HostNtpConfig", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "ntpConfig", doc: Some(r#####"The NTP configuration on the host.
"#####) },),
        ("ptp_config", NodeData { type_decl: "vim_rs::types::structs::HostPtpConfig", type_name: "HostPtpConfig", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "ptpConfig", doc: Some(r#####"The PTP configuration on the host.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("time_zone", NodeData { type_decl: "vim_rs::types::structs::HostDateTimeSystemTimeZone", type_name: "HostDateTimeSystemTimeZone", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "timeZone", doc: Some(r#####"The time zone of the host.
"#####) },),
        ("ptp_duration", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "ptpDuration", doc: Some(r#####"Provides a duration in simplified, human-readable form
for the lifetime of the ptp service.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("remote_ntp_server", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "remoteNtpServer", doc: Some(r#####"Provides the NTP server that the host is synced with from the
set of servers configured.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("in_fallback_state", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "inFallbackState", doc: Some(r#####"Tracks if NTP is providing time to ESXi due to PTP service failure.

This is set only if disableFallback is set to false.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "enabled", doc: Some(r#####"Present state of the time services subsystem.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("disable_events", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "disableEvents", doc: Some(r#####"When not disabled Network Time service or Precision Time service
will send events to Virtual Center when service fails or recovers.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
    ],
}),
        ("ClusterComputeResource", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (0, 11),
        (1, 33),
        (1, 0),
        (6, 18),
        (0, 27),
        (2, 26),
        (3, 1),
    ],
    entries: &[
        ("recent_task", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "recentTask", doc: Some(r#####"The set of recent tasks operating on this managed entity.

This is a subset
of *TaskManager.recentTask* belong to this entity. A task in this
list could be in one of the four states: pending, running, success or error.

This property can be used to deduce intermediate power states for
a virtual machine entity. For example, if the current powerState is "poweredOn"
and there is a running task performing the "suspend" operation, then the virtual
machine's intermediate state might be described as "suspending."

Most tasks (such as power operations) obtain exclusive access to the virtual
machine, so it is unusual for this list to contain more than one running task.
One exception, however, is the task of cloning a virtual machine.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("network_boot_mode", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "networkBootMode", doc: Some(r#####"Managed property indicating whether and what kind of netwoork boot mode
is configured for this compute resource.

Supported values are enumerated
in *ComputeResourceNetworkBootMode_enum*.
This property can be configured via the
*ComputeResource.EnableNetworkBoot_Task* method or during compute
resource creation through the
*ComputeResource.networkBootMode* property.

***Since:*** vSphere API Release 9.0.0.0

***Required privileges:*** System.View
"#####) },),
        ("configuration_ex", NodeData { type_decl: "Box<dyn vim_rs::types::traits::ComputeResourceConfigInfoTrait>", type_name: "ComputeResourceConfigInfo", is_optional: false, processing_type: FieldProcessingType::Trait, path_segment: "configurationEx", doc: Some(r#####"Configuration of the compute resource; applies to both standalone hosts
and clusters.

For a cluster this property will return a
*ClusterConfigInfoEx* object.
"#####) },),
        ("migration_history", NodeData { type_decl: "Vec<vim_rs::types::structs::ClusterDrsMigration>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfClusterDrsMigration"), path_segment: "migrationHistory", doc: Some(r#####"The set of migration decisions that have recently been performed.

This list is populated only when DRS is in automatic mode.
"#####) },),
        ("summary", NodeData { type_decl: "Box<dyn vim_rs::types::traits::ComputeResourceSummaryTrait>", type_name: "ComputeResourceSummary", is_optional: false, processing_type: FieldProcessingType::Trait, path_segment: "summary", doc: Some(r#####"Basic runtime information about a compute resource.

This information is used on
summary screens and in list views.
"#####) },),
        ("network", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "network", doc: Some(r#####"The subset of network objects available in the datacenter that is available in
this ComputeResource.

This property is computed as the aggregate set of networks available from all the
hosts that are part of this compute resource.

***Required privileges:*** System.View
"#####) },),
        ("config_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "configStatus", doc: Some(r#####"The configStatus indicates whether or not the system has detected a configuration
issue involving this entity.

For example, it might have detected a
duplicate IP address or MAC address, or a host in a cluster
might be out of compliance. The meanings of the configStatus values are:
- red: A problem has been detected involving the entity.
- yellow: A problem is about to occur or a transient condition
  has occurred (For example, reconfigure fail-over policy).
- green: No configuration issues have been detected.
- gray: The configuration status of the entity is not being monitored.
  
A green status indicates only that a problem has not been detected;
it is not a guarantee that the entity is problem-free.

The *configIssue* property contains a list of the
problems that have been detected.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("config_issue", NodeData { type_decl: "Vec<vim_rs::types::structs::Event>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfEvent"), path_segment: "configIssue", doc: Some(r#####"Current configuration issues that have been detected for this entity.

Typically,
these issues have already been logged as events. The entity stores these
events as long as they are still current. The
*configStatus* property provides an overall status
based on these events.
"#####) },),
        ("triggered_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "triggeredAlarmState", doc: Some(r#####"A set of alarm states for alarms triggered by this entity
or by its descendants.

Triggered alarms are propagated up the inventory hierarchy
so that a user can readily tell when a descendant has triggered an alarm.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.

***Required privileges:*** System.View
"#####) },),
        ("permission", NodeData { type_decl: "Vec<vim_rs::types::structs::Permission>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfPermission"), path_segment: "permission", doc: Some(r#####"List of permissions defined for this entity.
"#####) },),
        ("config_manager_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "configManagerEnabled", doc: Some(r#####"Flag indicating whether or not desired configuration
management platform is enabled on the compute resource.

This property can be set only at the time of creation or through the
*ComputeResource.EnableConfigurationManagement* method.

***Since:*** vSphere API Release 8.0.0.0

***Required privileges:*** System.View
"#####) },),
        ("disabled_method", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "disabledMethod", doc: Some(r#####"List of operations that are disabled, given the current runtime
state of the entity.

For example, a power-on operation always fails if a
virtual machine is already powered on. This list can be used by clients to
enable or disable operations in a graphical user interface.

Note: This list is determined by the current runtime state of an entity,
not by its permissions.

This list may include the following operations for a HostSystem:
- *HostSystem.EnterMaintenanceMode_Task*
- *HostSystem.ExitMaintenanceMode_Task*
- *HostSystem.RebootHost_Task*
- *HostSystem.ShutdownHost_Task*
- *HostSystem.ReconnectHost_Task*
- *HostSystem.DisconnectHost_Task*
  
This list may include the following operations for a VirtualMachine:
- *VirtualMachine.AnswerVM*
- *ManagedEntity.Rename_Task*
- *VirtualMachine.CloneVM_Task*
- *VirtualMachine.PowerOffVM_Task*
- *VirtualMachine.PowerOnVM_Task*
- *VirtualMachine.SuspendVM_Task*
- *VirtualMachine.ResetVM_Task*
- *VirtualMachine.ReconfigVM_Task*
- *VirtualMachine.RelocateVM_Task*
- *VirtualMachine.MigrateVM_Task*
- *VirtualMachine.CustomizeVM_Task*
- *VirtualMachine.ShutdownGuest*
- *VirtualMachine.StandbyGuest*
- *VirtualMachine.RebootGuest*
- *VirtualMachine.CreateSnapshot_Task*
- *VirtualMachine.RemoveAllSnapshots_Task*
- *VirtualMachine.RevertToCurrentSnapshot_Task*
- *VirtualMachine.MarkAsTemplate*
- *VirtualMachine.MarkAsVirtualMachine*
- *VirtualMachine.ResetGuestInformation*
- *VirtualMachine.MountToolsInstaller*
- *VirtualMachine.UnmountToolsInstaller*
- *ManagedEntity.Destroy_Task*
- *VirtualMachine.UpgradeVM_Task*
- *VirtualMachine.ExportVm*
  
This list may include the following operations for a ResourcePool:
- *ResourcePool.ImportVApp*
- *ResourcePool.CreateChildVM_Task*
- *ResourcePool.UpdateConfig*
- *Folder.CreateVM_Task*
- *ManagedEntity.Destroy_Task*
- *ManagedEntity.Rename_Task*
  
This list may include the following operations for a VirtualApp:
- *ManagedEntity.Destroy_Task*
- *VirtualApp.CloneVApp_Task*
- *VirtualApp.unregisterVApp_Task*
- *VirtualApp.ExportVApp*
- *VirtualApp.PowerOnVApp_Task*
- *VirtualApp.PowerOffVApp_Task*
- *VirtualApp.UpdateVAppConfig*
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "value", doc: Some(r#####"List of custom field values.

Each value uses a key to associate
an instance of a *CustomFieldStringValue* with
a custom field definition.

***Required privileges:*** System.View
"#####) },),
        ("action_history", NodeData { type_decl: "Vec<vim_rs::types::structs::ClusterActionHistory>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfClusterActionHistory"), path_segment: "actionHistory", doc: Some(r#####"The set of actions that have been performed recently.
"#####) },),
        ("declared_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "declaredAlarmState", doc: Some(r#####"A set of alarm states for alarms that apply to this managed entity.

The set includes alarms defined on this entity
and alarms inherited from the parent entity,
or from any ancestors in the inventory hierarchy.

Alarms are inherited if they can be triggered by this entity or its descendants.
This set does not include alarms that are defined on descendants of this entity.

***Required privileges:*** System.View
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"Name of this entity, unique relative to its parent.

Any / (slash), \\ (backslash), character used in this
name element will be escaped. Similarly, any % (percent) character used in
this name element will be escaped, unless it is used to start an escape
sequence. A slash is escaped as %2F or %2f. A backslash is escaped as %5C or
%5c, and a percent is escaped as %25.

***Required privileges:*** System.View
"#####) },),
        ("custom_value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "customValue", doc: Some(r#####"Custom field values.

***Required privileges:*** System.View
"#####) },),
        ("parent", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "parent", doc: Some(r#####"Parent of this entity.

This value is null for the root object and for
*VirtualMachine* objects that are part of
a *VirtualApp*.

***Required privileges:*** System.View
"#####) },),
        ("drs_recommendation", NodeData { type_decl: "Vec<vim_rs::types::structs::ClusterDrsRecommendation>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfClusterDrsRecommendation"), path_segment: "drsRecommendation", doc: Some(r#####"Deprecated as of VI API 2.5, use
*ClusterComputeResource.recommendation*.
vSphere 6.5 is the last version where this property is populated.
Later versions of vSphere no longer populate this property.

If DRS is enabled, this returns the set of recommended
migrations from the DRS module.
"#####) },),
        ("configuration", NodeData { type_decl: "vim_rs::types::structs::ClusterConfigInfo", type_name: "ClusterConfigInfo", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "configuration", doc: Some(r#####"Deprecated as of VI API 2.5, use *ComputeResource.configurationEx*,
which is a *ClusterConfigInfoEx* data object..

Configuration of the cluster.
"#####) },),
        ("alarm_actions_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "alarmActionsEnabled", doc: Some(r#####"Whether alarm actions are enabled for this entity.

True if enabled; false otherwise.

***Required privileges:*** System.Read
"#####) },),
        ("recommendation", NodeData { type_decl: "Vec<vim_rs::types::structs::ClusterRecommendation>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfClusterRecommendation"), path_segment: "recommendation", doc: Some(r#####"List of recommended actions for the cluster.

It is
possible that the current set of recommendations may be empty,
either due to not having any running dynamic recommendation
generation module, or since there may be no recommended actions
at this time.

***Required privileges:*** System.Read
"#####) },),
        ("hci_config", NodeData { type_decl: "vim_rs::types::structs::ClusterComputeResourceHciConfigInfo", type_name: "ClusterComputeResourceHCIConfigInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "hciConfig", doc: Some(r#####"This is applicable to clusters which are configured using the HCI
workflow and contains data related to the workflow and specification.
"#####) },),
        ("drs_fault", NodeData { type_decl: "Vec<vim_rs::types::structs::ClusterDrsFaults>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfClusterDrsFaults"), path_segment: "drsFault", doc: Some(r#####"A collection of the DRS faults generated in the last DRS invocation.

Each element of the collection is the set of faults generated in one
recommendation.
DRS faults are generated when DRS tries to make recommendations
for rule enforcement, power management, etc., and indexed in a tree
structure with reason for recommendations and VM to migrate (optional)
as the index keys.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.

***Required privileges:*** System.Read
"#####) },),
        ("effective_role", NodeData { type_decl: "Vec<i32>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfInt"), path_segment: "effectiveRole", doc: Some(r#####"Access rights the current session has to this entity.

***Required privileges:*** System.View
"#####) },),
        ("lifecycle_managed", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "lifecycleManaged", doc: Some(r#####"Flag indicating whether or not the lifecycle of the compute resource is
managed.

Once it is enabled, it cannot be disabled.
This property can be set only at the time of creation or through the
*ComputeResource.EnableLifecycleManagement* method.

***Required privileges:*** System.View
"#####) },),
        ("summary_ex", NodeData { type_decl: "vim_rs::types::structs::ClusterComputeResourceSummary", type_name: "ClusterComputeResourceSummary", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "summaryEx", doc: Some(r#####"Deprecated do not use this property.
The same information could be obtained via
*ComputeResource.summary*.

The cluster summary.

***Since:*** vSphere API Release 7.0.1.1
"#####) },),
        ("overall_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "overallStatus", doc: Some(r#####"General health of this managed entity.

The overall status of the managed entity is computed as the worst status
among its alarms and the configuration issues detected on the entity.
The status is reported as one of the following values:
- red: The entity has alarms or configuration issues with a red status.
- yellow: The entity does not have alarms or configuration issues with a
  red status, and has at least one with a yellow status.
- green: The entity does not have alarms or configuration issues with a
  red or yellow status, and has at least one with a green status.
- gray: All of the entity's alarms have a gray status and the
  configuration status of the entity is not being monitored.
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("available_field", NodeData { type_decl: "Vec<vim_rs::types::structs::CustomFieldDef>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldDef"), path_segment: "availableField", doc: Some(r#####"List of custom field definitions that are valid for the object's type.

The fields are sorted by *CustomFieldDef.name*.

***Required privileges:*** System.View
"#####) },),
        ("tag", NodeData { type_decl: "Vec<vim_rs::types::structs::Tag>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfTag"), path_segment: "tag", doc: Some(r#####"The set of tags associated with this managed entity.

Experimental. Subject to change.

***Required privileges:*** System.View
"#####) },),
        ("host", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "host", doc: Some(r#####"List of hosts that are part of this compute resource.

If the compute resource is a
standalone type, then this list contains just one element.

***Required privileges:*** System.View
"#####) },),
        ("datastore", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "datastore", doc: Some(r#####"The datastore property is the subset of datastore objects in the datacenter
available in this ComputeResource.

This property is computed as the aggregate set of datastores available from all
the hosts that are part of this compute resource.

***Required privileges:*** System.View
"#####) },),
        ("resource_pool", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "resourcePool", doc: Some(r#####"Reference to root resource pool.

***Required privileges:*** System.View
"#####) },),
        ("environment_browser", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "environmentBrowser", doc: Some(r#####"The environment browser object that identifies the environments that are supported
on this compute resource.

***Required privileges:*** System.View
"#####) },),
    ],
}),
        ("DVSPolicy", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("auto_pre_install_allowed", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "autoPreInstallAllowed", doc: Some(r#####"Whether downloading a new proxy VirtualSwitch module to the host is
allowed to be automatically executed by the switch.
"#####) },),
        ("partial_upgrade_allowed", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "partialUpgradeAllowed", doc: Some(r#####"Whether to allow upgrading a switch when some of the hosts failed to
install the needed module.

The vCenter Server will reattempt the
pre-install operation of the host module on those failed hosts,
whenever they reconnect to vCenter.
"#####) },),
        ("auto_upgrade_allowed", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "autoUpgradeAllowed", doc: Some(r#####"Whether upgrading of the switch is allowed to be automatically
executed by the switch.
"#####) },),
    ],
}),
        ("HostVFlashManagerVFlashCacheConfigInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("v_flash_module_config_option", NodeData { type_decl: "Vec<vim_rs::types::structs::HostVFlashManagerVFlashCacheConfigInfoVFlashModuleConfigOption>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostVFlashManagerVFlashCacheConfigInfoVFlashModuleConfigOption"), path_segment: "vFlashModuleConfigOption", doc: Some(r#####"Cache configuration options for the supported vFlash modules.
"#####) },),
        ("swap_cache_reservation_in_gb", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "swapCacheReservationInGB", doc: Some(r#####"Amount of vFlash resource is allocated to the host swap cache.

As long as set,
reservation will be permanent and retain regardless of host power state. The host
swap cache will be disabled if reservation is set to zero.
"#####) },),
        ("default_v_flash_module", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "defaultVFlashModule", doc: Some(r#####"Name of the default vFlash module for the read-write cache associated
with the VMs of this host.

This setting can be overridden by
*VirtualDiskVFlashCacheConfigInfo.vFlashModule*
per VMDK.
"#####) },),
    ],
}),
        ("ClusterUsageSummary", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (4, 13),
        (1, 0),
        (0, 0),
        (0, 11),
    ],
    entries: &[
        ("cpu_reservation_mhz", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "cpuReservationMhz", doc: Some(r#####"Sum of CPU reservation of all the Resource Pools and powered-on VMs in the cluster.
"#####) },),
        ("mem_entitled_mb", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "memEntitledMB", doc: Some(r#####"This is the current memory entitlement across the cluster
"#####) },),
        ("unreserved_tier_0_mem_mb", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "unreservedTier0MemMB", doc: Some(r#####"Total amount of Tier 0 memory available to satisfy reservation.

Available
reservation is calculated after accounting for DRS overheads and current
reservation.

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("mem_demand_mb", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "memDemandMB", doc: Some(r#####"Sum of memory demand of all the powered-on VMs in the cluster.
"#####) },),
        ("stats_gen_number", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "statsGenNumber", doc: Some(r#####"Generation number of the usage stats.

Updated during every DRS load
balancing call.
"#####) },),
        ("powered_off_cpu_reservation_mhz", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "poweredOffCpuReservationMhz", doc: Some(r#####"Sum of CPU reservation of all the powered-off VMs in the cluster.
"#####) },),
        ("cpu_entitled_mhz", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "cpuEntitledMhz", doc: Some(r#####"This is the current CPU entitlement across the cluster
"#####) },),
        ("total_mem_capacity_mb", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "totalMemCapacityMB", doc: Some(r#####"Total memory capacity of the cluster.
"#####) },),
        ("mem_reservation_mb", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "memReservationMB", doc: Some(r#####"Sum of memory reservation of all the Resource Pools and powered-on VMs in the cluster.
"#####) },),
        ("tier_0_mem_capacity_mb", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "tier0MemCapacityMB", doc: Some(r#####"Total Tier 0 memory capacity in a cluster.

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("cpu_demand_mhz", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "cpuDemandMhz", doc: Some(r#####"Sum of CPU demand of all the powered-on VMs in the cluster.
"#####) },),
        ("reserved_tier_0_mem_mb", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "reservedTier0MemMB", doc: Some(r#####"Total amount of Tier 0 memory used to satisfy virtual machine reservation.

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("powered_off_mem_reservation_mb", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "poweredOffMemReservationMB", doc: Some(r#####"Sum of memory reservation of all the powered-off VMs in the cluster.
"#####) },),
        ("total_cpu_capacity_mhz", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "totalCpuCapacityMhz", doc: Some(r#####"Total CPU capacity of the cluster.
"#####) },),
        ("powered_off_vm_count", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "poweredOffVmCount", doc: Some(r#####"The number of powered off VMs in the cluster
"#####) },),
        ("total_vm_count", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "totalVmCount", doc: Some(r#####"The number of VMs in the cluster
"#####) },),
    ],
}),
        ("ClusterComputeResourceSummary", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (9, 18),
        (3, 5),
        (5, 0),
        (21, 6),
        (0, 0),
    ],
    entries: &[
        ("current_evc_mode_key", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "currentEVCModeKey", doc: Some(r#####"The Enhanced VMotion Compatibility mode that is currently in effect
for all hosts in this cluster; unset if no EVC mode is active.

See also *Capability.supportedEVCMode*.
"#####) },),
        ("usage_summary", NodeData { type_decl: "vim_rs::types::structs::ClusterUsageSummary", type_name: "ClusterUsageSummary", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "usageSummary", doc: Some(r#####"The current usage summary for a DRS cluster.
"#####) },),
        ("num_hosts", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "numHosts", doc: Some(r#####"Total number of hosts.
"#####) },),
        ("num_vms_per_drs_score_bucket", NodeData { type_decl: "Vec<i32>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfInt"), path_segment: "numVmsPerDrsScoreBucket", doc: Some(r#####"The number of VMs in this cluster corresponding to each DRS score
bucket.

The buckets are defined as follows:
- 0% - 20%
- 21% - 40%
- 41% - 60%
- 61% - 80%
- 81% - 100%
"#####) },),
        ("effective_cpu", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "effectiveCpu", doc: Some(r#####"Effective CPU resources (in MHz) available to run virtual machines.

This is the
aggregated effective resource level from all running hosts. Hosts that are in
maintenance mode or are unresponsive are not counted. Resources used by the
VMware Service Console are not included in the aggregate. This value represents
the amount of resources available for the root resource pool for running
virtual machines.
"#####) },),
        ("vcs_slots", NodeData { type_decl: "Vec<vim_rs::types::structs::ClusterComputeResourceVcsSlots>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfClusterComputeResourceVcsSlots"), path_segment: "vcsSlots", doc: Some(r#####"Deprecated as of vSphere 9.0 with no replacement. In a future release
of vSphere, the vCLS functionality will be disabled, vCLS
system VMs will be deleted, and vCLS APIs will be removed.

An array of hosts and number of resource slots on the host for
vSphere Cluster Services in the cluster.

The number of resource slots on the host includes both following types:
1\. Number of vCS VMs running on the host (resource reserved and occupied).
2\. Number of reserved and unoccupied slots (reserved for new vCS VMs).

***Since:*** vSphere API Release 7.0.1.1
"#####) },),
        ("current_failover_level", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "currentFailoverLevel", doc: Some(r#####"Deprecated as of vSphere API 4.0, use
*ClusterFailoverLevelAdmissionControlInfo.currentFailoverLevel*.

Current failover level.

This is the number of physical host failures that can
be tolerated without impacting the ability to satisfy the minimums for all
running virtual machines. This represents the current value, as opposed to
desired value configured by the user.
"#####) },),
        ("current_evc_graphics_mode_key", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "currentEVCGraphicsModeKey", doc: Some(r#####"The Enhanced VMotion Compatibility Graphics mode that is currently in
effect for all hosts in this cluster; unset if no EVC mode is active.

See also *Capability.supportedEVCGraphicsMode*.

***Since:*** vSphere API Release 7.0.1.0
"#####) },),
        ("num_cpu_threads", NodeData { type_decl: "i16", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveShort"), path_segment: "numCpuThreads", doc: Some(r#####"Aggregated number of CPU threads.
"#####) },),
        ("num_effective_hosts", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "numEffectiveHosts", doc: Some(r#####"Total number of effective hosts.
"#####) },),
        ("num_vmotions", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "numVmotions", doc: Some(r#####"Total number of migrations with VMotion that have been done internal to this
cluster.
"#####) },),
        ("overall_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "overallStatus", doc: Some(r#####"Overall alarm status.

In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter.
Since this property is on a DataObject, an update returned by WaitForUpdatesEx may
contain values for this property when some other property on the DataObject changes.
If this update is a result of a call to WaitForUpdatesEx with a non-empty
version parameter, the value for this property may not be current.
"#####) },),
        ("total_cpu", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "totalCpu", doc: Some(r#####"Aggregated CPU resources of all hosts, in MHz.
"#####) },),
        ("total_memory", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "totalMemory", doc: Some(r#####"Aggregated memory resources of all hosts, in bytes.
"#####) },),
        ("current_balance", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "currentBalance", doc: Some(r#####"The current balance, in terms of standard deviation, for a DRS cluster.

Units are thousandths. For example, 12 represents 0.012.
"#####) },),
        ("vcs_health_status", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "vcsHealthStatus", doc: Some(r#####"Deprecated as of vSphere 9.0 with no replacement. In a future release
of vSphere, the vCLS functionality will be disabled, vCLS
system VMs will be deleted, and vCLS APIs will be removed.

The health status of the vSphere Cluster Services in the cluster.

Supported values are enumerated by the *VcsHealthStatus*
type.

***Since:*** vSphere API Release 7.0.1.1
"#####) },),
        ("drs_score", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "drsScore", doc: Some(r#####"The DRS score of this cluster, in percentage.
"#####) },),
        ("target_balance", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "targetBalance", doc: Some(r#####"The target balance, in terms of standard deviation, for a DRS cluster.

Units are thousandths. For example, 12 represents 0.012.
"#####) },),
        ("effective_memory", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "effectiveMemory", doc: Some(r#####"Effective memory resources (in MB) available to run virtual machines.

This is the aggregated effective resource level from all running hosts. Hosts
that are in maintenance mode or are unresponsive are not counted.
Resources used by the VMware Service Console are not included in the aggregate.
This value represents the amount of resources available for the root
resource pool for running virtual machines.
"#####) },),
        ("das_data", NodeData { type_decl: "Box<dyn vim_rs::types::traits::ClusterDasDataTrait>", type_name: "ClusterDasData", is_optional: true, processing_type: FieldProcessingType::Trait, path_segment: "dasData", doc: Some(r#####"Data pertaining to DAS.
"#####) },),
        ("admission_control_info", NodeData { type_decl: "Box<dyn vim_rs::types::traits::ClusterDasAdmissionControlInfoTrait>", type_name: "ClusterDasAdmissionControlInfo", is_optional: true, processing_type: FieldProcessingType::Trait, path_segment: "admissionControlInfo", doc: Some(r#####"Information about the current amount of resources available for a vSphere HA
cluster.

The actual type of admissionControlInfo will depend on what kind of
*ClusterDasAdmissionControlPolicy* was used to configure the cluster.
"#####) },),
        ("num_cpu_cores", NodeData { type_decl: "i16", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveShort"), path_segment: "numCpuCores", doc: Some(r#####"Number of physical CPU cores.

Physical CPU cores are the processors contained
by a CPU package.
"#####) },),
        ("cluster_maintenance_mode_status", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "clusterMaintenanceModeStatus", doc: Some(r#####"Configuration pertinent to state of the cluster maintenance mode.

Valid values are enumerated by the *ClusterMaintenanceModeStatus*
type.

***Since:*** vSphere API Release 7.0.0.2
"#####) },),
    ],
}),
        ("DVSTrafficShapingPolicy", ::phf::Map {
    key: 14108922650502679131,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("average_bandwidth", NodeData { type_decl: "vim_rs::types::structs::LongPolicy", type_name: "LongPolicy", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "averageBandwidth", doc: Some(r#####"The average bandwidth in bits per second if shaping is enabled on
the port.
"#####) },),
        ("inherited", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "inherited", doc: Some(r#####"Whether the configuration is set to inherited value.
"#####) },),
        ("peak_bandwidth", NodeData { type_decl: "vim_rs::types::structs::LongPolicy", type_name: "LongPolicy", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "peakBandwidth", doc: Some(r#####"The peak bandwidth during bursts in bits per second if traffic
shaping is enabled on the port.
"#####) },),
        ("burst_size", NodeData { type_decl: "vim_rs::types::structs::LongPolicy", type_name: "LongPolicy", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "burstSize", doc: Some(r#####"The maximum burst size allowed in bytes if shaping is enabled on
the port.
"#####) },),
        ("enabled", NodeData { type_decl: "vim_rs::types::structs::BoolPolicy", type_name: "BoolPolicy", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "enabled", doc: Some(r#####"The flag to indicate whether or not traffic shaper is enabled on
the port.
"#####) },),
    ],
}),
        ("HostVFlashManagerVFlashResourceRunTimeInfo", ::phf::Map {
    key: 2980949210194914378,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("capacity_for_vm_cache", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "capacityForVmCache", doc: Some(r#####"vFlash resource capacity can be allocated for VM caches
"#####) },),
        ("free_for_vm_cache", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "freeForVmCache", doc: Some(r#####"Free vFlash resource can be allocated for VM caches
"#####) },),
        ("capacity", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "capacity", doc: Some(r#####"Overall capacity of vFlash resource, in bytes.
"#####) },),
        ("accessible", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "accessible", doc: Some(r#####"True if all the included the VFFS volumes are accessible.

False if one or
multiple included VFFS volumes are inaccessible.
"#####) },),
        ("usage", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "usage", doc: Some(r#####"Overall usage of vFlash resource, in bytes.
"#####) },),
    ],
}),
        ("HostTdxInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("num_tdx_private_key_i_ds", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "numTDXPrivateKeyIDs", doc: Some(r#####"The number of TDX private key IDs on this host.
"#####) },),
        ("tdx_state", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "tdxState", doc: Some(r#####"State of TDX on the host.

The supported values are described in
*HostTdxInfoTdxState_enum*.
"#####) },),
    ],
}),
        ("HostDeploymentInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("booted_from_stateless_cache", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "bootedFromStatelessCache", doc: Some(r#####"Flag indicating if the host was booted from stateless cache.
"#####) },),
    ],
}),
        ("HostPowerPolicy", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("description", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "description", doc: Some(r#####"Power Policy Description.
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"Power Policy Name.
"#####) },),
        ("key", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "key", doc: Some(r#####"Power Policy Key.

Internally generated key which uniquely identifies power management
policy on a host.
"#####) },),
        ("short_name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "shortName", doc: Some(r#####"Power Policy Short Name.

This is not localizable property which can be used to identify specific
power managing policies like "custom" power policy. Custom power policy
has short name set to "custom".
"#####) },),
    ],
}),
        ("DatacenterConfigInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("default_hardware_version_key", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "defaultHardwareVersionKey", doc: Some(r#####"Key for Default Hardware Version used on this datacenter
in the format of *VirtualMachineConfigOptionDescriptor.key*.

This field affects
*VirtualMachineConfigOptionDescriptor.defaultConfigOption* returned
by *ComputeResource.environmentBrowser* of all its children
with this field unset.
"#####) },),
        ("maximum_hardware_version_key", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "maximumHardwareVersionKey", doc: Some(r#####"Key for Maximum Hardware Version used on this datacenter
in the format of *VirtualMachineConfigOptionDescriptor.key*.

This field affects
*VirtualMachineConfigOptionDescriptor.defaultConfigOption* returned
by *ComputeResource.environmentBrowser* of all its children
with this field unset.

***Since:*** vSphere API Release 7.0.2.0
"#####) },),
    ],
}),
        ("ReplicationConfigSpec", ::phf::Map {
    key: 7485420634051515786,
    disps: &[
        (1, 1),
        (9, 12),
        (2, 0),
    ],
    entries: &[
        ("data_sets_replication_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "dataSetsReplicationEnabled", doc: Some(r#####"Flag that indicates whether DataSets files are replicated or not.

***Since:*** vSphere API Release 8.0.0.0
"#####) },),
        ("generation", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "generation", doc: Some(r#####"A generation number (&gt;=0) that reflects the "freshness" of the
ReplicationConfigSpec on which a re-configuration is based.

The
generation number is used to detect and disallow concurrent
updates to a VM's replication settings.
For initial replication enablement, generation = 0. The
replication settings of every replication re-configuration
operation must reflect the latest generation number known to the
caller. It takes an explicit call to get the latest replication
settings to find out what the latest generation number is. The
update algorithm of the generation number is opaque to the
caller; e.g., the caller cannot assume that the generation
numbers are incremented by one every time replication is
(re)configured, not even that they are changing monotonically.
"#####) },),
        ("vm_replication_id", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "vmReplicationId", doc: Some(r#####"An opaque identifier that uniquely identifies a replicated VM
between primary and secondary sites.
"#####) },),
        ("net_encryption_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "netEncryptionEnabled", doc: Some(r#####"Flag that indicates whether or not encription should
be used when sending traffic over the network.

The primary will use the remoteCertificateThumbprint
to verify the identity of the remote server.
"#####) },),
        ("rpo", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "rpo", doc: Some(r#####"The Recovery Point Objective specified for this VM, in minutes.

Currently, valid values are in the range of 1 minute to 1440
minutes (24 hours).
"#####) },),
        ("quiesce_guest_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "quiesceGuestEnabled", doc: Some(r#####"Flag that indicates whether or not to quiesce the file system or
applications in the guest OS before a consistent replica is
created.
"#####) },),
        ("port", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "port", doc: Some(r#####"The port on the HBR Server in the secondary site where this VM
is replicated to.

Note: If net encryption is enabled, this is the port of the
encryption tunneling agent.
"#####) },),
        ("remote_certificate_thumbprint", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "remoteCertificateThumbprint", doc: Some(r#####"Deprecated field is deprecated, use
*HbrManager.HbrConfigureReplicationTargets_Task* instead.

The SHA256 thumbprint of the remote server certificate.

This field is only relevant when net encription is enabled.
"#####) },),
        ("opp_updates_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "oppUpdatesEnabled", doc: Some(r#####"Flag that indicates whether or not to perform opportunistic
updates in-between consistent replicas.
"#####) },),
        ("destination", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "destination", doc: Some(r#####"The IP address of the HBR Server in the secondary site
where this VM is replicated to.

Note: If net encryption is enabled, this is the address of the
encryption tunnelling agent.
"#####) },),
        ("encryption_destination", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "encryptionDestination", doc: Some(r#####"The IP address of the remote HBR server, target for encrypted LWD.

This field is required when net encryption is enabled, ignored otherwise.
"#####) },),
        ("net_compression_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "netCompressionEnabled", doc: Some(r#####"Flag that indicates whether or not compression should
be used when sending traffic over the network.

The primary will negotiate the best compression with
the server on the secondary if this is enabled.
"#####) },),
        ("encryption_port", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "encryptionPort", doc: Some(r#####"The port on the remote HBR server, target for encrypted LWD.

This field is only relevant when net encryption is enabled.
"#####) },),
        ("paused", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "paused", doc: Some(r#####"Flag that indicates whether or not the vm or group has been paused for
replication.
"#####) },),
        ("disk", NodeData { type_decl: "Vec<vim_rs::types::structs::ReplicationInfoDiskSettings>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfReplicationInfoDiskSettings"), path_segment: "disk", doc: Some(r#####"The set of the disks of this VM that are configured for
replication.
"#####) },),
    ],
}),
        ("AutoStartDefaults", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (3, 0),
    ],
    entries: &[
        ("enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "enabled", doc: Some(r#####"Indicates whether or not auto-start manager is enabled.
"#####) },),
        ("wait_for_heartbeat", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "waitForHeartbeat", doc: Some(r#####"System-default waitForHeartbeat setting.
"#####) },),
        ("start_delay", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "startDelay", doc: Some(r#####"System-default autoStart delay in seconds.

The default is 120 seconds.
"#####) },),
        ("stop_delay", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "stopDelay", doc: Some(r#####"System-default autoStop delay in seconds.

The default is 120 seconds.
"#####) },),
        ("stop_action", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "stopAction", doc: Some(r#####"System-default power-off action.

Used if the stopAction string in the
AutoPowerInfo object for a particular machine is set to systemDefault.
If stopAction and startAction for a virtual machine are both set to none,
that virtual machine is removed from the AutoStart sequence.
"#####) },),
    ],
}),
        ("HostSystemResourceInfo", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("config", NodeData { type_decl: "vim_rs::types::structs::ResourceConfigSpec", type_name: "ResourceConfigSpec", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "config", doc: Some(r#####"Configuration of this system resource group.
"#####) },),
        ("child", NodeData { type_decl: "Vec<vim_rs::types::structs::HostSystemResourceInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostSystemResourceInfo"), path_segment: "child", doc: Some(r#####"List of child resource groups.
"#####) },),
        ("key", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "key", doc: Some(r#####"ID of the system resource group.
"#####) },),
    ],
}),
        ("ToolsConfigInfoToolsLastInstallInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("counter", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "counter", doc: Some(r#####"Number of attempts that have been made to upgrade the version of tools
installed on this virtual machine.
"#####) },),
        ("fault", NodeData { type_decl: "vim_rs::types::structs::MethodFault", type_name: "MethodFault", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "fault", doc: Some(r#####"Error that happened, if any, during last attempt to upgrade tools.
"#####) },),
    ],
}),
        ("HostConfigManager", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (4, 26),
        (0, 10),
        (2, 6),
        (1, 0),
        (6, 3),
        (1, 2),
        (10, 31),
        (0, 3),
    ],
    entries: &[
        ("iscsi_manager", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "iscsiManager", doc: Some(r#####"Iscsi Management Operations managed entity

Refers instance of *IscsiManager*.
"#####) },),
        ("nvdimm_system", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "nvdimmSystem", doc: Some(r#####"Host Non-Volatile DIMM configuration manager

Refers instance of *HostNvdimmSystem*.
"#####) },),
        ("firewall_system", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "firewallSystem", doc: Some(r#####"The firewall configuration.

Refers instance of *HostFirewallSystem*.
"#####) },),
        ("cpu_scheduler", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "cpuScheduler", doc: Some(r#####"The CPU scheduler that determines which threads execute on a CPU
at any given time.

Refers instance of *HostCpuSchedulerSystem*.
"#####) },),
        ("service_system", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "serviceSystem", doc: Some(r#####"The configuration of the host services (for example,
SSH, FTP, and Telnet).

Refers instance of *HostServiceSystem*.
"#####) },),
        ("authentication_manager", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "authenticationManager", doc: Some(r#####"Authentication method configuration - for example, for Active Directory membership.

Refers instance of *HostAuthenticationManager*.
"#####) },),
        ("boot_device_system", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "bootDeviceSystem", doc: Some(r#####"Boot device order management.

Refers instance of *HostBootDeviceSystem*.
"#####) },),
        ("virtual_nic_manager", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "virtualNicManager", doc: Some(r#####"The VirtualNic configuration.

Refers instance of *HostVirtualNicManager*.
"#####) },),
        ("patch_manager", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "patchManager", doc: Some(r#####"Host patch management.

Refers instance of *HostPatchManager*.
"#####) },),
        ("date_time_system", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "dateTimeSystem", doc: Some(r#####"DateTime configuration

Refers instance of *HostDateTimeSystem*.
"#####) },),
        ("image_config_manager", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "imageConfigManager", doc: Some(r#####"Host image configuration management.

Refers instance of *HostImageConfigManager*.
"#####) },),
        ("cache_configuration_manager", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "cacheConfigurationManager", doc: Some(r#####"Host solid state drive cache configuration manager.

Refers instance of *HostCacheConfigurationManager*.
"#####) },),
        ("auto_start_manager", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "autoStartManager", doc: Some(r#####"Auto-start and auto-stop configuration.

Refers instance of *HostAutoStartManager*.
"#####) },),
        ("host_access_manager", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "hostAccessManager", doc: Some(r#####"Host access manager

Refers instance of *HostAccessManager*.
"#####) },),
        ("graphics_manager", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "graphicsManager", doc: Some(r#####"Host graphics manager.

Refers instance of *HostGraphicsManager*.
"#####) },),
        ("assignable_hardware_manager", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "assignableHardwareManager", doc: Some(r#####"Assignable Hardware manager.

Refers instance of *HostAssignableHardwareManager*.
"#####) },),
        ("advanced_option", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "advancedOption", doc: Some(r#####"Advanced options.

Refers instance of *OptionManager*.
"#####) },),
        ("account_manager", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "accountManager", doc: Some(r#####"A manager for host local user accounts.

Refers instance of *HostLocalAccountManager*.
"#####) },),
        ("user_directory", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "userDirectory", doc: Some(r#####"A user directory managed object.

Refers instance of *UserDirectory*.
"#####) },),
        ("certificate_manager", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "certificateManager", doc: Some(r#####"Host CertificateManager.

Refers instance of *HostCertificateManager*.
"#####) },),
        ("vmotion_system", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "vmotionSystem", doc: Some(r#####"Deprecated as of VI API 4.0, use *HostConfigManager.virtualNicManager*
to manage the VMotion configuration of the host.

The VMotion configuration.

Refers instance of *HostVMotionSystem*.
"#####) },),
        ("message_bus_proxy", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "messageBusProxy", doc: Some(r#####"Common Message Bus proxy service.

This API shall always be present in vSphere API 6.0 or later.

Refers instance of *MessageBusProxy*.
"#####) },),
        ("pci_passthru_system", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "pciPassthruSystem", doc: Some(r#####"PciDeviceSystem for passthru.

Refers instance of *HostPciPassthruSystem*.
"#####) },),
        ("memory_manager", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "memoryManager", doc: Some(r#####"The memory manager on the host.

Refers instance of *HostMemorySystem*.
"#####) },),
        ("esx_agent_host_manager", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "esxAgentHostManager", doc: Some(r#####"Esx Agent resource configuration manager

Refers instance of *HostEsxAgentHostManager*.
"#####) },),
        ("health_status_system", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "healthStatusSystem", doc: Some(r#####"System health status manager.

Refers instance of *HostHealthStatusSystem*.
"#####) },),
        ("license_manager", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "licenseManager", doc: Some(r#####"License manager

Refers instance of *LicenseManager*.
"#####) },),
        ("crypto_manager", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "cryptoManager", doc: Some(r#####"Host CryptoManager.

Refers instance of *CryptoManager*.
"#####) },),
        ("network_system", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "networkSystem", doc: Some(r#####"The network system configuration.

Refers instance of *HostNetworkSystem*.
"#####) },),
        ("diagnostic_system", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "diagnosticSystem", doc: Some(r#####"The diagnostic for the ESX Server system.

Refers instance of *HostDiagnosticSystem*.
"#####) },),
        ("kernel_module_system", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "kernelModuleSystem", doc: Some(r#####"Kernel module configuration management.

Refers instance of *HostKernelModuleSystem*.
"#####) },),
        ("firmware_system", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "firmwareSystem", doc: Some(r#####"Firmware management.

Refers instance of *HostFirmwareSystem*.
"#####) },),
        ("vsan_internal_system", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "vsanInternalSystem", doc: Some(r#####"VsanInternalSystem managed entity.

Refers instance of *HostVsanInternalSystem*.
"#####) },),
        ("v_flash_manager", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "vFlashManager", doc: Some(r#####"vFlash Manager

Refers instance of *HostVFlashManager*.
"#####) },),
        ("vsan_system", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "vsanSystem", doc: Some(r#####"VsanSystem managed entity.

Refers instance of *HostVsanSystem*.
"#####) },),
        ("storage_system", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "storageSystem", doc: Some(r#####"The storage configuration.

Refers instance of *HostStorageSystem*.
"#####) },),
        ("datastore_system", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "datastoreSystem", doc: Some(r#####"The datastore manager.

Refers instance of *HostDatastoreSystem*.
"#####) },),
        ("snmp_system", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "snmpSystem", doc: Some(r#####"Snmp configuration

Refers instance of *HostSnmpSystem*.
"#####) },),
        ("power_system", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "powerSystem", doc: Some(r#####"Power System manager.

Refers instance of *HostPowerSystem*.
"#####) },),
    ],
}),
        ("ScheduledHardwareUpgradeInfo", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("scheduled_hardware_upgrade_status", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "scheduledHardwareUpgradeStatus", doc: Some(r#####"Status for last attempt to run scheduled hardware upgrade.

See also *ScheduledHardwareUpgradeInfoHardwareUpgradeStatus_enum*.
"#####) },),
        ("upgrade_policy", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "upgradePolicy", doc: Some(r#####"Scheduled hardware upgrade policy setting for the virtual machine.

See also *ScheduledHardwareUpgradeInfoHardwareUpgradePolicy_enum*.
"#####) },),
        ("version_key", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "versionKey", doc: Some(r#####"Key for target hardware version to be used on next scheduled upgrade
in the format of *VirtualMachineConfigOptionDescriptor.key*.
"#####) },),
        ("fault", NodeData { type_decl: "vim_rs::types::structs::MethodFault", type_name: "MethodFault", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "fault", doc: Some(r#####"Contains information about the failure of last attempt to run
scheduled hardware upgrade.
"#####) },),
    ],
}),
        ("HostAssignableHardwareConfig", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("attribute_override", NodeData { type_decl: "Vec<vim_rs::types::structs::HostAssignableHardwareConfigAttributeOverride>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostAssignableHardwareConfigAttributeOverride"), path_segment: "attributeOverride", doc: Some(r#####"List of attribute overrides.
"#####) },),
    ],
}),
        ("HostVMotionInfo", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("net_config", NodeData { type_decl: "vim_rs::types::structs::HostVMotionNetConfig", type_name: "HostVMotionNetConfig", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "netConfig", doc: Some(r#####"VMotion network configuration.
"#####) },),
        ("ip_config", NodeData { type_decl: "Box<dyn vim_rs::types::traits::HostIpConfigTrait>", type_name: "HostIpConfig", is_optional: true, processing_type: FieldProcessingType::Trait, path_segment: "ipConfig", doc: Some(r#####"IP configuration of the VMotion VirtualNic.
"#####) },),
    ],
}),
        ("Datacenter", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
        (21, 1),
        (13, 14),
        (0, 14),
        (2, 17),
    ],
    entries: &[
        ("parent", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "parent", doc: Some(r#####"Parent of this entity.

This value is null for the root object and for
*VirtualMachine* objects that are part of
a *VirtualApp*.

***Required privileges:*** System.View
"#####) },),
        ("network", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "network", doc: Some(r#####"A collection of references to the network objects
available in this datacenter.
"#####) },),
        ("available_field", NodeData { type_decl: "Vec<vim_rs::types::structs::CustomFieldDef>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldDef"), path_segment: "availableField", doc: Some(r#####"List of custom field definitions that are valid for the object's type.

The fields are sorted by *CustomFieldDef.name*.

***Required privileges:*** System.View
"#####) },),
        ("disabled_method", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "disabledMethod", doc: Some(r#####"List of operations that are disabled, given the current runtime
state of the entity.

For example, a power-on operation always fails if a
virtual machine is already powered on. This list can be used by clients to
enable or disable operations in a graphical user interface.

Note: This list is determined by the current runtime state of an entity,
not by its permissions.

This list may include the following operations for a HostSystem:
- *HostSystem.EnterMaintenanceMode_Task*
- *HostSystem.ExitMaintenanceMode_Task*
- *HostSystem.RebootHost_Task*
- *HostSystem.ShutdownHost_Task*
- *HostSystem.ReconnectHost_Task*
- *HostSystem.DisconnectHost_Task*
  
This list may include the following operations for a VirtualMachine:
- *VirtualMachine.AnswerVM*
- *ManagedEntity.Rename_Task*
- *VirtualMachine.CloneVM_Task*
- *VirtualMachine.PowerOffVM_Task*
- *VirtualMachine.PowerOnVM_Task*
- *VirtualMachine.SuspendVM_Task*
- *VirtualMachine.ResetVM_Task*
- *VirtualMachine.ReconfigVM_Task*
- *VirtualMachine.RelocateVM_Task*
- *VirtualMachine.MigrateVM_Task*
- *VirtualMachine.CustomizeVM_Task*
- *VirtualMachine.ShutdownGuest*
- *VirtualMachine.StandbyGuest*
- *VirtualMachine.RebootGuest*
- *VirtualMachine.CreateSnapshot_Task*
- *VirtualMachine.RemoveAllSnapshots_Task*
- *VirtualMachine.RevertToCurrentSnapshot_Task*
- *VirtualMachine.MarkAsTemplate*
- *VirtualMachine.MarkAsVirtualMachine*
- *VirtualMachine.ResetGuestInformation*
- *VirtualMachine.MountToolsInstaller*
- *VirtualMachine.UnmountToolsInstaller*
- *ManagedEntity.Destroy_Task*
- *VirtualMachine.UpgradeVM_Task*
- *VirtualMachine.ExportVm*
  
This list may include the following operations for a ResourcePool:
- *ResourcePool.ImportVApp*
- *ResourcePool.CreateChildVM_Task*
- *ResourcePool.UpdateConfig*
- *Folder.CreateVM_Task*
- *ManagedEntity.Destroy_Task*
- *ManagedEntity.Rename_Task*
  
This list may include the following operations for a VirtualApp:
- *ManagedEntity.Destroy_Task*
- *VirtualApp.CloneVApp_Task*
- *VirtualApp.unregisterVApp_Task*
- *VirtualApp.ExportVApp*
- *VirtualApp.PowerOnVApp_Task*
- *VirtualApp.PowerOffVApp_Task*
- *VirtualApp.UpdateVAppConfig*
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("triggered_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "triggeredAlarmState", doc: Some(r#####"A set of alarm states for alarms triggered by this entity
or by its descendants.

Triggered alarms are propagated up the inventory hierarchy
so that a user can readily tell when a descendant has triggered an alarm.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.

***Required privileges:*** System.View
"#####) },),
        ("network_folder", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "networkFolder", doc: Some(r#####"A reference to the folder hierarchy that contains the network entities
for this datacenter.

The folder can include *Network*,
*DistributedVirtualSwitch*, and
*DistributedVirtualPortgroup* objects.

This folder is guaranteed to exist.

***Required privileges:*** System.View
"#####) },),
        ("config_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "configStatus", doc: Some(r#####"The configStatus indicates whether or not the system has detected a configuration
issue involving this entity.

For example, it might have detected a
duplicate IP address or MAC address, or a host in a cluster
might be out of compliance. The meanings of the configStatus values are:
- red: A problem has been detected involving the entity.
- yellow: A problem is about to occur or a transient condition
  has occurred (For example, reconfigure fail-over policy).
- green: No configuration issues have been detected.
- gray: The configuration status of the entity is not being monitored.
  
A green status indicates only that a problem has not been detected;
it is not a guarantee that the entity is problem-free.

The *configIssue* property contains a list of the
problems that have been detected.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("tag", NodeData { type_decl: "Vec<vim_rs::types::structs::Tag>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfTag"), path_segment: "tag", doc: Some(r#####"The set of tags associated with this managed entity.

Experimental. Subject to change.

***Required privileges:*** System.View
"#####) },),
        ("vm_folder", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "vmFolder", doc: Some(r#####"A reference to the folder hierarchy that contains *VirtualMachine*
virtual machine templates (identified by the *VirtualMachineConfigInfo.template*
property, and *VirtualApp* objects for this datacenter.

Note that a VirtualApp that is a child of a *ResourcePool*
may also be visible in this folder. VirtualApp objects can be nested,
but only the parent VirtualApp can be visible in the folder.

This folder is guaranteed to exist.

***Required privileges:*** System.View
"#####) },),
        ("host_folder", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "hostFolder", doc: Some(r#####"A reference to the folder hierarchy that contains
the compute resources, including hosts and clusters, for this datacenter.

This folder is guaranteed to exist.

***Required privileges:*** System.View
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"Name of this entity, unique relative to its parent.

Any / (slash), \\ (backslash), character used in this
name element will be escaped. Similarly, any % (percent) character used in
this name element will be escaped, unless it is used to start an escape
sequence. A slash is escaped as %2F or %2f. A backslash is escaped as %5C or
%5c, and a percent is escaped as %25.

***Required privileges:*** System.View
"#####) },),
        ("overall_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "overallStatus", doc: Some(r#####"General health of this managed entity.

The overall status of the managed entity is computed as the worst status
among its alarms and the configuration issues detected on the entity.
The status is reported as one of the following values:
- red: The entity has alarms or configuration issues with a red status.
- yellow: The entity does not have alarms or configuration issues with a
  red status, and has at least one with a yellow status.
- green: The entity does not have alarms or configuration issues with a
  red or yellow status, and has at least one with a green status.
- gray: All of the entity's alarms have a gray status and the
  configuration status of the entity is not being monitored.
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("datastore", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "datastore", doc: Some(r#####"A collection of references to the datastore objects
available in this datacenter.
"#####) },),
        ("recent_task", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "recentTask", doc: Some(r#####"The set of recent tasks operating on this managed entity.

This is a subset
of *TaskManager.recentTask* belong to this entity. A task in this
list could be in one of the four states: pending, running, success or error.

This property can be used to deduce intermediate power states for
a virtual machine entity. For example, if the current powerState is "poweredOn"
and there is a running task performing the "suspend" operation, then the virtual
machine's intermediate state might be described as "suspending."

Most tasks (such as power operations) obtain exclusive access to the virtual
machine, so it is unusual for this list to contain more than one running task.
One exception, however, is the task of cloning a virtual machine.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "value", doc: Some(r#####"List of custom field values.

Each value uses a key to associate
an instance of a *CustomFieldStringValue* with
a custom field definition.

***Required privileges:*** System.View
"#####) },),
        ("datastore_folder", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "datastoreFolder", doc: Some(r#####"A reference to the folder hierarchy that contains
the datastores for this datacenter.

This folder is guaranteed to exist.

***Required privileges:*** System.View
"#####) },),
        ("alarm_actions_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "alarmActionsEnabled", doc: Some(r#####"Whether alarm actions are enabled for this entity.

True if enabled; false otherwise.

***Required privileges:*** System.Read
"#####) },),
        ("effective_role", NodeData { type_decl: "Vec<i32>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfInt"), path_segment: "effectiveRole", doc: Some(r#####"Access rights the current session has to this entity.

***Required privileges:*** System.View
"#####) },),
        ("config_issue", NodeData { type_decl: "Vec<vim_rs::types::structs::Event>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfEvent"), path_segment: "configIssue", doc: Some(r#####"Current configuration issues that have been detected for this entity.

Typically,
these issues have already been logged as events. The entity stores these
events as long as they are still current. The
*configStatus* property provides an overall status
based on these events.
"#####) },),
        ("custom_value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "customValue", doc: Some(r#####"Custom field values.

***Required privileges:*** System.View
"#####) },),
        ("declared_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "declaredAlarmState", doc: Some(r#####"A set of alarm states for alarms that apply to this managed entity.

The set includes alarms defined on this entity
and alarms inherited from the parent entity,
or from any ancestors in the inventory hierarchy.

Alarms are inherited if they can be triggered by this entity or its descendants.
This set does not include alarms that are defined on descendants of this entity.

***Required privileges:*** System.View
"#####) },),
        ("configuration", NodeData { type_decl: "vim_rs::types::structs::DatacenterConfigInfo", type_name: "DatacenterConfigInfo", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "configuration", doc: Some(r#####"Configuration of the datacenter.

***Required privileges:*** System.View
"#####) },),
        ("permission", NodeData { type_decl: "Vec<vim_rs::types::structs::Permission>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfPermission"), path_segment: "permission", doc: Some(r#####"List of permissions defined for this entity.
"#####) },),
    ],
}),
        ("LongPolicy", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("inherited", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "inherited", doc: Some(r#####"Whether the configuration is set to inherited value.
"#####) },),
        ("value", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "value", doc: Some(r#####"The boolean value that is either set or inherited.
"#####) },),
    ],
}),
        ("StorageDrsAutomationConfig", ::phf::Map {
    key: 1937371814602216758,
    disps: &[
        (4, 0),
    ],
    entries: &[
        ("io_load_balance_automation_mode", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "ioLoadBalanceAutomationMode", doc: Some(r#####"Deprecated as of vSphere8.0 U3, and there is no replacement for it.

Specifies the behavior of Storage DRS when it generates
recommendations for correcting I/O load imbalance in a datastore
cluster.

See *StorageDrsPodConfigInfo*. If specified, this option
overrides the datastore cluster level automation behavior defined in the
*StorageDrsPodConfigInfo*.
"#####) },),
        ("space_load_balance_automation_mode", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "spaceLoadBalanceAutomationMode", doc: Some(r#####"Specifies the behavior of Storage DRS when it generates
recommendations for correcting space load imbalance in a
datastore cluster.

See *StorageDrsPodConfigInfo*. If specified, this option
overrides the datastore cluster level automation behavior defined in the
*StorageDrsPodConfigInfo*.
"#####) },),
        ("rule_enforcement_automation_mode", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "ruleEnforcementAutomationMode", doc: Some(r#####"Specifies the behavior of Storage DRS when it generates
recommendations for correcting affinity rule violations in a
datastore cluster.

See *StorageDrsPodConfigInfoBehavior_enum*. If
specified, this option overrides the datastore cluster level
automation behavior defined in the *StorageDrsPodConfigInfo* for
recommendations aimed at fixing rule violations.
"#####) },),
        ("policy_enforcement_automation_mode", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "policyEnforcementAutomationMode", doc: Some(r#####"Specifies the behavior of Storage DRS when it generates
recommendations for correcting storage and Vm policy violations
in a datastore cluster.

See *StorageDrsPodConfigInfoBehavior_enum*. If
specified, this option overrides the datastore cluster level
automation behavior defined in the *StorageDrsPodConfigInfo* for
recommendations aimed at fixing storage policy violations.
"#####) },),
        ("vm_evacuation_automation_mode", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "vmEvacuationAutomationMode", doc: Some(r#####"Specifies the behavior of Storage DRS when it generates
recommendations for datastore evacuations in a datastore
cluster.

See *StorageDrsPodConfigInfoBehavior_enum*. If specified, this
option overrides the datastore cluster level automation behavior
defined in the *StorageDrsPodConfigInfo* for recommendations aimed at
evacuating Vms from a datastore.
"#####) },),
    ],
}),
        ("VirtualMachineSnapshotInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("root_snapshot_list", NodeData { type_decl: "Vec<vim_rs::types::structs::VirtualMachineSnapshotTree>", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ArrayOfVirtualMachineSnapshotTree"), path_segment: "rootSnapshotList", doc: Some(r#####"Data for the entire set of snapshots for one virtual machine.
"#####) },),
        ("current_snapshot", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "currentSnapshot", doc: Some(r#####"Current snapshot of the virtual machine

This property is set by calling
*Snapshot.revert* or
*VirtualMachine.createSnapshot*.
This property will be empty when the working snapshot is at the root
of the snapshot tree.

Refers instance of *VirtualMachineSnapshot*.
"#####) },),
    ],
}),
        ("VirtualMachineVirtualDeviceSwap", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("lsi_to_pvscsi", NodeData { type_decl: "vim_rs::types::structs::VirtualMachineVirtualDeviceSwapDeviceSwapInfo", type_name: "VirtualMachineVirtualDeviceSwapDeviceSwapInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "lsiToPvscsi", doc: None },),
    ],
}),
        ("HostTpmAttestationInfo", ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("time", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveDateTime"), path_segment: "time", doc: Some(r#####"Time of TPM attestation.
"#####) },),
        ("message", NodeData { type_decl: "vim_rs::types::structs::LocalizableMessage", type_name: "LocalizableMessage", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "message", doc: Some(r#####"Message explaining TPM attestation failure.
"#####) },),
        ("status", NodeData { type_decl: "vim_rs::types::enums::HostTpmAttestationInfoAcceptanceStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("HostTpmAttestationInfoAcceptanceStatus"), path_segment: "status", doc: Some(r#####"Attestation status.

Valid values are enumerated by the
*HostTpmAttestationInfoAcceptanceStatus_enum* type.
"#####) },),
    ],
}),
        ("HostNetworkResourceRuntime", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("pnic_resource_info", NodeData { type_decl: "Vec<vim_rs::types::structs::HostPnicNetworkResourceInfo>", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ArrayOfHostPnicNetworkResourceInfo"), path_segment: "pnicResourceInfo", doc: Some(r#####"The network resource related information on each
physical NIC
"#####) },),
    ],
}),
        ("VirtualMachineStorageSummary", ::phf::Map {
    key: 14108922650502679131,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("uncommitted", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "uncommitted", doc: Some(r#####"Additional storage space, in bytes, potentially used by this virtual machine
on all datastores.

Essentially an aggregate of the property
*VirtualMachineUsageOnDatastore.uncommitted* across all
datastores that this virtual machine is located on.
"#####) },),
        ("unshared", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "unshared", doc: Some(r#####"Total storage space, in bytes, occupied by the virtual machine across
all datastores, that is not shared with any other virtual machine.
"#####) },),
        ("committed", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "committed", doc: Some(r#####"Total storage space, in bytes, committed to this virtual machine across
all datastores.

Essentially an aggregate of the property
*VirtualMachineUsageOnDatastore.committed* across all
datastores that this virtual machine is located on.
"#####) },),
        ("timestamp", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveDateTime"), path_segment: "timestamp", doc: Some(r#####"Time when values in this structure were last updated.
"#####) },),
    ],
}),
        ("ResourcePool", ::phf::Map {
    key: 8602556344903797927,
    disps: &[
        (1, 3),
        (14, 0),
        (0, 0),
        (0, 0),
        (9, 4),
    ],
    entries: &[
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"Name of this entity, unique relative to its parent.

Any / (slash), \\ (backslash), character used in this
name element will be escaped. Similarly, any % (percent) character used in
this name element will be escaped, unless it is used to start an escape
sequence. A slash is escaped as %2F or %2f. A backslash is escaped as %5C or
%5c, and a percent is escaped as %25.

***Required privileges:*** System.View
"#####) },),
        ("available_field", NodeData { type_decl: "Vec<vim_rs::types::structs::CustomFieldDef>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldDef"), path_segment: "availableField", doc: Some(r#####"List of custom field definitions that are valid for the object's type.

The fields are sorted by *CustomFieldDef.name*.

***Required privileges:*** System.View
"#####) },),
        ("config_issue", NodeData { type_decl: "Vec<vim_rs::types::structs::Event>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfEvent"), path_segment: "configIssue", doc: Some(r#####"Current configuration issues that have been detected for this entity.

Typically,
these issues have already been logged as events. The entity stores these
events as long as they are still current. The
*configStatus* property provides an overall status
based on these events.
"#####) },),
        ("triggered_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "triggeredAlarmState", doc: Some(r#####"A set of alarm states for alarms triggered by this entity
or by its descendants.

Triggered alarms are propagated up the inventory hierarchy
so that a user can readily tell when a descendant has triggered an alarm.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.

***Required privileges:*** System.View
"#####) },),
        ("runtime", NodeData { type_decl: "vim_rs::types::structs::ResourcePoolRuntimeInfo", type_name: "ResourcePoolRuntimeInfo", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "runtime", doc: Some(r#####"Runtime information about a resource pool.

The *ResourcePoolResourceUsage* information within
*ResourcePoolRuntimeInfo* can be transiently stale.
Use *ResourcePool.RefreshRuntime* method to
update the information.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("config_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "configStatus", doc: Some(r#####"The configStatus indicates whether or not the system has detected a configuration
issue involving this entity.

For example, it might have detected a
duplicate IP address or MAC address, or a host in a cluster
might be out of compliance. The meanings of the configStatus values are:
- red: A problem has been detected involving the entity.
- yellow: A problem is about to occur or a transient condition
  has occurred (For example, reconfigure fail-over policy).
- green: No configuration issues have been detected.
- gray: The configuration status of the entity is not being monitored.
  
A green status indicates only that a problem has not been detected;
it is not a guarantee that the entity is problem-free.

The *configIssue* property contains a list of the
problems that have been detected.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("owner", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "owner", doc: Some(r#####"The ComputeResource to which this set of one or more nested resource pools
belong.

***Required privileges:*** System.View
"#####) },),
        ("effective_role", NodeData { type_decl: "Vec<i32>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfInt"), path_segment: "effectiveRole", doc: Some(r#####"Access rights the current session has to this entity.

***Required privileges:*** System.View
"#####) },),
        ("resource_pool", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "resourcePool", doc: Some(r#####"The set of child resource pools.

***Required privileges:*** System.View
"#####) },),
        ("config", NodeData { type_decl: "vim_rs::types::structs::ResourceConfigSpec", type_name: "ResourceConfigSpec", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "config", doc: Some(r#####"Configuration of this resource pool.
"#####) },),
        ("declared_alarm_state", NodeData { type_decl: "Vec<vim_rs::types::structs::AlarmState>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfAlarmState"), path_segment: "declaredAlarmState", doc: Some(r#####"A set of alarm states for alarms that apply to this managed entity.

The set includes alarms defined on this entity
and alarms inherited from the parent entity,
or from any ancestors in the inventory hierarchy.

Alarms are inherited if they can be triggered by this entity or its descendants.
This set does not include alarms that are defined on descendants of this entity.

***Required privileges:*** System.View
"#####) },),
        ("custom_value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "customValue", doc: Some(r#####"Custom field values.

***Required privileges:*** System.View
"#####) },),
        ("child_configuration", NodeData { type_decl: "Vec<vim_rs::types::structs::ResourceConfigSpec>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfResourceConfigSpec"), path_segment: "childConfiguration", doc: Some(r#####"The resource configuration of all direct children (VirtualMachine and
ResourcePool) of this resource group.

Property collector update notifications might not be generated for this
property. To listen for the child configuration change, please create
PropertyCollector filter on the child entities directly.
"#####) },),
        ("disabled_method", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "disabledMethod", doc: Some(r#####"List of operations that are disabled, given the current runtime
state of the entity.

For example, a power-on operation always fails if a
virtual machine is already powered on. This list can be used by clients to
enable or disable operations in a graphical user interface.

Note: This list is determined by the current runtime state of an entity,
not by its permissions.

This list may include the following operations for a HostSystem:
- *HostSystem.EnterMaintenanceMode_Task*
- *HostSystem.ExitMaintenanceMode_Task*
- *HostSystem.RebootHost_Task*
- *HostSystem.ShutdownHost_Task*
- *HostSystem.ReconnectHost_Task*
- *HostSystem.DisconnectHost_Task*
  
This list may include the following operations for a VirtualMachine:
- *VirtualMachine.AnswerVM*
- *ManagedEntity.Rename_Task*
- *VirtualMachine.CloneVM_Task*
- *VirtualMachine.PowerOffVM_Task*
- *VirtualMachine.PowerOnVM_Task*
- *VirtualMachine.SuspendVM_Task*
- *VirtualMachine.ResetVM_Task*
- *VirtualMachine.ReconfigVM_Task*
- *VirtualMachine.RelocateVM_Task*
- *VirtualMachine.MigrateVM_Task*
- *VirtualMachine.CustomizeVM_Task*
- *VirtualMachine.ShutdownGuest*
- *VirtualMachine.StandbyGuest*
- *VirtualMachine.RebootGuest*
- *VirtualMachine.CreateSnapshot_Task*
- *VirtualMachine.RemoveAllSnapshots_Task*
- *VirtualMachine.RevertToCurrentSnapshot_Task*
- *VirtualMachine.MarkAsTemplate*
- *VirtualMachine.MarkAsVirtualMachine*
- *VirtualMachine.ResetGuestInformation*
- *VirtualMachine.MountToolsInstaller*
- *VirtualMachine.UnmountToolsInstaller*
- *ManagedEntity.Destroy_Task*
- *VirtualMachine.UpgradeVM_Task*
- *VirtualMachine.ExportVm*
  
This list may include the following operations for a ResourcePool:
- *ResourcePool.ImportVApp*
- *ResourcePool.CreateChildVM_Task*
- *ResourcePool.UpdateConfig*
- *Folder.CreateVM_Task*
- *ManagedEntity.Destroy_Task*
- *ManagedEntity.Rename_Task*
  
This list may include the following operations for a VirtualApp:
- *ManagedEntity.Destroy_Task*
- *VirtualApp.CloneVApp_Task*
- *VirtualApp.unregisterVApp_Task*
- *VirtualApp.ExportVApp*
- *VirtualApp.PowerOnVApp_Task*
- *VirtualApp.PowerOffVApp_Task*
- *VirtualApp.UpdateVAppConfig*
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("permission", NodeData { type_decl: "Vec<vim_rs::types::structs::Permission>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfPermission"), path_segment: "permission", doc: Some(r#####"List of permissions defined for this entity.
"#####) },),
        ("alarm_actions_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "alarmActionsEnabled", doc: Some(r#####"Whether alarm actions are enabled for this entity.

True if enabled; false otherwise.

***Required privileges:*** System.Read
"#####) },),
        ("recent_task", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "recentTask", doc: Some(r#####"The set of recent tasks operating on this managed entity.

This is a subset
of *TaskManager.recentTask* belong to this entity. A task in this
list could be in one of the four states: pending, running, success or error.

This property can be used to deduce intermediate power states for
a virtual machine entity. For example, if the current powerState is "poweredOn"
and there is a running task performing the "suspend" operation, then the virtual
machine's intermediate state might be described as "suspending."

Most tasks (such as power operations) obtain exclusive access to the virtual
machine, so it is unusual for this list to contain more than one running task.
One exception, however, is the task of cloning a virtual machine.
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("summary", NodeData { type_decl: "Box<dyn vim_rs::types::traits::ResourcePoolSummaryTrait>", type_name: "ResourcePoolSummary", is_optional: false, processing_type: FieldProcessingType::Trait, path_segment: "summary", doc: Some(r#####"Basic information about a resource pool.

In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("vm", NodeData { type_decl: "Vec<vim_rs::types::structs::ManagedObjectReference>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfManagedObjectReference"), path_segment: "vm", doc: Some(r#####"The set of virtual machines associated with this resource pool.

***Required privileges:*** System.View
"#####) },),
        ("namespace", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "namespace", doc: Some(r#####"The namespace with which the ResourcePool is associated.

Namespace is a
vAPI resource which divides cluster resources and allows administrators
to give Kubernetes environments to their development teams.
This property is set only at the time of creation and cannot change.

***Required privileges:*** System.View
"#####) },),
        ("overall_status", NodeData { type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ManagedEntityStatus"), path_segment: "overallStatus", doc: Some(r#####"General health of this managed entity.

The overall status of the managed entity is computed as the worst status
among its alarms and the configuration issues detected on the entity.
The status is reported as one of the following values:
- red: The entity has alarms or configuration issues with a red status.
- yellow: The entity does not have alarms or configuration issues with a
  red status, and has at least one with a yellow status.
- green: The entity does not have alarms or configuration issues with a
  red or yellow status, and has at least one with a green status.
- gray: All of the entity's alarms have a gray status and the
  configuration status of the entity is not being monitored.
  
In releases after vSphere API 5.0, vSphere Servers might not
generate property collector update notifications for this property.
To obtain the latest value of the property, you can use
PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
If you use the PropertyCollector.WaitForUpdatesEx method, specify
an empty string for the version parameter. Any other version value will not
produce any property values as no updates are generated.
"#####) },),
        ("parent", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "parent", doc: Some(r#####"Parent of this entity.

This value is null for the root object and for
*VirtualMachine* objects that are part of
a *VirtualApp*.

***Required privileges:*** System.View
"#####) },),
        ("tag", NodeData { type_decl: "Vec<vim_rs::types::structs::Tag>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfTag"), path_segment: "tag", doc: Some(r#####"The set of tags associated with this managed entity.

Experimental. Subject to change.

***Required privileges:*** System.View
"#####) },),
        ("value", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::CustomFieldValueTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfCustomFieldValue"), path_segment: "value", doc: Some(r#####"List of custom field values.

Each value uses a key to associate
an instance of a *CustomFieldStringValue* with
a custom field definition.

***Required privileges:*** System.View
"#####) },),
    ],
}),
        ("HostHardwareStatusInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (3, 0),
    ],
    entries: &[
        ("dpu_status_info", NodeData { type_decl: "Vec<vim_rs::types::structs::DpuStatusInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfDpuStatusInfo"), path_segment: "dpuStatusInfo", doc: Some(r#####"Status of one or more DPU elements

***Since:*** vSphere API Release 8.0.0.1
"#####) },),
        ("memory_status_info", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::HostHardwareElementInfoTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostHardwareElementInfo"), path_segment: "memoryStatusInfo", doc: Some(r#####"Status of the physical memory
"#####) },),
        ("cpu_status_info", NodeData { type_decl: "Vec<Box<dyn vim_rs::types::traits::HostHardwareElementInfoTrait>>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostHardwareElementInfo"), path_segment: "cpuStatusInfo", doc: Some(r#####"Status of the CPU packages
"#####) },),
        ("storage_status_info", NodeData { type_decl: "Vec<vim_rs::types::structs::HostStorageElementInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostStorageElementInfo"), path_segment: "storageStatusInfo", doc: Some(r#####"Status of the physical storage system
"#####) },),
    ],
}),
        ("StoragePodSummary", ::phf::Map {
    key: 8694567506910003252,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("free_space", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "freeSpace", doc: Some(r#####"Total free space on this storage pod, in bytes.

This value is the sum of the
free space on all datastores that are part of this storage pod, and is updated
periodically by the server.
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"The name of the storage pod.
"#####) },),
        ("capacity", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "capacity", doc: Some(r#####"Total capacity of this storage pod, in bytes.

This value is the sum of the
capacity of all datastores that are part of this storage pod, and is updated
periodically by the server.
"#####) },),
    ],
}),
        ("VsanHostConfigInfo", ::phf::Map {
    key: 14108922650502679131,
    disps: &[
        (2, 6),
        (0, 0),
    ],
    entries: &[
        ("storage_info", NodeData { type_decl: "vim_rs::types::structs::VsanHostConfigInfoStorageInfo", type_name: "VsanHostConfigInfoStorageInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "storageInfo", doc: Some(r#####"The VSAN storage configuration for this host.

VSAN storage configuration settings are independent of the
current value of *VsanHostConfigInfo.enabled*.
"#####) },),
        ("fault_domain_info", NodeData { type_decl: "vim_rs::types::structs::VsanHostFaultDomainInfo", type_name: "VsanHostFaultDomainInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "faultDomainInfo", doc: Some(r#####"The VSAN fault domain configuration for this host.

VSAN host fault domain settings are independent of the
current value of *VsanHostConfigInfo.enabled*.
"#####) },),
        ("vsan_esa_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vsanEsaEnabled", doc: Some(r#####"Whether the vSAN ESA is enabled on this host.

This can only be
enabled when vSAN is enabled on this host.

***Since:*** vSphere API Release 8.0.0.1
"#####) },),
        ("cluster_info", NodeData { type_decl: "vim_rs::types::structs::VsanHostConfigInfoClusterInfo", type_name: "VsanHostConfigInfoClusterInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "clusterInfo", doc: Some(r#####"The VSAN service cluster configuration for this host.
"#####) },),
        ("network_info", NodeData { type_decl: "vim_rs::types::structs::VsanHostConfigInfoNetworkInfo", type_name: "VsanHostConfigInfoNetworkInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "networkInfo", doc: Some(r#####"The VSAN network configuration for this host.

VSAN network configuration settings are independent of the
current value of *VsanHostConfigInfo.enabled*.
"#####) },),
        ("enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "enabled", doc: Some(r#####"Whether the VSAN service is currently enabled on this host.
"#####) },),
        ("host_system", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "hostSystem", doc: Some(r#####"The *HostSystem* for this host.

This argument is required when this configuration is specified as
an input to VC-level APIs. When this configuration is specified
to a host-level direct API, this argument may be omitted.

See also *ComputeResource.ReconfigureComputeResource_Task*, *HostVsanSystem.UpdateVsan_Task*.

Refers instance of *HostSystem*.
"#####) },),
    ],
}),
        ("HostReliableMemoryInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("memory_size", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "memorySize", doc: None },),
    ],
}),
        ("DatastoreSummary", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (3, 0),
        (0, 5),
    ],
    entries: &[
        ("capacity", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "capacity", doc: Some(r#####"Maximum capacity of this datastore, in bytes.

This value is updated
periodically by the server. It can be explicitly refreshed with the Refresh
operation. This property is guaranteed to be valid only if *DatastoreSummary.accessible*
is true.
"#####) },),
        ("free_space", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "freeSpace", doc: Some(r#####"Available space of this datastore, in bytes.

The server periodically
updates this value. It can be explicitly refreshed with the Refresh operation.
This property is guaranteed to be valid only if *DatastoreSummary.accessible* is true.
"#####) },),
        ("name", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "name", doc: Some(r#####"The name of the datastore.
"#####) },),
        ("r#type", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "type", doc: Some(r#####"Type of file system volume, such as VMFS or NFS.

See also *HostFileSystemVolume.type*.
"#####) },),
        ("accessible", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "accessible", doc: Some(r#####"The connectivity status of this datastore.

If this is set to false, meaning the
datastore is not accessible, this datastore's capacity and freespace properties
cannot be validated. Furthermore, if this property is set to false, some of the
properties in this summary and in *DatastoreInfo* should not be
used. Refer to the documentation for the property of your interest.
For datastores accessed from multiple hosts, vCenter Server reports
*DatastoreSummary.accessible* as an aggregated value of the
properties reported in *HostMountInfo*. For instance,
if a datastore is accessible through a subset of hosts, then the value of
*DatastoreSummary.accessible* will be reported as true by
vCenter Server. And the reason for a daastore being inaccessible from a host
will be reported in *HostMountInfo.inaccessibleReason*
"#####) },),
        ("datastore", NodeData { type_decl: "vim_rs::types::structs::ManagedObjectReference", type_name: "ManagedObjectReference", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "datastore", doc: Some(r#####"The reference to the managed object.

Refers instance of *Datastore*.
"#####) },),
        ("multiple_host_access", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "multipleHostAccess", doc: Some(r#####"More than one host in the datacenter has been configured with access to the
datastore.

This is only provided by VirtualCenter.
"#####) },),
        ("maintenance_mode", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "maintenanceMode", doc: Some(r#####"The current maintenance mode state of the datastore.

The set of
possible values is described in *DatastoreSummaryMaintenanceModeState_enum*.
"#####) },),
        ("uncommitted", NodeData { type_decl: "i64", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "uncommitted", doc: Some(r#####"Total additional storage space, in bytes, potentially used by all
virtual machines on this datastore.

The server periodically updates this
value.
It can be explicitly refreshed with the *Datastore.RefreshDatastoreStorageInfo* operation.
This property is valid only if *DatastoreSummary.accessible* is true.
"#####) },),
        ("url", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "url", doc: Some(r#####"The unique locator for the datastore.

This property is guaranteed to be valid
only if *DatastoreSummary.accessible* is true.
"#####) },),
    ],
}),
        ("HostDatastoreSystemCapabilities", ::phf::Map {
    key: 351906021642186605,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("nfs_mount_creation_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "nfsMountCreationSupported", doc: Some(r#####"Indicates whether mounting an NFS volume is supported
when a NAS datastore is created.

If this option is false,
then NAS datastores corresponding to NFS volumes can be created
only for already mounted NFS volumes.
"#####) },),
        ("vmfs_extent_expansion_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "vmfsExtentExpansionSupported", doc: Some(r#####"Indicates whether vmfs extent expansion is supported.
"#####) },),
        ("nfs_mount_creation_required", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "nfsMountCreationRequired", doc: Some(r#####"Indicates whether mounting the NFS volume is required to be done as part
of NAS datastore creation.

If this is set to true, then NAS datastores
cannot be created for currently mounted NFS volumes.
"#####) },),
        ("local_datastore_supported", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "localDatastoreSupported", doc: Some(r#####"Indicates whether local datastores are supported.
"#####) },),
    ],
}),
        ("DVSCapability", ::phf::Map {
    key: 4066803471364472071,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("dv_port_operation_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "dvPortOperationSupported", doc: Some(r#####"Indicates whether this switch allows vCenter users to modify
the switch configuration at the port level,
except for host member, policy, and scope operations.
"#####) },),
        ("compatible_host_component_product_info", NodeData { type_decl: "Vec<vim_rs::types::structs::DistributedVirtualSwitchHostProductSpec>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfDistributedVirtualSwitchHostProductSpec"), path_segment: "compatibleHostComponentProductInfo", doc: Some(r#####"List of host component product information that is compatible
with the current switch implementation.
"#####) },),
        ("features_supported", NodeData { type_decl: "Box<dyn vim_rs::types::traits::DvsFeatureCapabilityTrait>", type_name: "DVSFeatureCapability", is_optional: true, processing_type: FieldProcessingType::Trait, path_segment: "featuresSupported", doc: Some(r#####"Indicators for which version-specific distributed virtual switch
features are available on this switch.

This information is read-only, with the following exception.
For a third-party distributed switch implementation, you can
set the property
*DVSFeatureCapability*.*DVSFeatureCapability.vmDirectPathGen2Supported*
during switch creation or when you call the
*DistributedVirtualSwitch.UpdateDvsCapability* method.
"#####) },),
        ("dvs_operation_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "dvsOperationSupported", doc: Some(r#####"Indicates whether this switch allows vCenter users to modify
the switch configuration at the switch level,
except for host member, policy, and scope operations.
"#####) },),
        ("dv_port_group_operation_supported", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "dvPortGroupOperationSupported", doc: Some(r#####"Indicates whether this switch allows vCenter users to modify
the switch configuration at the portgroup level,
except for host member, policy, and scope operations.
"#####) },),
    ],
}),
        ("VAppConfigInfo", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (1, 0),
        (8, 6),
        (3, 4),
    ],
    entries: &[
        ("install_boot_stop_delay", NodeData { type_decl: "i32", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "installBootStopDelay", doc: Some(r#####"Specifies the delay in seconds to wait for the VM to power off after the initial
boot (used only if installBootRequired is true).

A value of 0 means wait forever.

Not relevant for vApps. This means that the value is always false when reading the
configuration and is ignored when setting the configuration.
"#####) },),
        ("ovf_environment_transport", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "ovfEnvironmentTransport", doc: Some(r#####"List the transports to use for properties.

Supported values are: iso and
com.vmware.guestInfo.
"#####) },),
        ("annotation", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "annotation", doc: Some(r#####"Description for the vApp.
"#####) },),
        ("property", NodeData { type_decl: "Vec<vim_rs::types::structs::VAppPropertyInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVAppPropertyInfo"), path_segment: "property", doc: Some(r#####"List of properties
"#####) },),
        ("instance_uuid", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "instanceUuid", doc: Some(r#####"vCenter-specific 128-bit UUID of a vApp, represented as a hexademical
string.

This identifier is used by vCenter to uniquely identify all
vApp instances.
"#####) },),
        ("ovf_section", NodeData { type_decl: "Vec<vim_rs::types::structs::VAppOvfSectionInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVAppOvfSectionInfo"), path_segment: "ovfSection", doc: Some(r#####"List of uninterpreted OVF meta-data sections.
"#####) },),
        ("product", NodeData { type_decl: "Vec<vim_rs::types::structs::VAppProductInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVAppProductInfo"), path_segment: "product", doc: Some(r#####"Information about the package content.
"#####) },),
        ("ip_assignment", NodeData { type_decl: "vim_rs::types::structs::VAppIpAssignmentInfo", type_name: "VAppIPAssignmentInfo", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "ipAssignment", doc: Some(r#####"IP assignment policy and DHCP support configuration.
"#####) },),
        ("eula", NodeData { type_decl: "Vec<String>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfString"), path_segment: "eula", doc: Some(r#####"End User Liceses Agreements.
"#####) },),
        ("entity_config", NodeData { type_decl: "Vec<vim_rs::types::structs::VAppEntityConfigInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfVAppEntityConfigInfo"), path_segment: "entityConfig", doc: Some(r#####"Configuration of sub-entities (virtual machine or vApp).
"#####) },),
        ("managed_by", NodeData { type_decl: "vim_rs::types::structs::ManagedByInfo", type_name: "ManagedByInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "managedBy", doc: Some(r#####"Specifies that this vApp is managed by a VC Extension.

See the
*managedBy* property in the
VAppConfigSpec for more details.
"#####) },),
        ("install_boot_required", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "installBootRequired", doc: Some(r#####"Specifies whether the VM needs an initial boot before the deployment is complete.

Not relevant for vApps. This means that the value is always false when reading the
configuration and is ignored when setting the configuration.

If a vApp requires an install boot (because one of its VMs does), this is visible
on the *VirtualAppSummary.installBootRequired* field of the vApp.
"#####) },),
    ],
}),
        ("HostLicensableResourceInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("resource", NodeData { type_decl: "Vec<vim_rs::types::structs::KeyAnyValue>", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ArrayOfKeyAnyValue"), path_segment: "resource", doc: Some(r#####"List of currently supported resources.

The type of every value is long. The key can be one of *HostLicensableResourceKey_enum*
or arbitrary string.

NOTE:
The values in this property may not be accurate for pre-5.0 hosts when returned by vCenter 5.0
"#####) },),
    ],
}),
        ("GuestInfoCustomizationInfo", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (3, 0),
    ],
    entries: &[
        ("error_msg", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "errorMsg", doc: Some(r#####"Description of the error if there is error for the customization
process inside the guest OS
"#####) },),
        ("start_time", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveDateTime"), path_segment: "startTime", doc: Some(r#####"The time when the customization process has started inside the
guest OS
"#####) },),
        ("customization_status", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "customizationStatus", doc: Some(r#####"The customization status for this VM

See also *GuestInfoCustomizationStatus_enum*for the list of supported values.
"#####) },),
        ("end_time", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveDateTime"), path_segment: "endTime", doc: Some(r#####"The time when the customization process has completed inside the
guest OS
"#####) },),
    ],
}),
        ("HostHardwareInfo", ::phf::Map {
    key: 7485420634051515786,
    disps: &[
        (1, 0),
        (4, 14),
        (4, 6),
        (0, 10),
    ],
    entries: &[
        ("memory_tiering_type", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "memoryTieringType", doc: Some(r#####"Type of memory tiering configured on this host.

See *HostMemoryTieringType_enum* for
supported values. This field will be unset for legacy hosts as well as
for hosts that don't support memory tiering.

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
        ("cpu_feature", NodeData { type_decl: "Vec<vim_rs::types::structs::HostCpuIdInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostCpuIdInfo"), path_segment: "cpuFeature", doc: Some(r#####"CPU feature set that is supported by the hardware.

This is the
intersection of the feature sets supported by the individual CPU
packages. This feature set is modified by the
*supportedCpuFeature*
array in the host capabilities to obtain the feature set supported by
the virtualization platform.
"#####) },),
        ("cpu_info", NodeData { type_decl: "vim_rs::types::structs::HostCpuInfo", type_name: "HostCpuInfo", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "cpuInfo", doc: Some(r#####"Overall CPU information.
"#####) },),
        ("numa_info", NodeData { type_decl: "vim_rs::types::structs::HostNumaInfo", type_name: "HostNumaInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "numaInfo", doc: Some(r#####"Information about the NUMA (non-uniform memory access).
"#####) },),
        ("sgx_info", NodeData { type_decl: "vim_rs::types::structs::HostSgxInfo", type_name: "HostSgxInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "sgxInfo", doc: Some(r#####"SGX configuration on this host.
"#####) },),
        ("memory_size", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "memorySize", doc: Some(r#####"Total amount of physical memory on the host in bytes.
"#####) },),
        ("sev_info", NodeData { type_decl: "vim_rs::types::structs::HostSevInfo", type_name: "HostSevInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "sevInfo", doc: Some(r#####"SEV configuration on this host.

***Since:*** vSphere API Release 7.0.1.0
"#####) },),
        ("dvx_classes", NodeData { type_decl: "Vec<vim_rs::types::structs::HostDvxClass>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostDvxClass"), path_segment: "dvxClasses", doc: Some(r#####"The list of Device Virtualization Extensions (DVX) classes
available on this host.

***Since:*** vSphere API Release 8.0.0.1
"#####) },),
        ("system_info", NodeData { type_decl: "vim_rs::types::structs::HostSystemInfo", type_name: "HostSystemInfo", is_optional: false, processing_type: FieldProcessingType::Struct, path_segment: "systemInfo", doc: Some(r#####"Information about the system as a whole.
"#####) },),
        ("persistent_memory_info", NodeData { type_decl: "vim_rs::types::structs::HostPersistentMemoryInfo", type_name: "HostPersistentMemoryInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "persistentMemoryInfo", doc: Some(r#####"Persistent memory configuration on this host.
"#####) },),
        ("smc_present", NodeData { type_decl: "bool", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "smcPresent", doc: Some(r#####"Presence of System Management Controller, indicates the host is
Apple hardware, and thus capable of running Mac OS guest as VM.
"#####) },),
        ("tdx_info", NodeData { type_decl: "vim_rs::types::structs::HostTdxInfo", type_name: "HostTdxInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "tdxInfo", doc: Some(r#####"TDX (Trust Domain Extensions) configuration on this host.

***Since:*** vSphere API Release 9.0.0.0
"#####) },),
        ("cpu_power_management_info", NodeData { type_decl: "vim_rs::types::structs::HostCpuPowerManagementInfo", type_name: "HostCpuPowerManagementInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "cpuPowerManagementInfo", doc: None },),
        ("pci_device", NodeData { type_decl: "Vec<vim_rs::types::structs::HostPciDevice>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostPciDevice"), path_segment: "pciDevice", doc: Some(r#####"The list of Peripheral Component Interconnect (PCI) devices
available on this host.
"#####) },),
        ("cpu_pkg", NodeData { type_decl: "Vec<vim_rs::types::structs::HostCpuPackage>", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("ArrayOfHostCpuPackage"), path_segment: "cpuPkg", doc: Some(r#####"Information about each of the physical CPU packages on the host.
"#####) },),
        ("reliable_memory_info", NodeData { type_decl: "vim_rs::types::structs::HostReliableMemoryInfo", type_name: "HostReliableMemoryInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "reliableMemoryInfo", doc: Some(r#####"Information about reliable memory.
"#####) },),
        ("bios_info", NodeData { type_decl: "vim_rs::types::structs::HostBiosInfo", type_name: "HostBIOSInfo", is_optional: true, processing_type: FieldProcessingType::Struct, path_segment: "biosInfo", doc: Some(r#####"Information about the system BIOS
"#####) },),
        ("memory_tier_info", NodeData { type_decl: "Vec<vim_rs::types::structs::HostMemoryTierInfo>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostMemoryTierInfo"), path_segment: "memoryTierInfo", doc: Some(r#####"Configuration of each memory tier on this host.

The array is populated in the
order of tiers (ie, tier 0 at array index 0, tier 1 at array index 1,
and so on).

***Since:*** vSphere API Release 7.0.3.0
"#####) },),
    ],
}),
        ("HostCpuInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("hz", NodeData { type_decl: "i64", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveLong"), path_segment: "hz", doc: Some(r#####"CPU speed per core.

This might be an averaged value if the speed
is not uniform across all cores. The total CPU speed of the box is
defined as hz \* numCpuCores
"#####) },),
        ("num_cpu_threads", NodeData { type_decl: "i16", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveShort"), path_segment: "numCpuThreads", doc: Some(r#####"Number of physical CPU threads on the host.
"#####) },),
        ("num_cpu_cores", NodeData { type_decl: "i16", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveShort"), path_segment: "numCpuCores", doc: Some(r#####"Number of physical CPU cores on the host.
"#####) },),
        ("num_cpu_packages", NodeData { type_decl: "i16", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveShort"), path_segment: "numCpuPackages", doc: Some(r#####"Number of physical CPU packages on the host.
"#####) },),
    ],
}),
        ("ManagedByInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("r#type", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "type", doc: Some(r#####"Managed entity type, as defined by the extension managing the entity.

An extension can manage different types of entities - different kinds
of virtual machines, vApps, etc. - and this property is used to find
the corresponding *managedEntityInfo*
entry from the extension.
"#####) },),
        ("extension_key", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "extensionKey", doc: Some(r#####"Key of the extension managing the entity.
"#####) },),
    ],
}),
        ("HostQualifiedName", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("value", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "value", doc: Some(r#####"The qualified name.
"#####) },),
        ("r#type", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "type", doc: Some(r#####"The type of the qualified name.

The list of supported values is specified in *HostQualifiedNameType_enum*.
"#####) },),
    ],
}),
        ("HostMultipathInfo", ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("lun", NodeData { type_decl: "Vec<vim_rs::types::structs::HostMultipathInfoLogicalUnit>", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("ArrayOfHostMultipathInfoLogicalUnit"), path_segment: "lun", doc: Some(r#####"List of logical units that can be configured for multipathing.
"#####) },),
    ],
}),
        ("StorageDrsSpaceLoadBalanceConfig", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("free_space_threshold_gb", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "freeSpaceThresholdGB", doc: Some(r#####"Storage DRS makes storage migration recommendations if
free space on one (or more) of the datastores falls below
the specified threshold.

The unit is in gigabytes and the minimum value is 1GB.
The maximum value is limited by the capacity of the smallest
datastore in a datastore cluster.
If not specified, the default value is 50GB.
"#####) },),
        ("space_threshold_mode", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "spaceThresholdMode", doc: None },),
        ("min_space_utilization_difference", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "minSpaceUtilizationDifference", doc: Some(r#####"Storage DRS considers making storage migration recommendations if
the difference in space utilization between the source and destination datastores
is higher than the specified threshold.

The valid values are in the range of 1 (i.e., 1%) to 50 (i.e., 50%).
If not specified, the default value is 5%.
"#####) },),
        ("space_utilization_threshold", NodeData { type_decl: "i32", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveInt"), path_segment: "spaceUtilizationThreshold", doc: Some(r#####"Storage DRS makes storage migration recommendations if
space utilization on one (or more) of the datastores is higher than
the specified threshold.

The valid values are in the range of 50 (i.e., 50%) to 100 (i.e., 100%).
If not specified, the default value is 80%.
"#####) },),
    ],
}),
        ("ComputeResourceConfigInfo", ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("default_hardware_version_key", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "defaultHardwareVersionKey", doc: Some(r#####"Key for Default Hardware Version used on this compute resource
in the format of *VirtualMachineConfigOptionDescriptor.key*.

This field affects
*VirtualMachineConfigOptionDescriptor.defaultConfigOption* returned
by *ComputeResource.environmentBrowser* of this object and all its children
with this field unset.
"#####) },),
        ("maximum_hardware_version_key", NodeData { type_decl: "String", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "maximumHardwareVersionKey", doc: Some(r#####"Key for Maximum Hardware Version used on this compute resource
in the format of *VirtualMachineConfigOptionDescriptor.key*.

This field affects
*VirtualMachineConfigOptionDescriptor.defaultConfigOption* returned
by *ComputeResource.environmentBrowser* of this object and all its children
with this field unset.

***Since:*** vSphere API Release 7.0.2.0
"#####) },),
        ("spbm_enabled", NodeData { type_decl: "bool", type_name: "", is_optional: true, processing_type: FieldProcessingType::Enum("PrimitiveBoolean"), path_segment: "spbmEnabled", doc: Some(r#####"Flag indicating whether or not the SPBM(Storage Policy Based Management)
feature is enabled on this compute resource
"#####) },),
        ("vm_swap_placement", NodeData { type_decl: "String", type_name: "", is_optional: false, processing_type: FieldProcessingType::Enum("PrimitiveString"), path_segment: "vmSwapPlacement", doc: Some(r#####"Swapfile placement policy for virtual machines within this compute
resource.

Any policy except for "inherit" is a valid value for this
property; the default is "vmDirectory". This setting will be honored
for each virtual machine within the compute resource for which the
following is true:
- The virtual machine is executing on a host that has the
  *perVmSwapFiles* capability.
- The virtual machine configuration's
  *swapPlacement* property is set
  to "inherit".
  
See also *VirtualMachineConfigInfoSwapPlacementType_enum*.
"#####) },),
    ],
}),
    ],
};
pub(crate) fn lookup_field_data(class: &str, field: &str) -> Result<&'static NodeData> {
    CLASS_FIELDS
        .get(class)
        .and_then(|fields| fields.get(field))
        .ok_or_else(|| {
            if CLASS_FIELDS.contains_key(class) {
                HierarchyError::UnknownField(class.to_string(), field.to_string())
            } else {
                HierarchyError::UnsupportedObjectType(class.to_string())
            }
        })
}
