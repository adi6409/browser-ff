/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsILoadURIDelegate.idl
 */

#ifndef __gen_nsILoadURIDelegate_h__
#define __gen_nsILoadURIDelegate_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsIPrincipal; /* forward declaration */


/* starting interface:    nsILoadURIDelegate */
#define NS_ILOADURIDELEGATE_IID_STR "78e42d37-a34c-4d96-b901-25385669aba4"

#define NS_ILOADURIDELEGATE_IID \
  {0x78e42d37, 0xa34c, 0x4d96, \
    { 0xb9, 0x01, 0x25, 0x38, 0x56, 0x69, 0xab, 0xa4 }}

class NS_NO_VTABLE nsILoadURIDelegate : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILOADURIDELEGATE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsILoadURIDelegate;

  /* boolean loadURI (in nsIURI aURI, in short aWhere, in long aFlags, in nsIPrincipal aTriggeringPrincipal); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD LoadURI(nsIURI *aURI, int16_t aWhere, int32_t aFlags, nsIPrincipal *aTriggeringPrincipal, bool *_retval) = 0;

  /* nsIURI handleLoadError (in nsIURI aURI, in nsresult aError, in short aErrorModule); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleLoadError(nsIURI *aURI, nsresult aError, int16_t aErrorModule, nsIURI **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsILoadURIDelegate, NS_ILOADURIDELEGATE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILOADURIDELEGATE \
  NS_IMETHOD LoadURI(nsIURI *aURI, int16_t aWhere, int32_t aFlags, nsIPrincipal *aTriggeringPrincipal, bool *_retval) override; \
  NS_IMETHOD HandleLoadError(nsIURI *aURI, nsresult aError, int16_t aErrorModule, nsIURI **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILOADURIDELEGATE \
  nsresult LoadURI(nsIURI *aURI, int16_t aWhere, int32_t aFlags, nsIPrincipal *aTriggeringPrincipal, bool *_retval); \
  nsresult HandleLoadError(nsIURI *aURI, nsresult aError, int16_t aErrorModule, nsIURI **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILOADURIDELEGATE(_to) \
  NS_IMETHOD LoadURI(nsIURI *aURI, int16_t aWhere, int32_t aFlags, nsIPrincipal *aTriggeringPrincipal, bool *_retval) override { return _to LoadURI(aURI, aWhere, aFlags, aTriggeringPrincipal, _retval); } \
  NS_IMETHOD HandleLoadError(nsIURI *aURI, nsresult aError, int16_t aErrorModule, nsIURI **_retval) override { return _to HandleLoadError(aURI, aError, aErrorModule, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILOADURIDELEGATE(_to) \
  NS_IMETHOD LoadURI(nsIURI *aURI, int16_t aWhere, int32_t aFlags, nsIPrincipal *aTriggeringPrincipal, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LoadURI(aURI, aWhere, aFlags, aTriggeringPrincipal, _retval); } \
  NS_IMETHOD HandleLoadError(nsIURI *aURI, nsresult aError, int16_t aErrorModule, nsIURI **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleLoadError(aURI, aError, aErrorModule, _retval); } 


#endif /* __gen_nsILoadURIDelegate_h__ */
