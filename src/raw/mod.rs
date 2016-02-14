//! 'Raw' functions, enums, and bitflags for the OpenCL C FFI.
//!
//! The thin layer between the FFI interfaces and the ocl types.
//!
//! Allows access to OpenCL FFI functions with a minimal layer of abstraction providing safety and convenience. Using functions in this module is only recommended for use when functionality has not yet been implemented on the 'standard' ocl interfaces although the 'raw' and 'standard' interfaces are all completely interoperable (and generally feature-equivalent).
//! 
//! Object pointers can generally be shared between threads except for kernel. 
//! See [clSetKernelArg documentation](https://www.khronos.org/registry/cl/sdk/1.2/docs/man/xhtml/clSetKernelArg.html)
//!
//! ## Even Lower Level: `cl_h`
//!
//! *Not as raw as...*
//!
//! If there's still something missing, or for some reason you need direct FFI access, use the functions in the `cl_h` module.
//!
//! # Performance
//!
//! Performance between all three levels of interface, `cl_h`, `raw`, and the standard types, is virtually identical (if not, file an issue).
//!
//! ## Safety
//!
//! At the time of writing, some functions still *may* break Rust's usual safety promises and have not been comprehensively tested or evaluated. Please file an [issue](https://github.com/cogciprocate/ocl/issues) if you discover something!
//!
//! ## Panics
//!
//! [NOT UP TO DATE: more and more functions are returning results] All functions will panic upon OpenCL error. This will be changing over time. Certain errors will eventually be returned as an `Error` type instead.
//!
//! ### Official Documentation
//!
//! [OpenCL 1.2 SDK Reference: https://www.khronos.org/registry/cl/sdk/1.2/docs/man/xhtml/](https://www.khronos.org/registry/cl/sdk/1.2/docs/man/xhtml/)
//!
//! ### Help Wanted
//!
//! Please help complete coverage of any FFI functions you may need by filing an [issue](https://github.com/cogciprocate/ocl/issues) or creating a [pull request](https://github.com/cogciprocate/ocl/pulls).

mod function;
mod cl;
mod custom;

use cl_h;
pub use self::function::{get_platform_ids, get_device_ids, create_context, create_build_program, create_kernel, set_kernel_arg, create_command_queue, create_buffer, create_image, enqueue_write_buffer, enqueue_read_buffer, enqueue_kernel, enqueue_copy_buffer, get_max_work_group_size, finish, wait_for_events, wait_for_event, get_event_status, set_event_callback, release_event, release_mem_object, platform_info, program_build_err, device_info, context_info, verify_context, release_context, release_command_queue, release_program, release_kernel};
pub use self::cl::abs::{PlatformIdRaw, DeviceIdRaw, ContextRaw, CommandQueueRaw, MemRaw, 
	ProgramRaw, KernelRaw, EventRaw, SamplerRaw};
pub use self::cl::image_st::{ImageFormat, ImageDescriptor};
pub use self::custom::enums::{KernelArg};

//=============================================================================
//================================ CONSTANTS ==================================
//=============================================================================

// pub const DEFAULT_DEVICE_TYPE: cl_h::cl_device_type = cl_h::CL_DEVICE_TYPE_DEFAULT;
pub const DEVICES_MAX: u32 = 16;
pub const DEFAULT_PLATFORM_IDX: usize = 0;
pub const DEFAULT_DEVICE_IDX: usize = 0;

//=============================================================================
//================================ BITFIELDS ==================================
//=============================================================================

/// cl_device_type - bitfield 
bitflags! {
    pub flags DeviceType: u64 {
		const DEVICE_TYPE_DEFAULT = 1 << 0,
		const DEVICE_TYPE_CPU = 1 << 1,
		const DEVICE_TYPE_GPU = 1 << 2,
		const DEVICE_TYPE_ACCELERATOR = 1 << 3,
		const DEVICE_TYPE_CUSTOM = 1 << 4,
		const DEVICE_TYPE_ALL = 0xFFFFFFFF,
    }
}

/// cl_device_fp_config - bitfield
bitflags! {
    pub flags DeviceFpConfig: u64 {
		const FP_DENORM = 1 << 0,
		const FP_INF_NAN = 1 << 1,
		const FP_ROUND_TO_NEAREST = 1 << 2,
		const FP_ROUND_TO_ZERO = 1 << 3,
		const FP_ROUND_TO_INF = 1 << 4,
		const FP_FMA = 1 << 5,
		const FP_SOFT_FLOAT = 1 << 6,
		const FP_CORRECTLY_ROUNDED_DIVIDE_SQRT = 1 << 7,
    }
}

/// cl_device_exec_capabilities - bitfield
bitflags! {
    pub flags DeviceExecCapabilities: u64 {
		const EXEC_KERNEL = 1 << 0,
		const EXEC_NATIVE_KERNEL = 1 << 1,
    }
}

/// cl_command_queue_properties - bitfield
bitflags! {
    pub flags CommandQueueProperties: u64 {
		const QUEUE_OUT_OF_ORDER_EXEC_MODE_ENABLE = 1 << 0,
		const QUEUE_PROFILING_ENABLE = 1 << 1,
    }
}

/// cl_device_affinity_domain
bitflags! {
    pub flags DeviceAffinityDomain: u64 {
		const DEVICE_AFFINITY_DOMAIN_NUMA = 1 << 0,
		const DEVICE_AFFINITY_DOMAIN_L4_CACHE = 1 << 1,
		const DEVICE_AFFINITY_DOMAIN_L3_CACHE = 1 << 2,
		const DEVICE_AFFINITY_DOMAIN_L2_CACHE = 1 << 3,
		const DEVICE_AFFINITY_DOMAIN_L1_CACHE = 1 << 4,
		const DEVICE_AFFINITY_DOMAIN_NEXT_PARTITIONABLE = 1 << 5,
    }
}

/// cl_mem_flags - bitfield
bitflags! {
    pub flags MemFlags: u64 {
		const MEM_READ_WRITE = 1 << 0,
		const MEM_WRITE_ONLY = 1 << 1,
		const MEM_READ_ONLY = 1 << 2,
		const MEM_USE_HOST_PTR = 1 << 3,
		const MEM_ALLOC_HOST_PTR = 1 << 4,
		const MEM_COPY_HOST_PTR = 1 << 5,
		// RESERVED            1<< 6,
		const MEM_HOST_WRITE_ONLY = 1 << 7,
		const MEM_HOST_READ_ONLY = 1 << 8,
		const MEM_HOST_NO_ACCESS = 1 << 9,
    }
}

/// cl_mem_migration_flags - bitfield
bitflags! {
    pub flags MemMigrationFlags: u64 {
		const MIGRATE_MEM_OBJECT_HOST = 1 << 0,
		const MIGRATE_MEM_OBJECT_CONTENT_UNDEFINED = 1 << 1,
    }
}

/// cl_map_flags - bitfield
bitflags! {
    pub flags MapFlags: u64 {
		const MAP_READ = 1 << 0,
		const MAP_WRITE = 1 << 1,
		const MAP_WRITE_INVALIDATE_REGION = 1 << 2,
    }
}

/// cl_program_binary_type
bitflags! {
    pub flags ProgramBinaryType: u64 {
		const PROGRAM_BINARY_TYPE_NONE = 0x0,
		const PROGRAM_BINARY_TYPE_COMPILED_OBJECT = 0x1,
		const PROGRAM_BINARY_TYPE_LIBRARY = 0x2,
		const PROGRAM_BINARY_TYPE_EXECUTABLE = 0x4,
    }
}

/// cl_kernel_arg_type_qualifer 
bitflags! {
    pub flags KernelArgTypeQualifier: u64 {
		const KERNEL_ARG_TYPE_NONE = 0,
		const KERNEL_ARG_TYPE_CONST = 1 << 0,
		const KERNEL_ARG_TYPE_RESTRICT = 1 << 1,
		const KERNEL_ARG_TYPE_VOLATILE = 1 << 2,
    }
}

//=============================================================================
//=============================== ENUMERATORS =================================
//=============================================================================


/// Specifies the number of channels and the channel layout i.e. the memory layout in which channels are stored in the image. Valid values are described in the table below. (from SDK)
#[derive(Clone, Copy)]
pub enum ImageChannelOrder {
    R = cl_h::CL_R as isize,
    A = cl_h::CL_A as isize,
    Rg = cl_h::CL_RG as isize,
    Ra = cl_h::CL_RA as isize,
    /// This format can only be used if channel data type = CL_UNORM_SHORT_565, CL_UNORM_SHORT_555 or CL_UNORM_INT101010:
    Rgb = cl_h::CL_RGB as isize,
    Rgba = cl_h::CL_RGBA as isize,
    /// This format can only be used if channel data type = CL_UNORM_INT8, CL_SNORM_INT8, CL_SIGNED_INT8 or CL_UNSIGNED_INT8:
    Bgra = cl_h::CL_BGRA as isize,
    /// This format can only be used if channel data type = CL_UNORM_INT8, CL_SNORM_INT8, CL_SIGNED_INT8 or CL_UNSIGNED_INT8:
    Argb = cl_h::CL_ARGB as isize,
    /// This format can only be used if channel data type = CL_UNORM_INT8, CL_UNORM_INT16, CL_SNORM_INT8, CL_SNORM_INT16, CL_HALF_FLOAT, or CL_FLOAT:
    Intensity = cl_h::CL_INTENSITY as isize,
    /// This format can only be used if channel data type = CL_UNORM_INT8, CL_UNORM_INT16, CL_SNORM_INT8, CL_SNORM_INT16, CL_HALF_FLOAT, or CL_FLOAT:
    Luminance = cl_h::CL_LUMINANCE as isize,
    Rx = cl_h::CL_Rx as isize,
    Rgx = cl_h::CL_RGx as isize,
    /// This format can only be used if channel data type = CL_UNORM_SHORT_565, CL_UNORM_SHORT_555 or CL_UNORM_INT101010:
    Rgbx = cl_h::CL_RGBx as isize,
    Depth = cl_h::CL_DEPTH as isize,
    DepthStencil = cl_h::CL_DEPTH_STENCIL as isize,
}

/// Describes the size of the channel data type. The number of bits per element determined by the image_channel_data_type and image_channel_order must be a power of two. The list of supported values is described in the table below. (from SDK)
#[derive(Clone, Copy)]
pub enum ImageChannelDataType {
    /// Each channel component is a normalized signed 8-bit integer value:
    SnormInt8 = cl_h::CL_SNORM_INT8 as isize,
    /// Each channel component is a normalized signed 16-bit integer value:
    SnormInt16 = cl_h::CL_SNORM_INT16 as isize,
    /// Each channel component is a normalized unsigned 8-bit integer value:
    UnormInt8 = cl_h::CL_UNORM_INT8 as isize,
    /// Each channel component is a normalized unsigned 16-bit integer value:
    UnormInt16 = cl_h::CL_UNORM_INT16 as isize,
    /// Represents a normalized 5-6-5 3-channel RGB image. The channel order must be CL_RGB or CL_RGBx:
    UnormShort565 = cl_h::CL_UNORM_SHORT_565 as isize,
    /// Represents a normalized x-5-5-5 4-channel xRGB image. The channel order must be CL_RGB or CL_RGBx:
    UnormShort555 = cl_h::CL_UNORM_SHORT_555 as isize,
    /// Represents a normalized x-10-10-10 4-channel xRGB image. The channel order must be CL_RGB or CL_RGBx:
    UnormInt101010 = cl_h::CL_UNORM_INT_101010 as isize,
    /// Each channel component is an unnormalized signed 8-bit integer value:
    SignedInt8 = cl_h::CL_SIGNED_INT8 as isize,
    /// Each channel component is an unnormalized signed 16-bit integer value:
    SignedInt16 = cl_h::CL_SIGNED_INT16 as isize,
    /// Each channel component is an unnormalized signed 32-bit integer value:
    SignedInt32 = cl_h::CL_SIGNED_INT32 as isize,
    /// Each channel component is an unnormalized unsigned 8-bit integer value:
    UnsignedInt8 = cl_h::CL_UNSIGNED_INT8 as isize,
    /// Each channel component is an unnormalized unsigned 16-bit integer value:
    UnsignedInt16 = cl_h::CL_UNSIGNED_INT16 as isize,
    /// Each channel component is an unnormalized unsigned 32-bit integer value:
    UnsignedInt32 = cl_h::CL_UNSIGNED_INT32 as isize,
    /// Each channel component is a 16-bit half-float value:
    HalfFloat = cl_h::CL_HALF_FLOAT as isize,
    /// Each channel component is a single precision floating-point value:
    Float = cl_h::CL_FLOAT as isize,
    /// Each channel component is a normalized unsigned 24-bit integer value:
    UnormInt24 = cl_h::CL_UNORM_INT24 as isize,
}


/// cl_bool
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Cbool {
        False = cl_h::CL_FALSE as isize,
        True = cl_h::CL_TRUE as isize,
    }
}

/// cl_bool: Polling
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Polling {
        Blocking = cl_h::CL_BLOCKING as isize,
        NonBlocking = cl_h::CL_NON_BLOCKING as isize,
    }
}

/// cl_platform_info 
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum PlatformInfo {
        Profile = cl_h::CL_PLATFORM_PROFILE as isize,
        Version = cl_h::CL_PLATFORM_VERSION as isize,
        Name = cl_h::CL_PLATFORM_NAME as isize,
        Vendor = cl_h::CL_PLATFORM_VENDOR as isize,
        Extensions = cl_h::CL_PLATFORM_EXTENSIONS as isize,
    }
}

/// cl_device_info 
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum DeviceInfo {
        Type = cl_h::CL_DEVICE_TYPE as isize,
        VendorId = cl_h::CL_DEVICE_VENDOR_ID as isize,
        MaxComputeUnits = cl_h::CL_DEVICE_MAX_COMPUTE_UNITS as isize,
        MaxWorkItemDimensions = cl_h::CL_DEVICE_MAX_WORK_ITEM_DIMENSIONS as isize,
        MaxWorkGroupSize = cl_h::CL_DEVICE_MAX_WORK_GROUP_SIZE as isize,
        MaxWorkItemSizes = cl_h::CL_DEVICE_MAX_WORK_ITEM_SIZES as isize,
        PreferredVectorWidthChar = cl_h::CL_DEVICE_PREFERRED_VECTOR_WIDTH_CHAR as isize,
        PreferredVectorWidthShort = cl_h::CL_DEVICE_PREFERRED_VECTOR_WIDTH_SHORT as isize,
        PreferredVectorWidthInt = cl_h::CL_DEVICE_PREFERRED_VECTOR_WIDTH_INT as isize,
        PreferredVectorWidthLong = cl_h::CL_DEVICE_PREFERRED_VECTOR_WIDTH_LONG as isize,
        PreferredVectorWidthFloat = cl_h::CL_DEVICE_PREFERRED_VECTOR_WIDTH_FLOAT as isize,
        PreferredVectorWidthDouble = cl_h::CL_DEVICE_PREFERRED_VECTOR_WIDTH_DOUBLE as isize,
        MaxClockFrequency = cl_h::CL_DEVICE_MAX_CLOCK_FREQUENCY as isize,
        AddressBits = cl_h::CL_DEVICE_ADDRESS_BITS as isize,
        MaxReadImageArgs = cl_h::CL_DEVICE_MAX_READ_IMAGE_ARGS as isize,
        MaxWriteImageArgs = cl_h::CL_DEVICE_MAX_WRITE_IMAGE_ARGS as isize,
        MaxMemAllocSize = cl_h::CL_DEVICE_MAX_MEM_ALLOC_SIZE as isize,
        Image2dMaxWidth = cl_h::CL_DEVICE_IMAGE2D_MAX_WIDTH as isize,
        Image2dMaxHeight = cl_h::CL_DEVICE_IMAGE2D_MAX_HEIGHT as isize,
        Image3dMaxWidth = cl_h::CL_DEVICE_IMAGE3D_MAX_WIDTH as isize,
        Image3dMaxHeight = cl_h::CL_DEVICE_IMAGE3D_MAX_HEIGHT as isize,
        Image3dMaxDepth = cl_h::CL_DEVICE_IMAGE3D_MAX_DEPTH as isize,
        ImageSupport = cl_h::CL_DEVICE_IMAGE_SUPPORT as isize,
        MaxParameterSize = cl_h::CL_DEVICE_MAX_PARAMETER_SIZE as isize,
        MaxSamplers = cl_h::CL_DEVICE_MAX_SAMPLERS as isize,
        MemBaseAddrAlign = cl_h::CL_DEVICE_MEM_BASE_ADDR_ALIGN as isize,
        MinDataTypeAlignSize = cl_h::CL_DEVICE_MIN_DATA_TYPE_ALIGN_SIZE as isize,
        SingleFpConfig = cl_h::CL_DEVICE_SINGLE_FP_CONFIG as isize,
        GlobalMemCacheType = cl_h::CL_DEVICE_GLOBAL_MEM_CACHE_TYPE as isize,
        GlobalMemCachelineSize = cl_h::CL_DEVICE_GLOBAL_MEM_CACHELINE_SIZE as isize,
        GlobalMemCacheSize = cl_h::CL_DEVICE_GLOBAL_MEM_CACHE_SIZE as isize,
        GlobalMemSize = cl_h::CL_DEVICE_GLOBAL_MEM_SIZE as isize,
        MaxConstantBufferSize = cl_h::CL_DEVICE_MAX_CONSTANT_BUFFER_SIZE as isize,
        MaxConstantArgs = cl_h::CL_DEVICE_MAX_CONSTANT_ARGS as isize,
        LocalMemType = cl_h::CL_DEVICE_LOCAL_MEM_TYPE as isize,
        LocalMemSize = cl_h::CL_DEVICE_LOCAL_MEM_SIZE as isize,
        ErrorCorrectionSupport = cl_h::CL_DEVICE_ERROR_CORRECTION_SUPPORT as isize,
        ProfilingTimerResolution = cl_h::CL_DEVICE_PROFILING_TIMER_RESOLUTION as isize,
        EndianLittle = cl_h::CL_DEVICE_ENDIAN_LITTLE as isize,
        Available = cl_h::CL_DEVICE_AVAILABLE as isize,
        CompilerAvailable = cl_h::CL_DEVICE_COMPILER_AVAILABLE as isize,
        ExecutionCapabilities = cl_h::CL_DEVICE_EXECUTION_CAPABILITIES as isize,
        QueueProperties = cl_h::CL_DEVICE_QUEUE_PROPERTIES as isize,
        Name = cl_h::CL_DEVICE_NAME as isize,
        Vendor = cl_h::CL_DEVICE_VENDOR as isize,
        DriverVersion = cl_h::CL_DRIVER_VERSION as isize,
        Profile = cl_h::CL_DEVICE_PROFILE as isize,
        Version = cl_h::CL_DEVICE_VERSION as isize,
        Extensions = cl_h::CL_DEVICE_EXTENSIONS as isize,
        Platform = cl_h::CL_DEVICE_PLATFORM as isize,
        DoubleFpConfig = cl_h::CL_DEVICE_DOUBLE_FP_CONFIG as isize,
        HalfFpConfig = cl_h::CL_DEVICE_HALF_FP_CONFIG as isize,
        PreferredVectorWidthHalf = cl_h::CL_DEVICE_PREFERRED_VECTOR_WIDTH_HALF as isize,
        HostUnifiedMemory = cl_h::CL_DEVICE_HOST_UNIFIED_MEMORY as isize,
        NativeVectorWidthChar = cl_h::CL_DEVICE_NATIVE_VECTOR_WIDTH_CHAR as isize,
        NativeVectorWidthShort = cl_h::CL_DEVICE_NATIVE_VECTOR_WIDTH_SHORT as isize,
        NativeVectorWidthInt = cl_h::CL_DEVICE_NATIVE_VECTOR_WIDTH_INT as isize,
        NativeVectorWidthLong = cl_h::CL_DEVICE_NATIVE_VECTOR_WIDTH_LONG as isize,
        NativeVectorWidthFloat = cl_h::CL_DEVICE_NATIVE_VECTOR_WIDTH_FLOAT as isize,
        NativeVectorWidthDouble = cl_h::CL_DEVICE_NATIVE_VECTOR_WIDTH_DOUBLE as isize,
        NativeVectorWidthHalf = cl_h::CL_DEVICE_NATIVE_VECTOR_WIDTH_HALF as isize,
        OpenclCVersion = cl_h::CL_DEVICE_OPENCL_C_VERSION as isize,
        LinkerAvailable = cl_h::CL_DEVICE_LINKER_AVAILABLE as isize,
        BuiltInKernels = cl_h::CL_DEVICE_BUILT_IN_KERNELS as isize,
        ImageMaxBufferSize = cl_h::CL_DEVICE_IMAGE_MAX_BUFFER_SIZE as isize,
        ImageMaxArraySize = cl_h::CL_DEVICE_IMAGE_MAX_ARRAY_SIZE as isize,
        ParentDevice = cl_h::CL_DEVICE_PARENT_DEVICE as isize,
        PartitionMaxSubDevices = cl_h::CL_DEVICE_PARTITION_MAX_SUB_DEVICES as isize,
        PartitionProperties = cl_h::CL_DEVICE_PARTITION_PROPERTIES as isize,
        PartitionAffinityDomain = cl_h::CL_DEVICE_PARTITION_AFFINITY_DOMAIN as isize,
        PartitionType = cl_h::CL_DEVICE_PARTITION_TYPE as isize,
        ReferenceCount = cl_h::CL_DEVICE_REFERENCE_COUNT as isize,
        PreferredInteropUserSync = cl_h::CL_DEVICE_PREFERRED_INTEROP_USER_SYNC as isize,
        PrintfBufferSize = cl_h::CL_DEVICE_PRINTF_BUFFER_SIZE as isize,
        ImagePitchAlignment = cl_h::CL_DEVICE_IMAGE_PITCH_ALIGNMENT as isize,
        ImageBaseAddressAlignment = cl_h::CL_DEVICE_IMAGE_BASE_ADDRESS_ALIGNMENT as isize,
    }
}

/// cl_mem_cache_type
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum DeviceMemCacheType {
        None = cl_h::CL_NONE as isize,
        ReadOnlyCache = cl_h::CL_READ_ONLY_CACHE as isize,
        ReadWriteCache = cl_h::CL_READ_WRITE_CACHE as isize,
    }
}

/// cl_device_local_mem_type
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum DeviceLocalMemType {
        Local = cl_h::CL_LOCAL as isize,
        Global = cl_h::CL_GLOBAL as isize,
    }
}

/// cl_context_info 
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum ContextInfo {
        ReferenceCount = cl_h::CL_CONTEXT_REFERENCE_COUNT as isize,
        Devices = cl_h::CL_CONTEXT_DEVICES as isize,
        Properties = cl_h::CL_CONTEXT_PROPERTIES as isize,
        NumDevices = cl_h::CL_CONTEXT_NUM_DEVICES as isize,
    }
}

/// cl_context_info + cl_context_properties
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum ContextInfoAndProperties {
        Platform = cl_h::CL_CONTEXT_PLATFORM as isize,
        InteropUserSync = cl_h::CL_CONTEXT_INTEROP_USER_SYNC as isize,
    }
}

/// cl_partition_property
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum PartitionProperty {
        PartitionEqually = cl_h::CL_DEVICE_PARTITION_EQUALLY as isize,
        PartitionByCounts = cl_h::CL_DEVICE_PARTITION_BY_COUNTS as isize,
        PartitionByCountsListEnd = cl_h::CL_DEVICE_PARTITION_BY_COUNTS_LIST_END as isize,
        PartitionByAffinityDomain = cl_h::CL_DEVICE_PARTITION_BY_AFFINITY_DOMAIN as isize,
    }
}

/// cl_command_queue_info 
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum CommandQueueInfo {
        Context = cl_h::CL_QUEUE_CONTEXT as isize,
        Device = cl_h::CL_QUEUE_DEVICE as isize,
        ReferenceCount = cl_h::CL_QUEUE_REFERENCE_COUNT as isize,
        Properties = cl_h::CL_QUEUE_PROPERTIES as isize,
    }
}

/// cl_channel_type
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum ChannelType {
        SnormInt8 = cl_h::CL_SNORM_INT8 as isize,
        SnormInt16 = cl_h::CL_SNORM_INT16 as isize,
        UnormInt8 = cl_h::CL_UNORM_INT8 as isize,
        UnormInt16 = cl_h::CL_UNORM_INT16 as isize,
        UnormShort_565 = cl_h::CL_UNORM_SHORT_565 as isize,
        UnormShort_555 = cl_h::CL_UNORM_SHORT_555 as isize,
        UnormInt_101010 = cl_h::CL_UNORM_INT_101010 as isize,
        SignedInt8 = cl_h::CL_SIGNED_INT8 as isize,
        SignedInt16 = cl_h::CL_SIGNED_INT16 as isize,
        SignedInt32 = cl_h::CL_SIGNED_INT32 as isize,
        UnsignedInt8 = cl_h::CL_UNSIGNED_INT8 as isize,
        UnsignedInt16 = cl_h::CL_UNSIGNED_INT16 as isize,
        UnsignedInt32 = cl_h::CL_UNSIGNED_INT32 as isize,
        HalfFloat = cl_h::CL_HALF_FLOAT as isize,
        Float = cl_h::CL_FLOAT as isize,
        UnormInt24 = cl_h::CL_UNORM_INT24 as isize,
    }
}

/// cl_mem_object_type
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum MemObjectType {
        Buffer = cl_h::CL_MEM_OBJECT_BUFFER as isize,
        Image2d = cl_h::CL_MEM_OBJECT_IMAGE2D as isize,
        Image3d = cl_h::CL_MEM_OBJECT_IMAGE3D as isize,
        Image2dArray = cl_h::CL_MEM_OBJECT_IMAGE2D_ARRAY as isize,
        Image1d = cl_h::CL_MEM_OBJECT_IMAGE1D as isize,
        Image1dArray = cl_h::CL_MEM_OBJECT_IMAGE1D_ARRAY as isize,
        Image1dBuffer = cl_h::CL_MEM_OBJECT_IMAGE1D_BUFFER as isize,
    }
}

/// cl_mem_info
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum MemInfo {
        Type = cl_h::CL_MEM_TYPE as isize,
        Flags = cl_h::CL_MEM_FLAGS as isize,
        Size = cl_h::CL_MEM_SIZE as isize,
        HostPtr = cl_h::CL_MEM_HOST_PTR as isize,
        MapCount = cl_h::CL_MEM_MAP_COUNT as isize,
        ReferenceCount = cl_h::CL_MEM_REFERENCE_COUNT as isize,
        Context = cl_h::CL_MEM_CONTEXT as isize,
        AssociatedMemobject = cl_h::CL_MEM_ASSOCIATED_MEMOBJECT as isize,
        Offset = cl_h::CL_MEM_OFFSET as isize,
    }
}

/// cl_image_info
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum ImageInfo {
        Format = cl_h::CL_IMAGE_FORMAT as isize,
        ElementSize = cl_h::CL_IMAGE_ELEMENT_SIZE as isize,
        RowPitch = cl_h::CL_IMAGE_ROW_PITCH as isize,
        SlicePitch = cl_h::CL_IMAGE_SLICE_PITCH as isize,
        Width = cl_h::CL_IMAGE_WIDTH as isize,
        Height = cl_h::CL_IMAGE_HEIGHT as isize,
        Depth = cl_h::CL_IMAGE_DEPTH as isize,
        ArraySize = cl_h::CL_IMAGE_ARRAY_SIZE as isize,
        Buffer = cl_h::CL_IMAGE_BUFFER as isize,
        NumMipLevels = cl_h::CL_IMAGE_NUM_MIP_LEVELS as isize,
        NumSamples = cl_h::CL_IMAGE_NUM_SAMPLES as isize,
    }
}

/// cl_addressing_mode
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum AddressingMode {
        None = cl_h::CL_ADDRESS_NONE as isize,
        ClampToEdge = cl_h::CL_ADDRESS_CLAMP_TO_EDGE as isize,
        Clamp = cl_h::CL_ADDRESS_CLAMP as isize,
        Repeat = cl_h::CL_ADDRESS_REPEAT as isize,
        MirroredRepeat = cl_h::CL_ADDRESS_MIRRORED_REPEAT as isize,
    }
}

/// cl_filter_mode
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum FilterMode {
        Nearest = cl_h::CL_FILTER_NEAREST as isize,
        Linear = cl_h::CL_FILTER_LINEAR as isize,
    }
}

/// cl_sampler_info
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum SamplerInfo {
        ReferenceCount = cl_h::CL_SAMPLER_REFERENCE_COUNT as isize,
        Context = cl_h::CL_SAMPLER_CONTEXT as isize,
        NormalizedCoords = cl_h::CL_SAMPLER_NORMALIZED_COORDS as isize,
        AddressingMode = cl_h::CL_SAMPLER_ADDRESSING_MODE as isize,
        FilterMode = cl_h::CL_SAMPLER_FILTER_MODE as isize,
    }
}

/// cl_program_info
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum ProgramInfo {
        ReferenceCount = cl_h::CL_PROGRAM_REFERENCE_COUNT as isize,
        Context = cl_h::CL_PROGRAM_CONTEXT as isize,
        NumDevices = cl_h::CL_PROGRAM_NUM_DEVICES as isize,
        Devices = cl_h::CL_PROGRAM_DEVICES as isize,
        Source = cl_h::CL_PROGRAM_SOURCE as isize,
        BinarySizes = cl_h::CL_PROGRAM_BINARY_SIZES as isize,
        Binaries = cl_h::CL_PROGRAM_BINARIES as isize,
        NumKernels = cl_h::CL_PROGRAM_NUM_KERNELS as isize,
        KernelNames = cl_h::CL_PROGRAM_KERNEL_NAMES as isize,
    }
}

/// cl_program_build_info
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum ProgramBuildInfo {
        BuildStatus = cl_h::CL_PROGRAM_BUILD_STATUS as isize,
        BuildOptions = cl_h::CL_PROGRAM_BUILD_OPTIONS as isize,
        BuildLog = cl_h::CL_PROGRAM_BUILD_LOG as isize,
        BinaryType = cl_h::CL_PROGRAM_BINARY_TYPE as isize,
    }
}


/// cl_build_status 
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum BuildStatus {
        Success = cl_h::CL_BUILD_SUCCESS as isize,
        None = cl_h::CL_BUILD_NONE as isize,
        Error = cl_h::CL_BUILD_ERROR as isize,
        InProgress = cl_h::CL_BUILD_IN_PROGRESS as isize,
    }
}

/// cl_kernel_info
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum KernelInfo {
        FunctionName = cl_h::CL_KERNEL_FUNCTION_NAME as isize,
        NumArgs = cl_h::CL_KERNEL_NUM_ARGS as isize,
        ReferenceCount = cl_h::CL_KERNEL_REFERENCE_COUNT as isize,
        Context = cl_h::CL_KERNEL_CONTEXT as isize,
        Program = cl_h::CL_KERNEL_PROGRAM as isize,
        Attributes = cl_h::CL_KERNEL_ATTRIBUTES as isize,
    }
}

/// cl_kernel_arg_info 
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum KernelArgInfo {
        AddressQualifier = cl_h::CL_KERNEL_ARG_ADDRESS_QUALIFIER as isize,
        AccessQualifier = cl_h::CL_KERNEL_ARG_ACCESS_QUALIFIER as isize,
        TypeName = cl_h::CL_KERNEL_ARG_TYPE_NAME as isize,
        TypeQualifier = cl_h::CL_KERNEL_ARG_TYPE_QUALIFIER as isize,
        Name = cl_h::CL_KERNEL_ARG_NAME as isize,
    }
}

/// cl_kernel_arg_address_qualifier 
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum KernelArgAddressQualifier {
        Global = cl_h::CL_KERNEL_ARG_ADDRESS_GLOBAL as isize,
        Local = cl_h::CL_KERNEL_ARG_ADDRESS_LOCAL as isize,
        Constant = cl_h::CL_KERNEL_ARG_ADDRESS_CONSTANT as isize,
        Private = cl_h::CL_KERNEL_ARG_ADDRESS_PRIVATE as isize,
    }
}

/// cl_kernel_arg_access_qualifier 
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum KernelArgAccessQualifier {
        ReadOnly = cl_h::CL_KERNEL_ARG_ACCESS_READ_ONLY as isize,
        WriteOnly = cl_h::CL_KERNEL_ARG_ACCESS_WRITE_ONLY as isize,
        ReadWrite = cl_h::CL_KERNEL_ARG_ACCESS_READ_WRITE as isize,
        None = cl_h::CL_KERNEL_ARG_ACCESS_NONE as isize,
     }
}

/// cl_kernel_work_group_info 
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum KernelWorkGroupinfo {
        WorkGroupSize = cl_h::CL_KERNEL_WORK_GROUP_SIZE as isize,
        CompileWorkGroupSize = cl_h::CL_KERNEL_COMPILE_WORK_GROUP_SIZE as isize,
        LocalMemSize = cl_h::CL_KERNEL_LOCAL_MEM_SIZE as isize,
        PreferredWorkGroupSizeMultiple = cl_h::CL_KERNEL_PREFERRED_WORK_GROUP_SIZE_MULTIPLE as isize,
        PrivateMemSize = cl_h::CL_KERNEL_PRIVATE_MEM_SIZE as isize,
        GlobalWorkSize = cl_h::CL_KERNEL_GLOBAL_WORK_SIZE as isize,
    }
}

/// cl_event_info 
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum EventInfo {
        CommandQueue = cl_h::CL_EVENT_COMMAND_QUEUE as isize,
        CommandType = cl_h::CL_EVENT_COMMAND_TYPE as isize,
        ReferenceCount = cl_h::CL_EVENT_REFERENCE_COUNT as isize,
        CommandExecutionStatus = cl_h::CL_EVENT_COMMAND_EXECUTION_STATUS as isize,
        Context = cl_h::CL_EVENT_CONTEXT as isize,
    }
}

/// cl_command_type
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum CommandType {
        NdrangeKernel = cl_h::CL_COMMAND_NDRANGE_KERNEL as isize,
        Task = cl_h::CL_COMMAND_TASK as isize,
        NativeKernel = cl_h::CL_COMMAND_NATIVE_KERNEL as isize,
        ReadBuffer = cl_h::CL_COMMAND_READ_BUFFER as isize,
        WriteBuffer = cl_h::CL_COMMAND_WRITE_BUFFER as isize,
        CopyBuffer = cl_h::CL_COMMAND_COPY_BUFFER as isize,
        ReadImage = cl_h::CL_COMMAND_READ_IMAGE as isize,
        WriteImage = cl_h::CL_COMMAND_WRITE_IMAGE as isize,
        CopyImage = cl_h::CL_COMMAND_COPY_IMAGE as isize,
        CopyImageToBuffer = cl_h::CL_COMMAND_COPY_IMAGE_TO_BUFFER as isize,
        CopyBufferToImage = cl_h::CL_COMMAND_COPY_BUFFER_TO_IMAGE as isize,
        MapBuffer = cl_h::CL_COMMAND_MAP_BUFFER as isize,
        MapImage = cl_h::CL_COMMAND_MAP_IMAGE as isize,
        UnmapMemObject = cl_h::CL_COMMAND_UNMAP_MEM_OBJECT as isize,
        Marker = cl_h::CL_COMMAND_MARKER as isize,
        AcquireGlObjects = cl_h::CL_COMMAND_ACQUIRE_GL_OBJECTS as isize,
        ReleaseGlObjects = cl_h::CL_COMMAND_RELEASE_GL_OBJECTS as isize,
        ReadBufferRect = cl_h::CL_COMMAND_READ_BUFFER_RECT as isize,
        WriteBufferRect = cl_h::CL_COMMAND_WRITE_BUFFER_RECT as isize,
        CopyBufferRect = cl_h::CL_COMMAND_COPY_BUFFER_RECT as isize,
        User = cl_h::CL_COMMAND_USER as isize,
        Barrier = cl_h::CL_COMMAND_BARRIER as isize,
        MigrateMemObjects = cl_h::CL_COMMAND_MIGRATE_MEM_OBJECTS as isize,
        FillBuffer = cl_h::CL_COMMAND_FILL_BUFFER as isize,
        FillImage = cl_h::CL_COMMAND_FILL_IMAGE as isize,
    }
}

/// command execution status
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum CommandExecutionStatus {
        Complete = cl_h::CL_COMPLETE as isize,
        Running = cl_h::CL_RUNNING as isize,
        Submitted = cl_h::CL_SUBMITTED as isize,
        Queued = cl_h::CL_QUEUED as isize,
    }
}

/// cl_buffer_create_type
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum BufferCreateType {
        Region = cl_h::CL_BUFFER_CREATE_TYPE_REGION as isize,
    }
}

/// cl_profiling_info 
enum_from_primitive! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum ProfilingInfo {
        Queued = cl_h::CL_PROFILING_COMMAND_QUEUED as isize,
        Submit = cl_h::CL_PROFILING_COMMAND_SUBMIT as isize,
        Start = cl_h::CL_PROFILING_COMMAND_START as isize,
        End = cl_h::CL_PROFILING_COMMAND_END as isize,
    }
}