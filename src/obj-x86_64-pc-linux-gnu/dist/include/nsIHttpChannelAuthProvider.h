/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIHttpChannelAuthProvider.idl
 */

#ifndef __gen_nsIHttpChannelAuthProvider_h__
#define __gen_nsIHttpChannelAuthProvider_h__


#ifndef __gen_nsICancelable_h__
#include "nsICancelable.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIHttpChannel; /* forward declaration */

class nsIHttpAuthenticableChannel; /* forward declaration */


/* starting interface:    nsIHttpChannelAuthProvider */
#define NS_IHTTPCHANNELAUTHPROVIDER_IID_STR "788f331b-2e1f-436c-b405-4f88a31a105b"

#define NS_IHTTPCHANNELAUTHPROVIDER_IID \
  {0x788f331b, 0x2e1f, 0x436c, \
    { 0xb4, 0x05, 0x4f, 0x88, 0xa3, 0x1a, 0x10, 0x5b }}

class NS_NO_VTABLE nsIHttpChannelAuthProvider : public nsICancelable {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHTTPCHANNELAUTHPROVIDER_IID)

  /* [must_use] void init (in nsIHttpAuthenticableChannel channel); */
  [[nodiscard]] NS_IMETHOD Init(nsIHttpAuthenticableChannel *channel) = 0;

  /* [must_use] void processAuthentication (in unsigned long httpStatus, in boolean sslConnectFailed); */
  [[nodiscard]] NS_IMETHOD ProcessAuthentication(uint32_t httpStatus, bool sslConnectFailed) = 0;

  /* [must_use] void addAuthorizationHeaders (in boolean dontUseCachedWWWCreds); */
  [[nodiscard]] NS_IMETHOD AddAuthorizationHeaders(bool dontUseCachedWWWCreds) = 0;

  /* [must_use] void checkForSuperfluousAuth (); */
  [[nodiscard]] NS_IMETHOD CheckForSuperfluousAuth(void) = 0;

  /* [must_use] void disconnect (in nsresult status); */
  [[nodiscard]] NS_IMETHOD Disconnect(nsresult status) = 0;

  /* void clearProxyIdent (); */
  NS_IMETHOD ClearProxyIdent(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHttpChannelAuthProvider, NS_IHTTPCHANNELAUTHPROVIDER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHTTPCHANNELAUTHPROVIDER \
  [[nodiscard]] NS_IMETHOD Init(nsIHttpAuthenticableChannel *channel) override; \
  [[nodiscard]] NS_IMETHOD ProcessAuthentication(uint32_t httpStatus, bool sslConnectFailed) override; \
  [[nodiscard]] NS_IMETHOD AddAuthorizationHeaders(bool dontUseCachedWWWCreds) override; \
  [[nodiscard]] NS_IMETHOD CheckForSuperfluousAuth(void) override; \
  [[nodiscard]] NS_IMETHOD Disconnect(nsresult status) override; \
  NS_IMETHOD ClearProxyIdent(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHTTPCHANNELAUTHPROVIDER \
  [[nodiscard]] nsresult Init(nsIHttpAuthenticableChannel *channel); \
  [[nodiscard]] nsresult ProcessAuthentication(uint32_t httpStatus, bool sslConnectFailed); \
  [[nodiscard]] nsresult AddAuthorizationHeaders(bool dontUseCachedWWWCreds); \
  [[nodiscard]] nsresult CheckForSuperfluousAuth(void); \
  [[nodiscard]] nsresult Disconnect(nsresult status); \
  nsresult ClearProxyIdent(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHTTPCHANNELAUTHPROVIDER(_to) \
  [[nodiscard]] NS_IMETHOD Init(nsIHttpAuthenticableChannel *channel) override { return _to Init(channel); } \
  [[nodiscard]] NS_IMETHOD ProcessAuthentication(uint32_t httpStatus, bool sslConnectFailed) override { return _to ProcessAuthentication(httpStatus, sslConnectFailed); } \
  [[nodiscard]] NS_IMETHOD AddAuthorizationHeaders(bool dontUseCachedWWWCreds) override { return _to AddAuthorizationHeaders(dontUseCachedWWWCreds); } \
  [[nodiscard]] NS_IMETHOD CheckForSuperfluousAuth(void) override { return _to CheckForSuperfluousAuth(); } \
  [[nodiscard]] NS_IMETHOD Disconnect(nsresult status) override { return _to Disconnect(status); } \
  NS_IMETHOD ClearProxyIdent(void) override { return _to ClearProxyIdent(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHTTPCHANNELAUTHPROVIDER(_to) \
  [[nodiscard]] NS_IMETHOD Init(nsIHttpAuthenticableChannel *channel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(channel); } \
  [[nodiscard]] NS_IMETHOD ProcessAuthentication(uint32_t httpStatus, bool sslConnectFailed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ProcessAuthentication(httpStatus, sslConnectFailed); } \
  [[nodiscard]] NS_IMETHOD AddAuthorizationHeaders(bool dontUseCachedWWWCreds) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddAuthorizationHeaders(dontUseCachedWWWCreds); } \
  [[nodiscard]] NS_IMETHOD CheckForSuperfluousAuth(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CheckForSuperfluousAuth(); } \
  [[nodiscard]] NS_IMETHOD Disconnect(nsresult status) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Disconnect(status); } \
  NS_IMETHOD ClearProxyIdent(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearProxyIdent(); } 


#endif /* __gen_nsIHttpChannelAuthProvider_h__ */
