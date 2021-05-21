/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsITLSServerSocket.idl
 */

#ifndef __gen_nsITLSServerSocket_h__
#define __gen_nsITLSServerSocket_h__


#ifndef __gen_nsIServerSocket_h__
#include "nsIServerSocket.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIX509Cert; /* forward declaration */

class nsITLSServerSecurityObserver; /* forward declaration */

class nsISocketTransport; /* forward declaration */


/* starting interface:    nsITLSServerSocket */
#define NS_ITLSSERVERSOCKET_IID_STR "cc2c30f9-cfaa-4b8a-bd44-c24881981b74"

#define NS_ITLSSERVERSOCKET_IID \
  {0xcc2c30f9, 0xcfaa, 0x4b8a, \
    { 0xbd, 0x44, 0xc2, 0x48, 0x81, 0x98, 0x1b, 0x74 }}

class NS_NO_VTABLE nsITLSServerSocket : public nsIServerSocket {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITLSSERVERSOCKET_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITLSServerSocket;

  /* attribute nsIX509Cert serverCert; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetServerCert(nsIX509Cert **aServerCert) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetServerCert(nsIX509Cert *aServerCert) = 0;

  /* void setSessionTickets (in boolean aSessionTickets); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSessionTickets(bool aSessionTickets) = 0;

  enum {
    REQUEST_NEVER = 0U,
    REQUEST_FIRST_HANDSHAKE = 1U,
    REQUEST_ALWAYS = 2U,
    REQUIRE_FIRST_HANDSHAKE = 3U,
    REQUIRE_ALWAYS = 4U
  };

  /* void setRequestClientCertificate (in unsigned long aRequestClientCert); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetRequestClientCertificate(uint32_t aRequestClientCert) = 0;

  /* void setVersionRange (in unsigned short aMinVersion, in unsigned short aMaxVersion); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetVersionRange(uint16_t aMinVersion, uint16_t aMaxVersion) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITLSServerSocket, NS_ITLSSERVERSOCKET_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITLSSERVERSOCKET \
  NS_IMETHOD GetServerCert(nsIX509Cert **aServerCert) override; \
  NS_IMETHOD SetServerCert(nsIX509Cert *aServerCert) override; \
  NS_IMETHOD SetSessionTickets(bool aSessionTickets) override; \
  NS_IMETHOD SetRequestClientCertificate(uint32_t aRequestClientCert) override; \
  NS_IMETHOD SetVersionRange(uint16_t aMinVersion, uint16_t aMaxVersion) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITLSSERVERSOCKET \
  nsresult GetServerCert(nsIX509Cert **aServerCert); \
  nsresult SetServerCert(nsIX509Cert *aServerCert); \
  nsresult SetSessionTickets(bool aSessionTickets); \
  nsresult SetRequestClientCertificate(uint32_t aRequestClientCert); \
  nsresult SetVersionRange(uint16_t aMinVersion, uint16_t aMaxVersion); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITLSSERVERSOCKET(_to) \
  NS_IMETHOD GetServerCert(nsIX509Cert **aServerCert) override { return _to GetServerCert(aServerCert); } \
  NS_IMETHOD SetServerCert(nsIX509Cert *aServerCert) override { return _to SetServerCert(aServerCert); } \
  NS_IMETHOD SetSessionTickets(bool aSessionTickets) override { return _to SetSessionTickets(aSessionTickets); } \
  NS_IMETHOD SetRequestClientCertificate(uint32_t aRequestClientCert) override { return _to SetRequestClientCertificate(aRequestClientCert); } \
  NS_IMETHOD SetVersionRange(uint16_t aMinVersion, uint16_t aMaxVersion) override { return _to SetVersionRange(aMinVersion, aMaxVersion); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITLSSERVERSOCKET(_to) \
  NS_IMETHOD GetServerCert(nsIX509Cert **aServerCert) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetServerCert(aServerCert); } \
  NS_IMETHOD SetServerCert(nsIX509Cert *aServerCert) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetServerCert(aServerCert); } \
  NS_IMETHOD SetSessionTickets(bool aSessionTickets) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSessionTickets(aSessionTickets); } \
  NS_IMETHOD SetRequestClientCertificate(uint32_t aRequestClientCert) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRequestClientCertificate(aRequestClientCert); } \
  NS_IMETHOD SetVersionRange(uint16_t aMinVersion, uint16_t aMaxVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetVersionRange(aMinVersion, aMaxVersion); } 


/* starting interface:    nsITLSClientStatus */
#define NS_ITLSCLIENTSTATUS_IID_STR "19668ea4-e5ad-4182-9698-7e890d48f327"

#define NS_ITLSCLIENTSTATUS_IID \
  {0x19668ea4, 0xe5ad, 0x4182, \
    { 0x96, 0x98, 0x7e, 0x89, 0x0d, 0x48, 0xf3, 0x27 }}

class NS_NO_VTABLE nsITLSClientStatus : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITLSCLIENTSTATUS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITLSClientStatus;

  /* readonly attribute nsIX509Cert peerCert; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPeerCert(nsIX509Cert **aPeerCert) = 0;

  enum {
    SSL_VERSION_3 = 768,
    TLS_VERSION_1 = 769,
    TLS_VERSION_1_1 = 770,
    TLS_VERSION_1_2 = 771,
    TLS_VERSION_1_3 = 772,
    TLS_VERSION_UNKNOWN = -1
  };

  /* readonly attribute short tlsVersionUsed; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTlsVersionUsed(int16_t *aTlsVersionUsed) = 0;

  /* readonly attribute ACString cipherName; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCipherName(nsACString& aCipherName) = 0;

  /* readonly attribute unsigned long keyLength; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetKeyLength(uint32_t *aKeyLength) = 0;

  /* readonly attribute unsigned long macLength; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMacLength(uint32_t *aMacLength) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITLSClientStatus, NS_ITLSCLIENTSTATUS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITLSCLIENTSTATUS \
  NS_IMETHOD GetPeerCert(nsIX509Cert **aPeerCert) override; \
  NS_IMETHOD GetTlsVersionUsed(int16_t *aTlsVersionUsed) override; \
  NS_IMETHOD GetCipherName(nsACString& aCipherName) override; \
  NS_IMETHOD GetKeyLength(uint32_t *aKeyLength) override; \
  NS_IMETHOD GetMacLength(uint32_t *aMacLength) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITLSCLIENTSTATUS \
  nsresult GetPeerCert(nsIX509Cert **aPeerCert); \
  nsresult GetTlsVersionUsed(int16_t *aTlsVersionUsed); \
  nsresult GetCipherName(nsACString& aCipherName); \
  nsresult GetKeyLength(uint32_t *aKeyLength); \
  nsresult GetMacLength(uint32_t *aMacLength); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITLSCLIENTSTATUS(_to) \
  NS_IMETHOD GetPeerCert(nsIX509Cert **aPeerCert) override { return _to GetPeerCert(aPeerCert); } \
  NS_IMETHOD GetTlsVersionUsed(int16_t *aTlsVersionUsed) override { return _to GetTlsVersionUsed(aTlsVersionUsed); } \
  NS_IMETHOD GetCipherName(nsACString& aCipherName) override { return _to GetCipherName(aCipherName); } \
  NS_IMETHOD GetKeyLength(uint32_t *aKeyLength) override { return _to GetKeyLength(aKeyLength); } \
  NS_IMETHOD GetMacLength(uint32_t *aMacLength) override { return _to GetMacLength(aMacLength); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITLSCLIENTSTATUS(_to) \
  NS_IMETHOD GetPeerCert(nsIX509Cert **aPeerCert) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPeerCert(aPeerCert); } \
  NS_IMETHOD GetTlsVersionUsed(int16_t *aTlsVersionUsed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTlsVersionUsed(aTlsVersionUsed); } \
  NS_IMETHOD GetCipherName(nsACString& aCipherName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCipherName(aCipherName); } \
  NS_IMETHOD GetKeyLength(uint32_t *aKeyLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetKeyLength(aKeyLength); } \
  NS_IMETHOD GetMacLength(uint32_t *aMacLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMacLength(aMacLength); } 


/* starting interface:    nsITLSServerConnectionInfo */
#define NS_ITLSSERVERCONNECTIONINFO_IID_STR "8a93f5d5-eddd-4c62-a4bd-bfd297653184"

#define NS_ITLSSERVERCONNECTIONINFO_IID \
  {0x8a93f5d5, 0xeddd, 0x4c62, \
    { 0xa4, 0xbd, 0xbf, 0xd2, 0x97, 0x65, 0x31, 0x84 }}

class NS_NO_VTABLE nsITLSServerConnectionInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITLSSERVERCONNECTIONINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITLSServerConnectionInfo;

  /* void setSecurityObserver (in nsITLSServerSecurityObserver observer); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSecurityObserver(nsITLSServerSecurityObserver *observer) = 0;

  /* readonly attribute nsITLSServerSocket serverSocket; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetServerSocket(nsITLSServerSocket **aServerSocket) = 0;

  /* readonly attribute nsITLSClientStatus status; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetStatus(nsITLSClientStatus **aStatus) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITLSServerConnectionInfo, NS_ITLSSERVERCONNECTIONINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITLSSERVERCONNECTIONINFO \
  NS_IMETHOD SetSecurityObserver(nsITLSServerSecurityObserver *observer) override; \
  NS_IMETHOD GetServerSocket(nsITLSServerSocket **aServerSocket) override; \
  NS_IMETHOD GetStatus(nsITLSClientStatus **aStatus) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITLSSERVERCONNECTIONINFO \
  nsresult SetSecurityObserver(nsITLSServerSecurityObserver *observer); \
  nsresult GetServerSocket(nsITLSServerSocket **aServerSocket); \
  nsresult GetStatus(nsITLSClientStatus **aStatus); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITLSSERVERCONNECTIONINFO(_to) \
  NS_IMETHOD SetSecurityObserver(nsITLSServerSecurityObserver *observer) override { return _to SetSecurityObserver(observer); } \
  NS_IMETHOD GetServerSocket(nsITLSServerSocket **aServerSocket) override { return _to GetServerSocket(aServerSocket); } \
  NS_IMETHOD GetStatus(nsITLSClientStatus **aStatus) override { return _to GetStatus(aStatus); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITLSSERVERCONNECTIONINFO(_to) \
  NS_IMETHOD SetSecurityObserver(nsITLSServerSecurityObserver *observer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSecurityObserver(observer); } \
  NS_IMETHOD GetServerSocket(nsITLSServerSocket **aServerSocket) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetServerSocket(aServerSocket); } \
  NS_IMETHOD GetStatus(nsITLSClientStatus **aStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStatus(aStatus); } 


/* starting interface:    nsITLSServerSecurityObserver */
#define NS_ITLSSERVERSECURITYOBSERVER_IID_STR "1f62e1ae-e546-4a38-8917-d428472ed736"

#define NS_ITLSSERVERSECURITYOBSERVER_IID \
  {0x1f62e1ae, 0xe546, 0x4a38, \
    { 0x89, 0x17, 0xd4, 0x28, 0x47, 0x2e, 0xd7, 0x36 }}

class NS_NO_VTABLE nsITLSServerSecurityObserver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITLSSERVERSECURITYOBSERVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITLSServerSecurityObserver;

  /* void onHandshakeDone (in nsITLSServerSocket aServer, in nsITLSClientStatus aStatus); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnHandshakeDone(nsITLSServerSocket *aServer, nsITLSClientStatus *aStatus) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITLSServerSecurityObserver, NS_ITLSSERVERSECURITYOBSERVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITLSSERVERSECURITYOBSERVER \
  NS_IMETHOD OnHandshakeDone(nsITLSServerSocket *aServer, nsITLSClientStatus *aStatus) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITLSSERVERSECURITYOBSERVER \
  nsresult OnHandshakeDone(nsITLSServerSocket *aServer, nsITLSClientStatus *aStatus); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITLSSERVERSECURITYOBSERVER(_to) \
  NS_IMETHOD OnHandshakeDone(nsITLSServerSocket *aServer, nsITLSClientStatus *aStatus) override { return _to OnHandshakeDone(aServer, aStatus); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITLSSERVERSECURITYOBSERVER(_to) \
  NS_IMETHOD OnHandshakeDone(nsITLSServerSocket *aServer, nsITLSClientStatus *aStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnHandshakeDone(aServer, aStatus); } 


#endif /* __gen_nsITLSServerSocket_h__ */
