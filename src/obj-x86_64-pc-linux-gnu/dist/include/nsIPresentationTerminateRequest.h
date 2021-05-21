/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationTerminateRequest.idl
 */

#ifndef __gen_nsIPresentationTerminateRequest_h__
#define __gen_nsIPresentationTerminateRequest_h__


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

#define PRESENTATION_TERMINATE_REQUEST_TOPIC "presentation-terminate-request"

/* starting interface:    nsIPresentationTerminateRequest */
#define NS_IPRESENTATIONTERMINATEREQUEST_IID_STR "3ddbf3a4-53ee-4b70-9bbc-58ac90dce6b5"

#define NS_IPRESENTATIONTERMINATEREQUEST_IID \
  {0x3ddbf3a4, 0x53ee, 0x4b70, \
    { 0x9b, 0xbc, 0x58, 0xac, 0x90, 0xdc, 0xe6, 0xb5 }}

class NS_NO_VTABLE nsIPresentationTerminateRequest : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRESENTATIONTERMINATEREQUEST_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPresentationTerminateRequest;

  /* readonly attribute nsIPresentationDevice device; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDevice(nsIPresentationDevice **aDevice) = 0;

  /* readonly attribute AString presentationId; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPresentationId(nsAString& aPresentationId) = 0;

  /* readonly attribute nsIPresentationControlChannel controlChannel; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetControlChannel(nsIPresentationControlChannel **aControlChannel) = 0;

  /* readonly attribute boolean isFromReceiver; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsFromReceiver(bool *aIsFromReceiver) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPresentationTerminateRequest, NS_IPRESENTATIONTERMINATEREQUEST_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRESENTATIONTERMINATEREQUEST \
  NS_IMETHOD GetDevice(nsIPresentationDevice **aDevice) override; \
  NS_IMETHOD GetPresentationId(nsAString& aPresentationId) override; \
  NS_IMETHOD GetControlChannel(nsIPresentationControlChannel **aControlChannel) override; \
  NS_IMETHOD GetIsFromReceiver(bool *aIsFromReceiver) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRESENTATIONTERMINATEREQUEST \
  nsresult GetDevice(nsIPresentationDevice **aDevice); \
  nsresult GetPresentationId(nsAString& aPresentationId); \
  nsresult GetControlChannel(nsIPresentationControlChannel **aControlChannel); \
  nsresult GetIsFromReceiver(bool *aIsFromReceiver); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRESENTATIONTERMINATEREQUEST(_to) \
  NS_IMETHOD GetDevice(nsIPresentationDevice **aDevice) override { return _to GetDevice(aDevice); } \
  NS_IMETHOD GetPresentationId(nsAString& aPresentationId) override { return _to GetPresentationId(aPresentationId); } \
  NS_IMETHOD GetControlChannel(nsIPresentationControlChannel **aControlChannel) override { return _to GetControlChannel(aControlChannel); } \
  NS_IMETHOD GetIsFromReceiver(bool *aIsFromReceiver) override { return _to GetIsFromReceiver(aIsFromReceiver); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRESENTATIONTERMINATEREQUEST(_to) \
  NS_IMETHOD GetDevice(nsIPresentationDevice **aDevice) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDevice(aDevice); } \
  NS_IMETHOD GetPresentationId(nsAString& aPresentationId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPresentationId(aPresentationId); } \
  NS_IMETHOD GetControlChannel(nsIPresentationControlChannel **aControlChannel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetControlChannel(aControlChannel); } \
  NS_IMETHOD GetIsFromReceiver(bool *aIsFromReceiver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsFromReceiver(aIsFromReceiver); } 


#endif /* __gen_nsIPresentationTerminateRequest_h__ */
