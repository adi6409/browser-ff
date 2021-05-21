/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/js/xpconnect/idl/xpcIJSGetFactory.idl
 */

#ifndef __gen_xpcIJSGetFactory_h__
#define __gen_xpcIJSGetFactory_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIFactory; /* forward declaration */


/* starting interface:    xpcIJSGetFactory */
#define XPCIJSGETFACTORY_IID_STR "3fe0c205-d75b-4cac-9347-d2b855050143"

#define XPCIJSGETFACTORY_IID \
  {0x3fe0c205, 0xd75b, 0x4cac, \
    { 0x93, 0x47, 0xd2, 0xb8, 0x55, 0x05, 0x01, 0x43 }}

class NS_NO_VTABLE xpcIJSGetFactory : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(XPCIJSGETFACTORY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = xpcIJSGetFactory;

  /* nsIFactory get (in nsCIDRef aCID); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Get(const nsCID & aCID, nsIFactory **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(xpcIJSGetFactory, XPCIJSGETFACTORY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_XPCIJSGETFACTORY \
  NS_IMETHOD Get(const nsCID & aCID, nsIFactory **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_XPCIJSGETFACTORY \
  nsresult Get(const nsCID & aCID, nsIFactory **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_XPCIJSGETFACTORY(_to) \
  NS_IMETHOD Get(const nsCID & aCID, nsIFactory **_retval) override { return _to Get(aCID, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_XPCIJSGETFACTORY(_to) \
  NS_IMETHOD Get(const nsCID & aCID, nsIFactory **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Get(aCID, _retval); } 


#endif /* __gen_xpcIJSGetFactory_h__ */
