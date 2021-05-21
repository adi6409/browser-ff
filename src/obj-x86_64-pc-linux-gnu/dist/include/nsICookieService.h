/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cookie/nsICookieService.idl
 */

#ifndef __gen_nsICookieService_h__
#define __gen_nsICookieService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsIChannel; /* forward declaration */

namespace mozilla {
namespace dom {
class Document; /* webidl Document */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsICookieTransactionCallback */
#define NS_ICOOKIETRANSACTIONCALLBACK_IID_STR "0fc41ffb-f1b7-42d9-9a42-8dc420c158c1"

#define NS_ICOOKIETRANSACTIONCALLBACK_IID \
  {0x0fc41ffb, 0xf1b7, 0x42d9, \
    { 0x9a, 0x42, 0x8d, 0xc4, 0x20, 0xc1, 0x58, 0xc1 }}

class NS_NO_VTABLE nsICookieTransactionCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICOOKIETRANSACTIONCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICookieTransactionCallback;

  /* void callback (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Callback(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICookieTransactionCallback, NS_ICOOKIETRANSACTIONCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICOOKIETRANSACTIONCALLBACK \
  NS_IMETHOD Callback(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICOOKIETRANSACTIONCALLBACK \
  nsresult Callback(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICOOKIETRANSACTIONCALLBACK(_to) \
  NS_IMETHOD Callback(void) override { return _to Callback(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICOOKIETRANSACTIONCALLBACK(_to) \
  NS_IMETHOD Callback(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Callback(); } 


/* starting interface:    nsICookieService */
#define NS_ICOOKIESERVICE_IID_STR "1e94e283-2811-4f43-b947-d22b1549d824"

#define NS_ICOOKIESERVICE_IID \
  {0x1e94e283, 0x2811, 0x4f43, \
    { 0xb9, 0x47, 0xd2, 0x2b, 0x15, 0x49, 0xd8, 0x24 }}

class NS_NO_VTABLE nsICookieService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICOOKIESERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICookieService;

  enum {
    BEHAVIOR_ACCEPT = 0U,
    BEHAVIOR_REJECT_FOREIGN = 1U,
    BEHAVIOR_REJECT = 2U,
    BEHAVIOR_LIMIT_FOREIGN = 3U,
    BEHAVIOR_REJECT_TRACKER = 4U,
    BEHAVIOR_REJECT_TRACKER_AND_PARTITION_FOREIGN = 5U,
    BEHAVIOR_LAST = 5U,
    ACCEPT_NORMALLY = 0U,
    ACCEPT_SESSION = 2U
  };

  /* ACString getCookieStringFromDocument (in Document aDocument); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCookieStringFromDocument(mozilla::dom::Document *aDocument, nsACString& _retval) = 0;

  /* ACString getCookieStringFromHttp (in nsIURI aURI, in nsIChannel aChannel); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCookieStringFromHttp(nsIURI *aURI, nsIChannel *aChannel, nsACString& _retval) = 0;

  /* void setCookieStringFromDocument (in Document aDocument, in ACString aCookie); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCookieStringFromDocument(mozilla::dom::Document *aDocument, const nsACString& aCookie) = 0;

  /* void setCookieStringFromHttp (in nsIURI aURI, in ACString aCookie, in nsIChannel aChannel); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCookieStringFromHttp(nsIURI *aURI, const nsACString& aCookie, nsIChannel *aChannel) = 0;

  /* void runInTransaction (in nsICookieTransactionCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RunInTransaction(nsICookieTransactionCallback *aCallback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICookieService, NS_ICOOKIESERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICOOKIESERVICE \
  NS_IMETHOD GetCookieStringFromDocument(mozilla::dom::Document *aDocument, nsACString& _retval) override; \
  NS_IMETHOD GetCookieStringFromHttp(nsIURI *aURI, nsIChannel *aChannel, nsACString& _retval) override; \
  NS_IMETHOD SetCookieStringFromDocument(mozilla::dom::Document *aDocument, const nsACString& aCookie) override; \
  NS_IMETHOD SetCookieStringFromHttp(nsIURI *aURI, const nsACString& aCookie, nsIChannel *aChannel) override; \
  NS_IMETHOD RunInTransaction(nsICookieTransactionCallback *aCallback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICOOKIESERVICE \
  nsresult GetCookieStringFromDocument(mozilla::dom::Document *aDocument, nsACString& _retval); \
  nsresult GetCookieStringFromHttp(nsIURI *aURI, nsIChannel *aChannel, nsACString& _retval); \
  nsresult SetCookieStringFromDocument(mozilla::dom::Document *aDocument, const nsACString& aCookie); \
  nsresult SetCookieStringFromHttp(nsIURI *aURI, const nsACString& aCookie, nsIChannel *aChannel); \
  nsresult RunInTransaction(nsICookieTransactionCallback *aCallback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICOOKIESERVICE(_to) \
  NS_IMETHOD GetCookieStringFromDocument(mozilla::dom::Document *aDocument, nsACString& _retval) override { return _to GetCookieStringFromDocument(aDocument, _retval); } \
  NS_IMETHOD GetCookieStringFromHttp(nsIURI *aURI, nsIChannel *aChannel, nsACString& _retval) override { return _to GetCookieStringFromHttp(aURI, aChannel, _retval); } \
  NS_IMETHOD SetCookieStringFromDocument(mozilla::dom::Document *aDocument, const nsACString& aCookie) override { return _to SetCookieStringFromDocument(aDocument, aCookie); } \
  NS_IMETHOD SetCookieStringFromHttp(nsIURI *aURI, const nsACString& aCookie, nsIChannel *aChannel) override { return _to SetCookieStringFromHttp(aURI, aCookie, aChannel); } \
  NS_IMETHOD RunInTransaction(nsICookieTransactionCallback *aCallback) override { return _to RunInTransaction(aCallback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICOOKIESERVICE(_to) \
  NS_IMETHOD GetCookieStringFromDocument(mozilla::dom::Document *aDocument, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCookieStringFromDocument(aDocument, _retval); } \
  NS_IMETHOD GetCookieStringFromHttp(nsIURI *aURI, nsIChannel *aChannel, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCookieStringFromHttp(aURI, aChannel, _retval); } \
  NS_IMETHOD SetCookieStringFromDocument(mozilla::dom::Document *aDocument, const nsACString& aCookie) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCookieStringFromDocument(aDocument, aCookie); } \
  NS_IMETHOD SetCookieStringFromHttp(nsIURI *aURI, const nsACString& aCookie, nsIChannel *aChannel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCookieStringFromHttp(aURI, aCookie, aChannel); } \
  NS_IMETHOD RunInTransaction(nsICookieTransactionCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RunInTransaction(aCallback); } 


#endif /* __gen_nsICookieService_h__ */
