/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsPISocketTransportService.idl
 */

#ifndef __gen_nsPISocketTransportService_h__
#define __gen_nsPISocketTransportService_h__


#ifndef __gen_nsISocketTransportService_h__
#include "nsISocketTransportService.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsPISocketTransportService */
#define NS_PISOCKETTRANSPORTSERVICE_IID_STR "18f73bf1-b35b-4b7b-aa9a-11bcbdbc389c"

#define NS_PISOCKETTRANSPORTSERVICE_IID \
  {0x18f73bf1, 0xb35b, 0x4b7b, \
    { 0xaa, 0x9a, 0x11, 0xbc, 0xbd, 0xbc, 0x38, 0x9c }}

class NS_NO_VTABLE nsPISocketTransportService : public nsIRoutedSocketTransportService {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_PISOCKETTRANSPORTSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsPISocketTransportService;

  /* void init (); */
  NS_IMETHOD Init(void) = 0;

  /* void shutdown (in bool aXpcomShutdown); */
  NS_IMETHOD Shutdown(bool aXpcomShutdown) = 0;

  /* readonly attribute long sendBufferSize; */
  NS_IMETHOD GetSendBufferSize(int32_t *aSendBufferSize) = 0;

  /* attribute boolean offline; */
  NS_IMETHOD GetOffline(bool *aOffline) = 0;
  NS_IMETHOD SetOffline(bool aOffline) = 0;

  /* readonly attribute long keepaliveIdleTime; */
  NS_IMETHOD GetKeepaliveIdleTime(int32_t *aKeepaliveIdleTime) = 0;

  /* readonly attribute long keepaliveRetryInterval; */
  NS_IMETHOD GetKeepaliveRetryInterval(int32_t *aKeepaliveRetryInterval) = 0;

  /* readonly attribute long keepaliveProbeCount; */
  NS_IMETHOD GetKeepaliveProbeCount(int32_t *aKeepaliveProbeCount) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsPISocketTransportService, NS_PISOCKETTRANSPORTSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSPISOCKETTRANSPORTSERVICE \
  NS_IMETHOD Init(void) override; \
  NS_IMETHOD Shutdown(bool aXpcomShutdown) override; \
  NS_IMETHOD GetSendBufferSize(int32_t *aSendBufferSize) override; \
  NS_IMETHOD GetOffline(bool *aOffline) override; \
  NS_IMETHOD SetOffline(bool aOffline) override; \
  NS_IMETHOD GetKeepaliveIdleTime(int32_t *aKeepaliveIdleTime) override; \
  NS_IMETHOD GetKeepaliveRetryInterval(int32_t *aKeepaliveRetryInterval) override; \
  NS_IMETHOD GetKeepaliveProbeCount(int32_t *aKeepaliveProbeCount) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSPISOCKETTRANSPORTSERVICE \
  nsresult Init(void); \
  nsresult Shutdown(bool aXpcomShutdown); \
  nsresult GetSendBufferSize(int32_t *aSendBufferSize); \
  nsresult GetOffline(bool *aOffline); \
  nsresult SetOffline(bool aOffline); \
  nsresult GetKeepaliveIdleTime(int32_t *aKeepaliveIdleTime); \
  nsresult GetKeepaliveRetryInterval(int32_t *aKeepaliveRetryInterval); \
  nsresult GetKeepaliveProbeCount(int32_t *aKeepaliveProbeCount); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSPISOCKETTRANSPORTSERVICE(_to) \
  NS_IMETHOD Init(void) override { return _to Init(); } \
  NS_IMETHOD Shutdown(bool aXpcomShutdown) override { return _to Shutdown(aXpcomShutdown); } \
  NS_IMETHOD GetSendBufferSize(int32_t *aSendBufferSize) override { return _to GetSendBufferSize(aSendBufferSize); } \
  NS_IMETHOD GetOffline(bool *aOffline) override { return _to GetOffline(aOffline); } \
  NS_IMETHOD SetOffline(bool aOffline) override { return _to SetOffline(aOffline); } \
  NS_IMETHOD GetKeepaliveIdleTime(int32_t *aKeepaliveIdleTime) override { return _to GetKeepaliveIdleTime(aKeepaliveIdleTime); } \
  NS_IMETHOD GetKeepaliveRetryInterval(int32_t *aKeepaliveRetryInterval) override { return _to GetKeepaliveRetryInterval(aKeepaliveRetryInterval); } \
  NS_IMETHOD GetKeepaliveProbeCount(int32_t *aKeepaliveProbeCount) override { return _to GetKeepaliveProbeCount(aKeepaliveProbeCount); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSPISOCKETTRANSPORTSERVICE(_to) \
  NS_IMETHOD Init(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(); } \
  NS_IMETHOD Shutdown(bool aXpcomShutdown) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Shutdown(aXpcomShutdown); } \
  NS_IMETHOD GetSendBufferSize(int32_t *aSendBufferSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSendBufferSize(aSendBufferSize); } \
  NS_IMETHOD GetOffline(bool *aOffline) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOffline(aOffline); } \
  NS_IMETHOD SetOffline(bool aOffline) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetOffline(aOffline); } \
  NS_IMETHOD GetKeepaliveIdleTime(int32_t *aKeepaliveIdleTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetKeepaliveIdleTime(aKeepaliveIdleTime); } \
  NS_IMETHOD GetKeepaliveRetryInterval(int32_t *aKeepaliveRetryInterval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetKeepaliveRetryInterval(aKeepaliveRetryInterval); } \
  NS_IMETHOD GetKeepaliveProbeCount(int32_t *aKeepaliveProbeCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetKeepaliveProbeCount(aKeepaliveProbeCount); } 

/*
 * I/O activity observer topic. Sends out information about the
 * amount of data we're sending/receiving via sockets and disk files.
 *
 * Activated via the "io.activity.enabled" preference.
 */
#define NS_IO_ACTIVITY "io-activity"

#endif /* __gen_nsPISocketTransportService_h__ */
