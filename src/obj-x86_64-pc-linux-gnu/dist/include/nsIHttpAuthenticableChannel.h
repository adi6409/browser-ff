/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIHttpAuthenticableChannel.idl
 */

#ifndef __gen_nsIHttpAuthenticableChannel_h__
#define __gen_nsIHttpAuthenticableChannel_h__


#ifndef __gen_nsIProxiedChannel_h__
#include "nsIProxiedChannel.h"
#endif

#ifndef __gen_nsIRequest_h__
#include "nsIRequest.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsILoadGroup; /* forward declaration */

class nsIURI; /* forward declaration */

class nsIInterfaceRequestor; /* forward declaration */


/* starting interface:    nsIHttpAuthenticableChannel */
#define NS_IHTTPAUTHENTICABLECHANNEL_IID_STR "701093ac-5c7f-429c-99e3-423b041fccb4"

#define NS_IHTTPAUTHENTICABLECHANNEL_IID \
  {0x701093ac, 0x5c7f, 0x429c, \
    { 0x99, 0xe3, 0x42, 0x3b, 0x04, 0x1f, 0xcc, 0xb4 }}

class NS_NO_VTABLE nsIHttpAuthenticableChannel : public nsIProxiedChannel {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHTTPAUTHENTICABLECHANNEL_IID)

  /* [must_use] readonly attribute boolean isSSL; */
  [[nodiscard]] NS_IMETHOD GetIsSSL(bool *aIsSSL) = 0;

  /* [must_use] readonly attribute boolean proxyMethodIsConnect; */
  [[nodiscard]] NS_IMETHOD GetProxyMethodIsConnect(bool *aProxyMethodIsConnect) = 0;

  /* [must_use] void cancel (in nsresult aStatus); */
  [[nodiscard]] NS_IMETHOD Cancel(nsresult aStatus) = 0;

  /* [must_use] readonly attribute nsLoadFlags loadFlags; */
  [[nodiscard]] NS_IMETHOD GetLoadFlags(nsLoadFlags *aLoadFlags) = 0;

  /* [must_use] readonly attribute nsIURI URI; */
  [[nodiscard]] NS_IMETHOD GetURI(nsIURI **aURI) = 0;

  /* [must_use] readonly attribute nsILoadGroup loadGroup; */
  [[nodiscard]] NS_IMETHOD GetLoadGroup(nsILoadGroup **aLoadGroup) = 0;

  /* [must_use] readonly attribute nsIInterfaceRequestor notificationCallbacks; */
  [[nodiscard]] NS_IMETHOD GetNotificationCallbacks(nsIInterfaceRequestor **aNotificationCallbacks) = 0;

  /* [must_use] readonly attribute ACString requestMethod; */
  [[nodiscard]] NS_IMETHOD GetRequestMethod(nsACString& aRequestMethod) = 0;

  /* [must_use] readonly attribute ACString serverResponseHeader; */
  [[nodiscard]] NS_IMETHOD GetServerResponseHeader(nsACString& aServerResponseHeader) = 0;

  /* [must_use] readonly attribute ACString proxyChallenges; */
  [[nodiscard]] NS_IMETHOD GetProxyChallenges(nsACString& aProxyChallenges) = 0;

  /* [must_use] readonly attribute ACString WWWChallenges; */
  [[nodiscard]] NS_IMETHOD GetWWWChallenges(nsACString& aWWWChallenges) = 0;

  /* [must_use] void setProxyCredentials (in ACString credentials); */
  [[nodiscard]] NS_IMETHOD SetProxyCredentials(const nsACString& credentials) = 0;

  /* [must_use] void setWWWCredentials (in ACString credentials); */
  [[nodiscard]] NS_IMETHOD SetWWWCredentials(const nsACString& credentials) = 0;

  /* [must_use] void onAuthAvailable (); */
  [[nodiscard]] NS_IMETHOD OnAuthAvailable(void) = 0;

  /* [must_use] void onAuthCancelled (in boolean userCancel); */
  [[nodiscard]] NS_IMETHOD OnAuthCancelled(bool userCancel) = 0;

  /* [must_use] void closeStickyConnection (); */
  [[nodiscard]] NS_IMETHOD CloseStickyConnection(void) = 0;

  /* void connectionRestartable (in boolean restartable); */
  NS_IMETHOD ConnectionRestartable(bool restartable) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHttpAuthenticableChannel, NS_IHTTPAUTHENTICABLECHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHTTPAUTHENTICABLECHANNEL \
  [[nodiscard]] NS_IMETHOD GetIsSSL(bool *aIsSSL) override; \
  [[nodiscard]] NS_IMETHOD GetProxyMethodIsConnect(bool *aProxyMethodIsConnect) override; \
  [[nodiscard]] NS_IMETHOD Cancel(nsresult aStatus) override; \
  [[nodiscard]] NS_IMETHOD GetLoadFlags(nsLoadFlags *aLoadFlags) override; \
  [[nodiscard]] NS_IMETHOD GetURI(nsIURI **aURI) override; \
  [[nodiscard]] NS_IMETHOD GetLoadGroup(nsILoadGroup **aLoadGroup) override; \
  [[nodiscard]] NS_IMETHOD GetNotificationCallbacks(nsIInterfaceRequestor **aNotificationCallbacks) override; \
  [[nodiscard]] NS_IMETHOD GetRequestMethod(nsACString& aRequestMethod) override; \
  [[nodiscard]] NS_IMETHOD GetServerResponseHeader(nsACString& aServerResponseHeader) override; \
  [[nodiscard]] NS_IMETHOD GetProxyChallenges(nsACString& aProxyChallenges) override; \
  [[nodiscard]] NS_IMETHOD GetWWWChallenges(nsACString& aWWWChallenges) override; \
  [[nodiscard]] NS_IMETHOD SetProxyCredentials(const nsACString& credentials) override; \
  [[nodiscard]] NS_IMETHOD SetWWWCredentials(const nsACString& credentials) override; \
  [[nodiscard]] NS_IMETHOD OnAuthAvailable(void) override; \
  [[nodiscard]] NS_IMETHOD OnAuthCancelled(bool userCancel) override; \
  [[nodiscard]] NS_IMETHOD CloseStickyConnection(void) override; \
  NS_IMETHOD ConnectionRestartable(bool restartable) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHTTPAUTHENTICABLECHANNEL \
  [[nodiscard]] nsresult GetIsSSL(bool *aIsSSL); \
  [[nodiscard]] nsresult GetProxyMethodIsConnect(bool *aProxyMethodIsConnect); \
  [[nodiscard]] nsresult Cancel(nsresult aStatus); \
  [[nodiscard]] nsresult GetLoadFlags(nsLoadFlags *aLoadFlags); \
  [[nodiscard]] nsresult GetURI(nsIURI **aURI); \
  [[nodiscard]] nsresult GetLoadGroup(nsILoadGroup **aLoadGroup); \
  [[nodiscard]] nsresult GetNotificationCallbacks(nsIInterfaceRequestor **aNotificationCallbacks); \
  [[nodiscard]] nsresult GetRequestMethod(nsACString& aRequestMethod); \
  [[nodiscard]] nsresult GetServerResponseHeader(nsACString& aServerResponseHeader); \
  [[nodiscard]] nsresult GetProxyChallenges(nsACString& aProxyChallenges); \
  [[nodiscard]] nsresult GetWWWChallenges(nsACString& aWWWChallenges); \
  [[nodiscard]] nsresult SetProxyCredentials(const nsACString& credentials); \
  [[nodiscard]] nsresult SetWWWCredentials(const nsACString& credentials); \
  [[nodiscard]] nsresult OnAuthAvailable(void); \
  [[nodiscard]] nsresult OnAuthCancelled(bool userCancel); \
  [[nodiscard]] nsresult CloseStickyConnection(void); \
  nsresult ConnectionRestartable(bool restartable); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHTTPAUTHENTICABLECHANNEL(_to) \
  [[nodiscard]] NS_IMETHOD GetIsSSL(bool *aIsSSL) override { return _to GetIsSSL(aIsSSL); } \
  [[nodiscard]] NS_IMETHOD GetProxyMethodIsConnect(bool *aProxyMethodIsConnect) override { return _to GetProxyMethodIsConnect(aProxyMethodIsConnect); } \
  [[nodiscard]] NS_IMETHOD Cancel(nsresult aStatus) override { return _to Cancel(aStatus); } \
  [[nodiscard]] NS_IMETHOD GetLoadFlags(nsLoadFlags *aLoadFlags) override { return _to GetLoadFlags(aLoadFlags); } \
  [[nodiscard]] NS_IMETHOD GetURI(nsIURI **aURI) override { return _to GetURI(aURI); } \
  [[nodiscard]] NS_IMETHOD GetLoadGroup(nsILoadGroup **aLoadGroup) override { return _to GetLoadGroup(aLoadGroup); } \
  [[nodiscard]] NS_IMETHOD GetNotificationCallbacks(nsIInterfaceRequestor **aNotificationCallbacks) override { return _to GetNotificationCallbacks(aNotificationCallbacks); } \
  [[nodiscard]] NS_IMETHOD GetRequestMethod(nsACString& aRequestMethod) override { return _to GetRequestMethod(aRequestMethod); } \
  [[nodiscard]] NS_IMETHOD GetServerResponseHeader(nsACString& aServerResponseHeader) override { return _to GetServerResponseHeader(aServerResponseHeader); } \
  [[nodiscard]] NS_IMETHOD GetProxyChallenges(nsACString& aProxyChallenges) override { return _to GetProxyChallenges(aProxyChallenges); } \
  [[nodiscard]] NS_IMETHOD GetWWWChallenges(nsACString& aWWWChallenges) override { return _to GetWWWChallenges(aWWWChallenges); } \
  [[nodiscard]] NS_IMETHOD SetProxyCredentials(const nsACString& credentials) override { return _to SetProxyCredentials(credentials); } \
  [[nodiscard]] NS_IMETHOD SetWWWCredentials(const nsACString& credentials) override { return _to SetWWWCredentials(credentials); } \
  [[nodiscard]] NS_IMETHOD OnAuthAvailable(void) override { return _to OnAuthAvailable(); } \
  [[nodiscard]] NS_IMETHOD OnAuthCancelled(bool userCancel) override { return _to OnAuthCancelled(userCancel); } \
  [[nodiscard]] NS_IMETHOD CloseStickyConnection(void) override { return _to CloseStickyConnection(); } \
  NS_IMETHOD ConnectionRestartable(bool restartable) override { return _to ConnectionRestartable(restartable); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHTTPAUTHENTICABLECHANNEL(_to) \
  [[nodiscard]] NS_IMETHOD GetIsSSL(bool *aIsSSL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsSSL(aIsSSL); } \
  [[nodiscard]] NS_IMETHOD GetProxyMethodIsConnect(bool *aProxyMethodIsConnect) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProxyMethodIsConnect(aProxyMethodIsConnect); } \
  [[nodiscard]] NS_IMETHOD Cancel(nsresult aStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Cancel(aStatus); } \
  [[nodiscard]] NS_IMETHOD GetLoadFlags(nsLoadFlags *aLoadFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadFlags(aLoadFlags); } \
  [[nodiscard]] NS_IMETHOD GetURI(nsIURI **aURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetURI(aURI); } \
  [[nodiscard]] NS_IMETHOD GetLoadGroup(nsILoadGroup **aLoadGroup) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadGroup(aLoadGroup); } \
  [[nodiscard]] NS_IMETHOD GetNotificationCallbacks(nsIInterfaceRequestor **aNotificationCallbacks) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNotificationCallbacks(aNotificationCallbacks); } \
  [[nodiscard]] NS_IMETHOD GetRequestMethod(nsACString& aRequestMethod) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRequestMethod(aRequestMethod); } \
  [[nodiscard]] NS_IMETHOD GetServerResponseHeader(nsACString& aServerResponseHeader) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetServerResponseHeader(aServerResponseHeader); } \
  [[nodiscard]] NS_IMETHOD GetProxyChallenges(nsACString& aProxyChallenges) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProxyChallenges(aProxyChallenges); } \
  [[nodiscard]] NS_IMETHOD GetWWWChallenges(nsACString& aWWWChallenges) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWWWChallenges(aWWWChallenges); } \
  [[nodiscard]] NS_IMETHOD SetProxyCredentials(const nsACString& credentials) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetProxyCredentials(credentials); } \
  [[nodiscard]] NS_IMETHOD SetWWWCredentials(const nsACString& credentials) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetWWWCredentials(credentials); } \
  [[nodiscard]] NS_IMETHOD OnAuthAvailable(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnAuthAvailable(); } \
  [[nodiscard]] NS_IMETHOD OnAuthCancelled(bool userCancel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnAuthCancelled(userCancel); } \
  [[nodiscard]] NS_IMETHOD CloseStickyConnection(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CloseStickyConnection(); } \
  NS_IMETHOD ConnectionRestartable(bool restartable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ConnectionRestartable(restartable); } 


#endif /* __gen_nsIHttpAuthenticableChannel_h__ */
