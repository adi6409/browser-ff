/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIPKCS11Slot.idl
 */

#ifndef __gen_nsIPKCS11Slot_h__
#define __gen_nsIPKCS11Slot_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPK11Token; /* forward declaration */


/* starting interface:    nsIPKCS11Slot */
#define NS_IPKCS11SLOT_IID_STR "c2d4f296-ee60-11d4-998b-00b0d02354a0"

#define NS_IPKCS11SLOT_IID \
  {0xc2d4f296, 0xee60, 0x11d4, \
    { 0x99, 0x8b, 0x00, 0xb0, 0xd0, 0x23, 0x54, 0xa0 }}

class NS_NO_VTABLE nsIPKCS11Slot : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPKCS11SLOT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPKCS11Slot;

  /* [must_use] readonly attribute AUTF8String name; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetName(nsACString& aName) = 0;

  /* [must_use] readonly attribute AUTF8String desc; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetDesc(nsACString& aDesc) = 0;

  /* [must_use] readonly attribute AUTF8String manID; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetManID(nsACString& aManID) = 0;

  /* [must_use] readonly attribute AUTF8String HWVersion; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetHWVersion(nsACString& aHWVersion) = 0;

  /* [must_use] readonly attribute AUTF8String FWVersion; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetFWVersion(nsACString& aFWVersion) = 0;

  enum {
    SLOT_DISABLED = 0U,
    SLOT_NOT_PRESENT = 1U,
    SLOT_UNINITIALIZED = 2U,
    SLOT_NOT_LOGGED_IN = 3U,
    SLOT_LOGGED_IN = 4U,
    SLOT_READY = 5U
  };

  /* [must_use] readonly attribute unsigned long status; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetStatus(uint32_t *aStatus) = 0;

  /* [must_use] nsIPK11Token getToken (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetToken(nsIPK11Token **_retval) = 0;

  /* [must_use] readonly attribute AUTF8String tokenName; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetTokenName(nsACString& aTokenName) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPKCS11Slot, NS_IPKCS11SLOT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPKCS11SLOT \
  [[nodiscard]] NS_IMETHOD GetName(nsACString& aName) override; \
  [[nodiscard]] NS_IMETHOD GetDesc(nsACString& aDesc) override; \
  [[nodiscard]] NS_IMETHOD GetManID(nsACString& aManID) override; \
  [[nodiscard]] NS_IMETHOD GetHWVersion(nsACString& aHWVersion) override; \
  [[nodiscard]] NS_IMETHOD GetFWVersion(nsACString& aFWVersion) override; \
  [[nodiscard]] NS_IMETHOD GetStatus(uint32_t *aStatus) override; \
  [[nodiscard]] NS_IMETHOD GetToken(nsIPK11Token **_retval) override; \
  [[nodiscard]] NS_IMETHOD GetTokenName(nsACString& aTokenName) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPKCS11SLOT \
  [[nodiscard]] nsresult GetName(nsACString& aName); \
  [[nodiscard]] nsresult GetDesc(nsACString& aDesc); \
  [[nodiscard]] nsresult GetManID(nsACString& aManID); \
  [[nodiscard]] nsresult GetHWVersion(nsACString& aHWVersion); \
  [[nodiscard]] nsresult GetFWVersion(nsACString& aFWVersion); \
  [[nodiscard]] nsresult GetStatus(uint32_t *aStatus); \
  [[nodiscard]] nsresult GetToken(nsIPK11Token **_retval); \
  [[nodiscard]] nsresult GetTokenName(nsACString& aTokenName); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPKCS11SLOT(_to) \
  [[nodiscard]] NS_IMETHOD GetName(nsACString& aName) override { return _to GetName(aName); } \
  [[nodiscard]] NS_IMETHOD GetDesc(nsACString& aDesc) override { return _to GetDesc(aDesc); } \
  [[nodiscard]] NS_IMETHOD GetManID(nsACString& aManID) override { return _to GetManID(aManID); } \
  [[nodiscard]] NS_IMETHOD GetHWVersion(nsACString& aHWVersion) override { return _to GetHWVersion(aHWVersion); } \
  [[nodiscard]] NS_IMETHOD GetFWVersion(nsACString& aFWVersion) override { return _to GetFWVersion(aFWVersion); } \
  [[nodiscard]] NS_IMETHOD GetStatus(uint32_t *aStatus) override { return _to GetStatus(aStatus); } \
  [[nodiscard]] NS_IMETHOD GetToken(nsIPK11Token **_retval) override { return _to GetToken(_retval); } \
  [[nodiscard]] NS_IMETHOD GetTokenName(nsACString& aTokenName) override { return _to GetTokenName(aTokenName); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPKCS11SLOT(_to) \
  [[nodiscard]] NS_IMETHOD GetName(nsACString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetName(aName); } \
  [[nodiscard]] NS_IMETHOD GetDesc(nsACString& aDesc) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDesc(aDesc); } \
  [[nodiscard]] NS_IMETHOD GetManID(nsACString& aManID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetManID(aManID); } \
  [[nodiscard]] NS_IMETHOD GetHWVersion(nsACString& aHWVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHWVersion(aHWVersion); } \
  [[nodiscard]] NS_IMETHOD GetFWVersion(nsACString& aFWVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFWVersion(aFWVersion); } \
  [[nodiscard]] NS_IMETHOD GetStatus(uint32_t *aStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStatus(aStatus); } \
  [[nodiscard]] NS_IMETHOD GetToken(nsIPK11Token **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetToken(_retval); } \
  [[nodiscard]] NS_IMETHOD GetTokenName(nsACString& aTokenName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTokenName(aTokenName); } 


#endif /* __gen_nsIPKCS11Slot_h__ */
