/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIAuthPromptAdapterFactory.idl
 */

#ifndef __gen_nsIAuthPromptAdapterFactory_h__
#define __gen_nsIAuthPromptAdapterFactory_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIAuthPrompt; /* forward declaration */

class nsIAuthPrompt2; /* forward declaration */


/* starting interface:    nsIAuthPromptAdapterFactory */
#define NS_IAUTHPROMPTADAPTERFACTORY_IID_STR "60e46383-bb9a-4860-8962-80d9c5c05ddc"

#define NS_IAUTHPROMPTADAPTERFACTORY_IID \
  {0x60e46383, 0xbb9a, 0x4860, \
    { 0x89, 0x62, 0x80, 0xd9, 0xc5, 0xc0, 0x5d, 0xdc }}

class NS_NO_VTABLE nsIAuthPromptAdapterFactory : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAUTHPROMPTADAPTERFACTORY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAuthPromptAdapterFactory;

  /* nsIAuthPrompt2 createAdapter (in nsIAuthPrompt aPrompt); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CreateAdapter(nsIAuthPrompt *aPrompt, nsIAuthPrompt2 **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAuthPromptAdapterFactory, NS_IAUTHPROMPTADAPTERFACTORY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAUTHPROMPTADAPTERFACTORY \
  NS_IMETHOD CreateAdapter(nsIAuthPrompt *aPrompt, nsIAuthPrompt2 **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAUTHPROMPTADAPTERFACTORY \
  nsresult CreateAdapter(nsIAuthPrompt *aPrompt, nsIAuthPrompt2 **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAUTHPROMPTADAPTERFACTORY(_to) \
  NS_IMETHOD CreateAdapter(nsIAuthPrompt *aPrompt, nsIAuthPrompt2 **_retval) override { return _to CreateAdapter(aPrompt, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAUTHPROMPTADAPTERFACTORY(_to) \
  NS_IMETHOD CreateAdapter(nsIAuthPrompt *aPrompt, nsIAuthPrompt2 **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateAdapter(aPrompt, _retval); } 


#endif /* __gen_nsIAuthPromptAdapterFactory_h__ */
