/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache/nsICacheVisitor.idl
 */

#ifndef __gen_nsICacheVisitor_h__
#define __gen_nsICacheVisitor_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsICacheDeviceInfo; /* forward declaration */

class nsICacheEntryInfo; /* forward declaration */


/* starting interface:    nsICacheVisitor */
#define NS_ICACHEVISITOR_IID_STR "f8c08c4b-d778-49d1-a59b-866fdc500d95"

#define NS_ICACHEVISITOR_IID \
  {0xf8c08c4b, 0xd778, 0x49d1, \
    { 0xa5, 0x9b, 0x86, 0x6f, 0xdc, 0x50, 0x0d, 0x95 }}

class NS_NO_VTABLE nsICacheVisitor : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICACHEVISITOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICacheVisitor;

  /* boolean visitDevice (in string deviceID, in nsICacheDeviceInfo deviceInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD VisitDevice(const char * deviceID, nsICacheDeviceInfo *deviceInfo, bool *_retval) = 0;

  /* boolean visitEntry (in string deviceID, in nsICacheEntryInfo entryInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD VisitEntry(const char * deviceID, nsICacheEntryInfo *entryInfo, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICacheVisitor, NS_ICACHEVISITOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICACHEVISITOR \
  NS_IMETHOD VisitDevice(const char * deviceID, nsICacheDeviceInfo *deviceInfo, bool *_retval) override; \
  NS_IMETHOD VisitEntry(const char * deviceID, nsICacheEntryInfo *entryInfo, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICACHEVISITOR \
  nsresult VisitDevice(const char * deviceID, nsICacheDeviceInfo *deviceInfo, bool *_retval); \
  nsresult VisitEntry(const char * deviceID, nsICacheEntryInfo *entryInfo, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICACHEVISITOR(_to) \
  NS_IMETHOD VisitDevice(const char * deviceID, nsICacheDeviceInfo *deviceInfo, bool *_retval) override { return _to VisitDevice(deviceID, deviceInfo, _retval); } \
  NS_IMETHOD VisitEntry(const char * deviceID, nsICacheEntryInfo *entryInfo, bool *_retval) override { return _to VisitEntry(deviceID, entryInfo, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICACHEVISITOR(_to) \
  NS_IMETHOD VisitDevice(const char * deviceID, nsICacheDeviceInfo *deviceInfo, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->VisitDevice(deviceID, deviceInfo, _retval); } \
  NS_IMETHOD VisitEntry(const char * deviceID, nsICacheEntryInfo *entryInfo, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->VisitEntry(deviceID, entryInfo, _retval); } 


/* starting interface:    nsICacheDeviceInfo */
#define NS_ICACHEDEVICEINFO_IID_STR "31d1c294-1dd2-11b2-be3a-c79230dca297"

#define NS_ICACHEDEVICEINFO_IID \
  {0x31d1c294, 0x1dd2, 0x11b2, \
    { 0xbe, 0x3a, 0xc7, 0x92, 0x30, 0xdc, 0xa2, 0x97 }}

class NS_NO_VTABLE nsICacheDeviceInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICACHEDEVICEINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICacheDeviceInfo;

  /* readonly attribute ACString description; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDescription(nsACString& aDescription) = 0;

  /* readonly attribute ACString usageReport; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUsageReport(nsACString& aUsageReport) = 0;

  /* readonly attribute unsigned long entryCount; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEntryCount(uint32_t *aEntryCount) = 0;

  /* readonly attribute unsigned long totalSize; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTotalSize(uint32_t *aTotalSize) = 0;

  /* readonly attribute unsigned long maximumSize; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMaximumSize(uint32_t *aMaximumSize) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICacheDeviceInfo, NS_ICACHEDEVICEINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICACHEDEVICEINFO \
  NS_IMETHOD GetDescription(nsACString& aDescription) override; \
  NS_IMETHOD GetUsageReport(nsACString& aUsageReport) override; \
  NS_IMETHOD GetEntryCount(uint32_t *aEntryCount) override; \
  NS_IMETHOD GetTotalSize(uint32_t *aTotalSize) override; \
  NS_IMETHOD GetMaximumSize(uint32_t *aMaximumSize) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICACHEDEVICEINFO \
  nsresult GetDescription(nsACString& aDescription); \
  nsresult GetUsageReport(nsACString& aUsageReport); \
  nsresult GetEntryCount(uint32_t *aEntryCount); \
  nsresult GetTotalSize(uint32_t *aTotalSize); \
  nsresult GetMaximumSize(uint32_t *aMaximumSize); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICACHEDEVICEINFO(_to) \
  NS_IMETHOD GetDescription(nsACString& aDescription) override { return _to GetDescription(aDescription); } \
  NS_IMETHOD GetUsageReport(nsACString& aUsageReport) override { return _to GetUsageReport(aUsageReport); } \
  NS_IMETHOD GetEntryCount(uint32_t *aEntryCount) override { return _to GetEntryCount(aEntryCount); } \
  NS_IMETHOD GetTotalSize(uint32_t *aTotalSize) override { return _to GetTotalSize(aTotalSize); } \
  NS_IMETHOD GetMaximumSize(uint32_t *aMaximumSize) override { return _to GetMaximumSize(aMaximumSize); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICACHEDEVICEINFO(_to) \
  NS_IMETHOD GetDescription(nsACString& aDescription) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDescription(aDescription); } \
  NS_IMETHOD GetUsageReport(nsACString& aUsageReport) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUsageReport(aUsageReport); } \
  NS_IMETHOD GetEntryCount(uint32_t *aEntryCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEntryCount(aEntryCount); } \
  NS_IMETHOD GetTotalSize(uint32_t *aTotalSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTotalSize(aTotalSize); } \
  NS_IMETHOD GetMaximumSize(uint32_t *aMaximumSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMaximumSize(aMaximumSize); } 


/* starting interface:    nsICacheEntryInfo */
#define NS_ICACHEENTRYINFO_IID_STR "fab51c92-95c3-4468-b317-7de4d7588254"

#define NS_ICACHEENTRYINFO_IID \
  {0xfab51c92, 0x95c3, 0x4468, \
    { 0xb3, 0x17, 0x7d, 0xe4, 0xd7, 0x58, 0x82, 0x54 }}

class NS_NO_VTABLE nsICacheEntryInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICACHEENTRYINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICacheEntryInfo;

  /* readonly attribute ACString clientID; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetClientID(nsACString& aClientID) = 0;

  /* readonly attribute ACString deviceID; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDeviceID(nsACString& aDeviceID) = 0;

  /* readonly attribute ACString key; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetKey(nsACString& aKey) = 0;

  /* readonly attribute long fetchCount; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFetchCount(int32_t *aFetchCount) = 0;

  /* readonly attribute uint32_t lastFetched; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLastFetched(uint32_t *aLastFetched) = 0;

  /* readonly attribute uint32_t lastModified; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLastModified(uint32_t *aLastModified) = 0;

  /* readonly attribute uint32_t expirationTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetExpirationTime(uint32_t *aExpirationTime) = 0;

  /* readonly attribute unsigned long dataSize; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDataSize(uint32_t *aDataSize) = 0;

  /* boolean isStreamBased (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsStreamBased(bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICacheEntryInfo, NS_ICACHEENTRYINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICACHEENTRYINFO \
  NS_IMETHOD GetClientID(nsACString& aClientID) override; \
  NS_IMETHOD GetDeviceID(nsACString& aDeviceID) override; \
  NS_IMETHOD GetKey(nsACString& aKey) override; \
  NS_IMETHOD GetFetchCount(int32_t *aFetchCount) override; \
  NS_IMETHOD GetLastFetched(uint32_t *aLastFetched) override; \
  NS_IMETHOD GetLastModified(uint32_t *aLastModified) override; \
  NS_IMETHOD GetExpirationTime(uint32_t *aExpirationTime) override; \
  NS_IMETHOD GetDataSize(uint32_t *aDataSize) override; \
  NS_IMETHOD IsStreamBased(bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICACHEENTRYINFO \
  nsresult GetClientID(nsACString& aClientID); \
  nsresult GetDeviceID(nsACString& aDeviceID); \
  nsresult GetKey(nsACString& aKey); \
  nsresult GetFetchCount(int32_t *aFetchCount); \
  nsresult GetLastFetched(uint32_t *aLastFetched); \
  nsresult GetLastModified(uint32_t *aLastModified); \
  nsresult GetExpirationTime(uint32_t *aExpirationTime); \
  nsresult GetDataSize(uint32_t *aDataSize); \
  nsresult IsStreamBased(bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICACHEENTRYINFO(_to) \
  NS_IMETHOD GetClientID(nsACString& aClientID) override { return _to GetClientID(aClientID); } \
  NS_IMETHOD GetDeviceID(nsACString& aDeviceID) override { return _to GetDeviceID(aDeviceID); } \
  NS_IMETHOD GetKey(nsACString& aKey) override { return _to GetKey(aKey); } \
  NS_IMETHOD GetFetchCount(int32_t *aFetchCount) override { return _to GetFetchCount(aFetchCount); } \
  NS_IMETHOD GetLastFetched(uint32_t *aLastFetched) override { return _to GetLastFetched(aLastFetched); } \
  NS_IMETHOD GetLastModified(uint32_t *aLastModified) override { return _to GetLastModified(aLastModified); } \
  NS_IMETHOD GetExpirationTime(uint32_t *aExpirationTime) override { return _to GetExpirationTime(aExpirationTime); } \
  NS_IMETHOD GetDataSize(uint32_t *aDataSize) override { return _to GetDataSize(aDataSize); } \
  NS_IMETHOD IsStreamBased(bool *_retval) override { return _to IsStreamBased(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICACHEENTRYINFO(_to) \
  NS_IMETHOD GetClientID(nsACString& aClientID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetClientID(aClientID); } \
  NS_IMETHOD GetDeviceID(nsACString& aDeviceID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDeviceID(aDeviceID); } \
  NS_IMETHOD GetKey(nsACString& aKey) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetKey(aKey); } \
  NS_IMETHOD GetFetchCount(int32_t *aFetchCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFetchCount(aFetchCount); } \
  NS_IMETHOD GetLastFetched(uint32_t *aLastFetched) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastFetched(aLastFetched); } \
  NS_IMETHOD GetLastModified(uint32_t *aLastModified) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastModified(aLastModified); } \
  NS_IMETHOD GetExpirationTime(uint32_t *aExpirationTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExpirationTime(aExpirationTime); } \
  NS_IMETHOD GetDataSize(uint32_t *aDataSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDataSize(aDataSize); } \
  NS_IMETHOD IsStreamBased(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsStreamBased(_retval); } 


#endif /* __gen_nsICacheVisitor_h__ */
