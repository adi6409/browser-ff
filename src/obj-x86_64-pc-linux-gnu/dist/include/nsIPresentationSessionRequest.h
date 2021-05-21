/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationSessionRequest.idl
 */

#ifndef __gen_nsIPresentationSessionRequest_h__
#define __gen_nsIPresentationSessionRequest_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPresentationDevice; /* forward declaration */

class nsIPresentationControlChannel; /* forward declaration */

#define PRESENTATION_SESSION_REQUEST_TOPIC "presentation-session-request"
#define PRESENTATION_RECONNECT_REQUEST_TOPIC "presentation-reconnect-request"

/* starting interface:    nsIPresentationSessionRequest */
#define NS_IPRESENTATIONSESSIONREQUEST_IID_STR "d808a084-d0f8-455a-a8df-5879e05a755b"

#define NS_IPRESENTATIONSESSIONREQUEST_IID \
  {0xd808a084, 0xd0f8, 0x455a, \
    { 0xa8, 0xdf, 0x58, 0x79, 0xe0, 0x5a, 0x75, 0x5b }}

class NS_NO_VTABLE nsIPresentationSessionRequest : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRESENTATIONSESSIONREQUEST_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPresentationSessionRequest;

  /* readonly attribute nsIPresentationDevice device; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDevice(nsIPresentationDevice **aDevice) = 0;

  /* readonly attribute AString url; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUrl(nsAString& aUrl) = 0;

  /* readonly attribute AString presentationId; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPresentationId(nsAString& aPresentationId) = 0;

  /* readonly attribute nsIPresentationControlChannel controlChannel; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetControlChannel(nsIPresentationControlChannel **aControlChannel) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPresentationSessionRequest, NS_IPRESENTATIONSESSIONREQUEST_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRESENTATIONSESSIONREQUEST \
  NS_IMETHOD GetDevice(nsIPresentationDevice **aDevice) override; \
  NS_IMETHOD GetUrl(nsAString& aUrl) override; \
  NS_IMETHOD GetPresentationId(nsAString& aPresentationId) override; \
  NS_IMETHOD GetControlChannel(nsIPresentationControlChannel **aControlChannel) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRESENTATIONSESSIONREQUEST \
  nsresult GetDevice(nsIPresentationDevice **aDevice); \
  nsresult GetUrl(nsAString& aUrl); \
  nsresult GetPresentationId(nsAString& aPresentationId); \
  nsresult GetControlChannel(nsIPresentationControlChannel **aControlChannel); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRESENTATIONSESSIONREQUEST(_to) \
  NS_IMETHOD GetDevice(nsIPresentationDevice **aDevice) override { return _to GetDevice(aDevice); } \
  NS_IMETHOD GetUrl(nsAString& aUrl) override { return _to GetUrl(aUrl); } \
  NS_IMETHOD GetPresentationId(nsAString& aPresentationId) override { return _to GetPresentationId(aPresentationId); } \
  NS_IMETHOD GetControlChannel(nsIPresentationControlChannel **aControlChannel) override { return _to GetControlChannel(aControlChannel); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRESENTATIONSESSIONREQUEST(_to) \
  NS_IMETHOD GetDevice(nsIPresentationDevice **aDevice) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDevice(aDevice); } \
  NS_IMETHOD GetUrl(nsAString& aUrl) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUrl(aUrl); } \
  NS_IMETHOD GetPresentationId(nsAString& aPresentationId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPresentationId(aPresentationId); } \
  NS_IMETHOD GetControlChannel(nsIPresentationControlChannel **aControlChannel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetControlChannel(aControlChannel); } 


#endif /* __gen_nsIPresentationSessionRequest_h__ */
