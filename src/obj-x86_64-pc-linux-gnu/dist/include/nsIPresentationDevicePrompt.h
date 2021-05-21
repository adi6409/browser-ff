/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationDevicePrompt.idl
 */

#ifndef __gen_nsIPresentationDevicePrompt_h__
#define __gen_nsIPresentationDevicePrompt_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIArray; /* forward declaration */

class nsIPresentationDevice; /* forward declaration */

class nsIPrincipal; /* forward declaration */

namespace mozilla {
namespace dom {
class EventTarget; /* webidl EventTarget */
} // namespace dom
} // namespace mozilla

#define PRESENTATION_DEVICE_PROMPT_CONTRACTID "@mozilla.org/presentation-device/prompt;1"

/* starting interface:    nsIPresentationDeviceRequest */
#define NS_IPRESENTATIONDEVICEREQUEST_IID_STR "b2aa7f6a-9448-446a-bba4-9c29638b0ed4"

#define NS_IPRESENTATIONDEVICEREQUEST_IID \
  {0xb2aa7f6a, 0x9448, 0x446a, \
    { 0xbb, 0xa4, 0x9c, 0x29, 0x63, 0x8b, 0x0e, 0xd4 }}

class NS_NO_VTABLE nsIPresentationDeviceRequest : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRESENTATIONDEVICEREQUEST_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPresentationDeviceRequest;

  /* readonly attribute AString origin; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOrigin(nsAString& aOrigin) = 0;

  /* readonly attribute nsIArray requestURLs; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRequestURLs(nsIArray **aRequestURLs) = 0;

  /* readonly attribute EventTarget chromeEventHandler; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetChromeEventHandler(mozilla::dom::EventTarget **aChromeEventHandler) = 0;

  /* readonly attribute nsIPrincipal principal; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) = 0;

  /* void select (in nsIPresentationDevice device); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Select(nsIPresentationDevice *device) = 0;

  /* void cancel (in nsresult reason); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Cancel(nsresult reason) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPresentationDeviceRequest, NS_IPRESENTATIONDEVICEREQUEST_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRESENTATIONDEVICEREQUEST \
  NS_IMETHOD GetOrigin(nsAString& aOrigin) override; \
  NS_IMETHOD GetRequestURLs(nsIArray **aRequestURLs) override; \
  NS_IMETHOD GetChromeEventHandler(mozilla::dom::EventTarget **aChromeEventHandler) override; \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override; \
  NS_IMETHOD Select(nsIPresentationDevice *device) override; \
  NS_IMETHOD Cancel(nsresult reason) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRESENTATIONDEVICEREQUEST \
  nsresult GetOrigin(nsAString& aOrigin); \
  nsresult GetRequestURLs(nsIArray **aRequestURLs); \
  nsresult GetChromeEventHandler(mozilla::dom::EventTarget **aChromeEventHandler); \
  nsresult GetPrincipal(nsIPrincipal **aPrincipal); \
  nsresult Select(nsIPresentationDevice *device); \
  nsresult Cancel(nsresult reason); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRESENTATIONDEVICEREQUEST(_to) \
  NS_IMETHOD GetOrigin(nsAString& aOrigin) override { return _to GetOrigin(aOrigin); } \
  NS_IMETHOD GetRequestURLs(nsIArray **aRequestURLs) override { return _to GetRequestURLs(aRequestURLs); } \
  NS_IMETHOD GetChromeEventHandler(mozilla::dom::EventTarget **aChromeEventHandler) override { return _to GetChromeEventHandler(aChromeEventHandler); } \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override { return _to GetPrincipal(aPrincipal); } \
  NS_IMETHOD Select(nsIPresentationDevice *device) override { return _to Select(device); } \
  NS_IMETHOD Cancel(nsresult reason) override { return _to Cancel(reason); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRESENTATIONDEVICEREQUEST(_to) \
  NS_IMETHOD GetOrigin(nsAString& aOrigin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOrigin(aOrigin); } \
  NS_IMETHOD GetRequestURLs(nsIArray **aRequestURLs) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRequestURLs(aRequestURLs); } \
  NS_IMETHOD GetChromeEventHandler(mozilla::dom::EventTarget **aChromeEventHandler) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChromeEventHandler(aChromeEventHandler); } \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrincipal(aPrincipal); } \
  NS_IMETHOD Select(nsIPresentationDevice *device) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Select(device); } \
  NS_IMETHOD Cancel(nsresult reason) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Cancel(reason); } 


/* starting interface:    nsIPresentationDevicePrompt */
#define NS_IPRESENTATIONDEVICEPROMPT_IID_STR "ac1a7e44-de86-454f-a9f1-276de2539831"

#define NS_IPRESENTATIONDEVICEPROMPT_IID \
  {0xac1a7e44, 0xde86, 0x454f, \
    { 0xa9, 0xf1, 0x27, 0x6d, 0xe2, 0x53, 0x98, 0x31 }}

class NS_NO_VTABLE nsIPresentationDevicePrompt : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRESENTATIONDEVICEPROMPT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPresentationDevicePrompt;

  /* void promptDeviceSelection (in nsIPresentationDeviceRequest request); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PromptDeviceSelection(nsIPresentationDeviceRequest *request) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPresentationDevicePrompt, NS_IPRESENTATIONDEVICEPROMPT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRESENTATIONDEVICEPROMPT \
  NS_IMETHOD PromptDeviceSelection(nsIPresentationDeviceRequest *request) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRESENTATIONDEVICEPROMPT \
  nsresult PromptDeviceSelection(nsIPresentationDeviceRequest *request); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRESENTATIONDEVICEPROMPT(_to) \
  NS_IMETHOD PromptDeviceSelection(nsIPresentationDeviceRequest *request) override { return _to PromptDeviceSelection(request); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRESENTATIONDEVICEPROMPT(_to) \
  NS_IMETHOD PromptDeviceSelection(nsIPresentationDeviceRequest *request) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PromptDeviceSelection(request); } 


#endif /* __gen_nsIPresentationDevicePrompt_h__ */
