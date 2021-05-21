/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/tools/profiler/gecko/nsIProfiler.idl
 */

#ifndef __gen_nsIProfiler_h__
#define __gen_nsIProfiler_h__


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
#include "mozilla/Maybe.h"
#include "nsTArrayForwardDeclare.h"
#include "nsStringFwd.h"

/* starting interface:    nsIProfilerStartParams */
#define NS_IPROFILERSTARTPARAMS_IID_STR "0a175ba7-8fcf-4ce9-9c4b-ccc6272f4425"

#define NS_IPROFILERSTARTPARAMS_IID \
  {0x0a175ba7, 0x8fcf, 0x4ce9, \
    { 0x9c, 0x4b, 0xcc, 0xc6, 0x27, 0x2f, 0x44, 0x25 }}

class NS_NO_VTABLE nsIProfilerStartParams : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPROFILERSTARTPARAMS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIProfilerStartParams;

  /* readonly attribute uint32_t entries; */
  NS_IMETHOD GetEntries(uint32_t *aEntries) = 0;

  /* readonly attribute double duration; */
  NS_IMETHOD GetDuration(double *aDuration) = 0;

  /* readonly attribute double interval; */
  NS_IMETHOD GetInterval(double *aInterval) = 0;

  /* readonly attribute uint32_t features; */
  NS_IMETHOD GetFeatures(uint32_t *aFeatures) = 0;

  /* readonly attribute uint64_t activeBrowsingContextID; */
  NS_IMETHOD GetActiveBrowsingContextID(uint64_t *aActiveBrowsingContextID) = 0;

  /* [noscript,nostdcall,notxpcom] StringArrayRef getFilters (); */
  virtual const nsTArray<nsCString> & GetFilters(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIProfilerStartParams, NS_IPROFILERSTARTPARAMS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPROFILERSTARTPARAMS \
  NS_IMETHOD GetEntries(uint32_t *aEntries) override; \
  NS_IMETHOD GetDuration(double *aDuration) override; \
  NS_IMETHOD GetInterval(double *aInterval) override; \
  NS_IMETHOD GetFeatures(uint32_t *aFeatures) override; \
  NS_IMETHOD GetActiveBrowsingContextID(uint64_t *aActiveBrowsingContextID) override; \
  virtual const nsTArray<nsCString> & GetFilters(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPROFILERSTARTPARAMS \
  nsresult GetEntries(uint32_t *aEntries); \
  nsresult GetDuration(double *aDuration); \
  nsresult GetInterval(double *aInterval); \
  nsresult GetFeatures(uint32_t *aFeatures); \
  nsresult GetActiveBrowsingContextID(uint64_t *aActiveBrowsingContextID); \
  const nsTArray<nsCString> & GetFilters(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPROFILERSTARTPARAMS(_to) \
  NS_IMETHOD GetEntries(uint32_t *aEntries) override { return _to GetEntries(aEntries); } \
  NS_IMETHOD GetDuration(double *aDuration) override { return _to GetDuration(aDuration); } \
  NS_IMETHOD GetInterval(double *aInterval) override { return _to GetInterval(aInterval); } \
  NS_IMETHOD GetFeatures(uint32_t *aFeatures) override { return _to GetFeatures(aFeatures); } \
  NS_IMETHOD GetActiveBrowsingContextID(uint64_t *aActiveBrowsingContextID) override { return _to GetActiveBrowsingContextID(aActiveBrowsingContextID); } \
  virtual const nsTArray<nsCString> & GetFilters(void) override { return _to GetFilters(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPROFILERSTARTPARAMS(_to) \
  NS_IMETHOD GetEntries(uint32_t *aEntries) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEntries(aEntries); } \
  NS_IMETHOD GetDuration(double *aDuration) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDuration(aDuration); } \
  NS_IMETHOD GetInterval(double *aInterval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInterval(aInterval); } \
  NS_IMETHOD GetFeatures(uint32_t *aFeatures) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFeatures(aFeatures); } \
  NS_IMETHOD GetActiveBrowsingContextID(uint64_t *aActiveBrowsingContextID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetActiveBrowsingContextID(aActiveBrowsingContextID); } \
  virtual const nsTArray<nsCString> & GetFilters(void) override; 


/* starting interface:    nsIProfiler */
#define NS_IPROFILER_IID_STR "ead3f75c-0e0e-4fbb-901c-1e5392ef5b2a"

#define NS_IPROFILER_IID \
  {0xead3f75c, 0x0e0e, 0x4fbb, \
    { 0x90, 0x1c, 0x1e, 0x53, 0x92, 0xef, 0x5b, 0x2a }}

class NS_NO_VTABLE nsIProfiler : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPROFILER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIProfiler;

  /* boolean CanProfile (); */
  NS_IMETHOD CanProfile(bool *_retval) = 0;

  /* void StartProfiler (in uint32_t aEntries, in double aInterval, in Array<AUTF8String> aFeatures, [optional] in Array<AUTF8String> aFilters, [optional] in uint64_t aActiveBrowsingContextID, [optional] in double aDuration); */
  NS_IMETHOD StartProfiler(uint32_t aEntries, double aInterval, const nsTArray<nsCString >& aFeatures, const nsTArray<nsCString >& aFilters, uint64_t aActiveBrowsingContextID, double aDuration) = 0;

  /* void StopProfiler (); */
  NS_IMETHOD StopProfiler(void) = 0;

  /* boolean IsPaused (); */
  NS_IMETHOD IsPaused(bool *_retval) = 0;

  /* void Pause (); */
  NS_IMETHOD Pause(void) = 0;

  /* void Resume (); */
  NS_IMETHOD Resume(void) = 0;

  /* boolean IsSamplingPaused (); */
  NS_IMETHOD IsSamplingPaused(bool *_retval) = 0;

  /* void PauseSampling (); */
  NS_IMETHOD PauseSampling(void) = 0;

  /* void ResumeSampling (); */
  NS_IMETHOD ResumeSampling(void) = 0;

  /* [implicit_jscontext] Promise waitOnePeriodicSampling (); */
  NS_IMETHOD WaitOnePeriodicSampling(JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* string GetProfile ([optional] in double aSinceTime); */
  NS_IMETHOD GetProfile(double aSinceTime, char * *_retval) = 0;

  /* [implicit_jscontext] jsval getProfileData ([optional] in double aSinceTime); */
  NS_IMETHOD GetProfileData(double aSinceTime, JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] Promise getProfileDataAsync ([optional] in double aSinceTime); */
  NS_IMETHOD GetProfileDataAsync(double aSinceTime, JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* [implicit_jscontext] Promise getProfileDataAsArrayBuffer ([optional] in double aSinceTime); */
  NS_IMETHOD GetProfileDataAsArrayBuffer(double aSinceTime, JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* [implicit_jscontext] Promise getProfileDataAsGzippedArrayBuffer ([optional] in double aSinceTime); */
  NS_IMETHOD GetProfileDataAsGzippedArrayBuffer(double aSinceTime, JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* [implicit_jscontext] Promise dumpProfileToFileAsync (in ACString aFilename, [optional] in double aSinceTime); */
  NS_IMETHOD DumpProfileToFileAsync(const nsACString& aFilename, double aSinceTime, JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* boolean IsActive (); */
  NS_IMETHOD IsActive(bool *_retval) = 0;

  /* void ClearAllPages (); */
  NS_IMETHOD ClearAllPages(void) = 0;

  /* Array<AUTF8String> GetFeatures (); */
  NS_IMETHOD GetFeatures(nsTArray<nsCString >& _retval) = 0;

  /* [implicit_jscontext] readonly attribute jsval activeConfiguration; */
  NS_IMETHOD GetActiveConfiguration(JSContext* cx, JS::MutableHandleValue aActiveConfiguration) = 0;

  /* Array<AUTF8String> GetAllFeatures (); */
  NS_IMETHOD GetAllFeatures(nsTArray<nsCString >& _retval) = 0;

  /* void GetBufferInfo (out uint32_t aCurrentPosition, out uint32_t aTotalSize, out uint32_t aGeneration); */
  NS_IMETHOD GetBufferInfo(uint32_t *aCurrentPosition, uint32_t *aTotalSize, uint32_t *aGeneration) = 0;

  /* double getElapsedTime (); */
  NS_IMETHOD GetElapsedTime(double *_retval) = 0;

  /* [implicit_jscontext] readonly attribute jsval sharedLibraries; */
  NS_IMETHOD GetSharedLibraries(JSContext* cx, JS::MutableHandleValue aSharedLibraries) = 0;

  /* [implicit_jscontext] Promise getSymbolTable (in ACString aDebugPath, in ACString aBreakpadID); */
  NS_IMETHOD GetSymbolTable(const nsACString& aDebugPath, const nsACString& aBreakpadID, JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* void dumpProfileToFile (in string aFilename); */
  NS_IMETHOD DumpProfileToFile(const char * aFilename) = 0;

  /* [noscript,nostdcall,notxpcom] void receiveShutdownProfile (in nsCString aProfile); */
  virtual void ReceiveShutdownProfile(const nsCString & aProfile) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIProfiler, NS_IPROFILER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPROFILER \
  NS_IMETHOD CanProfile(bool *_retval) override; \
  NS_IMETHOD StartProfiler(uint32_t aEntries, double aInterval, const nsTArray<nsCString >& aFeatures, const nsTArray<nsCString >& aFilters, uint64_t aActiveBrowsingContextID, double aDuration) override; \
  NS_IMETHOD StopProfiler(void) override; \
  NS_IMETHOD IsPaused(bool *_retval) override; \
  NS_IMETHOD Pause(void) override; \
  NS_IMETHOD Resume(void) override; \
  NS_IMETHOD IsSamplingPaused(bool *_retval) override; \
  NS_IMETHOD PauseSampling(void) override; \
  NS_IMETHOD ResumeSampling(void) override; \
  NS_IMETHOD WaitOnePeriodicSampling(JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetProfile(double aSinceTime, char * *_retval) override; \
  NS_IMETHOD GetProfileData(double aSinceTime, JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetProfileDataAsync(double aSinceTime, JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetProfileDataAsArrayBuffer(double aSinceTime, JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetProfileDataAsGzippedArrayBuffer(double aSinceTime, JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD DumpProfileToFileAsync(const nsACString& aFilename, double aSinceTime, JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD IsActive(bool *_retval) override; \
  NS_IMETHOD ClearAllPages(void) override; \
  NS_IMETHOD GetFeatures(nsTArray<nsCString >& _retval) override; \
  NS_IMETHOD GetActiveConfiguration(JSContext* cx, JS::MutableHandleValue aActiveConfiguration) override; \
  NS_IMETHOD GetAllFeatures(nsTArray<nsCString >& _retval) override; \
  NS_IMETHOD GetBufferInfo(uint32_t *aCurrentPosition, uint32_t *aTotalSize, uint32_t *aGeneration) override; \
  NS_IMETHOD GetElapsedTime(double *_retval) override; \
  NS_IMETHOD GetSharedLibraries(JSContext* cx, JS::MutableHandleValue aSharedLibraries) override; \
  NS_IMETHOD GetSymbolTable(const nsACString& aDebugPath, const nsACString& aBreakpadID, JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD DumpProfileToFile(const char * aFilename) override; \
  virtual void ReceiveShutdownProfile(const nsCString & aProfile) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPROFILER \
  nsresult CanProfile(bool *_retval); \
  nsresult StartProfiler(uint32_t aEntries, double aInterval, const nsTArray<nsCString >& aFeatures, const nsTArray<nsCString >& aFilters, uint64_t aActiveBrowsingContextID, double aDuration); \
  nsresult StopProfiler(void); \
  nsresult IsPaused(bool *_retval); \
  nsresult Pause(void); \
  nsresult Resume(void); \
  nsresult IsSamplingPaused(bool *_retval); \
  nsresult PauseSampling(void); \
  nsresult ResumeSampling(void); \
  nsresult WaitOnePeriodicSampling(JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  nsresult GetProfile(double aSinceTime, char * *_retval); \
  nsresult GetProfileData(double aSinceTime, JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult GetProfileDataAsync(double aSinceTime, JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  nsresult GetProfileDataAsArrayBuffer(double aSinceTime, JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  nsresult GetProfileDataAsGzippedArrayBuffer(double aSinceTime, JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  nsresult DumpProfileToFileAsync(const nsACString& aFilename, double aSinceTime, JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  nsresult IsActive(bool *_retval); \
  nsresult ClearAllPages(void); \
  nsresult GetFeatures(nsTArray<nsCString >& _retval); \
  nsresult GetActiveConfiguration(JSContext* cx, JS::MutableHandleValue aActiveConfiguration); \
  nsresult GetAllFeatures(nsTArray<nsCString >& _retval); \
  nsresult GetBufferInfo(uint32_t *aCurrentPosition, uint32_t *aTotalSize, uint32_t *aGeneration); \
  nsresult GetElapsedTime(double *_retval); \
  nsresult GetSharedLibraries(JSContext* cx, JS::MutableHandleValue aSharedLibraries); \
  nsresult GetSymbolTable(const nsACString& aDebugPath, const nsACString& aBreakpadID, JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  nsresult DumpProfileToFile(const char * aFilename); \
  void ReceiveShutdownProfile(const nsCString & aProfile); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPROFILER(_to) \
  NS_IMETHOD CanProfile(bool *_retval) override { return _to CanProfile(_retval); } \
  NS_IMETHOD StartProfiler(uint32_t aEntries, double aInterval, const nsTArray<nsCString >& aFeatures, const nsTArray<nsCString >& aFilters, uint64_t aActiveBrowsingContextID, double aDuration) override { return _to StartProfiler(aEntries, aInterval, aFeatures, aFilters, aActiveBrowsingContextID, aDuration); } \
  NS_IMETHOD StopProfiler(void) override { return _to StopProfiler(); } \
  NS_IMETHOD IsPaused(bool *_retval) override { return _to IsPaused(_retval); } \
  NS_IMETHOD Pause(void) override { return _to Pause(); } \
  NS_IMETHOD Resume(void) override { return _to Resume(); } \
  NS_IMETHOD IsSamplingPaused(bool *_retval) override { return _to IsSamplingPaused(_retval); } \
  NS_IMETHOD PauseSampling(void) override { return _to PauseSampling(); } \
  NS_IMETHOD ResumeSampling(void) override { return _to ResumeSampling(); } \
  NS_IMETHOD WaitOnePeriodicSampling(JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to WaitOnePeriodicSampling(cx, _retval); } \
  NS_IMETHOD GetProfile(double aSinceTime, char * *_retval) override { return _to GetProfile(aSinceTime, _retval); } \
  NS_IMETHOD GetProfileData(double aSinceTime, JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetProfileData(aSinceTime, cx, _retval); } \
  NS_IMETHOD GetProfileDataAsync(double aSinceTime, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to GetProfileDataAsync(aSinceTime, cx, _retval); } \
  NS_IMETHOD GetProfileDataAsArrayBuffer(double aSinceTime, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to GetProfileDataAsArrayBuffer(aSinceTime, cx, _retval); } \
  NS_IMETHOD GetProfileDataAsGzippedArrayBuffer(double aSinceTime, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to GetProfileDataAsGzippedArrayBuffer(aSinceTime, cx, _retval); } \
  NS_IMETHOD DumpProfileToFileAsync(const nsACString& aFilename, double aSinceTime, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to DumpProfileToFileAsync(aFilename, aSinceTime, cx, _retval); } \
  NS_IMETHOD IsActive(bool *_retval) override { return _to IsActive(_retval); } \
  NS_IMETHOD ClearAllPages(void) override { return _to ClearAllPages(); } \
  NS_IMETHOD GetFeatures(nsTArray<nsCString >& _retval) override { return _to GetFeatures(_retval); } \
  NS_IMETHOD GetActiveConfiguration(JSContext* cx, JS::MutableHandleValue aActiveConfiguration) override { return _to GetActiveConfiguration(cx, aActiveConfiguration); } \
  NS_IMETHOD GetAllFeatures(nsTArray<nsCString >& _retval) override { return _to GetAllFeatures(_retval); } \
  NS_IMETHOD GetBufferInfo(uint32_t *aCurrentPosition, uint32_t *aTotalSize, uint32_t *aGeneration) override { return _to GetBufferInfo(aCurrentPosition, aTotalSize, aGeneration); } \
  NS_IMETHOD GetElapsedTime(double *_retval) override { return _to GetElapsedTime(_retval); } \
  NS_IMETHOD GetSharedLibraries(JSContext* cx, JS::MutableHandleValue aSharedLibraries) override { return _to GetSharedLibraries(cx, aSharedLibraries); } \
  NS_IMETHOD GetSymbolTable(const nsACString& aDebugPath, const nsACString& aBreakpadID, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to GetSymbolTable(aDebugPath, aBreakpadID, cx, _retval); } \
  NS_IMETHOD DumpProfileToFile(const char * aFilename) override { return _to DumpProfileToFile(aFilename); } \
  virtual void ReceiveShutdownProfile(const nsCString & aProfile) override { return _to ReceiveShutdownProfile(aProfile); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPROFILER(_to) \
  NS_IMETHOD CanProfile(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CanProfile(_retval); } \
  NS_IMETHOD StartProfiler(uint32_t aEntries, double aInterval, const nsTArray<nsCString >& aFeatures, const nsTArray<nsCString >& aFilters, uint64_t aActiveBrowsingContextID, double aDuration) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StartProfiler(aEntries, aInterval, aFeatures, aFilters, aActiveBrowsingContextID, aDuration); } \
  NS_IMETHOD StopProfiler(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StopProfiler(); } \
  NS_IMETHOD IsPaused(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsPaused(_retval); } \
  NS_IMETHOD Pause(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Pause(); } \
  NS_IMETHOD Resume(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Resume(); } \
  NS_IMETHOD IsSamplingPaused(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsSamplingPaused(_retval); } \
  NS_IMETHOD PauseSampling(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PauseSampling(); } \
  NS_IMETHOD ResumeSampling(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResumeSampling(); } \
  NS_IMETHOD WaitOnePeriodicSampling(JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WaitOnePeriodicSampling(cx, _retval); } \
  NS_IMETHOD GetProfile(double aSinceTime, char * *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProfile(aSinceTime, _retval); } \
  NS_IMETHOD GetProfileData(double aSinceTime, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProfileData(aSinceTime, cx, _retval); } \
  NS_IMETHOD GetProfileDataAsync(double aSinceTime, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProfileDataAsync(aSinceTime, cx, _retval); } \
  NS_IMETHOD GetProfileDataAsArrayBuffer(double aSinceTime, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProfileDataAsArrayBuffer(aSinceTime, cx, _retval); } \
  NS_IMETHOD GetProfileDataAsGzippedArrayBuffer(double aSinceTime, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProfileDataAsGzippedArrayBuffer(aSinceTime, cx, _retval); } \
  NS_IMETHOD DumpProfileToFileAsync(const nsACString& aFilename, double aSinceTime, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DumpProfileToFileAsync(aFilename, aSinceTime, cx, _retval); } \
  NS_IMETHOD IsActive(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsActive(_retval); } \
  NS_IMETHOD ClearAllPages(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearAllPages(); } \
  NS_IMETHOD GetFeatures(nsTArray<nsCString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFeatures(_retval); } \
  NS_IMETHOD GetActiveConfiguration(JSContext* cx, JS::MutableHandleValue aActiveConfiguration) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetActiveConfiguration(cx, aActiveConfiguration); } \
  NS_IMETHOD GetAllFeatures(nsTArray<nsCString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllFeatures(_retval); } \
  NS_IMETHOD GetBufferInfo(uint32_t *aCurrentPosition, uint32_t *aTotalSize, uint32_t *aGeneration) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBufferInfo(aCurrentPosition, aTotalSize, aGeneration); } \
  NS_IMETHOD GetElapsedTime(double *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetElapsedTime(_retval); } \
  NS_IMETHOD GetSharedLibraries(JSContext* cx, JS::MutableHandleValue aSharedLibraries) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSharedLibraries(cx, aSharedLibraries); } \
  NS_IMETHOD GetSymbolTable(const nsACString& aDebugPath, const nsACString& aBreakpadID, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSymbolTable(aDebugPath, aBreakpadID, cx, _retval); } \
  NS_IMETHOD DumpProfileToFile(const char * aFilename) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DumpProfileToFile(aFilename); } \
  virtual void ReceiveShutdownProfile(const nsCString & aProfile) override; 


#endif /* __gen_nsIProfiler_h__ */
