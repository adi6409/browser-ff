/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIGfxInfoDebug.idl
 */

#ifndef __gen_nsIGfxInfoDebug_h__
#define __gen_nsIGfxInfoDebug_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIGfxInfoDebug */
#define NS_IGFXINFODEBUG_IID_STR "ca7b0bc7-c67c-4b79-8270-ed7ba002af08"

#define NS_IGFXINFODEBUG_IID \
  {0xca7b0bc7, 0xc67c, 0x4b79, \
    { 0x82, 0x70, 0xed, 0x7b, 0xa0, 0x02, 0xaf, 0x08 }}

class NS_NO_VTABLE nsIGfxInfoDebug : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IGFXINFODEBUG_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIGfxInfoDebug;

  /* void spoofVendorID (in AString aVendorID); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SpoofVendorID(const nsAString& aVendorID) = 0;

  /* void spoofDeviceID (in AString aDeviceID); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SpoofDeviceID(const nsAString& aDeviceID) = 0;

  /* void spoofDriverVersion (in AString aDriverVersion); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SpoofDriverVersion(const nsAString& aDriverVersion) = 0;

  /* void spoofOSVersion (in unsigned long aVersion); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SpoofOSVersion(uint32_t aVersion) = 0;

  /* void fireTestProcess (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FireTestProcess(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIGfxInfoDebug, NS_IGFXINFODEBUG_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIGFXINFODEBUG \
  NS_IMETHOD SpoofVendorID(const nsAString& aVendorID) override; \
  NS_IMETHOD SpoofDeviceID(const nsAString& aDeviceID) override; \
  NS_IMETHOD SpoofDriverVersion(const nsAString& aDriverVersion) override; \
  NS_IMETHOD SpoofOSVersion(uint32_t aVersion) override; \
  NS_IMETHOD FireTestProcess(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIGFXINFODEBUG \
  nsresult SpoofVendorID(const nsAString& aVendorID); \
  nsresult SpoofDeviceID(const nsAString& aDeviceID); \
  nsresult SpoofDriverVersion(const nsAString& aDriverVersion); \
  nsresult SpoofOSVersion(uint32_t aVersion); \
  nsresult FireTestProcess(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIGFXINFODEBUG(_to) \
  NS_IMETHOD SpoofVendorID(const nsAString& aVendorID) override { return _to SpoofVendorID(aVendorID); } \
  NS_IMETHOD SpoofDeviceID(const nsAString& aDeviceID) override { return _to SpoofDeviceID(aDeviceID); } \
  NS_IMETHOD SpoofDriverVersion(const nsAString& aDriverVersion) override { return _to SpoofDriverVersion(aDriverVersion); } \
  NS_IMETHOD SpoofOSVersion(uint32_t aVersion) override { return _to SpoofOSVersion(aVersion); } \
  NS_IMETHOD FireTestProcess(void) override { return _to FireTestProcess(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIGFXINFODEBUG(_to) \
  NS_IMETHOD SpoofVendorID(const nsAString& aVendorID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SpoofVendorID(aVendorID); } \
  NS_IMETHOD SpoofDeviceID(const nsAString& aDeviceID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SpoofDeviceID(aDeviceID); } \
  NS_IMETHOD SpoofDriverVersion(const nsAString& aDriverVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SpoofDriverVersion(aDriverVersion); } \
  NS_IMETHOD SpoofOSVersion(uint32_t aVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SpoofOSVersion(aVersion); } \
  NS_IMETHOD FireTestProcess(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FireTestProcess(); } 


#endif /* __gen_nsIGfxInfoDebug_h__ */
