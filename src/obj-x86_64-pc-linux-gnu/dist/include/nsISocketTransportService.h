/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISocketTransportService.idl
 */

#ifndef __gen_nsISocketTransportService_h__
#define __gen_nsISocketTransportService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIDNSRecord; /* forward declaration */

class nsIFile; /* forward declaration */

class nsISocketTransport; /* forward declaration */

class nsIProxyInfo; /* forward declaration */

class nsIRunnable; /* forward declaration */

class nsASocketHandler;
struct PRFileDesc;

/* starting interface:    nsISTSShutdownObserver */
#define NS_ISTSSHUTDOWNOBSERVER_IID_STR "338947df-2f3b-4d24-9ce4-ecf161c1b7df"

#define NS_ISTSSHUTDOWNOBSERVER_IID \
  {0x338947df, 0x2f3b, 0x4d24, \
    { 0x9c, 0xe4, 0xec, 0xf1, 0x61, 0xc1, 0xb7, 0xdf }}

class NS_NO_VTABLE nsISTSShutdownObserver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISTSSHUTDOWNOBSERVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISTSShutdownObserver;

  /* void observe (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Observe(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISTSShutdownObserver, NS_ISTSSHUTDOWNOBSERVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISTSSHUTDOWNOBSERVER \
  NS_IMETHOD Observe(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISTSSHUTDOWNOBSERVER \
  nsresult Observe(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISTSSHUTDOWNOBSERVER(_to) \
  NS_IMETHOD Observe(void) override { return _to Observe(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISTSSHUTDOWNOBSERVER(_to) \
  NS_IMETHOD Observe(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Observe(); } 


/* starting interface:    nsISocketTransportService */
#define NS_ISOCKETTRANSPORTSERVICE_IID_STR "ad56b25f-e6bb-4db3-9f7b-5b7db33fd2b1"

#define NS_ISOCKETTRANSPORTSERVICE_IID \
  {0xad56b25f, 0xe6bb, 0x4db3, \
    { 0x9f, 0x7b, 0x5b, 0x7d, 0xb3, 0x3f, 0xd2, 0xb1 }}

class NS_NO_VTABLE nsISocketTransportService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISOCKETTRANSPORTSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISocketTransportService;

  /* nsISocketTransport createTransport (in Array<ACString> aSocketTypes, in AUTF8String aHost, in long aPort, in nsIProxyInfo aProxyInfo, in nsIDNSRecord dnsRecord); */
  NS_IMETHOD CreateTransport(const nsTArray<nsCString >& aSocketTypes, const nsACString& aHost, int32_t aPort, nsIProxyInfo *aProxyInfo, nsIDNSRecord *dnsRecord, nsISocketTransport **_retval) = 0;

  /* nsISocketTransport createUnixDomainTransport (in nsIFile aPath); */
  NS_IMETHOD CreateUnixDomainTransport(nsIFile *aPath, nsISocketTransport **_retval) = 0;

  /* nsISocketTransport createUnixDomainAbstractAddressTransport (in ACString aName); */
  NS_IMETHOD CreateUnixDomainAbstractAddressTransport(const nsACString& aName, nsISocketTransport **_retval) = 0;

  /* [noscript] void attachSocket (in PRFileDescPtr aFd, in nsASocketHandlerPtr aHandler); */
  NS_IMETHOD AttachSocket(PRFileDesc * aFd, nsASocketHandler * aHandler) = 0;

  /* [noscript] void notifyWhenCanAttachSocket (in nsIRunnable aEvent); */
  NS_IMETHOD NotifyWhenCanAttachSocket(nsIRunnable *aEvent) = 0;

  /* [noscript] void addShutdownObserver (in nsISTSShutdownObserver aObserver); */
  NS_IMETHOD AddShutdownObserver(nsISTSShutdownObserver *aObserver) = 0;

  /* [noscript] void removeShutdownObserver (in nsISTSShutdownObserver aObserver); */
  NS_IMETHOD RemoveShutdownObserver(nsISTSShutdownObserver *aObserver) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISocketTransportService, NS_ISOCKETTRANSPORTSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISOCKETTRANSPORTSERVICE \
  NS_IMETHOD CreateTransport(const nsTArray<nsCString >& aSocketTypes, const nsACString& aHost, int32_t aPort, nsIProxyInfo *aProxyInfo, nsIDNSRecord *dnsRecord, nsISocketTransport **_retval) override; \
  NS_IMETHOD CreateUnixDomainTransport(nsIFile *aPath, nsISocketTransport **_retval) override; \
  NS_IMETHOD CreateUnixDomainAbstractAddressTransport(const nsACString& aName, nsISocketTransport **_retval) override; \
  NS_IMETHOD AttachSocket(PRFileDesc * aFd, nsASocketHandler * aHandler) override; \
  NS_IMETHOD NotifyWhenCanAttachSocket(nsIRunnable *aEvent) override; \
  NS_IMETHOD AddShutdownObserver(nsISTSShutdownObserver *aObserver) override; \
  NS_IMETHOD RemoveShutdownObserver(nsISTSShutdownObserver *aObserver) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISOCKETTRANSPORTSERVICE \
  nsresult CreateTransport(const nsTArray<nsCString >& aSocketTypes, const nsACString& aHost, int32_t aPort, nsIProxyInfo *aProxyInfo, nsIDNSRecord *dnsRecord, nsISocketTransport **_retval); \
  nsresult CreateUnixDomainTransport(nsIFile *aPath, nsISocketTransport **_retval); \
  nsresult CreateUnixDomainAbstractAddressTransport(const nsACString& aName, nsISocketTransport **_retval); \
  nsresult AttachSocket(PRFileDesc * aFd, nsASocketHandler * aHandler); \
  nsresult NotifyWhenCanAttachSocket(nsIRunnable *aEvent); \
  nsresult AddShutdownObserver(nsISTSShutdownObserver *aObserver); \
  nsresult RemoveShutdownObserver(nsISTSShutdownObserver *aObserver); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISOCKETTRANSPORTSERVICE(_to) \
  NS_IMETHOD CreateTransport(const nsTArray<nsCString >& aSocketTypes, const nsACString& aHost, int32_t aPort, nsIProxyInfo *aProxyInfo, nsIDNSRecord *dnsRecord, nsISocketTransport **_retval) override { return _to CreateTransport(aSocketTypes, aHost, aPort, aProxyInfo, dnsRecord, _retval); } \
  NS_IMETHOD CreateUnixDomainTransport(nsIFile *aPath, nsISocketTransport **_retval) override { return _to CreateUnixDomainTransport(aPath, _retval); } \
  NS_IMETHOD CreateUnixDomainAbstractAddressTransport(const nsACString& aName, nsISocketTransport **_retval) override { return _to CreateUnixDomainAbstractAddressTransport(aName, _retval); } \
  NS_IMETHOD AttachSocket(PRFileDesc * aFd, nsASocketHandler * aHandler) override { return _to AttachSocket(aFd, aHandler); } \
  NS_IMETHOD NotifyWhenCanAttachSocket(nsIRunnable *aEvent) override { return _to NotifyWhenCanAttachSocket(aEvent); } \
  NS_IMETHOD AddShutdownObserver(nsISTSShutdownObserver *aObserver) override { return _to AddShutdownObserver(aObserver); } \
  NS_IMETHOD RemoveShutdownObserver(nsISTSShutdownObserver *aObserver) override { return _to RemoveShutdownObserver(aObserver); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISOCKETTRANSPORTSERVICE(_to) \
  NS_IMETHOD CreateTransport(const nsTArray<nsCString >& aSocketTypes, const nsACString& aHost, int32_t aPort, nsIProxyInfo *aProxyInfo, nsIDNSRecord *dnsRecord, nsISocketTransport **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateTransport(aSocketTypes, aHost, aPort, aProxyInfo, dnsRecord, _retval); } \
  NS_IMETHOD CreateUnixDomainTransport(nsIFile *aPath, nsISocketTransport **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateUnixDomainTransport(aPath, _retval); } \
  NS_IMETHOD CreateUnixDomainAbstractAddressTransport(const nsACString& aName, nsISocketTransport **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateUnixDomainAbstractAddressTransport(aName, _retval); } \
  NS_IMETHOD AttachSocket(PRFileDesc * aFd, nsASocketHandler * aHandler) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AttachSocket(aFd, aHandler); } \
  NS_IMETHOD NotifyWhenCanAttachSocket(nsIRunnable *aEvent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyWhenCanAttachSocket(aEvent); } \
  NS_IMETHOD AddShutdownObserver(nsISTSShutdownObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddShutdownObserver(aObserver); } \
  NS_IMETHOD RemoveShutdownObserver(nsISTSShutdownObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveShutdownObserver(aObserver); } 


/* starting interface:    nsIRoutedSocketTransportService */
#define NS_IROUTEDSOCKETTRANSPORTSERVICE_IID_STR "c5204623-5b58-4a16-8b2e-67c34dd02e3f"

#define NS_IROUTEDSOCKETTRANSPORTSERVICE_IID \
  {0xc5204623, 0x5b58, 0x4a16, \
    { 0x8b, 0x2e, 0x67, 0xc3, 0x4d, 0xd0, 0x2e, 0x3f }}

class NS_NO_VTABLE nsIRoutedSocketTransportService : public nsISocketTransportService {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IROUTEDSOCKETTRANSPORTSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIRoutedSocketTransportService;

  /* nsISocketTransport createRoutedTransport (in Array<ACString> aSocketTypes, in AUTF8String aHost, in long aPort, in AUTF8String aHostRoute, in long aPortRoute, in nsIProxyInfo aProxyInfo, in nsIDNSRecord aDnsRecord); */
  NS_IMETHOD CreateRoutedTransport(const nsTArray<nsCString >& aSocketTypes, const nsACString& aHost, int32_t aPort, const nsACString& aHostRoute, int32_t aPortRoute, nsIProxyInfo *aProxyInfo, nsIDNSRecord *aDnsRecord, nsISocketTransport **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIRoutedSocketTransportService, NS_IROUTEDSOCKETTRANSPORTSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIROUTEDSOCKETTRANSPORTSERVICE \
  NS_IMETHOD CreateRoutedTransport(const nsTArray<nsCString >& aSocketTypes, const nsACString& aHost, int32_t aPort, const nsACString& aHostRoute, int32_t aPortRoute, nsIProxyInfo *aProxyInfo, nsIDNSRecord *aDnsRecord, nsISocketTransport **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIROUTEDSOCKETTRANSPORTSERVICE \
  nsresult CreateRoutedTransport(const nsTArray<nsCString >& aSocketTypes, const nsACString& aHost, int32_t aPort, const nsACString& aHostRoute, int32_t aPortRoute, nsIProxyInfo *aProxyInfo, nsIDNSRecord *aDnsRecord, nsISocketTransport **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIROUTEDSOCKETTRANSPORTSERVICE(_to) \
  NS_IMETHOD CreateRoutedTransport(const nsTArray<nsCString >& aSocketTypes, const nsACString& aHost, int32_t aPort, const nsACString& aHostRoute, int32_t aPortRoute, nsIProxyInfo *aProxyInfo, nsIDNSRecord *aDnsRecord, nsISocketTransport **_retval) override { return _to CreateRoutedTransport(aSocketTypes, aHost, aPort, aHostRoute, aPortRoute, aProxyInfo, aDnsRecord, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIROUTEDSOCKETTRANSPORTSERVICE(_to) \
  NS_IMETHOD CreateRoutedTransport(const nsTArray<nsCString >& aSocketTypes, const nsACString& aHost, int32_t aPort, const nsACString& aHostRoute, int32_t aPortRoute, nsIProxyInfo *aProxyInfo, nsIDNSRecord *aDnsRecord, nsISocketTransport **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateRoutedTransport(aSocketTypes, aHost, aPort, aHostRoute, aPortRoute, aProxyInfo, aDnsRecord, _retval); } 


#endif /* __gen_nsISocketTransportService_h__ */
