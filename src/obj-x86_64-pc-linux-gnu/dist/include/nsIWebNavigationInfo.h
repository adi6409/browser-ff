/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIWebNavigationInfo.idl
 */

#ifndef __gen_nsIWebNavigationInfo_h__
#define __gen_nsIWebNavigationInfo_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIWebNavigation; /* forward declaration */


/* starting interface:    nsIWebNavigationInfo */
#define NS_IWEBNAVIGATIONINFO_IID_STR "62a93afb-93a1-465c-84c8-0432264229de"

#define NS_IWEBNAVIGATIONINFO_IID \
  {0x62a93afb, 0x93a1, 0x465c, \
    { 0x84, 0xc8, 0x04, 0x32, 0x26, 0x42, 0x29, 0xde }}

class NS_NO_VTABLE nsIWebNavigationInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWEBNAVIGATIONINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWebNavigationInfo;

  enum {
    UNSUPPORTED = 0U,
    IMAGE = 1U,
    PLUGIN = 2U,
    OTHER = 32768U
  };

  /* unsigned long isTypeSupported (in ACString aType, in nsIWebNavigation aWebNav); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsTypeSupported(const nsACString& aType, nsIWebNavigation *aWebNav, uint32_t *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWebNavigationInfo, NS_IWEBNAVIGATIONINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWEBNAVIGATIONINFO \
  NS_IMETHOD IsTypeSupported(const nsACString& aType, nsIWebNavigation *aWebNav, uint32_t *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWEBNAVIGATIONINFO \
  nsresult IsTypeSupported(const nsACString& aType, nsIWebNavigation *aWebNav, uint32_t *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWEBNAVIGATIONINFO(_to) \
  NS_IMETHOD IsTypeSupported(const nsACString& aType, nsIWebNavigation *aWebNav, uint32_t *_retval) override { return _to IsTypeSupported(aType, aWebNav, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWEBNAVIGATIONINFO(_to) \
  NS_IMETHOD IsTypeSupported(const nsACString& aType, nsIWebNavigation *aWebNav, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsTypeSupported(aType, aWebNav, _retval); } 


#endif /* __gen_nsIWebNavigationInfo_h__ */
