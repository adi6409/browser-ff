/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIGfxInfo.idl
 */

#ifndef __gen_nsIGfxInfo_h__
#define __gen_nsIGfxInfo_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/Value.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIGfxInfo */
#define NS_IGFXINFO_IID_STR "1accd618-4c80-4703-9d29-ecf257d397c8"

#define NS_IGFXINFO_IID \
  {0x1accd618, 0x4c80, 0x4703, \
    { 0x9d, 0x29, 0xec, 0xf2, 0x57, 0xd3, 0x97, 0xc8 }}

class NS_NO_VTABLE nsIGfxInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IGFXINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIGfxInfo;

  /* readonly attribute boolean D2DEnabled; */
  NS_IMETHOD GetD2DEnabled(bool *aD2DEnabled) = 0;

  /* readonly attribute boolean DWriteEnabled; */
  NS_IMETHOD GetDWriteEnabled(bool *aDWriteEnabled) = 0;

  /* readonly attribute boolean EmbeddedInFirefoxReality; */
  NS_IMETHOD GetEmbeddedInFirefoxReality(bool *aEmbeddedInFirefoxReality) = 0;

  /* readonly attribute boolean usingGPUProcess; */
  NS_IMETHOD GetUsingGPUProcess(bool *aUsingGPUProcess) = 0;

  /* readonly attribute boolean hasBattery; */
  NS_IMETHOD GetHasBattery(bool *aHasBattery) = 0;

  /* readonly attribute AString DWriteVersion; */
  NS_IMETHOD GetDWriteVersion(nsAString& aDWriteVersion) = 0;

  /* readonly attribute AString cleartypeParameters; */
  NS_IMETHOD GetCleartypeParameters(nsAString& aCleartypeParameters) = 0;

  /* readonly attribute AString windowProtocol; */
  NS_IMETHOD GetWindowProtocol(nsAString& aWindowProtocol) = 0;

  /* readonly attribute AString desktopEnvironment; */
  NS_IMETHOD GetDesktopEnvironment(nsAString& aDesktopEnvironment) = 0;

  /* readonly attribute AString testType; */
  NS_IMETHOD GetTestType(nsAString& aTestType) = 0;

  /* readonly attribute AString ContentBackend; */
  NS_IMETHOD GetContentBackend(nsAString& aContentBackend) = 0;

  /* readonly attribute boolean WebRenderEnabled; */
  NS_IMETHOD GetWebRenderEnabled(bool *aWebRenderEnabled) = 0;

  /* readonly attribute boolean isHeadless; */
  NS_IMETHOD GetIsHeadless(bool *aIsHeadless) = 0;

  /* readonly attribute boolean UsesTiling; */
  NS_IMETHOD GetUsesTiling(bool *aUsesTiling) = 0;

  /* readonly attribute boolean ContentUsesTiling; */
  NS_IMETHOD GetContentUsesTiling(bool *aContentUsesTiling) = 0;

  /* readonly attribute boolean OffMainThreadPaintEnabled; */
  NS_IMETHOD GetOffMainThreadPaintEnabled(bool *aOffMainThreadPaintEnabled) = 0;

  /* readonly attribute long OffMainThreadPaintWorkerCount; */
  NS_IMETHOD GetOffMainThreadPaintWorkerCount(int32_t *aOffMainThreadPaintWorkerCount) = 0;

  /* readonly attribute unsigned long TargetFrameRate; */
  NS_IMETHOD GetTargetFrameRate(uint32_t *aTargetFrameRate) = 0;

  /* readonly attribute AString adapterDescription; */
  NS_IMETHOD GetAdapterDescription(nsAString& aAdapterDescription) = 0;

  /* readonly attribute AString adapterDescription2; */
  NS_IMETHOD GetAdapterDescription2(nsAString& aAdapterDescription2) = 0;

  /* readonly attribute AString adapterDriver; */
  NS_IMETHOD GetAdapterDriver(nsAString& aAdapterDriver) = 0;

  /* readonly attribute AString adapterDriver2; */
  NS_IMETHOD GetAdapterDriver2(nsAString& aAdapterDriver2) = 0;

  /* readonly attribute AString adapterVendorID; */
  NS_IMETHOD GetAdapterVendorID(nsAString& aAdapterVendorID) = 0;

  /* readonly attribute AString adapterVendorID2; */
  NS_IMETHOD GetAdapterVendorID2(nsAString& aAdapterVendorID2) = 0;

  /* readonly attribute AString adapterDeviceID; */
  NS_IMETHOD GetAdapterDeviceID(nsAString& aAdapterDeviceID) = 0;

  /* readonly attribute AString adapterDeviceID2; */
  NS_IMETHOD GetAdapterDeviceID2(nsAString& aAdapterDeviceID2) = 0;

  /* readonly attribute AString adapterSubsysID; */
  NS_IMETHOD GetAdapterSubsysID(nsAString& aAdapterSubsysID) = 0;

  /* readonly attribute AString adapterSubsysID2; */
  NS_IMETHOD GetAdapterSubsysID2(nsAString& aAdapterSubsysID2) = 0;

  /* readonly attribute unsigned long adapterRAM; */
  NS_IMETHOD GetAdapterRAM(uint32_t *aAdapterRAM) = 0;

  /* readonly attribute unsigned long adapterRAM2; */
  NS_IMETHOD GetAdapterRAM2(uint32_t *aAdapterRAM2) = 0;

  /* readonly attribute AString adapterDriverVendor; */
  NS_IMETHOD GetAdapterDriverVendor(nsAString& aAdapterDriverVendor) = 0;

  /* readonly attribute AString adapterDriverVendor2; */
  NS_IMETHOD GetAdapterDriverVendor2(nsAString& aAdapterDriverVendor2) = 0;

  /* readonly attribute AString adapterDriverVersion; */
  NS_IMETHOD GetAdapterDriverVersion(nsAString& aAdapterDriverVersion) = 0;

  /* readonly attribute AString adapterDriverVersion2; */
  NS_IMETHOD GetAdapterDriverVersion2(nsAString& aAdapterDriverVersion2) = 0;

  /* readonly attribute AString adapterDriverDate; */
  NS_IMETHOD GetAdapterDriverDate(nsAString& aAdapterDriverDate) = 0;

  /* readonly attribute AString adapterDriverDate2; */
  NS_IMETHOD GetAdapterDriverDate2(nsAString& aAdapterDriverDate2) = 0;

  /* readonly attribute boolean isGPU2Active; */
  NS_IMETHOD GetIsGPU2Active(bool *aIsGPU2Active) = 0;

  /* readonly attribute ACString drmRenderDevice; */
  NS_IMETHOD GetDrmRenderDevice(nsACString& aDrmRenderDevice) = 0;

  /* readonly attribute Array<AString> displayInfo; */
  NS_IMETHOD GetDisplayInfo(nsTArray<nsString >& aDisplayInfo) = 0;

  /* readonly attribute Array<unsigned long> displayWidth; */
  NS_IMETHOD GetDisplayWidth(nsTArray<uint32_t >& aDisplayWidth) = 0;

  /* readonly attribute Array<unsigned long> displayHeight; */
  NS_IMETHOD GetDisplayHeight(nsTArray<uint32_t >& aDisplayHeight) = 0;

  /* [implicit_jscontext] jsval getMonitors (); */
  NS_IMETHOD GetMonitors(JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* void refreshMonitors (); */
  NS_IMETHOD RefreshMonitors(void) = 0;

  /* Array<ACString> getFailures (out Array<long> indices); */
  NS_IMETHOD GetFailures(nsTArray<int32_t >& indices, nsTArray<nsCString >& _retval) = 0;

  /* [noscript,notxpcom] void logFailure (in ACString failure); */
  NS_IMETHOD_(void) LogFailure(const nsACString& failure) = 0;

  enum {
    FEATURE_DIRECT2D = 1,
    FEATURE_DIRECT3D_9_LAYERS = 2,
    FEATURE_DIRECT3D_10_LAYERS = 3,
    FEATURE_DIRECT3D_10_1_LAYERS = 4,
    FEATURE_OPENGL_LAYERS = 5,
    FEATURE_WEBGL_OPENGL = 6,
    FEATURE_WEBGL_ANGLE = 7,
    UNUSED_FEATURE_WEBGL_MSAA = 8,
    FEATURE_STAGEFRIGHT = 9,
    FEATURE_WEBRTC_HW_ACCELERATION_H264 = 10,
    FEATURE_DIRECT3D_11_LAYERS = 11,
    FEATURE_HARDWARE_VIDEO_DECODING = 12,
    FEATURE_DIRECT3D_11_ANGLE = 13,
    FEATURE_WEBRTC_HW_ACCELERATION_ENCODE = 14,
    FEATURE_WEBRTC_HW_ACCELERATION_DECODE = 15,
    FEATURE_CANVAS2D_ACCELERATION = 16,
    FEATURE_VP8_HW_DECODE = 17,
    FEATURE_VP9_HW_DECODE = 18,
    FEATURE_DX_INTEROP2 = 19,
    FEATURE_GPU_PROCESS = 20,
    FEATURE_WEBGL2 = 21,
    FEATURE_D3D11_KEYED_MUTEX = 22,
    FEATURE_WEBRENDER = 23,
    FEATURE_DX_NV12 = 24,
    FEATURE_DX_P010 = 25,
    FEATURE_DX_P016 = 26,
    FEATURE_GL_SWIZZLE = 27,
    FEATURE_WEBRENDER_COMPOSITOR = 28,
    FEATURE_WEBRENDER_SCISSORED_CACHE_CLEARS = 29,
    FEATURE_ALLOW_WEBGL_OUT_OF_PROCESS = 30,
    FEATURE_THREADSAFE_GL = 31,
    FEATURE_WEBRENDER_SOFTWARE = 32,
    FEATURE_WEBRENDER_OPTIMIZED_SHADERS = 33,
    FEATURE_MAX_VALUE = 33,
    FEATURE_STATUS_OK = 1,
    FEATURE_STATUS_UNKNOWN = 2,
    FEATURE_BLOCKED_DRIVER_VERSION = 3,
    FEATURE_BLOCKED_DEVICE = 4,
    FEATURE_DISCOURAGED = 5,
    FEATURE_BLOCKED_OS_VERSION = 6,
    FEATURE_BLOCKED_MISMATCHED_VERSION = 7,
    FEATURE_DENIED = 8,
    FEATURE_ALLOW_ALWAYS = 9,
    FEATURE_ALLOW_QUALIFIED = 10
  };

  /* long getFeatureStatus (in long aFeature, [optional] out ACString aFailureId); */
  NS_IMETHOD GetFeatureStatus(int32_t aFeature, nsACString& aFailureId, int32_t *_retval) = 0;

  /* AString getFeatureSuggestedDriverVersion (in long aFeature); */
  NS_IMETHOD GetFeatureSuggestedDriverVersion(int32_t aFeature, nsAString& _retval) = 0;

  /* [noscript,notxpcom] void GetData (); */
  NS_IMETHOD_(void) GetData(void) = 0;

  /* [noscript,notxpcom] long GetMaxRefreshRate (out boolean aMixed); */
  NS_IMETHOD_(int32_t) GetMaxRefreshRate(bool *aMixed) = 0;

  /* [implicit_jscontext] jsval getInfo (); */
  NS_IMETHOD GetInfo(JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] jsval getFeatureLog (); */
  NS_IMETHOD GetFeatureLog(JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] jsval getFeatures (); */
  NS_IMETHOD GetFeatures(JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] jsval getActiveCrashGuards (); */
  NS_IMETHOD GetActiveCrashGuards(JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* boolean controlGPUProcessForXPCShell (in boolean aEnable); */
  NS_IMETHOD ControlGPUProcessForXPCShell(bool aEnable, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIGfxInfo, NS_IGFXINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIGFXINFO \
  NS_IMETHOD GetD2DEnabled(bool *aD2DEnabled) override; \
  NS_IMETHOD GetDWriteEnabled(bool *aDWriteEnabled) override; \
  NS_IMETHOD GetEmbeddedInFirefoxReality(bool *aEmbeddedInFirefoxReality) override; \
  NS_IMETHOD GetUsingGPUProcess(bool *aUsingGPUProcess) override; \
  NS_IMETHOD GetHasBattery(bool *aHasBattery) override; \
  NS_IMETHOD GetDWriteVersion(nsAString& aDWriteVersion) override; \
  NS_IMETHOD GetCleartypeParameters(nsAString& aCleartypeParameters) override; \
  NS_IMETHOD GetWindowProtocol(nsAString& aWindowProtocol) override; \
  NS_IMETHOD GetDesktopEnvironment(nsAString& aDesktopEnvironment) override; \
  NS_IMETHOD GetTestType(nsAString& aTestType) override; \
  NS_IMETHOD GetContentBackend(nsAString& aContentBackend) override; \
  NS_IMETHOD GetWebRenderEnabled(bool *aWebRenderEnabled) override; \
  NS_IMETHOD GetIsHeadless(bool *aIsHeadless) override; \
  NS_IMETHOD GetUsesTiling(bool *aUsesTiling) override; \
  NS_IMETHOD GetContentUsesTiling(bool *aContentUsesTiling) override; \
  NS_IMETHOD GetOffMainThreadPaintEnabled(bool *aOffMainThreadPaintEnabled) override; \
  NS_IMETHOD GetOffMainThreadPaintWorkerCount(int32_t *aOffMainThreadPaintWorkerCount) override; \
  NS_IMETHOD GetTargetFrameRate(uint32_t *aTargetFrameRate) override; \
  NS_IMETHOD GetAdapterDescription(nsAString& aAdapterDescription) override; \
  NS_IMETHOD GetAdapterDescription2(nsAString& aAdapterDescription2) override; \
  NS_IMETHOD GetAdapterDriver(nsAString& aAdapterDriver) override; \
  NS_IMETHOD GetAdapterDriver2(nsAString& aAdapterDriver2) override; \
  NS_IMETHOD GetAdapterVendorID(nsAString& aAdapterVendorID) override; \
  NS_IMETHOD GetAdapterVendorID2(nsAString& aAdapterVendorID2) override; \
  NS_IMETHOD GetAdapterDeviceID(nsAString& aAdapterDeviceID) override; \
  NS_IMETHOD GetAdapterDeviceID2(nsAString& aAdapterDeviceID2) override; \
  NS_IMETHOD GetAdapterSubsysID(nsAString& aAdapterSubsysID) override; \
  NS_IMETHOD GetAdapterSubsysID2(nsAString& aAdapterSubsysID2) override; \
  NS_IMETHOD GetAdapterRAM(uint32_t *aAdapterRAM) override; \
  NS_IMETHOD GetAdapterRAM2(uint32_t *aAdapterRAM2) override; \
  NS_IMETHOD GetAdapterDriverVendor(nsAString& aAdapterDriverVendor) override; \
  NS_IMETHOD GetAdapterDriverVendor2(nsAString& aAdapterDriverVendor2) override; \
  NS_IMETHOD GetAdapterDriverVersion(nsAString& aAdapterDriverVersion) override; \
  NS_IMETHOD GetAdapterDriverVersion2(nsAString& aAdapterDriverVersion2) override; \
  NS_IMETHOD GetAdapterDriverDate(nsAString& aAdapterDriverDate) override; \
  NS_IMETHOD GetAdapterDriverDate2(nsAString& aAdapterDriverDate2) override; \
  NS_IMETHOD GetIsGPU2Active(bool *aIsGPU2Active) override; \
  NS_IMETHOD GetDrmRenderDevice(nsACString& aDrmRenderDevice) override; \
  NS_IMETHOD GetDisplayInfo(nsTArray<nsString >& aDisplayInfo) override; \
  NS_IMETHOD GetDisplayWidth(nsTArray<uint32_t >& aDisplayWidth) override; \
  NS_IMETHOD GetDisplayHeight(nsTArray<uint32_t >& aDisplayHeight) override; \
  NS_IMETHOD GetMonitors(JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD RefreshMonitors(void) override; \
  NS_IMETHOD GetFailures(nsTArray<int32_t >& indices, nsTArray<nsCString >& _retval) override; \
  NS_IMETHOD_(void) LogFailure(const nsACString& failure) override; \
  NS_IMETHOD GetFeatureStatus(int32_t aFeature, nsACString& aFailureId, int32_t *_retval) override; \
  NS_IMETHOD GetFeatureSuggestedDriverVersion(int32_t aFeature, nsAString& _retval) override; \
  NS_IMETHOD_(void) GetData(void) override; \
  NS_IMETHOD_(int32_t) GetMaxRefreshRate(bool *aMixed) override; \
  NS_IMETHOD GetInfo(JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetFeatureLog(JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetFeatures(JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetActiveCrashGuards(JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD ControlGPUProcessForXPCShell(bool aEnable, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIGFXINFO \
  nsresult GetD2DEnabled(bool *aD2DEnabled); \
  nsresult GetDWriteEnabled(bool *aDWriteEnabled); \
  nsresult GetEmbeddedInFirefoxReality(bool *aEmbeddedInFirefoxReality); \
  nsresult GetUsingGPUProcess(bool *aUsingGPUProcess); \
  nsresult GetHasBattery(bool *aHasBattery); \
  nsresult GetDWriteVersion(nsAString& aDWriteVersion); \
  nsresult GetCleartypeParameters(nsAString& aCleartypeParameters); \
  nsresult GetWindowProtocol(nsAString& aWindowProtocol); \
  nsresult GetDesktopEnvironment(nsAString& aDesktopEnvironment); \
  nsresult GetTestType(nsAString& aTestType); \
  nsresult GetContentBackend(nsAString& aContentBackend); \
  nsresult GetWebRenderEnabled(bool *aWebRenderEnabled); \
  nsresult GetIsHeadless(bool *aIsHeadless); \
  nsresult GetUsesTiling(bool *aUsesTiling); \
  nsresult GetContentUsesTiling(bool *aContentUsesTiling); \
  nsresult GetOffMainThreadPaintEnabled(bool *aOffMainThreadPaintEnabled); \
  nsresult GetOffMainThreadPaintWorkerCount(int32_t *aOffMainThreadPaintWorkerCount); \
  nsresult GetTargetFrameRate(uint32_t *aTargetFrameRate); \
  nsresult GetAdapterDescription(nsAString& aAdapterDescription); \
  nsresult GetAdapterDescription2(nsAString& aAdapterDescription2); \
  nsresult GetAdapterDriver(nsAString& aAdapterDriver); \
  nsresult GetAdapterDriver2(nsAString& aAdapterDriver2); \
  nsresult GetAdapterVendorID(nsAString& aAdapterVendorID); \
  nsresult GetAdapterVendorID2(nsAString& aAdapterVendorID2); \
  nsresult GetAdapterDeviceID(nsAString& aAdapterDeviceID); \
  nsresult GetAdapterDeviceID2(nsAString& aAdapterDeviceID2); \
  nsresult GetAdapterSubsysID(nsAString& aAdapterSubsysID); \
  nsresult GetAdapterSubsysID2(nsAString& aAdapterSubsysID2); \
  nsresult GetAdapterRAM(uint32_t *aAdapterRAM); \
  nsresult GetAdapterRAM2(uint32_t *aAdapterRAM2); \
  nsresult GetAdapterDriverVendor(nsAString& aAdapterDriverVendor); \
  nsresult GetAdapterDriverVendor2(nsAString& aAdapterDriverVendor2); \
  nsresult GetAdapterDriverVersion(nsAString& aAdapterDriverVersion); \
  nsresult GetAdapterDriverVersion2(nsAString& aAdapterDriverVersion2); \
  nsresult GetAdapterDriverDate(nsAString& aAdapterDriverDate); \
  nsresult GetAdapterDriverDate2(nsAString& aAdapterDriverDate2); \
  nsresult GetIsGPU2Active(bool *aIsGPU2Active); \
  nsresult GetDrmRenderDevice(nsACString& aDrmRenderDevice); \
  nsresult GetDisplayInfo(nsTArray<nsString >& aDisplayInfo); \
  nsresult GetDisplayWidth(nsTArray<uint32_t >& aDisplayWidth); \
  nsresult GetDisplayHeight(nsTArray<uint32_t >& aDisplayHeight); \
  nsresult GetMonitors(JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult RefreshMonitors(void); \
  nsresult GetFailures(nsTArray<int32_t >& indices, nsTArray<nsCString >& _retval); \
  nsresult_(void) LogFailure(const nsACString& failure); \
  nsresult GetFeatureStatus(int32_t aFeature, nsACString& aFailureId, int32_t *_retval); \
  nsresult GetFeatureSuggestedDriverVersion(int32_t aFeature, nsAString& _retval); \
  nsresult_(void) GetData(void); \
  nsresult_(int32_t) GetMaxRefreshRate(bool *aMixed); \
  nsresult GetInfo(JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult GetFeatureLog(JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult GetFeatures(JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult GetActiveCrashGuards(JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult ControlGPUProcessForXPCShell(bool aEnable, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIGFXINFO(_to) \
  NS_IMETHOD GetD2DEnabled(bool *aD2DEnabled) override { return _to GetD2DEnabled(aD2DEnabled); } \
  NS_IMETHOD GetDWriteEnabled(bool *aDWriteEnabled) override { return _to GetDWriteEnabled(aDWriteEnabled); } \
  NS_IMETHOD GetEmbeddedInFirefoxReality(bool *aEmbeddedInFirefoxReality) override { return _to GetEmbeddedInFirefoxReality(aEmbeddedInFirefoxReality); } \
  NS_IMETHOD GetUsingGPUProcess(bool *aUsingGPUProcess) override { return _to GetUsingGPUProcess(aUsingGPUProcess); } \
  NS_IMETHOD GetHasBattery(bool *aHasBattery) override { return _to GetHasBattery(aHasBattery); } \
  NS_IMETHOD GetDWriteVersion(nsAString& aDWriteVersion) override { return _to GetDWriteVersion(aDWriteVersion); } \
  NS_IMETHOD GetCleartypeParameters(nsAString& aCleartypeParameters) override { return _to GetCleartypeParameters(aCleartypeParameters); } \
  NS_IMETHOD GetWindowProtocol(nsAString& aWindowProtocol) override { return _to GetWindowProtocol(aWindowProtocol); } \
  NS_IMETHOD GetDesktopEnvironment(nsAString& aDesktopEnvironment) override { return _to GetDesktopEnvironment(aDesktopEnvironment); } \
  NS_IMETHOD GetTestType(nsAString& aTestType) override { return _to GetTestType(aTestType); } \
  NS_IMETHOD GetContentBackend(nsAString& aContentBackend) override { return _to GetContentBackend(aContentBackend); } \
  NS_IMETHOD GetWebRenderEnabled(bool *aWebRenderEnabled) override { return _to GetWebRenderEnabled(aWebRenderEnabled); } \
  NS_IMETHOD GetIsHeadless(bool *aIsHeadless) override { return _to GetIsHeadless(aIsHeadless); } \
  NS_IMETHOD GetUsesTiling(bool *aUsesTiling) override { return _to GetUsesTiling(aUsesTiling); } \
  NS_IMETHOD GetContentUsesTiling(bool *aContentUsesTiling) override { return _to GetContentUsesTiling(aContentUsesTiling); } \
  NS_IMETHOD GetOffMainThreadPaintEnabled(bool *aOffMainThreadPaintEnabled) override { return _to GetOffMainThreadPaintEnabled(aOffMainThreadPaintEnabled); } \
  NS_IMETHOD GetOffMainThreadPaintWorkerCount(int32_t *aOffMainThreadPaintWorkerCount) override { return _to GetOffMainThreadPaintWorkerCount(aOffMainThreadPaintWorkerCount); } \
  NS_IMETHOD GetTargetFrameRate(uint32_t *aTargetFrameRate) override { return _to GetTargetFrameRate(aTargetFrameRate); } \
  NS_IMETHOD GetAdapterDescription(nsAString& aAdapterDescription) override { return _to GetAdapterDescription(aAdapterDescription); } \
  NS_IMETHOD GetAdapterDescription2(nsAString& aAdapterDescription2) override { return _to GetAdapterDescription2(aAdapterDescription2); } \
  NS_IMETHOD GetAdapterDriver(nsAString& aAdapterDriver) override { return _to GetAdapterDriver(aAdapterDriver); } \
  NS_IMETHOD GetAdapterDriver2(nsAString& aAdapterDriver2) override { return _to GetAdapterDriver2(aAdapterDriver2); } \
  NS_IMETHOD GetAdapterVendorID(nsAString& aAdapterVendorID) override { return _to GetAdapterVendorID(aAdapterVendorID); } \
  NS_IMETHOD GetAdapterVendorID2(nsAString& aAdapterVendorID2) override { return _to GetAdapterVendorID2(aAdapterVendorID2); } \
  NS_IMETHOD GetAdapterDeviceID(nsAString& aAdapterDeviceID) override { return _to GetAdapterDeviceID(aAdapterDeviceID); } \
  NS_IMETHOD GetAdapterDeviceID2(nsAString& aAdapterDeviceID2) override { return _to GetAdapterDeviceID2(aAdapterDeviceID2); } \
  NS_IMETHOD GetAdapterSubsysID(nsAString& aAdapterSubsysID) override { return _to GetAdapterSubsysID(aAdapterSubsysID); } \
  NS_IMETHOD GetAdapterSubsysID2(nsAString& aAdapterSubsysID2) override { return _to GetAdapterSubsysID2(aAdapterSubsysID2); } \
  NS_IMETHOD GetAdapterRAM(uint32_t *aAdapterRAM) override { return _to GetAdapterRAM(aAdapterRAM); } \
  NS_IMETHOD GetAdapterRAM2(uint32_t *aAdapterRAM2) override { return _to GetAdapterRAM2(aAdapterRAM2); } \
  NS_IMETHOD GetAdapterDriverVendor(nsAString& aAdapterDriverVendor) override { return _to GetAdapterDriverVendor(aAdapterDriverVendor); } \
  NS_IMETHOD GetAdapterDriverVendor2(nsAString& aAdapterDriverVendor2) override { return _to GetAdapterDriverVendor2(aAdapterDriverVendor2); } \
  NS_IMETHOD GetAdapterDriverVersion(nsAString& aAdapterDriverVersion) override { return _to GetAdapterDriverVersion(aAdapterDriverVersion); } \
  NS_IMETHOD GetAdapterDriverVersion2(nsAString& aAdapterDriverVersion2) override { return _to GetAdapterDriverVersion2(aAdapterDriverVersion2); } \
  NS_IMETHOD GetAdapterDriverDate(nsAString& aAdapterDriverDate) override { return _to GetAdapterDriverDate(aAdapterDriverDate); } \
  NS_IMETHOD GetAdapterDriverDate2(nsAString& aAdapterDriverDate2) override { return _to GetAdapterDriverDate2(aAdapterDriverDate2); } \
  NS_IMETHOD GetIsGPU2Active(bool *aIsGPU2Active) override { return _to GetIsGPU2Active(aIsGPU2Active); } \
  NS_IMETHOD GetDrmRenderDevice(nsACString& aDrmRenderDevice) override { return _to GetDrmRenderDevice(aDrmRenderDevice); } \
  NS_IMETHOD GetDisplayInfo(nsTArray<nsString >& aDisplayInfo) override { return _to GetDisplayInfo(aDisplayInfo); } \
  NS_IMETHOD GetDisplayWidth(nsTArray<uint32_t >& aDisplayWidth) override { return _to GetDisplayWidth(aDisplayWidth); } \
  NS_IMETHOD GetDisplayHeight(nsTArray<uint32_t >& aDisplayHeight) override { return _to GetDisplayHeight(aDisplayHeight); } \
  NS_IMETHOD GetMonitors(JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetMonitors(cx, _retval); } \
  NS_IMETHOD RefreshMonitors(void) override { return _to RefreshMonitors(); } \
  NS_IMETHOD GetFailures(nsTArray<int32_t >& indices, nsTArray<nsCString >& _retval) override { return _to GetFailures(indices, _retval); } \
  NS_IMETHOD_(void) LogFailure(const nsACString& failure) override { return _to LogFailure(failure); } \
  NS_IMETHOD GetFeatureStatus(int32_t aFeature, nsACString& aFailureId, int32_t *_retval) override { return _to GetFeatureStatus(aFeature, aFailureId, _retval); } \
  NS_IMETHOD GetFeatureSuggestedDriverVersion(int32_t aFeature, nsAString& _retval) override { return _to GetFeatureSuggestedDriverVersion(aFeature, _retval); } \
  NS_IMETHOD_(void) GetData(void) override { return _to GetData(); } \
  NS_IMETHOD_(int32_t) GetMaxRefreshRate(bool *aMixed) override { return _to GetMaxRefreshRate(aMixed); } \
  NS_IMETHOD GetInfo(JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetInfo(cx, _retval); } \
  NS_IMETHOD GetFeatureLog(JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetFeatureLog(cx, _retval); } \
  NS_IMETHOD GetFeatures(JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetFeatures(cx, _retval); } \
  NS_IMETHOD GetActiveCrashGuards(JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetActiveCrashGuards(cx, _retval); } \
  NS_IMETHOD ControlGPUProcessForXPCShell(bool aEnable, bool *_retval) override { return _to ControlGPUProcessForXPCShell(aEnable, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIGFXINFO(_to) \
  NS_IMETHOD GetD2DEnabled(bool *aD2DEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetD2DEnabled(aD2DEnabled); } \
  NS_IMETHOD GetDWriteEnabled(bool *aDWriteEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDWriteEnabled(aDWriteEnabled); } \
  NS_IMETHOD GetEmbeddedInFirefoxReality(bool *aEmbeddedInFirefoxReality) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEmbeddedInFirefoxReality(aEmbeddedInFirefoxReality); } \
  NS_IMETHOD GetUsingGPUProcess(bool *aUsingGPUProcess) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUsingGPUProcess(aUsingGPUProcess); } \
  NS_IMETHOD GetHasBattery(bool *aHasBattery) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHasBattery(aHasBattery); } \
  NS_IMETHOD GetDWriteVersion(nsAString& aDWriteVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDWriteVersion(aDWriteVersion); } \
  NS_IMETHOD GetCleartypeParameters(nsAString& aCleartypeParameters) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCleartypeParameters(aCleartypeParameters); } \
  NS_IMETHOD GetWindowProtocol(nsAString& aWindowProtocol) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWindowProtocol(aWindowProtocol); } \
  NS_IMETHOD GetDesktopEnvironment(nsAString& aDesktopEnvironment) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDesktopEnvironment(aDesktopEnvironment); } \
  NS_IMETHOD GetTestType(nsAString& aTestType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTestType(aTestType); } \
  NS_IMETHOD GetContentBackend(nsAString& aContentBackend) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentBackend(aContentBackend); } \
  NS_IMETHOD GetWebRenderEnabled(bool *aWebRenderEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWebRenderEnabled(aWebRenderEnabled); } \
  NS_IMETHOD GetIsHeadless(bool *aIsHeadless) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsHeadless(aIsHeadless); } \
  NS_IMETHOD GetUsesTiling(bool *aUsesTiling) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUsesTiling(aUsesTiling); } \
  NS_IMETHOD GetContentUsesTiling(bool *aContentUsesTiling) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentUsesTiling(aContentUsesTiling); } \
  NS_IMETHOD GetOffMainThreadPaintEnabled(bool *aOffMainThreadPaintEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOffMainThreadPaintEnabled(aOffMainThreadPaintEnabled); } \
  NS_IMETHOD GetOffMainThreadPaintWorkerCount(int32_t *aOffMainThreadPaintWorkerCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOffMainThreadPaintWorkerCount(aOffMainThreadPaintWorkerCount); } \
  NS_IMETHOD GetTargetFrameRate(uint32_t *aTargetFrameRate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTargetFrameRate(aTargetFrameRate); } \
  NS_IMETHOD GetAdapterDescription(nsAString& aAdapterDescription) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAdapterDescription(aAdapterDescription); } \
  NS_IMETHOD GetAdapterDescription2(nsAString& aAdapterDescription2) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAdapterDescription2(aAdapterDescription2); } \
  NS_IMETHOD GetAdapterDriver(nsAString& aAdapterDriver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAdapterDriver(aAdapterDriver); } \
  NS_IMETHOD GetAdapterDriver2(nsAString& aAdapterDriver2) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAdapterDriver2(aAdapterDriver2); } \
  NS_IMETHOD GetAdapterVendorID(nsAString& aAdapterVendorID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAdapterVendorID(aAdapterVendorID); } \
  NS_IMETHOD GetAdapterVendorID2(nsAString& aAdapterVendorID2) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAdapterVendorID2(aAdapterVendorID2); } \
  NS_IMETHOD GetAdapterDeviceID(nsAString& aAdapterDeviceID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAdapterDeviceID(aAdapterDeviceID); } \
  NS_IMETHOD GetAdapterDeviceID2(nsAString& aAdapterDeviceID2) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAdapterDeviceID2(aAdapterDeviceID2); } \
  NS_IMETHOD GetAdapterSubsysID(nsAString& aAdapterSubsysID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAdapterSubsysID(aAdapterSubsysID); } \
  NS_IMETHOD GetAdapterSubsysID2(nsAString& aAdapterSubsysID2) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAdapterSubsysID2(aAdapterSubsysID2); } \
  NS_IMETHOD GetAdapterRAM(uint32_t *aAdapterRAM) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAdapterRAM(aAdapterRAM); } \
  NS_IMETHOD GetAdapterRAM2(uint32_t *aAdapterRAM2) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAdapterRAM2(aAdapterRAM2); } \
  NS_IMETHOD GetAdapterDriverVendor(nsAString& aAdapterDriverVendor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAdapterDriverVendor(aAdapterDriverVendor); } \
  NS_IMETHOD GetAdapterDriverVendor2(nsAString& aAdapterDriverVendor2) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAdapterDriverVendor2(aAdapterDriverVendor2); } \
  NS_IMETHOD GetAdapterDriverVersion(nsAString& aAdapterDriverVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAdapterDriverVersion(aAdapterDriverVersion); } \
  NS_IMETHOD GetAdapterDriverVersion2(nsAString& aAdapterDriverVersion2) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAdapterDriverVersion2(aAdapterDriverVersion2); } \
  NS_IMETHOD GetAdapterDriverDate(nsAString& aAdapterDriverDate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAdapterDriverDate(aAdapterDriverDate); } \
  NS_IMETHOD GetAdapterDriverDate2(nsAString& aAdapterDriverDate2) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAdapterDriverDate2(aAdapterDriverDate2); } \
  NS_IMETHOD GetIsGPU2Active(bool *aIsGPU2Active) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsGPU2Active(aIsGPU2Active); } \
  NS_IMETHOD GetDrmRenderDevice(nsACString& aDrmRenderDevice) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDrmRenderDevice(aDrmRenderDevice); } \
  NS_IMETHOD GetDisplayInfo(nsTArray<nsString >& aDisplayInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDisplayInfo(aDisplayInfo); } \
  NS_IMETHOD GetDisplayWidth(nsTArray<uint32_t >& aDisplayWidth) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDisplayWidth(aDisplayWidth); } \
  NS_IMETHOD GetDisplayHeight(nsTArray<uint32_t >& aDisplayHeight) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDisplayHeight(aDisplayHeight); } \
  NS_IMETHOD GetMonitors(JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMonitors(cx, _retval); } \
  NS_IMETHOD RefreshMonitors(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RefreshMonitors(); } \
  NS_IMETHOD GetFailures(nsTArray<int32_t >& indices, nsTArray<nsCString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFailures(indices, _retval); } \
  NS_IMETHOD_(void) LogFailure(const nsACString& failure) override; \
  NS_IMETHOD GetFeatureStatus(int32_t aFeature, nsACString& aFailureId, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFeatureStatus(aFeature, aFailureId, _retval); } \
  NS_IMETHOD GetFeatureSuggestedDriverVersion(int32_t aFeature, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFeatureSuggestedDriverVersion(aFeature, _retval); } \
  NS_IMETHOD_(void) GetData(void) override; \
  NS_IMETHOD_(int32_t) GetMaxRefreshRate(bool *aMixed) override; \
  NS_IMETHOD GetInfo(JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInfo(cx, _retval); } \
  NS_IMETHOD GetFeatureLog(JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFeatureLog(cx, _retval); } \
  NS_IMETHOD GetFeatures(JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFeatures(cx, _retval); } \
  NS_IMETHOD GetActiveCrashGuards(JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetActiveCrashGuards(cx, _retval); } \
  NS_IMETHOD ControlGPUProcessForXPCShell(bool aEnable, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ControlGPUProcessForXPCShell(aEnable, _retval); } 


#endif /* __gen_nsIGfxInfo_h__ */
