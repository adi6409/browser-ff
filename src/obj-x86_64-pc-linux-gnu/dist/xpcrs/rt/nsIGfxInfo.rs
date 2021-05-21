//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIGfxInfo.idl
//


/// `interface nsIGfxInfo : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIGfxInfo {
    vtable: *const nsIGfxInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIGfxInfo.
unsafe impl XpCom for nsIGfxInfo {
    const IID: nsIID = nsID(0x1accd618, 0x4c80, 0x4703,
        [0x9d, 0x29, 0xec, 0xf2, 0x57, 0xd3, 0x97, 0xc8]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIGfxInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIGfxInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIGfxInfoCoerce {
    /// Cheaply cast a value of this type from a `nsIGfxInfo`.
    fn coerce_from(v: &nsIGfxInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIGfxInfoCoerce for nsIGfxInfo {
    #[inline]
    fn coerce_from(v: &nsIGfxInfo) -> &Self {
        v
    }
}

impl nsIGfxInfo {
    /// Cast this `nsIGfxInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIGfxInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIGfxInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsCoerce> nsIGfxInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGfxInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIGfxInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIGfxInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean D2DEnabled; */
    pub GetD2DEnabled: unsafe extern "system" fn (this: *const nsIGfxInfo, aD2DEnabled: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean DWriteEnabled; */
    pub GetDWriteEnabled: unsafe extern "system" fn (this: *const nsIGfxInfo, aDWriteEnabled: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean EmbeddedInFirefoxReality; */
    pub GetEmbeddedInFirefoxReality: unsafe extern "system" fn (this: *const nsIGfxInfo, aEmbeddedInFirefoxReality: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean usingGPUProcess; */
    pub GetUsingGPUProcess: unsafe extern "system" fn (this: *const nsIGfxInfo, aUsingGPUProcess: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean hasBattery; */
    pub GetHasBattery: unsafe extern "system" fn (this: *const nsIGfxInfo, aHasBattery: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute AString DWriteVersion; */
    pub GetDWriteVersion: unsafe extern "system" fn (this: *const nsIGfxInfo, aDWriteVersion: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString cleartypeParameters; */
    pub GetCleartypeParameters: unsafe extern "system" fn (this: *const nsIGfxInfo, aCleartypeParameters: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString windowProtocol; */
    pub GetWindowProtocol: unsafe extern "system" fn (this: *const nsIGfxInfo, aWindowProtocol: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString desktopEnvironment; */
    pub GetDesktopEnvironment: unsafe extern "system" fn (this: *const nsIGfxInfo, aDesktopEnvironment: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString testType; */
    pub GetTestType: unsafe extern "system" fn (this: *const nsIGfxInfo, aTestType: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString ContentBackend; */
    pub GetContentBackend: unsafe extern "system" fn (this: *const nsIGfxInfo, aContentBackend: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute boolean WebRenderEnabled; */
    pub GetWebRenderEnabled: unsafe extern "system" fn (this: *const nsIGfxInfo, aWebRenderEnabled: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean isHeadless; */
    pub GetIsHeadless: unsafe extern "system" fn (this: *const nsIGfxInfo, aIsHeadless: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean UsesTiling; */
    pub GetUsesTiling: unsafe extern "system" fn (this: *const nsIGfxInfo, aUsesTiling: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean ContentUsesTiling; */
    pub GetContentUsesTiling: unsafe extern "system" fn (this: *const nsIGfxInfo, aContentUsesTiling: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean OffMainThreadPaintEnabled; */
    pub GetOffMainThreadPaintEnabled: unsafe extern "system" fn (this: *const nsIGfxInfo, aOffMainThreadPaintEnabled: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute long OffMainThreadPaintWorkerCount; */
    pub GetOffMainThreadPaintWorkerCount: unsafe extern "system" fn (this: *const nsIGfxInfo, aOffMainThreadPaintWorkerCount: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long TargetFrameRate; */
    pub GetTargetFrameRate: unsafe extern "system" fn (this: *const nsIGfxInfo, aTargetFrameRate: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute AString adapterDescription; */
    pub GetAdapterDescription: unsafe extern "system" fn (this: *const nsIGfxInfo, aAdapterDescription: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString adapterDescription2; */
    pub GetAdapterDescription2: unsafe extern "system" fn (this: *const nsIGfxInfo, aAdapterDescription2: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString adapterDriver; */
    pub GetAdapterDriver: unsafe extern "system" fn (this: *const nsIGfxInfo, aAdapterDriver: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString adapterDriver2; */
    pub GetAdapterDriver2: unsafe extern "system" fn (this: *const nsIGfxInfo, aAdapterDriver2: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString adapterVendorID; */
    pub GetAdapterVendorID: unsafe extern "system" fn (this: *const nsIGfxInfo, aAdapterVendorID: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString adapterVendorID2; */
    pub GetAdapterVendorID2: unsafe extern "system" fn (this: *const nsIGfxInfo, aAdapterVendorID2: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString adapterDeviceID; */
    pub GetAdapterDeviceID: unsafe extern "system" fn (this: *const nsIGfxInfo, aAdapterDeviceID: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString adapterDeviceID2; */
    pub GetAdapterDeviceID2: unsafe extern "system" fn (this: *const nsIGfxInfo, aAdapterDeviceID2: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString adapterSubsysID; */
    pub GetAdapterSubsysID: unsafe extern "system" fn (this: *const nsIGfxInfo, aAdapterSubsysID: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString adapterSubsysID2; */
    pub GetAdapterSubsysID2: unsafe extern "system" fn (this: *const nsIGfxInfo, aAdapterSubsysID2: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute unsigned long adapterRAM; */
    pub GetAdapterRAM: unsafe extern "system" fn (this: *const nsIGfxInfo, aAdapterRAM: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long adapterRAM2; */
    pub GetAdapterRAM2: unsafe extern "system" fn (this: *const nsIGfxInfo, aAdapterRAM2: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute AString adapterDriverVendor; */
    pub GetAdapterDriverVendor: unsafe extern "system" fn (this: *const nsIGfxInfo, aAdapterDriverVendor: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString adapterDriverVendor2; */
    pub GetAdapterDriverVendor2: unsafe extern "system" fn (this: *const nsIGfxInfo, aAdapterDriverVendor2: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString adapterDriverVersion; */
    pub GetAdapterDriverVersion: unsafe extern "system" fn (this: *const nsIGfxInfo, aAdapterDriverVersion: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString adapterDriverVersion2; */
    pub GetAdapterDriverVersion2: unsafe extern "system" fn (this: *const nsIGfxInfo, aAdapterDriverVersion2: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString adapterDriverDate; */
    pub GetAdapterDriverDate: unsafe extern "system" fn (this: *const nsIGfxInfo, aAdapterDriverDate: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString adapterDriverDate2; */
    pub GetAdapterDriverDate2: unsafe extern "system" fn (this: *const nsIGfxInfo, aAdapterDriverDate2: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute boolean isGPU2Active; */
    pub GetIsGPU2Active: unsafe extern "system" fn (this: *const nsIGfxInfo, aIsGPU2Active: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute ACString drmRenderDevice; */
    pub GetDrmRenderDevice: unsafe extern "system" fn (this: *const nsIGfxInfo, aDrmRenderDevice: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute Array<AString> displayInfo; */
    pub GetDisplayInfo: unsafe extern "system" fn (this: *const nsIGfxInfo, aDisplayInfo: *mut thin_vec::ThinVec<::nsstring::nsString>) -> ::nserror::nsresult,

    /* readonly attribute Array<unsigned long> displayWidth; */
    pub GetDisplayWidth: unsafe extern "system" fn (this: *const nsIGfxInfo, aDisplayWidth: *mut thin_vec::ThinVec<u32>) -> ::nserror::nsresult,

    /* readonly attribute Array<unsigned long> displayHeight; */
    pub GetDisplayHeight: unsafe extern "system" fn (this: *const nsIGfxInfo, aDisplayHeight: *mut thin_vec::ThinVec<u32>) -> ::nserror::nsresult,

    /* [implicit_jscontext] jsval getMonitors (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetMonitors: *const ::libc::c_void,

    /* void refreshMonitors (); */
    pub RefreshMonitors: unsafe extern "system" fn (this: *const nsIGfxInfo) -> ::nserror::nsresult,

    /* Array<ACString> getFailures (out Array<long> indices); */
    pub GetFailures: unsafe extern "system" fn (this: *const nsIGfxInfo, indices: *mut thin_vec::ThinVec<i32>, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* [noscript,notxpcom] void logFailure (in ACString failure); */
    pub LogFailure: unsafe extern "system" fn (this: *const nsIGfxInfo, failure: *const ::nsstring::nsACString) -> libc::c_void,

    /* long getFeatureStatus (in long aFeature, [optional] out ACString aFailureId); */
    pub GetFeatureStatus: unsafe extern "system" fn (this: *const nsIGfxInfo, aFeature: i32, aFailureId: *mut ::nsstring::nsACString, _retval: *mut i32) -> ::nserror::nsresult,

    /* AString getFeatureSuggestedDriverVersion (in long aFeature); */
    pub GetFeatureSuggestedDriverVersion: unsafe extern "system" fn (this: *const nsIGfxInfo, aFeature: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [noscript,notxpcom] void GetData (); */
    pub GetData: unsafe extern "system" fn (this: *const nsIGfxInfo) -> libc::c_void,

    /* [noscript,notxpcom] long GetMaxRefreshRate (out boolean aMixed); */
    pub GetMaxRefreshRate: unsafe extern "system" fn (this: *const nsIGfxInfo, aMixed: *mut bool) -> i32,

    /* [implicit_jscontext] jsval getInfo (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetInfo: *const ::libc::c_void,

    /* [implicit_jscontext] jsval getFeatureLog (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetFeatureLog: *const ::libc::c_void,

    /* [implicit_jscontext] jsval getFeatures (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetFeatures: *const ::libc::c_void,

    /* [implicit_jscontext] jsval getActiveCrashGuards (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetActiveCrashGuards: *const ::libc::c_void,

    /* boolean controlGPUProcessForXPCShell (in boolean aEnable); */
    pub ControlGPUProcessForXPCShell: unsafe extern "system" fn (this: *const nsIGfxInfo, aEnable: bool, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIGfxInfo {

    pub const FEATURE_DIRECT2D: i64 = 1;


    pub const FEATURE_DIRECT3D_9_LAYERS: i64 = 2;


    pub const FEATURE_DIRECT3D_10_LAYERS: i64 = 3;


    pub const FEATURE_DIRECT3D_10_1_LAYERS: i64 = 4;


    pub const FEATURE_OPENGL_LAYERS: i64 = 5;


    pub const FEATURE_WEBGL_OPENGL: i64 = 6;


    pub const FEATURE_WEBGL_ANGLE: i64 = 7;


    pub const UNUSED_FEATURE_WEBGL_MSAA: i64 = 8;


    pub const FEATURE_STAGEFRIGHT: i64 = 9;


    pub const FEATURE_WEBRTC_HW_ACCELERATION_H264: i64 = 10;


    pub const FEATURE_DIRECT3D_11_LAYERS: i64 = 11;


    pub const FEATURE_HARDWARE_VIDEO_DECODING: i64 = 12;


    pub const FEATURE_DIRECT3D_11_ANGLE: i64 = 13;


    pub const FEATURE_WEBRTC_HW_ACCELERATION_ENCODE: i64 = 14;


    pub const FEATURE_WEBRTC_HW_ACCELERATION_DECODE: i64 = 15;


    pub const FEATURE_CANVAS2D_ACCELERATION: i64 = 16;


    pub const FEATURE_VP8_HW_DECODE: i64 = 17;


    pub const FEATURE_VP9_HW_DECODE: i64 = 18;


    pub const FEATURE_DX_INTEROP2: i64 = 19;


    pub const FEATURE_GPU_PROCESS: i64 = 20;


    pub const FEATURE_WEBGL2: i64 = 21;


    pub const FEATURE_D3D11_KEYED_MUTEX: i64 = 22;


    pub const FEATURE_WEBRENDER: i64 = 23;


    pub const FEATURE_DX_NV12: i64 = 24;


    pub const FEATURE_DX_P010: i64 = 25;


    pub const FEATURE_DX_P016: i64 = 26;


    pub const FEATURE_GL_SWIZZLE: i64 = 27;


    pub const FEATURE_WEBRENDER_COMPOSITOR: i64 = 28;


    pub const FEATURE_WEBRENDER_SCISSORED_CACHE_CLEARS: i64 = 29;


    pub const FEATURE_ALLOW_WEBGL_OUT_OF_PROCESS: i64 = 30;


    pub const FEATURE_THREADSAFE_GL: i64 = 31;


    pub const FEATURE_WEBRENDER_SOFTWARE: i64 = 32;


    pub const FEATURE_WEBRENDER_OPTIMIZED_SHADERS: i64 = 33;


    pub const FEATURE_MAX_VALUE: i64 = 33;


    pub const FEATURE_STATUS_OK: i64 = 1;


    pub const FEATURE_STATUS_UNKNOWN: i64 = 2;


    pub const FEATURE_BLOCKED_DRIVER_VERSION: i64 = 3;


    pub const FEATURE_BLOCKED_DEVICE: i64 = 4;


    pub const FEATURE_DISCOURAGED: i64 = 5;


    pub const FEATURE_BLOCKED_OS_VERSION: i64 = 6;


    pub const FEATURE_BLOCKED_MISMATCHED_VERSION: i64 = 7;


    pub const FEATURE_DENIED: i64 = 8;


    pub const FEATURE_ALLOW_ALWAYS: i64 = 9;


    pub const FEATURE_ALLOW_QUALIFIED: i64 = 10;


    /// `readonly attribute boolean D2DEnabled;`
    #[inline]
    pub unsafe fn GetD2DEnabled(&self, aD2DEnabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetD2DEnabled)(self, aD2DEnabled)
    }



    /// `readonly attribute boolean DWriteEnabled;`
    #[inline]
    pub unsafe fn GetDWriteEnabled(&self, aDWriteEnabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetDWriteEnabled)(self, aDWriteEnabled)
    }



    /// `readonly attribute boolean EmbeddedInFirefoxReality;`
    #[inline]
    pub unsafe fn GetEmbeddedInFirefoxReality(&self, aEmbeddedInFirefoxReality: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetEmbeddedInFirefoxReality)(self, aEmbeddedInFirefoxReality)
    }



    /// `readonly attribute boolean usingGPUProcess;`
    #[inline]
    pub unsafe fn GetUsingGPUProcess(&self, aUsingGPUProcess: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetUsingGPUProcess)(self, aUsingGPUProcess)
    }



    /// `readonly attribute boolean hasBattery;`
    #[inline]
    pub unsafe fn GetHasBattery(&self, aHasBattery: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetHasBattery)(self, aHasBattery)
    }



    /// `readonly attribute AString DWriteVersion;`
    #[inline]
    pub unsafe fn GetDWriteVersion(&self, aDWriteVersion: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDWriteVersion)(self, aDWriteVersion)
    }



    /// `readonly attribute AString cleartypeParameters;`
    #[inline]
    pub unsafe fn GetCleartypeParameters(&self, aCleartypeParameters: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetCleartypeParameters)(self, aCleartypeParameters)
    }



    /// `readonly attribute AString windowProtocol;`
    #[inline]
    pub unsafe fn GetWindowProtocol(&self, aWindowProtocol: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetWindowProtocol)(self, aWindowProtocol)
    }



    /// `readonly attribute AString desktopEnvironment;`
    #[inline]
    pub unsafe fn GetDesktopEnvironment(&self, aDesktopEnvironment: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDesktopEnvironment)(self, aDesktopEnvironment)
    }



    /// `readonly attribute AString testType;`
    #[inline]
    pub unsafe fn GetTestType(&self, aTestType: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetTestType)(self, aTestType)
    }



    /// `readonly attribute AString ContentBackend;`
    #[inline]
    pub unsafe fn GetContentBackend(&self, aContentBackend: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetContentBackend)(self, aContentBackend)
    }



    /// `readonly attribute boolean WebRenderEnabled;`
    #[inline]
    pub unsafe fn GetWebRenderEnabled(&self, aWebRenderEnabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetWebRenderEnabled)(self, aWebRenderEnabled)
    }



    /// `readonly attribute boolean isHeadless;`
    #[inline]
    pub unsafe fn GetIsHeadless(&self, aIsHeadless: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsHeadless)(self, aIsHeadless)
    }



    /// `readonly attribute boolean UsesTiling;`
    #[inline]
    pub unsafe fn GetUsesTiling(&self, aUsesTiling: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetUsesTiling)(self, aUsesTiling)
    }



    /// `readonly attribute boolean ContentUsesTiling;`
    #[inline]
    pub unsafe fn GetContentUsesTiling(&self, aContentUsesTiling: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetContentUsesTiling)(self, aContentUsesTiling)
    }



    /// `readonly attribute boolean OffMainThreadPaintEnabled;`
    #[inline]
    pub unsafe fn GetOffMainThreadPaintEnabled(&self, aOffMainThreadPaintEnabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetOffMainThreadPaintEnabled)(self, aOffMainThreadPaintEnabled)
    }



    /// `readonly attribute long OffMainThreadPaintWorkerCount;`
    #[inline]
    pub unsafe fn GetOffMainThreadPaintWorkerCount(&self, aOffMainThreadPaintWorkerCount: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetOffMainThreadPaintWorkerCount)(self, aOffMainThreadPaintWorkerCount)
    }



    /// `readonly attribute unsigned long TargetFrameRate;`
    #[inline]
    pub unsafe fn GetTargetFrameRate(&self, aTargetFrameRate: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetTargetFrameRate)(self, aTargetFrameRate)
    }


    /// ```text
    /// /**
    ///    * The name of the display adapter.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString adapterDescription;`
    #[inline]
    pub unsafe fn GetAdapterDescription(&self, aAdapterDescription: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetAdapterDescription)(self, aAdapterDescription)
    }



    /// `readonly attribute AString adapterDescription2;`
    #[inline]
    pub unsafe fn GetAdapterDescription2(&self, aAdapterDescription2: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetAdapterDescription2)(self, aAdapterDescription2)
    }



    /// `readonly attribute AString adapterDriver;`
    #[inline]
    pub unsafe fn GetAdapterDriver(&self, aAdapterDriver: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetAdapterDriver)(self, aAdapterDriver)
    }



    /// `readonly attribute AString adapterDriver2;`
    #[inline]
    pub unsafe fn GetAdapterDriver2(&self, aAdapterDriver2: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetAdapterDriver2)(self, aAdapterDriver2)
    }



    /// `readonly attribute AString adapterVendorID;`
    #[inline]
    pub unsafe fn GetAdapterVendorID(&self, aAdapterVendorID: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetAdapterVendorID)(self, aAdapterVendorID)
    }



    /// `readonly attribute AString adapterVendorID2;`
    #[inline]
    pub unsafe fn GetAdapterVendorID2(&self, aAdapterVendorID2: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetAdapterVendorID2)(self, aAdapterVendorID2)
    }



    /// `readonly attribute AString adapterDeviceID;`
    #[inline]
    pub unsafe fn GetAdapterDeviceID(&self, aAdapterDeviceID: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetAdapterDeviceID)(self, aAdapterDeviceID)
    }



    /// `readonly attribute AString adapterDeviceID2;`
    #[inline]
    pub unsafe fn GetAdapterDeviceID2(&self, aAdapterDeviceID2: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetAdapterDeviceID2)(self, aAdapterDeviceID2)
    }



    /// `readonly attribute AString adapterSubsysID;`
    #[inline]
    pub unsafe fn GetAdapterSubsysID(&self, aAdapterSubsysID: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetAdapterSubsysID)(self, aAdapterSubsysID)
    }



    /// `readonly attribute AString adapterSubsysID2;`
    #[inline]
    pub unsafe fn GetAdapterSubsysID2(&self, aAdapterSubsysID2: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetAdapterSubsysID2)(self, aAdapterSubsysID2)
    }


    /// ```text
    /// /**
    ///    * The amount of RAM in MB in the display adapter.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long adapterRAM;`
    #[inline]
    pub unsafe fn GetAdapterRAM(&self, aAdapterRAM: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetAdapterRAM)(self, aAdapterRAM)
    }



    /// `readonly attribute unsigned long adapterRAM2;`
    #[inline]
    pub unsafe fn GetAdapterRAM2(&self, aAdapterRAM2: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetAdapterRAM2)(self, aAdapterRAM2)
    }



    /// `readonly attribute AString adapterDriverVendor;`
    #[inline]
    pub unsafe fn GetAdapterDriverVendor(&self, aAdapterDriverVendor: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetAdapterDriverVendor)(self, aAdapterDriverVendor)
    }



    /// `readonly attribute AString adapterDriverVendor2;`
    #[inline]
    pub unsafe fn GetAdapterDriverVendor2(&self, aAdapterDriverVendor2: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetAdapterDriverVendor2)(self, aAdapterDriverVendor2)
    }



    /// `readonly attribute AString adapterDriverVersion;`
    #[inline]
    pub unsafe fn GetAdapterDriverVersion(&self, aAdapterDriverVersion: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetAdapterDriverVersion)(self, aAdapterDriverVersion)
    }



    /// `readonly attribute AString adapterDriverVersion2;`
    #[inline]
    pub unsafe fn GetAdapterDriverVersion2(&self, aAdapterDriverVersion2: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetAdapterDriverVersion2)(self, aAdapterDriverVersion2)
    }



    /// `readonly attribute AString adapterDriverDate;`
    #[inline]
    pub unsafe fn GetAdapterDriverDate(&self, aAdapterDriverDate: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetAdapterDriverDate)(self, aAdapterDriverDate)
    }



    /// `readonly attribute AString adapterDriverDate2;`
    #[inline]
    pub unsafe fn GetAdapterDriverDate2(&self, aAdapterDriverDate2: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetAdapterDriverDate2)(self, aAdapterDriverDate2)
    }



    /// `readonly attribute boolean isGPU2Active;`
    #[inline]
    pub unsafe fn GetIsGPU2Active(&self, aIsGPU2Active: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsGPU2Active)(self, aIsGPU2Active)
    }



    /// `readonly attribute ACString drmRenderDevice;`
    #[inline]
    pub unsafe fn GetDrmRenderDevice(&self, aDrmRenderDevice: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetDrmRenderDevice)(self, aDrmRenderDevice)
    }


    /// ```text
    /// /**
    ///    * Information about display devices
    ///    */
    /// ```
    ///

    /// `readonly attribute Array<AString> displayInfo;`
    #[inline]
    pub unsafe fn GetDisplayInfo(&self, aDisplayInfo: *mut thin_vec::ThinVec<::nsstring::nsString>) -> ::nserror::nsresult {
        ((*self.vtable).GetDisplayInfo)(self, aDisplayInfo)
    }



    /// `readonly attribute Array<unsigned long> displayWidth;`
    #[inline]
    pub unsafe fn GetDisplayWidth(&self, aDisplayWidth: *mut thin_vec::ThinVec<u32>) -> ::nserror::nsresult {
        ((*self.vtable).GetDisplayWidth)(self, aDisplayWidth)
    }



    /// `readonly attribute Array<unsigned long> displayHeight;`
    #[inline]
    pub unsafe fn GetDisplayHeight(&self, aDisplayHeight: *mut thin_vec::ThinVec<u32>) -> ::nserror::nsresult {
        ((*self.vtable).GetDisplayHeight)(self, aDisplayHeight)
    }


    /// ```text
    /// /**
    ///    * Returns an array of objects describing each monitor. Guaranteed properties
    ///    * are "screenWidth" and "screenHeight". This is only implemented on Desktop.
    ///    *
    ///    * Windows additionally supplies "refreshRate" and "pseudoDisplay".
    ///    *
    ///    * OS X additionally supplies "scale".
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] jsval getMonitors ();`
    const _GetMonitors: () = ();


    /// `void refreshMonitors ();`
    #[inline]
    pub unsafe fn RefreshMonitors(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).RefreshMonitors)(self, )
    }



    /// `Array<ACString> getFailures (out Array<long> indices);`
    #[inline]
    pub unsafe fn GetFailures(&self, indices: *mut thin_vec::ThinVec<i32>, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetFailures)(self, indices, _retval)
    }



    /// `[noscript,notxpcom] void logFailure (in ACString failure);`
    #[inline]
    pub unsafe fn LogFailure(&self, failure: *const ::nsstring::nsACString) -> libc::c_void {
        ((*self.vtable).LogFailure)(self, failure)
    }


    /// ```text
    /// /**
    ///    * Ask about a feature, and return the status of that feature.
    ///    * If the feature is not ok then aFailureId will give a unique failure Id
    ///    * otherwise it will be empty.
    ///    */
    /// ```
    ///

    /// `long getFeatureStatus (in long aFeature, [optional] out ACString aFailureId);`
    #[inline]
    pub unsafe fn GetFeatureStatus(&self, aFeature: i32, aFailureId: *mut ::nsstring::nsACString, _retval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetFeatureStatus)(self, aFeature, aFailureId, _retval)
    }



    /// `AString getFeatureSuggestedDriverVersion (in long aFeature);`
    #[inline]
    pub unsafe fn GetFeatureSuggestedDriverVersion(&self, aFeature: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetFeatureSuggestedDriverVersion)(self, aFeature, _retval)
    }



    /// `[noscript,notxpcom] void GetData ();`
    #[inline]
    pub unsafe fn GetData(&self, ) -> libc::c_void {
        ((*self.vtable).GetData)(self, )
    }


    /// ```text
    /// /**
    ///    * Maximum refresh rate among detected monitors. -1 if unknown. aMixed is set
    ///    * to true if we know there are multiple displays and they have different
    ///    * refresh rates, else false.
    ///    */
    /// ```
    ///

    /// `[noscript,notxpcom] long GetMaxRefreshRate (out boolean aMixed);`
    #[inline]
    pub unsafe fn GetMaxRefreshRate(&self, aMixed: *mut bool) -> i32 {
        ((*self.vtable).GetMaxRefreshRate)(self, aMixed)
    }



    /// `[implicit_jscontext] jsval getInfo ();`
    const _GetInfo: () = ();


    /// `[implicit_jscontext] jsval getFeatureLog ();`
    const _GetFeatureLog: () = ();


    /// `[implicit_jscontext] jsval getFeatures ();`
    const _GetFeatures: () = ();


    /// `[implicit_jscontext] jsval getActiveCrashGuards ();`
    const _GetActiveCrashGuards: () = ();


    /// `boolean controlGPUProcessForXPCShell (in boolean aEnable);`
    #[inline]
    pub unsafe fn ControlGPUProcessForXPCShell(&self, aEnable: bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ControlGPUProcessForXPCShell)(self, aEnable, _retval)
    }


}


