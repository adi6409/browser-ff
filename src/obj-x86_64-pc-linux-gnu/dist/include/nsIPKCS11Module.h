/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIPKCS11Module.idl
 */

#ifndef __gen_nsIPKCS11Module_h__
#define __gen_nsIPKCS11Module_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPKCS11Slot; /* forward declaration */

class nsISimpleEnumerator; /* forward declaration */


/* starting interface:    nsIPKCS11Module */
#define NS_IPKCS11MODULE_IID_STR "8a44bdf9-d1a5-4734-bd5a-34ed7fe564c2"

#define NS_IPKCS11MODULE_IID \
  {0x8a44bdf9, 0xd1a5, 0x4734, \
    { 0xbd, 0x5a, 0x34, 0xed, 0x7f, 0xe5, 0x64, 0xc2 }}

class NS_NO_VTABLE nsIPKCS11Module : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPKCS11MODULE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPKCS11Module;

  /* [must_use] readonly attribute AUTF8String name; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetName(nsACString& aName) = 0;

  /* [must_use] readonly attribute AUTF8String libName; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetLibName(nsACString& aLibName) = 0;

  /* [must_use] nsISimpleEnumerator listSlots (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD ListSlots(nsISimpleEnumerator **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPKCS11Module, NS_IPKCS11MODULE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPKCS11MODULE \
  [[nodiscard]] NS_IMETHOD GetName(nsACString& aName) override; \
  [[nodiscard]] NS_IMETHOD GetLibName(nsACString& aLibName) override; \
  [[nodiscard]] NS_IMETHOD ListSlots(nsISimpleEnumerator **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPKCS11MODULE \
  [[nodiscard]] nsresult GetName(nsACString& aName); \
  [[nodiscard]] nsresult GetLibName(nsACString& aLibName); \
  [[nodiscard]] nsresult ListSlots(nsISimpleEnumerator **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPKCS11MODULE(_to) \
  [[nodiscard]] NS_IMETHOD GetName(nsACString& aName) override { return _to GetName(aName); } \
  [[nodiscard]] NS_IMETHOD GetLibName(nsACString& aLibName) override { return _to GetLibName(aLibName); } \
  [[nodiscard]] NS_IMETHOD ListSlots(nsISimpleEnumerator **_retval) override { return _to ListSlots(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPKCS11MODULE(_to) \
  [[nodiscard]] NS_IMETHOD GetName(nsACString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetName(aName); } \
  [[nodiscard]] NS_IMETHOD GetLibName(nsACString& aLibName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLibName(aLibName); } \
  [[nodiscard]] NS_IMETHOD ListSlots(nsISimpleEnumerator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ListSlots(_retval); } 


#endif /* __gen_nsIPKCS11Module_h__ */
