/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationRequestUIGlue.idl
 */

#ifndef __gen_nsIPresentationRequestUIGlue_h__
#define __gen_nsIPresentationRequestUIGlue_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPresentationDevice; /* forward declaration */

#define PRESENTATION_REQUEST_UI_GLUE_CONTRACTID \
  "@mozilla.org/presentation/requestuiglue;1"

/* starting interface:    nsIPresentationRequestUIGlue */
#define NS_IPRESENTATIONREQUESTUIGLUE_IID_STR "faa45119-6fb5-496c-aa4c-f740177a38b5"

#define NS_IPRESENTATIONREQUESTUIGLUE_IID \
  {0xfaa45119, 0x6fb5, 0x496c, \
    { 0xaa, 0x4c, 0xf7, 0x40, 0x17, 0x7a, 0x38, 0xb5 }}

class NS_NO_VTABLE nsIPresentationRequestUIGlue : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRESENTATIONREQUESTUIGLUE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPresentationRequestUIGlue;

  /* Promise sendRequest (in AString url, in AString sessionId, in nsIPresentationDevice device); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SendRequest(const nsAString& url, const nsAString& sessionId, nsIPresentationDevice *device, ::mozilla::dom::Promise * * _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPresentationRequestUIGlue, NS_IPRESENTATIONREQUESTUIGLUE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRESENTATIONREQUESTUIGLUE \
  NS_IMETHOD SendRequest(const nsAString& url, const nsAString& sessionId, nsIPresentationDevice *device, ::mozilla::dom::Promise * * _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRESENTATIONREQUESTUIGLUE \
  nsresult SendRequest(const nsAString& url, const nsAString& sessionId, nsIPresentationDevice *device, ::mozilla::dom::Promise * * _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRESENTATIONREQUESTUIGLUE(_to) \
  NS_IMETHOD SendRequest(const nsAString& url, const nsAString& sessionId, nsIPresentationDevice *device, ::mozilla::dom::Promise * * _retval) override { return _to SendRequest(url, sessionId, device, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRESENTATIONREQUESTUIGLUE(_to) \
  NS_IMETHOD SendRequest(const nsAString& url, const nsAString& sessionId, nsIPresentationDevice *device, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendRequest(url, sessionId, device, _retval); } 


#endif /* __gen_nsIPresentationRequestUIGlue_h__ */
