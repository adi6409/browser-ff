/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/shell/nsIToolkitShellService.idl
 */

#ifndef __gen_nsIToolkitShellService_h__
#define __gen_nsIToolkitShellService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIToolkitShellService */
#define NS_ITOOLKITSHELLSERVICE_IID_STR "9246cad6-926a-4c17-88b0-ecba8078d143"

#define NS_ITOOLKITSHELLSERVICE_IID \
  {0x9246cad6, 0x926a, 0x4c17, \
    { 0x88, 0xb0, 0xec, 0xba, 0x80, 0x78, 0xd1, 0x43 }}

class NS_NO_VTABLE nsIToolkitShellService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITOOLKITSHELLSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIToolkitShellService;

  /* boolean isDefaultApplication (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsDefaultApplication(bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIToolkitShellService, NS_ITOOLKITSHELLSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITOOLKITSHELLSERVICE \
  NS_IMETHOD IsDefaultApplication(bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITOOLKITSHELLSERVICE \
  nsresult IsDefaultApplication(bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITOOLKITSHELLSERVICE(_to) \
  NS_IMETHOD IsDefaultApplication(bool *_retval) override { return _to IsDefaultApplication(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITOOLKITSHELLSERVICE(_to) \
  NS_IMETHOD IsDefaultApplication(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsDefaultApplication(_retval); } 

#define NS_TOOLKITSHELLSERVICE_CONTRACTID "@mozilla.org/toolkit/shell-service;1"

#endif /* __gen_nsIToolkitShellService_h__ */
