/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIDashboard.idl
 */

#ifndef __gen_nsIDashboard_h__
#define __gen_nsIDashboard_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsINetDashboardCallback */
#define NS_INETDASHBOARDCALLBACK_IID_STR "19d7f24f-a95a-4fd9-87e2-d96e9e4b1f6d"

#define NS_INETDASHBOARDCALLBACK_IID \
  {0x19d7f24f, 0xa95a, 0x4fd9, \
    { 0x87, 0xe2, 0xd9, 0x6e, 0x9e, 0x4b, 0x1f, 0x6d }}

class NS_NO_VTABLE nsINetDashboardCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INETDASHBOARDCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINetDashboardCallback;

  /* void onDashboardDataAvailable (in jsval data); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnDashboardDataAvailable(JS::HandleValue data) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINetDashboardCallback, NS_INETDASHBOARDCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINETDASHBOARDCALLBACK \
  NS_IMETHOD OnDashboardDataAvailable(JS::HandleValue data) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINETDASHBOARDCALLBACK \
  nsresult OnDashboardDataAvailable(JS::HandleValue data); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINETDASHBOARDCALLBACK(_to) \
  NS_IMETHOD OnDashboardDataAvailable(JS::HandleValue data) override { return _to OnDashboardDataAvailable(data); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINETDASHBOARDCALLBACK(_to) \
  NS_IMETHOD OnDashboardDataAvailable(JS::HandleValue data) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnDashboardDataAvailable(data); } 


/* starting interface:    nsIDashboard */
#define NS_IDASHBOARD_IID_STR "c79eb3c6-091a-45a6-8544-5a8d1ab79537"

#define NS_IDASHBOARD_IID \
  {0xc79eb3c6, 0x091a, 0x45a6, \
    { 0x85, 0x44, 0x5a, 0x8d, 0x1a, 0xb7, 0x95, 0x37 }}

class NS_NO_VTABLE nsIDashboard : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDASHBOARD_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDashboard;

  /* void requestSockets (in nsINetDashboardCallback cb); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RequestSockets(nsINetDashboardCallback *cb) = 0;

  /* void requestHttpConnections (in nsINetDashboardCallback cb); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RequestHttpConnections(nsINetDashboardCallback *cb) = 0;

  /* void requestWebsocketConnections (in nsINetDashboardCallback cb); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RequestWebsocketConnections(nsINetDashboardCallback *cb) = 0;

  /* void requestDNSInfo (in nsINetDashboardCallback cb); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RequestDNSInfo(nsINetDashboardCallback *cb) = 0;

  /* void requestConnection (in ACString aHost, in unsigned long aPort, in string aProtocol, in unsigned long aTimeout, in nsINetDashboardCallback cb); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RequestConnection(const nsACString& aHost, uint32_t aPort, const char * aProtocol, uint32_t aTimeout, nsINetDashboardCallback *cb) = 0;

  /* attribute boolean enableLogging; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEnableLogging(bool *aEnableLogging) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetEnableLogging(bool aEnableLogging) = 0;

  /* void requestDNSLookup (in ACString aHost, in nsINetDashboardCallback cb); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RequestDNSLookup(const nsACString& aHost, nsINetDashboardCallback *cb) = 0;

  /* void requestDNSHTTPSRRLookup (in ACString aHost, in nsINetDashboardCallback cb); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RequestDNSHTTPSRRLookup(const nsACString& aHost, nsINetDashboardCallback *cb) = 0;

  /* void requestRcwnStats (in nsINetDashboardCallback cb); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RequestRcwnStats(nsINetDashboardCallback *cb) = 0;

  /* AUTF8String getLogPath (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLogPath(nsACString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDashboard, NS_IDASHBOARD_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDASHBOARD \
  NS_IMETHOD RequestSockets(nsINetDashboardCallback *cb) override; \
  NS_IMETHOD RequestHttpConnections(nsINetDashboardCallback *cb) override; \
  NS_IMETHOD RequestWebsocketConnections(nsINetDashboardCallback *cb) override; \
  NS_IMETHOD RequestDNSInfo(nsINetDashboardCallback *cb) override; \
  NS_IMETHOD RequestConnection(const nsACString& aHost, uint32_t aPort, const char * aProtocol, uint32_t aTimeout, nsINetDashboardCallback *cb) override; \
  NS_IMETHOD GetEnableLogging(bool *aEnableLogging) override; \
  NS_IMETHOD SetEnableLogging(bool aEnableLogging) override; \
  NS_IMETHOD RequestDNSLookup(const nsACString& aHost, nsINetDashboardCallback *cb) override; \
  NS_IMETHOD RequestDNSHTTPSRRLookup(const nsACString& aHost, nsINetDashboardCallback *cb) override; \
  NS_IMETHOD RequestRcwnStats(nsINetDashboardCallback *cb) override; \
  NS_IMETHOD GetLogPath(nsACString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDASHBOARD \
  nsresult RequestSockets(nsINetDashboardCallback *cb); \
  nsresult RequestHttpConnections(nsINetDashboardCallback *cb); \
  nsresult RequestWebsocketConnections(nsINetDashboardCallback *cb); \
  nsresult RequestDNSInfo(nsINetDashboardCallback *cb); \
  nsresult RequestConnection(const nsACString& aHost, uint32_t aPort, const char * aProtocol, uint32_t aTimeout, nsINetDashboardCallback *cb); \
  nsresult GetEnableLogging(bool *aEnableLogging); \
  nsresult SetEnableLogging(bool aEnableLogging); \
  nsresult RequestDNSLookup(const nsACString& aHost, nsINetDashboardCallback *cb); \
  nsresult RequestDNSHTTPSRRLookup(const nsACString& aHost, nsINetDashboardCallback *cb); \
  nsresult RequestRcwnStats(nsINetDashboardCallback *cb); \
  nsresult GetLogPath(nsACString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDASHBOARD(_to) \
  NS_IMETHOD RequestSockets(nsINetDashboardCallback *cb) override { return _to RequestSockets(cb); } \
  NS_IMETHOD RequestHttpConnections(nsINetDashboardCallback *cb) override { return _to RequestHttpConnections(cb); } \
  NS_IMETHOD RequestWebsocketConnections(nsINetDashboardCallback *cb) override { return _to RequestWebsocketConnections(cb); } \
  NS_IMETHOD RequestDNSInfo(nsINetDashboardCallback *cb) override { return _to RequestDNSInfo(cb); } \
  NS_IMETHOD RequestConnection(const nsACString& aHost, uint32_t aPort, const char * aProtocol, uint32_t aTimeout, nsINetDashboardCallback *cb) override { return _to RequestConnection(aHost, aPort, aProtocol, aTimeout, cb); } \
  NS_IMETHOD GetEnableLogging(bool *aEnableLogging) override { return _to GetEnableLogging(aEnableLogging); } \
  NS_IMETHOD SetEnableLogging(bool aEnableLogging) override { return _to SetEnableLogging(aEnableLogging); } \
  NS_IMETHOD RequestDNSLookup(const nsACString& aHost, nsINetDashboardCallback *cb) override { return _to RequestDNSLookup(aHost, cb); } \
  NS_IMETHOD RequestDNSHTTPSRRLookup(const nsACString& aHost, nsINetDashboardCallback *cb) override { return _to RequestDNSHTTPSRRLookup(aHost, cb); } \
  NS_IMETHOD RequestRcwnStats(nsINetDashboardCallback *cb) override { return _to RequestRcwnStats(cb); } \
  NS_IMETHOD GetLogPath(nsACString& _retval) override { return _to GetLogPath(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDASHBOARD(_to) \
  NS_IMETHOD RequestSockets(nsINetDashboardCallback *cb) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RequestSockets(cb); } \
  NS_IMETHOD RequestHttpConnections(nsINetDashboardCallback *cb) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RequestHttpConnections(cb); } \
  NS_IMETHOD RequestWebsocketConnections(nsINetDashboardCallback *cb) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RequestWebsocketConnections(cb); } \
  NS_IMETHOD RequestDNSInfo(nsINetDashboardCallback *cb) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RequestDNSInfo(cb); } \
  NS_IMETHOD RequestConnection(const nsACString& aHost, uint32_t aPort, const char * aProtocol, uint32_t aTimeout, nsINetDashboardCallback *cb) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RequestConnection(aHost, aPort, aProtocol, aTimeout, cb); } \
  NS_IMETHOD GetEnableLogging(bool *aEnableLogging) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEnableLogging(aEnableLogging); } \
  NS_IMETHOD SetEnableLogging(bool aEnableLogging) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEnableLogging(aEnableLogging); } \
  NS_IMETHOD RequestDNSLookup(const nsACString& aHost, nsINetDashboardCallback *cb) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RequestDNSLookup(aHost, cb); } \
  NS_IMETHOD RequestDNSHTTPSRRLookup(const nsACString& aHost, nsINetDashboardCallback *cb) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RequestDNSHTTPSRRLookup(aHost, cb); } \
  NS_IMETHOD RequestRcwnStats(nsINetDashboardCallback *cb) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RequestRcwnStats(cb); } \
  NS_IMETHOD GetLogPath(nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLogPath(_retval); } 


#endif /* __gen_nsIDashboard_h__ */
