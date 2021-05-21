/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/windowwatcher/nsIPromptFactory.idl
 */

#ifndef __gen_nsIPromptFactory_h__
#define __gen_nsIPromptFactory_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIDOMWindowProxy; /* forward declaration */


/* starting interface:    nsIPromptFactory */
#define NS_IPROMPTFACTORY_IID_STR "2803541c-c96a-4ff1-bd7c-9cb566d46aeb"

#define NS_IPROMPTFACTORY_IID \
  {0x2803541c, 0xc96a, 0x4ff1, \
    { 0xbd, 0x7c, 0x9c, 0xb5, 0x66, 0xd4, 0x6a, 0xeb }}

class NS_NO_VTABLE nsIPromptFactory : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPROMPTFACTORY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPromptFactory;

  /* void getPrompt (in mozIDOMWindowProxy aParent, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPrompt(mozIDOMWindowProxy *aParent, const nsIID & iid, void * * result) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPromptFactory, NS_IPROMPTFACTORY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPROMPTFACTORY \
  NS_IMETHOD GetPrompt(mozIDOMWindowProxy *aParent, const nsIID & iid, void * * result) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPROMPTFACTORY \
  nsresult GetPrompt(mozIDOMWindowProxy *aParent, const nsIID & iid, void * * result); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPROMPTFACTORY(_to) \
  NS_IMETHOD GetPrompt(mozIDOMWindowProxy *aParent, const nsIID & iid, void * * result) override { return _to GetPrompt(aParent, iid, result); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPROMPTFACTORY(_to) \
  NS_IMETHOD GetPrompt(mozIDOMWindowProxy *aParent, const nsIID & iid, void * * result) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrompt(aParent, iid, result); } 


#endif /* __gen_nsIPromptFactory_h__ */
