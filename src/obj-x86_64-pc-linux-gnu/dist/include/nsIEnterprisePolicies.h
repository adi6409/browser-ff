/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/enterprisepolicies/nsIEnterprisePolicies.idl
 */

#ifndef __gen_nsIEnterprisePolicies_h__
#define __gen_nsIEnterprisePolicies_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIURI_h__
#include "nsIURI.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIEnterprisePolicies */
#define NS_IENTERPRISEPOLICIES_IID_STR "6a568972-cc91-4bf5-963e-3768f3319b8a"

#define NS_IENTERPRISEPOLICIES_IID \
  {0x6a568972, 0xcc91, 0x4bf5, \
    { 0x96, 0x3e, 0x37, 0x68, 0xf3, 0x31, 0x9b, 0x8a }}

class NS_NO_VTABLE nsIEnterprisePolicies : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IENTERPRISEPOLICIES_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIEnterprisePolicies;

  enum {
    UNINITIALIZED = -1,
    INACTIVE = 0,
    ACTIVE = 1,
    FAILED = 2
  };

  /* readonly attribute short status; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetStatus(int16_t *aStatus) = 0;

  /* bool isAllowed (in ACString feature); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsAllowed(const nsACString& feature, bool *_retval) = 0;

  /* jsval getActivePolicies (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetActivePolicies(JS::MutableHandleValue _retval) = 0;

  /* jsval getSupportMenu (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSupportMenu(JS::MutableHandleValue _retval) = 0;

  /* jsval getExtensionPolicy (in ACString extensionID); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetExtensionPolicy(const nsACString& extensionID, JS::MutableHandleValue _retval) = 0;

  /* jsval getExtensionSettings (in ACString extensionID); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetExtensionSettings(const nsACString& extensionID, JS::MutableHandleValue _retval) = 0;

  /* bool mayInstallAddon (in jsval addon); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MayInstallAddon(JS::HandleValue addon, bool *_retval) = 0;

  /* bool allowedInstallSource (in nsIURI uri); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AllowedInstallSource(nsIURI *uri, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIEnterprisePolicies, NS_IENTERPRISEPOLICIES_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIENTERPRISEPOLICIES \
  NS_IMETHOD GetStatus(int16_t *aStatus) override; \
  NS_IMETHOD IsAllowed(const nsACString& feature, bool *_retval) override; \
  NS_IMETHOD GetActivePolicies(JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetSupportMenu(JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetExtensionPolicy(const nsACString& extensionID, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetExtensionSettings(const nsACString& extensionID, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD MayInstallAddon(JS::HandleValue addon, bool *_retval) override; \
  NS_IMETHOD AllowedInstallSource(nsIURI *uri, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIENTERPRISEPOLICIES \
  nsresult GetStatus(int16_t *aStatus); \
  nsresult IsAllowed(const nsACString& feature, bool *_retval); \
  nsresult GetActivePolicies(JS::MutableHandleValue _retval); \
  nsresult GetSupportMenu(JS::MutableHandleValue _retval); \
  nsresult GetExtensionPolicy(const nsACString& extensionID, JS::MutableHandleValue _retval); \
  nsresult GetExtensionSettings(const nsACString& extensionID, JS::MutableHandleValue _retval); \
  nsresult MayInstallAddon(JS::HandleValue addon, bool *_retval); \
  nsresult AllowedInstallSource(nsIURI *uri, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIENTERPRISEPOLICIES(_to) \
  NS_IMETHOD GetStatus(int16_t *aStatus) override { return _to GetStatus(aStatus); } \
  NS_IMETHOD IsAllowed(const nsACString& feature, bool *_retval) override { return _to IsAllowed(feature, _retval); } \
  NS_IMETHOD GetActivePolicies(JS::MutableHandleValue _retval) override { return _to GetActivePolicies(_retval); } \
  NS_IMETHOD GetSupportMenu(JS::MutableHandleValue _retval) override { return _to GetSupportMenu(_retval); } \
  NS_IMETHOD GetExtensionPolicy(const nsACString& extensionID, JS::MutableHandleValue _retval) override { return _to GetExtensionPolicy(extensionID, _retval); } \
  NS_IMETHOD GetExtensionSettings(const nsACString& extensionID, JS::MutableHandleValue _retval) override { return _to GetExtensionSettings(extensionID, _retval); } \
  NS_IMETHOD MayInstallAddon(JS::HandleValue addon, bool *_retval) override { return _to MayInstallAddon(addon, _retval); } \
  NS_IMETHOD AllowedInstallSource(nsIURI *uri, bool *_retval) override { return _to AllowedInstallSource(uri, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIENTERPRISEPOLICIES(_to) \
  NS_IMETHOD GetStatus(int16_t *aStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStatus(aStatus); } \
  NS_IMETHOD IsAllowed(const nsACString& feature, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsAllowed(feature, _retval); } \
  NS_IMETHOD GetActivePolicies(JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetActivePolicies(_retval); } \
  NS_IMETHOD GetSupportMenu(JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSupportMenu(_retval); } \
  NS_IMETHOD GetExtensionPolicy(const nsACString& extensionID, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExtensionPolicy(extensionID, _retval); } \
  NS_IMETHOD GetExtensionSettings(const nsACString& extensionID, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExtensionSettings(extensionID, _retval); } \
  NS_IMETHOD MayInstallAddon(JS::HandleValue addon, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MayInstallAddon(addon, _retval); } \
  NS_IMETHOD AllowedInstallSource(nsIURI *uri, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AllowedInstallSource(uri, _retval); } 


#endif /* __gen_nsIEnterprisePolicies_h__ */
